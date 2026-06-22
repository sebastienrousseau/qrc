//! Two ways to make a QR that doesn't look like a plain QR:
//!
//!  1. `to_art_image` — an OFFLINE, deterministic blend that weaves a supplied
//!     image (logo/photo) into the code. No model, guaranteed scannable.
//!  2. `to_control_image` — export a clean, high-contrast control image to feed
//!     to an AI pipeline (Stable Diffusion + a QR ControlNet) for the
//!     generated-art look. That step runs in a model, not in this crate.
//!
//! In a real app the background comes from the user's upload:
//! `let bg = image::open("photo.jpg")?.into_rgba8();`
//!
//! Run with: `cargo run --example art_qr`

use image::{ImageBuffer, Rgba, RgbaImage};
use qrc::encode::{Ecc, QrOptions};
use qrc::render::art::BlendOptions;
use qrc::render::control::ControlOptions;
use qrc::QRCode;

fn main() {
    let qr = QRCode::from_string("https://example.com".to_string());
    // Art QR leans on the redundancy of high error correction.
    let opts = QrOptions::new().ecc(Ecc::High);

    // The "uploaded" image (here a synthetic colour gradient).
    let background: RgbaImage = ImageBuffer::from_fn(640, 640, |x, y| {
        let (fx, fy) = (x as f32 / 640.0, y as f32 / 640.0);
        Rgba([
            (255.0 * (0.9 - 0.5 * fy)) as u8,
            (120.0 + 100.0 * fx * (1.0 - fy)) as u8,
            (200.0 * fy) as u8,
            255,
        ])
    });

    // 1. Offline artistic blend. Tune `strength`/`dot_ratio` to trade image
    //    visibility against scan reliability.
    let art = qr
        .to_art_bytes(
            &opts,
            &background,
            &BlendOptions {
                module_size: 14,
                strength: 0.75,
                dot_ratio: 0.66,
                ..BlendOptions::default()
            },
            image::ImageFormat::Png,
        )
        .unwrap();
    let art_path = std::env::temp_dir().join("qrc_art_qr.png");
    std::fs::write(&art_path, &art).unwrap();
    println!(
        "Offline art QR -> {} ({} bytes)",
        art_path.display(),
        art.len()
    );

    // 2. ControlNet-ready control image for an AI pipeline.
    let control = qr
        .to_control_image_bytes(
            &opts,
            &ControlOptions::with_size(768),
            image::ImageFormat::Png,
        )
        .unwrap();
    let control_path = std::env::temp_dir().join("qrc_control.png");
    std::fs::write(&control_path, &control).unwrap();
    println!(
        "ControlNet control image -> {} ({} bytes)",
        control_path.display(),
        control.len()
    );
    println!("Feed the control image to Stable Diffusion + a QR ControlNet for the AI-art look.");
}
