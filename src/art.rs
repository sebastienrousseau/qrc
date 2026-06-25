// Copyright © 2022-2026 QR Code Library (QRC). All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Offline "art QR" primitives.
//!
//! Two model-free building blocks for artistic QR codes:
//!
//! * [`QRCode::to_control_image`](crate::QRCode::to_control_image) exports a
//!   clean, high-contrast, square control image — the input a Stable Diffusion
//!   QR ControlNet (e.g. *QR Code Monster*) expects.
//! * [`QRCode::blend_image`](crate::QRCode::blend_image) weaves a supplied
//!   background image through the code's data modules while keeping finder
//!   patterns, the quiet zone, and a centre dot in every module solid, so the
//!   result stays scannable without any model at all.
//!
//! Pair both with the highest error-correction level
//! ([`with_ec_level`](crate::QRCode::with_ec_level) + `EcLevel::H`) so the
//! blended regions stay recoverable.

use image::{imageops, ImageBuffer, Rgba, RgbaImage};
use qrcode::{Color, QrCode};

/// Quiet-zone width, in modules (the QR specification mandates 4).
const QUIET: u32 = 4;

const BLACK: [u8; 4] = [0, 0, 0, 255];
const WHITE: [u8; 4] = [255, 255, 255, 255];

/// Tuning for [`QRCode::blend_image`](crate::QRCode::blend_image).
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
}

impl Default for BlendOptions {
    fn default() -> Self {
        BlendOptions {
            module_size: 12,
            strength: 0.75,
            dot_ratio: 0.66,
        }
    }
}

/// Renders `code` to a square, centred, high-contrast control image with a
/// generous quiet zone. The modules are integer-scaled to fill `size` as
/// closely as possible; if they cannot fit, the canvas grows to the next whole
/// module rather than distorting them.
pub(crate) fn control_image(code: &QrCode, size: u32) -> RgbaImage {
    let n = code.width() as u32;
    let total = n + 2 * QUIET;
    let module_px = (size / total).max(1);
    let qr_dim = module_px * total;
    let dim = size.max(qr_dim);
    let offset = (dim - qr_dim) / 2;

    let mut img: RgbaImage = ImageBuffer::from_pixel(dim, dim, Rgba(WHITE));
    for y in 0..n {
        for x in 0..n {
            if code[(x as usize, y as usize)] != Color::Dark {
                continue;
            }
            let px0 = offset + (x + QUIET) * module_px;
            let py0 = offset + (y + QUIET) * module_px;
            for dy in 0..module_px {
                for dx in 0..module_px {
                    img.put_pixel(px0 + dx, py0 + dy, Rgba(BLACK));
                }
            }
        }
    }
    img
}

/// Weaves `background` into `code`, returning a branded, scannable image. The
/// background is resized to the output dimensions; an empty background is
/// treated as a blank light canvas.
pub(crate) fn blend(code: &QrCode, background: &RgbaImage, opts: &BlendOptions) -> RgbaImage {
    let n = code.width() as u32;
    let total = n + 2 * QUIET;
    let m = opts.module_size.max(1);
    let dim = total * m;

    let bg: RgbaImage = if background.width() == 0 || background.height() == 0 {
        ImageBuffer::from_pixel(dim, dim, Rgba(WHITE))
    } else {
        imageops::resize(background, dim, dim, imageops::FilterType::Lanczos3)
    };

    let mut out: RgbaImage = ImageBuffer::new(dim, dim);
    let center = (m as f32 - 1.0) / 2.0;
    let dot_r = m as f32 * opts.dot_ratio.clamp(0.0, 1.0) / 2.0;

    for my in 0..total {
        for mx in 0..total {
            let is_quiet = mx < QUIET || my < QUIET || mx >= QUIET + n || my >= QUIET + n;
            let (dx_mod, dy_mod) = (mx.wrapping_sub(QUIET), my.wrapping_sub(QUIET));
            let dark_module = !is_quiet && code[(dx_mod as usize, dy_mod as usize)] == Color::Dark;
            let finder = !is_quiet && in_finder(dx_mod as usize, dy_mod as usize, n as usize);
            let tint = if dark_module { BLACK } else { WHITE };

            for dy in 0..m {
                for dx in 0..m {
                    let (px, py) = (mx * m + dx, my * m + dy);
                    let pixel = if is_quiet || finder {
                        Rgba(tint)
                    } else {
                        let ddx = dx as f32 - center;
                        let ddy = dy as f32 - center;
                        if (ddx * ddx + ddy * ddy).sqrt() <= dot_r {
                            Rgba(tint) // solid centre dot
                        } else {
                            mix(*bg.get_pixel(px, py), tint, opts.strength)
                        }
                    };
                    out.put_pixel(px, py, pixel);
                }
            }
        }
    }
    out
}

/// Whether data coordinate `(x, y)` lies in one of the three 7×7 finder
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
