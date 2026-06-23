//! Element: ControlNet control-image export — `ControlOptions`,
//! `to_control_image`, `to_control_image_bytes`, and the free `render`.
//!
//! Run: `cargo run --example control_options`

use image::ImageFormat;
use qrc::encode::{Ecc, QrOptions};
use qrc::render::control::{self, ControlOptions};
use qrc::QRCode;

fn main() {
    let qr = QRCode::from_string("https://example.com".to_string());
    // Control images use high ECC so the model has redundancy to hide art.
    let opts = QrOptions::new().ecc(Ecc::High);

    // Exact-square, high-contrast hint for an AI pipeline (default 768).
    let img = qr
        .to_control_image(&opts, &ControlOptions::default())
        .unwrap();
    println!("control image: {:?}", img.dimensions());

    // A custom target size.
    let img512 = qr
        .to_control_image(&opts, &ControlOptions::with_size(512))
        .unwrap();
    println!("control image @512: {:?}", img512.dimensions());

    // Bytes to feed straight to Stable Diffusion + a QR ControlNet.
    let png = qr
        .to_control_image_bytes(&opts, &ControlOptions::default(), ImageFormat::Png)
        .unwrap();
    println!("control png: {} bytes", png.len());

    // Free renderer from a Matrix.
    let matrix = qr.encode(&opts).unwrap();
    println!(
        "free control::render: {:?}",
        control::render(&matrix, &ControlOptions::default()).dimensions()
    );
}
