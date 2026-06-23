//! Element: offline artistic blend — `BlendOptions`, `to_art_image`,
//! `to_art_bytes`, and the free `blend`.
//!
//! Run: `cargo run --example blend_options`

use image::{ImageBuffer, ImageFormat, Rgba, RgbaImage};
use qrc::encode::{Ecc, QrOptions};
use qrc::render::art::{self, BlendOptions};
use qrc::QRCode;

fn main() {
    let qr = QRCode::from_string("https://example.com".to_string());
    let opts = QrOptions::new().ecc(Ecc::High);

    // The "uploaded" background (here a colour gradient).
    let bg: RgbaImage = ImageBuffer::from_fn(512, 512, |x, y| {
        Rgba([(x / 2) as u8, (y / 2) as u8, 160, 255])
    });

    // Tune strength (image visibility vs. scan reliability) and dot_ratio.
    let blend = BlendOptions {
        module_size: 12,
        strength: 0.75,
        dot_ratio: 0.66,
        ..BlendOptions::default()
    };

    let img = qr.to_art_image(&opts, &bg, &blend).unwrap();
    println!("art image: {:?}", img.dimensions());
    let png = qr
        .to_art_bytes(&opts, &bg, &blend, ImageFormat::Png)
        .unwrap();
    println!("art png: {} bytes", png.len());

    // Free function from a Matrix.
    let matrix = qr.encode(&opts).unwrap();
    println!(
        "free blend: {:?}",
        art::blend(&matrix, &bg, &BlendOptions::default()).dimensions()
    );
}
