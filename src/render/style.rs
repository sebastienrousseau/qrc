// Copyright © 2022-2026 QR Code Library (QRC). All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Shared styling primitives for renderers.

/// An 8-bit-per-channel RGBA color.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Color(pub [u8; 4]);

impl Color {
    /// Opaque black — the default dark-module color.
    pub const BLACK: Color = Color([0, 0, 0, 255]);
    /// Opaque white — the default light-module/background color.
    pub const WHITE: Color = Color([255, 255, 255, 255]);

    /// Creates an opaque color from RGB components.
    #[must_use]
    pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Color([r, g, b, 255])
    }

    /// Creates a color from RGBA components.
    #[must_use]
    pub const fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Color([r, g, b, a])
    }

    /// The RGBA byte array.
    #[must_use]
    pub const fn to_array(self) -> [u8; 4] {
        self.0
    }

    /// `#RRGGBB` hex for SVG. The alpha channel is exposed separately via
    /// [`Color::opacity`] because SVG carries opacity as its own attribute.
    #[must_use]
    pub fn to_hex(self) -> String {
        let [r, g, b, _] = self.0;
        format!("#{r:02X}{g:02X}{b:02X}")
    }

    /// Alpha as an SVG opacity value in `0.0..=1.0`.
    #[must_use]
    pub fn opacity(self) -> f32 {
        f32::from(self.0[3]) / 255.0
    }
}

impl Default for Color {
    fn default() -> Self {
        Color::BLACK
    }
}

/// The shape used to draw each dark module.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum ModuleShape {
    /// Solid squares — the classic, most-compatible style.
    #[default]
    Square,
    /// Squares with rounded corners.
    Rounded,
    /// Circular dots.
    Circle,
}
