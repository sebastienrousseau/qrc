// Copyright © 2022-2026 QR Code Library (QRC). All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! The rendering layer: turn a [`Matrix`](crate::matrix::Matrix) into output.
//!
//! Renderers are independent of the encoding backend. Three are provided:
//!
//! - [`svg`] — resolution-independent, styleable, the print/branding default.
//! - [`raster`] — RGBA images plus real PNG/JPEG/GIF byte encoders (`raster` feature).
//! - [`unicode`] — terminal output via half-block characters.

pub mod style;
pub mod svg;
pub mod unicode;

#[cfg(feature = "raster")]
pub mod raster;

pub use style::{Color, ModuleShape};
