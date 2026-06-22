// Copyright © 2022-2026 QR Code Library (QRC). All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Error types for the `qrc` crate.
//!
//! All fallible operations return [`QrError`] so that untrusted input can be
//! handled without panicking. The type implements [`core::fmt::Display`] and
//! [`std::error::Error`] when the `std` feature is enabled.

use core::fmt;

/// A specialized [`Result`](core::result::Result) for QR operations.
pub type Result<T> = core::result::Result<T, QrError>;

/// Errors that can occur while encoding or rendering a QR code.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum QrError {
    /// The data was too large to fit in a QR code at the requested settings.
    DataTooLong,
    /// An explicitly requested version (1–40) was outside the valid range,
    /// or could not hold the data at the requested error-correction level.
    InvalidVersion(u8),
    /// The underlying engine failed to encode the data.
    Encode(&'static str),
    /// Rendering to the requested image format failed.
    Render(&'static str),
}

impl fmt::Display for QrError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            QrError::DataTooLong => f.write_str("data is too long to encode as a QR code"),
            QrError::InvalidVersion(v) => {
                write!(f, "invalid or insufficient QR version: {v}")
            }
            QrError::Encode(msg) => write!(f, "QR encoding failed: {msg}"),
            QrError::Render(msg) => write!(f, "QR rendering failed: {msg}"),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for QrError {}

impl From<qrcode::types::QrError> for QrError {
    fn from(err: qrcode::types::QrError) -> Self {
        use qrcode::types::QrError as E;
        match err {
            E::DataTooLong => QrError::DataTooLong,
            E::InvalidVersion => QrError::Encode("invalid version"),
            E::UnsupportedCharacterSet => QrError::Encode("unsupported character set"),
            E::InvalidEciDesignator => QrError::Encode("invalid ECI designator"),
            E::InvalidCharacter => QrError::Encode("invalid character"),
        }
    }
}
