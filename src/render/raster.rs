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
use image::{DynamicImage, ImageBuffer, ImageFormat, Rgba, RgbaImage};
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
