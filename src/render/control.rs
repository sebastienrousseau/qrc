// Copyright © 2022-2026 QR Code Library (QRC). All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! ControlNet-ready control-image export.
//!
//! AI "art QR" pipelines (Stable Diffusion + a QR ControlNet such as *QR Code
//! Monster*) need a clean, high-contrast control image at a fixed square size
//! (typically 512 or 768). This renderer produces exactly that: pure two-tone
//! modules, integer-scaled and centred on a square canvas of the requested
//! size, with a generous quiet zone. Pair it with [`Ecc::High`] so the model
//! has the most redundancy to hide art behind.
//!
//! ```
//! use qrc::encode::{Ecc, QrOptions};
//! use qrc::render::control::ControlOptions;
//! use qrc::QRCode;
//!
//! let qr = QRCode::from_string("https://example.com".to_string());
//! let img = qr
//!     .to_control_image(&QrOptions::new().ecc(Ecc::High), &ControlOptions::default())
//!     .unwrap();
//! assert_eq!(img.dimensions(), (768, 768)); // exact, square, model-ready
//! ```
//!
//! [`Ecc::High`]: crate::encode::Ecc::High

use crate::matrix::Matrix;
use crate::render::style::Color;
use image::{ImageBuffer, Rgba, RgbaImage};

/// Options for [`render`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ControlOptions {
    /// Side length of the (square) output image in pixels. The modules are
    /// integer-scaled to fill it as closely as possible and centred; if the
    /// modules cannot fit, the canvas grows to the next whole module instead
    /// of distorting them.
    pub size: u32,
    /// Dark-module color (default opaque black — best for ControlNet).
    pub dark: Color,
    /// Light/background color (default opaque white).
    pub light: Color,
}

impl Default for ControlOptions {
    fn default() -> Self {
        ControlOptions {
            size: 768,
            dark: Color::BLACK,
            light: Color::WHITE,
        }
    }
}

impl ControlOptions {
    /// Control options targeting a specific square pixel size.
    #[must_use]
    pub fn with_size(size: u32) -> Self {
        ControlOptions {
            size,
            ..Self::default()
        }
    }
}

/// Renders `matrix` to a square, centred, high-contrast control image.
#[must_use]
pub fn render(matrix: &Matrix, opts: &ControlOptions) -> RgbaImage {
    let total = matrix.total_size() as u32;
    let module_px = (opts.size / total).max(1);
    let qr_dim = module_px * total;
    // Never distort modules: if they cannot fit the requested size, grow the
    // canvas to the next whole module instead.
    let dim = opts.size.max(qr_dim);
    let offset = (dim - qr_dim) / 2;
    let qz = matrix.quiet_zone() as u32;

    let light = Rgba(opts.light.to_array());
    let dark = Rgba(opts.dark.to_array());
    let mut img: RgbaImage = ImageBuffer::from_pixel(dim, dim, light);

    for y in 0..matrix.size() {
        for x in 0..matrix.size() {
            if !matrix.is_dark(x, y) {
                continue;
            }
            let px0 = offset + (x as u32 + qz) * module_px;
            let py0 = offset + (y as u32 + qz) * module_px;
            for dy in 0..module_px {
                for dx in 0..module_px {
                    img.put_pixel(px0 + dx, py0 + dy, dark);
                }
            }
        }
    }
    img
}
