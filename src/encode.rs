// Copyright © 2022-2026 QR Code Library (QRC). All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! The encoding layer: turn bytes into a [`Matrix`].
//!
//! Encoding is abstracted behind the [`Engine`] trait so the backend can be
//! swapped (e.g. for a faster encoder, or one supporting Micro QR / rMQR)
//! without changing the public API or any renderer. The default backend,
//! [`QrcodeEngine`], wraps the `qrcode` crate.

use crate::error::{QrError, Result};
use crate::matrix::Matrix;
use qrcode::{EcLevel, QrCode, Version};

/// QR error-correction level — the proportion of the symbol that can be
/// damaged or obscured (for example by a logo) and still decode.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Ecc {
    /// ~7% recovery.
    Low,
    /// ~15% recovery (the standard default).
    #[default]
    Medium,
    /// ~25% recovery.
    Quartile,
    /// ~30% recovery — recommended when embedding a logo.
    High,
}

impl Ecc {
    fn to_qrcode(self) -> EcLevel {
        match self {
            Ecc::Low => EcLevel::L,
            Ecc::Medium => EcLevel::M,
            Ecc::Quartile => EcLevel::Q,
            Ecc::High => EcLevel::H,
        }
    }
}

/// Options controlling how data is encoded into a QR [`Matrix`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct QrOptions {
    /// Error-correction level.
    pub ecc: Ecc,
    /// Force a specific QR version (1–40). `None` selects the smallest that
    /// fits the data at the chosen [`Ecc`].
    pub version: Option<u8>,
    /// Quiet-zone width in modules carried on the [`Matrix`] for renderers.
    /// The QR standard mandates 4; values below 4 may not scan.
    pub quiet_zone: u8,
}

impl Default for QrOptions {
    fn default() -> Self {
        QrOptions {
            ecc: Ecc::default(),
            version: None,
            quiet_zone: 4,
        }
    }
}

impl QrOptions {
    /// A new options set with default error correction and quiet zone.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the error-correction level.
    #[must_use]
    pub fn ecc(mut self, ecc: Ecc) -> Self {
        self.ecc = ecc;
        self
    }

    /// Forces a specific version (1–40).
    #[must_use]
    pub fn version(mut self, version: u8) -> Self {
        self.version = Some(version);
        self
    }

    /// Sets the quiet-zone width in modules.
    #[must_use]
    pub fn quiet_zone(mut self, modules: u8) -> Self {
        self.quiet_zone = modules;
        self
    }
}

/// A QR encoding backend. Implementors turn raw bytes plus [`QrOptions`] into a
/// renderer-ready [`Matrix`].
pub trait Engine {
    /// Encodes `data` into a [`Matrix`], or returns a [`QrError`] if the data
    /// cannot be represented at the requested settings.
    ///
    /// # Errors
    ///
    /// Returns [`QrError`] when the data exceeds capacity or the requested
    /// version is invalid.
    fn encode(&self, data: &[u8], options: &QrOptions) -> Result<Matrix>;
}

/// The default [`Engine`], backed by the `qrcode` crate.
#[derive(Clone, Copy, Debug, Default)]
pub struct QrcodeEngine;

impl QrcodeEngine {
    /// Encodes data into the backend `QrCode` honouring the version override.
    fn build(data: &[u8], options: &QrOptions) -> Result<QrCode> {
        let ec = options.ecc.to_qrcode();
        match options.version {
            Some(v) => {
                if !(1..=40).contains(&v) {
                    return Err(QrError::InvalidVersion(v));
                }
                QrCode::with_version(data, Version::Normal(i16::from(v)), ec)
                    .map_err(|_| QrError::InvalidVersion(v))
            }
            None => QrCode::with_error_correction_level(data, ec).map_err(QrError::from),
        }
    }
}

impl Engine for QrcodeEngine {
    fn encode(&self, data: &[u8], options: &QrOptions) -> Result<Matrix> {
        let code = Self::build(data, options)?;
        let size = code.width();
        let mut modules = Vec::with_capacity(size * size);
        for y in 0..size {
            for x in 0..size {
                modules.push(code[(x, y)] == qrcode::Color::Dark);
            }
        }
        Ok(Matrix::new(size, options.quiet_zone as usize, modules))
    }
}

/// Convenience: encode `data` with the default engine and options.
///
/// # Errors
///
/// Returns [`QrError`] if the data cannot be encoded.
pub fn encode(data: &[u8], options: &QrOptions) -> Result<Matrix> {
    QrcodeEngine.encode(data, options)
}
