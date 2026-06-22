// Copyright © 2022-2026 QR Code Library (QRC). All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! A backend-agnostic QR module matrix.
//!
//! [`Matrix`] is the boundary between the [encoding](crate::encode) layer and
//! the [rendering](crate::render) layer. Engines produce a `Matrix`; renderers
//! consume one. This keeps renderers independent of any particular QR backend.

/// A grid of QR modules (`true` = dark, `false` = light) plus the surrounding
/// quiet zone that every renderer must honour.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Matrix {
    size: usize,
    quiet_zone: usize,
    /// Row-major, `size * size` entries. `true` is a dark module.
    modules: Vec<bool>,
}

impl Matrix {
    /// Creates a matrix from a row-major slice of `size * size` modules.
    ///
    /// # Panics
    ///
    /// Panics if `modules.len() != size * size`.
    #[must_use]
    pub fn new(size: usize, quiet_zone: usize, modules: Vec<bool>) -> Self {
        assert_eq!(
            modules.len(),
            size * size,
            "module buffer does not match the declared size"
        );
        Matrix {
            size,
            quiet_zone,
            modules,
        }
    }

    /// The number of modules along one side, excluding the quiet zone.
    #[must_use]
    pub fn size(&self) -> usize {
        self.size
    }

    /// The quiet-zone width in modules (applied on every side).
    #[must_use]
    pub fn quiet_zone(&self) -> usize {
        self.quiet_zone
    }

    /// The side length in modules including the quiet zone on both sides.
    #[must_use]
    pub fn total_size(&self) -> usize {
        self.size + 2 * self.quiet_zone
    }

    /// Returns whether the module at data coordinates `(x, y)` is dark.
    /// Coordinates are relative to the symbol, excluding the quiet zone, and
    /// out-of-range coordinates are treated as light (part of the quiet zone).
    #[must_use]
    pub fn is_dark(&self, x: usize, y: usize) -> bool {
        if x >= self.size || y >= self.size {
            return false;
        }
        self.modules[y * self.size + x]
    }

    /// Returns whether the module at coordinates `(x, y)` *including the quiet
    /// zone* is dark. `(0, 0)` is the top-left corner of the quiet zone.
    #[must_use]
    pub fn is_dark_with_quiet_zone(&self, x: usize, y: usize) -> bool {
        if x < self.quiet_zone || y < self.quiet_zone {
            return false;
        }
        self.is_dark(x - self.quiet_zone, y - self.quiet_zone)
    }
}
