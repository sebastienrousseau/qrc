//! Element: logo embedding — `LogoOptions`, `to_image_with_logo`,
//! `to_image_bytes_with_logo`, and the free `embed_logo`/`render_with_logo`.
//!
//! Run: `cargo run --example logo_options`

use image::{ImageBuffer, ImageFormat, Rgba, RgbaImage};
use qrc::encode::{Ecc, QrOptions};
use qrc::render::raster::{self, LogoOptions, RasterOptions};
use qrc::QRCode;

fn main() {
    let qr = QRCode::from_string("https://example.com".to_string());
    // Logos obscure modules — always pair with high error correction.
    let opts = QrOptions::new().ecc(Ecc::High);
    let raster = RasterOptions {
        module_size: 12,
        ..RasterOptions::default()
    };

    // The user's uploaded logo (here a synthetic blue square).
    let logo: RgbaImage = ImageBuffer::from_pixel(96, 96, Rgba([0x12, 0x6E, 0xE0, 255]));
    let logo_opts = LogoOptions {
        size_ratio: 0.2,
        padding: 8,
        ..LogoOptions::default()
    };

    // High-level: image and bytes.
    let img = qr
        .to_image_with_logo(&opts, &raster, &logo, &logo_opts)
        .unwrap();
    println!("branded image: {:?}", img.dimensions());
    let png = qr
        .to_image_bytes_with_logo(&opts, &raster, &logo, &logo_opts, ImageFormat::Png)
        .unwrap();
    println!("branded png: {} bytes", png.len());

    // Free functions from a Matrix.
    let matrix = qr.encode(&opts).unwrap();
    let with_logo = raster::render_with_logo(&matrix, &raster, &logo, &logo_opts);
    println!("render_with_logo: {:?}", with_logo.dimensions());

    // embed_logo edits an existing image in place.
    let mut canvas = raster::render(&matrix, &raster);
    raster::embed_logo(&mut canvas, &logo, &LogoOptions::default());
    println!("embed_logo (in place): {:?}", canvas.dimensions());
}
