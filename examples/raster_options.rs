//! Element: raster rendering — `RasterOptions`, the real PNG/JPEG/GIF byte
//! encoders, the generic `to_image_bytes`, and the free render helpers.
//!
//! Run: `cargo run --example raster_options`

use image::ImageFormat;
use qrc::encode::QrOptions;
use qrc::render::raster::{self, RasterOptions};
use qrc::render::style::Color;
use qrc::QRCode;

fn main() {
    let qr = QRCode::from_string("https://example.com".to_string());
    let opts = QrOptions::new();
    let raster = RasterOptions::default();

    // Dedicated, real byte encoders (these are genuinely PNG/JPEG/GIF bytes).
    let png = qr.to_png_bytes(&opts, &raster).unwrap();
    let jpeg = qr
        .to_jpeg_bytes(
            &opts,
            &RasterOptions {
                module_size: 10,
                ..raster
            },
        )
        .unwrap();
    let gif = qr.to_gif_bytes(&opts, &raster).unwrap();
    println!(
        "png {} | jpeg {} | gif {} bytes",
        png.len(),
        jpeg.len(),
        gif.len()
    );

    // Generic encoder for any other format (BMP/TIFF/WebP/...).
    let bmp = qr.to_image_bytes(&opts, &raster, ImageFormat::Bmp).unwrap();
    println!("bmp magic: {:?}", &bmp[..2]);

    // Custom colors.
    let branded = RasterOptions {
        module_size: 8,
        dark: Color::rgb(0x22, 0x33, 0x88),
        light: Color::rgb(0xF0, 0xF0, 0xFF),
    };
    println!(
        "branded png: {} bytes",
        qr.to_png_bytes(&opts, &branded).unwrap().len()
    );

    // fit_width picks the largest module size within a pixel budget.
    let matrix = qr.encode(&opts).unwrap();
    let fitted = RasterOptions::fit_width(&matrix, 300);
    println!("fit 300px -> module size {}", fitted.module_size);

    // Free render helpers from a Matrix.
    let img = raster::render(&matrix, &raster);
    println!("free raster::render -> {:?}", img.dimensions());
    let tiff = raster::to_bytes(&matrix, &raster, ImageFormat::Tiff).unwrap();
    let again = raster::image_to_bytes(&img, ImageFormat::Png).unwrap();
    println!(
        "free to_bytes (TIFF) {} | image_to_bytes (PNG) {}",
        tiff.len(),
        again.len()
    );
}
