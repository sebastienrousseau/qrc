// Copyright © 2022-2026 QR Code Library (QRC). All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Offline "artistic" QR codes that blend a supplied image into the symbol.
//!
//! This is a deterministic, model-free alternative to AI art-QR pipelines: a
//! background image (logo, photo, pattern) shows through the code while a
//! centred dot in every module plus solid finder patterns preserve the
//! contrast a scanner needs. Use [`Ecc::High`] so the blended regions stay
//! recoverable.
//!
//! For the AI-generated look (Stable Diffusion + a QR ControlNet) export a
//! control image with [`crate::render::control`] and feed it to a model.
//!
//! [`Ecc::High`]: crate::encode::Ecc::High

use crate::matrix::Matrix;
use crate::render::style::Color;
use image::{imageops, ImageBuffer, Rgba, RgbaImage};

/// Options for [`blend`].
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BlendOptions {
    /// Pixels per module in the output image.
    pub module_size: u32,
    /// How strongly each module tints the background outside its centre dot,
    /// in `0.0..=1.0` (0 = image only, 1 = solid module). Higher scans more
    /// reliably; lower shows more of the image.
    pub strength: f32,
    /// Diameter of the always-solid centre dot relative to the module, in
    /// `0.0..=1.0`. The dots carry most of the scannable signal.
    pub dot_ratio: f32,
    /// Dark-module tint color.
    pub dark: Color,
    /// Light-module tint color.
    pub light: Color,
}

impl Default for BlendOptions {
    fn default() -> Self {
        BlendOptions {
            module_size: 12,
            strength: 0.75,
            dot_ratio: 0.66,
            dark: Color::BLACK,
            light: Color::WHITE,
        }
    }
}

/// Returns whether data-coordinate `(x, y)` lies in one of the three 7×7 finder
/// patterns, which are always rendered solidly so detection stays robust.
fn in_finder(x: usize, y: usize, size: usize) -> bool {
    const F: usize = 7;
    let (left, right) = (x < F, x >= size - F);
    let (top, bottom) = (y < F, y >= size - F);
    (top && (left || right)) || (bottom && left)
}

/// Linearly blends `bg` toward `tint` by `strength`.
fn mix(bg: Rgba<u8>, tint: [u8; 4], strength: f32) -> Rgba<u8> {
    let s = strength.clamp(0.0, 1.0);
    let ch = |b: u8, t: u8| ((1.0 - s) * f32::from(b) + s * f32::from(t)) as u8;
    Rgba([
        ch(bg[0], tint[0]),
        ch(bg[1], tint[1]),
        ch(bg[2], tint[2]),
        255,
    ])
}

/// Weaves `background` into the QR `matrix`, returning a branded, scannable
/// image. The background is resized to the output dimensions; an empty
/// background is treated as a blank light canvas.
#[must_use]
pub fn blend(matrix: &Matrix, background: &RgbaImage, opts: &BlendOptions) -> RgbaImage {
    let total = matrix.total_size() as u32;
    let m = opts.module_size.max(1);
    let dim = total * m;
    let qz = matrix.quiet_zone() as u32;
    let size = matrix.size() as u32;

    let dark = opts.dark.to_array();
    let light = opts.light.to_array();

    // Resize the background to fill the code (or use a light canvas if empty).
    let bg: RgbaImage = if background.width() == 0 || background.height() == 0 {
        ImageBuffer::from_pixel(dim, dim, Rgba(light))
    } else {
        imageops::resize(background, dim, dim, imageops::FilterType::Lanczos3)
    };

    let mut out: RgbaImage = ImageBuffer::new(dim, dim);
    let center = (m as f32 - 1.0) / 2.0;
    let dot_r = m as f32 * opts.dot_ratio.clamp(0.0, 1.0) / 2.0;

    for my in 0..total {
        for mx in 0..total {
            // Quiet-zone modules are solid light so the border stays clean.
            let (dark_module, is_quiet) =
                if mx < qz || my < qz || mx >= qz + size || my >= qz + size {
                    (false, true)
                } else {
                    (
                        matrix.is_dark((mx - qz) as usize, (my - qz) as usize),
                        false,
                    )
                };
            let finder =
                !is_quiet && in_finder((mx - qz) as usize, (my - qz) as usize, size as usize);
            let tint = if dark_module { dark } else { light };

            for dy in 0..m {
                for dx in 0..m {
                    let px = mx * m + dx;
                    let py = my * m + dy;
                    let bg_px = *bg.get_pixel(px, py);

                    let pixel = if is_quiet || finder {
                        // Solid — required for a clean quiet zone / detection.
                        Rgba(tint)
                    } else {
                        let ddx = dx as f32 - center;
                        let ddy = dy as f32 - center;
                        if (ddx * ddx + ddy * ddy).sqrt() <= dot_r {
                            Rgba(tint) // solid centre dot
                        } else {
                            mix(bg_px, tint, opts.strength) // image shows through
                        }
                    };
                    out.put_pixel(px, py, pixel);
                }
            }
        }
    }
    out
}
