// Copyright © 2022-2026 QR Code Library (QRC). All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! A [`Provider`] backed by [Replicate](https://replicate.com)'s prediction API.
//!
//! HTTP is abstracted behind [`HttpClient`] so the request-building and
//! response-parsing logic is fully testable without a network; [`UreqClient`]
//! is the real, blocking implementation.

use super::{ArtRequest, Provider};
use crate::error::{QrError, Result};
use base64::Engine as _;
use serde_json::{json, Value};

/// A minimal HTTP response: status code plus raw body.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HttpResponse {
    /// HTTP status code.
    pub status: u16,
    /// Raw response body.
    pub body: Vec<u8>,
}

/// A pluggable HTTP transport. Implement this to use a different client (or a
/// mock in tests); [`UreqClient`] is the default real implementation.
pub trait HttpClient {
    /// Sends a request and returns the response (any status code is `Ok`; only
    /// transport failures are `Err`).
    ///
    /// # Errors
    ///
    /// Returns [`QrError::Api`] on a transport-level failure.
    fn send(
        &self,
        method: &str,
        url: &str,
        headers: &[(&str, &str)],
        body: Option<&[u8]>,
    ) -> Result<HttpResponse>;
}

/// The default [`HttpClient`], backed by the blocking `ureq` crate.
#[derive(Clone, Debug, Default)]
pub struct UreqClient;

impl UreqClient {
    /// Creates a new client.
    #[must_use]
    pub fn new() -> Self {
        UreqClient
    }
}

impl HttpClient for UreqClient {
    fn send(
        &self,
        method: &str,
        url: &str,
        headers: &[(&str, &str)],
        body: Option<&[u8]>,
    ) -> Result<HttpResponse> {
        let mut req = ureq::request(method, url);
        for (k, v) in headers {
            req = req.set(k, v);
        }
        let outcome = match body {
            Some(b) => req.send_bytes(b),
            None => req.call(),
        };
        match outcome {
            Ok(resp) => Ok(read_response(&resp.status(), resp)),
            // Replicate may signal application errors via 4xx/5xx; surface them
            // as a normal response so the caller can read the body.
            Err(ureq::Error::Status(code, resp)) => Ok(read_response(&code, resp)),
            Err(ureq::Error::Transport(t)) => {
                Err(QrError::Api(format!("HTTP transport error: {t}")))
            }
        }
    }
}

/// Reads a `ureq` response into an [`HttpResponse`].
fn read_response(status: &u16, resp: ureq::Response) -> HttpResponse {
    let mut body = Vec::new();
    // Read errors leave the body empty rather than failing the whole request.
    let _ = std::io::Read::read_to_end(&mut resp.into_reader(), &mut body);
    HttpResponse {
        status: *status,
        body,
    }
}

/// Generates art-QRs through Replicate's prediction API.
///
/// Use [`ReplicateProvider::new`] for production and
/// [`ReplicateProvider::with_client`] to inject a mock client (or point at a
/// different base URL) in tests.
/// (The HTTP client is type-erased behind `Box<dyn HttpClient>` so there is a
/// single, fully-testable code path rather than one per client type.)
pub struct ReplicateProvider {
    /// Type-erased HTTP transport (real `UreqClient` or a test mock).
    client: Box<dyn HttpClient>,
    /// Replicate API token, sent as `Authorization: Token …`.
    token: String,
    /// `owner/model:version` (or bare version) identifier to run.
    model_version: String,
    /// API base URL (overridable so tests can target a local server).
    base_url: String,
}

impl core::fmt::Debug for ReplicateProvider {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("ReplicateProvider")
            .field("model_version", &self.model_version)
            .field("base_url", &self.base_url)
            .finish_non_exhaustive()
    }
}

impl ReplicateProvider {
    /// Creates a provider for the given API token and `owner/model:version`
    /// (or bare `version`) identifier, using the real HTTP client.
    #[must_use]
    pub fn new(token: impl Into<String>, model_version: impl Into<String>) -> Self {
        ReplicateProvider::with_client(
            UreqClient::new(),
            token,
            model_version,
            "https://api.replicate.com",
        )
    }

    /// Creates a provider with an explicit HTTP client and base URL.
    pub fn with_client(
        client: impl HttpClient + 'static,
        token: impl Into<String>,
        model_version: impl Into<String>,
        base_url: impl Into<String>,
    ) -> Self {
        ReplicateProvider {
            client: Box::new(client),
            token: token.into(),
            model_version: model_version.into(),
            base_url: base_url.into(),
        }
    }

    /// Builds the JSON request body for a prediction.
    fn request_body(&self, control_png: &[u8], request: &ArtRequest) -> String {
        let data_uri = format!(
            "data:image/png;base64,{}",
            base64::engine::general_purpose::STANDARD.encode(control_png)
        );
        let mut input = json!({
            "prompt": request.prompt,
            "negative_prompt": request.negative_prompt,
            "image": data_uri,
            "controlnet_conditioning_scale": request.conditioning_scale,
        });
        if let Some(payload) = &request.qr_payload {
            input["qr_code_content"] = json!(payload);
        }
        json!({ "version": self.model_version, "input": input }).to_string()
    }
}

/// Extracts the first output image URL from a prediction response body.
fn output_url(body: &[u8]) -> Result<String> {
    let value: Value =
        serde_json::from_slice(body).map_err(|e| QrError::Api(format!("invalid JSON: {e}")))?;

    if let Some(status) = value.get("status").and_then(Value::as_str) {
        if status == "failed" || status == "canceled" {
            let detail = value
                .get("error")
                .and_then(Value::as_str)
                .unwrap_or("prediction did not succeed");
            return Err(QrError::Api(format!("prediction {status}: {detail}")));
        }
    }

    match value.get("output") {
        Some(Value::String(url)) => Ok(url.clone()),
        Some(Value::Array(items)) => items
            .iter()
            .find_map(Value::as_str)
            .map(str::to_string)
            .ok_or_else(|| QrError::Api("prediction output array had no URL".to_string())),
        _ => Err(QrError::Api("prediction had no output URL".to_string())),
    }
}

impl Provider for ReplicateProvider {
    fn generate(&self, control_png: &[u8], request: &ArtRequest) -> Result<Vec<u8>> {
        let auth = format!("Token {}", self.token);
        let body = self.request_body(control_png, request);

        // `Prefer: wait` asks Replicate to return the finished prediction
        // synchronously instead of requiring us to poll.
        let create = self.client.send(
            "POST",
            &format!("{}/v1/predictions", self.base_url),
            &[
                ("Authorization", auth.as_str()),
                ("Content-Type", "application/json"),
                ("Prefer", "wait"),
            ],
            Some(body.as_bytes()),
        )?;
        if !(200..300).contains(&create.status) {
            return Err(QrError::Api(format!(
                "create prediction returned HTTP {}",
                create.status
            )));
        }

        let url = output_url(&create.body)?;

        let image = self
            .client
            .send("GET", &url, &[("Authorization", auth.as_str())], None)?;
        if !(200..300).contains(&image.status) {
            return Err(QrError::Api(format!(
                "fetching output returned HTTP {}",
                image.status
            )));
        }
        Ok(image.body)
    }
}
