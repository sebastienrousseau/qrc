//! Cloud AI art-QR via Replicate (feature `api`).
//!
//! Generates a QR woven into AI artwork, verifying the result scans and
//! retrying if it doesn't. Set credentials to run for real:
//!
//! ```sh
//! export REPLICATE_API_TOKEN=r8_...
//! export REPLICATE_QR_MODEL="owner/model:versionhash"   # a QR-ControlNet model
//! cargo run --example ai_art --features api
//! ```
//!
//! Without a token it prints guidance and exits, so it always builds and runs.

use std::env;

use qrc::api::replicate::ReplicateProvider;
use qrc::api::{ArtRequest, RetryOptions};
use qrc::encode::{Ecc, QrOptions};
use qrc::render::control::ControlOptions;
use qrc::QRCode;

fn main() {
    let Ok(token) = env::var("REPLICATE_API_TOKEN") else {
        println!("Set REPLICATE_API_TOKEN (and REPLICATE_QR_MODEL) to generate AI art-QRs.");
        println!("This crate builds the control image, calls the model, and verifies the");
        println!("result scans — retrying until it does.");
        return;
    };
    let model = env::var("REPLICATE_QR_MODEL").unwrap_or_else(|_| "owner/model:version".into());

    let qr = QRCode::from_string("https://example.com".to_string());
    let provider = ReplicateProvider::new(token, model);
    let request = ArtRequest::new("a serene koi pond, japanese ink wash painting, soft light")
        .conditioning_scale(1.5);

    match qr.to_ai_art(
        &QrOptions::new().ecc(Ecc::High),
        &ControlOptions::default(),
        &provider,
        &request,
        &RetryOptions::default(),
    ) {
        Ok(png) => {
            let path = env::temp_dir().join("qrc_ai_art.png");
            std::fs::write(&path, &png).unwrap();
            println!(
                "Scannable AI art-QR ({} bytes) -> {}",
                png.len(),
                path.display()
            );
        }
        Err(e) => println!("Generation failed: {e}"),
    }
}
