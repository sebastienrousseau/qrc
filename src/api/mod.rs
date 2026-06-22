// Copyright © 2022-2026 QR Code Library (QRC). All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Cloud AI art-QR generation (feature `api`).
//!
//! This is the plug-and-play counterpart to the offline tools: it exports a
//! [control image](crate::render::control), hands it plus a text prompt to a
//! cloud diffusion provider (Stable Diffusion + a QR ControlNet), and — because
//! AI art-QRs frequently come back unscannable — **verifies the result decodes
//! and retries** until it does.
//!
//! Generation is abstracted behind the [`Provider`] trait so the orchestration
//! is fully testable without a network, and so alternative back ends can be
//! dropped in. A [`replicate::ReplicateProvider`] implementation is included.
//!
//! ```no_run
//! use qrc::api::{generate, ArtRequest, RetryOptions};
//! use qrc::api::replicate::ReplicateProvider;
//! use qrc::encode::{Ecc, QrOptions};
//! use qrc::render::control::ControlOptions;
//! use qrc::QRCode;
//!
//! let qr = QRCode::from_string("https://example.com".to_string());
//! let provider = ReplicateProvider::new("r8_token", "model:version");
//! let png = generate(
//!     &qr,
//!     &QrOptions::new().ecc(Ecc::High),
//!     &ControlOptions::default(),
//!     &provider,
//!     &ArtRequest::new("a serene koi pond, ink painting"),
//!     &RetryOptions::default(),
//! ).unwrap();
//! ```

pub mod replicate;

use crate::error::{QrError, Result};
use crate::render::control::ControlOptions;
use crate::QRCode;

/// What to generate: the text prompt plus diffusion knobs.
#[derive(Clone, Debug, PartialEq)]
pub struct ArtRequest {
    /// Positive prompt describing the desired artwork.
    pub prompt: String,
    /// Negative prompt (things to avoid).
    pub negative_prompt: String,
    /// How strongly the QR structure constrains the art. Higher scans more
    /// reliably but looks less like free art; ~1.1–2.0 is typical.
    pub conditioning_scale: f32,
    /// Optional explicit QR payload for models that accept the content
    /// directly rather than (or as well as) a control image.
    pub qr_payload: Option<String>,
}

impl ArtRequest {
    /// A request with the given prompt and sensible defaults.
    #[must_use]
    pub fn new(prompt: impl Into<String>) -> Self {
        ArtRequest {
            prompt: prompt.into(),
            negative_prompt: "ugly, blurry, low quality, distorted".to_string(),
            conditioning_scale: 1.5,
            qr_payload: None,
        }
    }

    /// Sets the negative prompt.
    #[must_use]
    pub fn negative_prompt(mut self, value: impl Into<String>) -> Self {
        self.negative_prompt = value.into();
        self
    }

    /// Sets the ControlNet conditioning scale.
    #[must_use]
    pub fn conditioning_scale(mut self, scale: f32) -> Self {
        self.conditioning_scale = scale;
        self
    }

    /// Sets an explicit QR payload to pass to the model.
    #[must_use]
    pub fn qr_payload(mut self, payload: impl Into<String>) -> Self {
        self.qr_payload = Some(payload.into());
        self
    }
}

/// How many times to ask the provider for a *scannable* result before giving up.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct RetryOptions {
    /// Maximum generation attempts (clamped to at least 1).
    pub max_attempts: u32,
}

impl Default for RetryOptions {
    fn default() -> Self {
        RetryOptions { max_attempts: 3 }
    }
}

/// A cloud art-QR back end. Implementors take a PNG control image plus a
/// request and return generated image bytes.
pub trait Provider {
    /// Generates an art image from `control_png` and `request`.
    ///
    /// # Errors
    ///
    /// Returns [`QrError::Api`] on any provider/network failure.
    fn generate(&self, control_png: &[u8], request: &ArtRequest) -> Result<Vec<u8>>;
}

/// Returns whether `image_bytes` is a decodable QR code.
fn scans(image_bytes: &[u8]) -> bool {
    match image::load_from_memory(image_bytes) {
        Ok(img) => {
            let luma = img.into_luma8();
            let mut prepared = rqrr::PreparedImage::prepare(luma);
            prepared.detect_grids().iter().any(|g| g.decode().is_ok())
        }
        Err(_) => false,
    }
}

/// Generates an AI art-QR: exports a control image, asks `provider` to paint it,
/// and returns the first result that actually scans.
///
/// # Errors
///
/// Returns [`QrError`] if the data cannot be encoded, or [`QrError::Api`] if no
/// scannable result was produced within [`RetryOptions::max_attempts`].
pub fn generate<P: Provider>(
    qr: &QRCode,
    options: &crate::encode::QrOptions,
    control_options: &ControlOptions,
    provider: &P,
    request: &ArtRequest,
    retry: &RetryOptions,
) -> Result<Vec<u8>> {
    let control = qr.to_control_image_bytes(options, control_options, image::ImageFormat::Png)?;

    let mut last = String::from("no attempts were made");
    for _ in 0..retry.max_attempts.max(1) {
        match provider.generate(&control, request) {
            Ok(bytes) if scans(&bytes) => return Ok(bytes),
            Ok(_) => last = "generated image did not scan".to_string(),
            Err(e) => last = e.to_string(),
        }
    }
    Err(QrError::Api(format!(
        "no scannable art-QR after {} attempt(s): {last}",
        retry.max_attempts.max(1)
    )))
}
