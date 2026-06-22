//! Branded business-card QR: build a vCard contact and embed a custom logo.
//!
//! In a real app the logo comes from the user's upload:
//!
//! ```no_run
//! let logo = image::open("my-logo.png")?.into_rgba8();
//! ```
//!
//! Here we synthesise a logo so the example is self-contained. The output is
//! written to the system temp dir and then removed.
//!
//! Run with: `cargo run --example business_card`

use image::{ImageBuffer, Rgba, RgbaImage};
use qrc::encode::{Ecc, QrOptions};
use qrc::payload::vcard::BusinessCard;
use qrc::render::raster::{LogoOptions, RasterOptions};
use qrc::render::style::Color;
use qrc::QRCode;

fn main() {
    // 1. Build the contact / business card.
    let card = BusinessCard::new("Jane Doe")
        .name("Jane", "Doe")
        .organization("Acme, Inc.")
        .title("Chief Executive Officer")
        .phone("+1-555-0100")
        .email("jane@acme.example")
        .url("https://acme.example")
        .address("1 Market Street, Springfield")
        .note("Scan to add me to your contacts");

    let vcard = card.to_vcard();
    println!("vCard payload:\n{vcard}\n");

    let qr = QRCode::from_string(vcard);

    // 2. The user's uploaded logo (synthesised here as a blue rounded badge).
    let logo: RgbaImage = ImageBuffer::from_fn(120, 120, |x, y| {
        let (dx, dy) = (x as i32 - 60, y as i32 - 60);
        if dx * dx + dy * dy <= 58 * 58 {
            Rgba([0x12, 0x6E, 0xE0, 255])
        } else {
            Rgba([0, 0, 0, 0]) // transparent outside the circle
        }
    });

    // 3. Render a branded PNG. Use high error correction so the modules behind
    //    the logo are recoverable, and keep the logo to ~18% of the width.
    let png = qr
        .to_image_bytes_with_logo(
            &QrOptions::new().ecc(Ecc::High),
            &RasterOptions {
                module_size: 12,
                dark: Color::rgb(0x10, 0x10, 0x2A),
                light: Color::WHITE,
            },
            &logo,
            &LogoOptions {
                size_ratio: 0.18,
                padding: 8,
                background: Some(Color::WHITE),
            },
            image::ImageFormat::Png,
        )
        .unwrap();

    let path = std::env::temp_dir().join("qrc_business_card.png");
    std::fs::write(&path, &png).unwrap();
    println!(
        "Wrote branded business-card QR ({} bytes) to {}",
        png.len(),
        path.display()
    );
    println!("Open it to scan, or replace the synthetic logo with `image::open(\"my-logo.png\")?.into_rgba8()`.");
}
