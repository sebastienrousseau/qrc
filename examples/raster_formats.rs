//! Raster rendering: real PNG/JPEG/GIF byte encoders, the generic
//! `to_image_bytes` for other formats, custom colors, and `fit_width` scaling.
//!
//! Run with: `cargo run --example raster_formats`

use image::ImageFormat;
use qrc::encode::QrOptions;
use qrc::render::raster::RasterOptions;
use qrc::render::style::Color;
use qrc::QRCode;

fn main() {
    let qr = QRCode::from_string("https://example.com/raster".to_string());
    let opts = QrOptions::new();

    // Dedicated byte encoders — these return real, scannable image bytes.
    let png = qr.to_png_bytes(&opts, &RasterOptions::default()).unwrap();
    let jpeg = qr
        .to_jpeg_bytes(
            &opts,
            &RasterOptions {
                module_size: 10,
                ..RasterOptions::default()
            },
        )
        .unwrap();
    let gif = qr.to_gif_bytes(&opts, &RasterOptions::default()).unwrap();
    println!(
        "png {} bytes, jpeg {} bytes, gif {} bytes",
        png.len(),
        jpeg.len(),
        gif.len()
    );

    // Generic encoder for any other image format (BMP, TIFF, WebP, ...).
    let bmp = qr
        .to_image_bytes(&opts, &RasterOptions::default(), ImageFormat::Bmp)
        .unwrap();
    println!("bmp {} bytes (magic {:?})", bmp.len(), &bmp[..2]);

    // Custom colors.
    let branded = qr
        .to_png_bytes(
            &opts,
            &RasterOptions {
                module_size: 8,
                dark: Color::rgb(0x22, 0x33, 0x88),
                light: Color::rgb(0xF0, 0xF0, 0xFF),
            },
        )
        .unwrap();
    println!("branded png {} bytes", branded.len());

    // The free renderer functions, driven from a `Matrix` directly.
    let matrix = qr.encode(&opts).unwrap();
    let rgba = qrc::render::raster::render(&matrix, &RasterOptions::default());
    println!("free raster::render -> {:?} image", rgba.dimensions());
    let tiff = qrc::render::raster::to_bytes(&matrix, &RasterOptions::default(), ImageFormat::Tiff)
        .unwrap();
    println!("free raster::to_bytes (TIFF) -> {} bytes", tiff.len());

    // `fit_width` picks the largest module size that stays within a budget.
    let fitted = RasterOptions::fit_width(&matrix, 300);
    println!(
        "fit 300px -> module_size {} -> {}px square",
        fitted.module_size,
        matrix.total_size() as u32 * fitted.module_size
    );

    // Over-capacity input is a recoverable error, not a panic.
    let huge = QRCode::from_string("Z".repeat(8000));
    assert!(huge.to_png_bytes(&opts, &RasterOptions::default()).is_err());
    println!("over-capacity raster request correctly rejected");
}
