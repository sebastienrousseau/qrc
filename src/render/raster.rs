// Copyright © 2022-2026 QR Code Library (QRC). All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Raster rendering and real image-format byte encoders.
//!
//! Unlike the historic API, these functions perform integer module scaling
//! (every module is identical), always emit the quiet zone, draw dark modules
//! as opaque colors, and the `*_bytes` helpers actually encode the requested
//! format rather than returning a raw buffer.

use crate::error::{QrError, Result};
use crate::matrix::Matrix;
use crate::render::style::Color;
use image::{imageops, DynamicImage, ImageBuffer, ImageFormat, Rgba, RgbaImage};
use std::io::Cursor;

/// Options for raster rendering.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct RasterOptions {
    /// Size of a single module in pixels.
    pub module_size: u32,
    /// Dark-module color.
    pub dark: Color,
    /// Light/background color.
    pub light: Color,
}

impl Default for RasterOptions {
    fn default() -> Self {
        RasterOptions {
            module_size: 8,
            dark: Color::BLACK,
            light: Color::WHITE,
        }
    }
}

impl RasterOptions {
    /// Picks the largest module size that keeps the rendered image at or below
    /// `target_width` pixels (minimum 1px/module), so output never exceeds the
    /// requested size and every module stays identical.
    #[must_use]
    pub fn fit_width(matrix: &Matrix, target_width: u32) -> Self {
        let total = matrix.total_size() as u32;
        let module_size = (target_width / total.max(1)).max(1);
        RasterOptions {
            module_size,
            ..Self::default()
        }
    }
}

/// Renders `matrix` to an opaque, quiet-zoned, integer-scaled RGBA image.
#[must_use]
pub fn render(matrix: &Matrix, opts: &RasterOptions) -> RgbaImage {
    let m = opts.module_size.max(1);
    let total = matrix.total_size() as u32;
    let dim = total * m;
    let qz = matrix.quiet_zone() as u32;

    let light = Rgba(opts.light.to_array());
    let dark = Rgba(opts.dark.to_array());
    let mut img: RgbaImage = ImageBuffer::from_pixel(dim, dim, light);

    for y in 0..matrix.size() {
        for x in 0..matrix.size() {
            if !matrix.is_dark(x, y) {
                continue;
            }
            let px0 = (x as u32 + qz) * m;
            let py0 = (y as u32 + qz) * m;
            for dy in 0..m {
                for dx in 0..m {
                    img.put_pixel(px0 + dx, py0 + dy, dark);
                }
            }
        }
    }
    img
}

/// Encodes an RGBA image to `format` bytes, flattening alpha for JPEG and
/// mapping any image-library failure to [`QrError::Render`].
fn encode(img: &RgbaImage, format: ImageFormat) -> Result<Vec<u8>> {
    let mut buf = Vec::new();
    let dynimg = DynamicImage::ImageRgba8(img.clone());
    // JPEG has no alpha channel; flatten to RGB first.
    let result = if format == ImageFormat::Jpeg {
        dynimg
            .to_rgb8()
            .write_to(&mut Cursor::new(&mut buf), format)
    } else {
        dynimg.write_to(&mut Cursor::new(&mut buf), format)
    };
    result.map_err(|_| QrError::Render("image encoding failed"))?;
    Ok(buf)
}

/// Renders `matrix` and encodes it as bytes in an arbitrary [`ImageFormat`]
/// supported by the `image` crate (e.g. PNG, JPEG, GIF, BMP, TIFF, WebP).
///
/// The dedicated [`to_png_bytes`]/[`to_jpeg_bytes`]/[`to_gif_bytes`] helpers
/// cover the common cases; use this when you need another format.
///
/// # Errors
///
/// Returns [`QrError::Render`] if the requested format has no encoder or
/// encoding otherwise fails.
pub fn to_bytes(matrix: &Matrix, opts: &RasterOptions, format: ImageFormat) -> Result<Vec<u8>> {
    encode(&render(matrix, opts), format)
}

/// Encodes `matrix` as PNG bytes.
///
/// # Errors
/// Returns [`QrError::Render`] if encoding fails.
pub fn to_png_bytes(matrix: &Matrix, opts: &RasterOptions) -> Result<Vec<u8>> {
    to_bytes(matrix, opts, ImageFormat::Png)
}

/// Encodes `matrix` as JPEG bytes (alpha is flattened).
///
/// # Errors
/// Returns [`QrError::Render`] if encoding fails.
pub fn to_jpeg_bytes(matrix: &Matrix, opts: &RasterOptions) -> Result<Vec<u8>> {
    to_bytes(matrix, opts, ImageFormat::Jpeg)
}

/// Encodes `matrix` as GIF bytes.
///
/// # Errors
/// Returns [`QrError::Render`] if encoding fails.
pub fn to_gif_bytes(matrix: &Matrix, opts: &RasterOptions) -> Result<Vec<u8>> {
    to_bytes(matrix, opts, ImageFormat::Gif)
}

/// Encodes an already-rendered RGBA image (e.g. one with an embedded logo) to
/// `format` bytes.
///
/// # Errors
/// Returns [`QrError::Render`] if the format has no encoder or encoding fails.
pub fn image_to_bytes(img: &RgbaImage, format: ImageFormat) -> Result<Vec<u8>> {
    encode(img, format)
}

/// Controls how a logo is embedded into the centre of a QR image to produce a
/// branded code (e.g. a business card). Always pair with [`Ecc::High`] error
/// correction so the obscured modules remain recoverable.
///
/// [`Ecc::High`]: crate::encode::Ecc::High
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LogoOptions {
    /// Fraction of the QR width the logo should occupy (clamped to `0.0..=1.0`).
    /// Keep at or below ~0.25 so a level-H code (≈30% recovery) still scans.
    pub size_ratio: f32,
    /// Pixels of background padding drawn around the logo (a "knockout" that
    /// clears nearby modules and improves scannability).
    pub padding: u32,
    /// Fill color of the padding/knockout behind the logo. `None` composites
    /// the logo directly onto the modules with no knockout.
    pub background: Option<Color>,
}

impl Default for LogoOptions {
    fn default() -> Self {
        LogoOptions {
            size_ratio: 0.22,
            padding: 6,
            background: Some(Color::WHITE),
        }
    }
}

/// Embeds `logo` into the centre of `img` in place, according to `opts`.
///
/// The logo is scaled to fit `size_ratio` of the QR width (preserving aspect),
/// an optional padded knockout is drawn behind it, and it is alpha-composited
/// on top. Empty logos are ignored.
pub fn embed_logo(img: &mut RgbaImage, logo: &RgbaImage, opts: &LogoOptions) {
    let (w, h) = img.dimensions();
    let (lw, lh) = logo.dimensions();
    if lw == 0 || lh == 0 {
        return;
    }

    // Target size, aspect-preserving, fitting within `size_ratio` of the QR.
    let qr_dim = w.min(h);
    let target = ((qr_dim as f32 * opts.size_ratio.clamp(0.0, 1.0)) as u32).max(1);
    let scale = target as f32 / lw.max(lh) as f32;
    let nw = ((lw as f32 * scale) as u32).max(1).min(w);
    let nh = ((lh as f32 * scale) as u32).max(1).min(h);
    let resized = imageops::resize(logo, nw, nh, imageops::FilterType::Lanczos3);

    let (cx, cy) = (w / 2, h / 2);

    // Optional padded knockout behind the logo.
    if let Some(bg) = opts.background {
        let pad_w = (nw + 2 * opts.padding).min(w);
        let pad_h = (nh + 2 * opts.padding).min(h);
        let x0 = cx.saturating_sub(pad_w / 2);
        let y0 = cy.saturating_sub(pad_h / 2);
        let bg_px = Rgba(bg.to_array());
        for y in y0..(y0 + pad_h).min(h) {
            for x in x0..(x0 + pad_w).min(w) {
                img.put_pixel(x, y, bg_px);
            }
        }
    }

    // Alpha-composite the logo, bounded so it never writes out of range.
    let lx0 = cx.saturating_sub(nw / 2);
    let ly0 = cy.saturating_sub(nh / 2);
    for dy in 0..nh.min(h - ly0) {
        for dx in 0..nw.min(w - lx0) {
            let src = resized.get_pixel(dx, dy);
            let alpha = f32::from(src[3]) / 255.0;
            let base = img.get_pixel(lx0 + dx, ly0 + dy);
            let mix = |b: u8, s: u8| ((1.0 - alpha) * f32::from(b) + alpha * f32::from(s)) as u8;
            img.put_pixel(
                lx0 + dx,
                ly0 + dy,
                Rgba([
                    mix(base[0], src[0]),
                    mix(base[1], src[1]),
                    mix(base[2], src[2]),
                    255,
                ]),
            );
        }
    }
}

/// Renders `matrix` and embeds `logo` at its centre, returning a branded image.
#[must_use]
pub fn render_with_logo(
    matrix: &Matrix,
    opts: &RasterOptions,
    logo: &RgbaImage,
    logo_opts: &LogoOptions,
) -> RgbaImage {
    let mut img = render(matrix, opts);
    embed_logo(&mut img, logo, logo_opts);
    img
}
