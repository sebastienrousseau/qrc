//! Element: the legacy `QRCode` helper methods kept for backwards
//! compatibility (colorize, resize, watermark, overlay, combine, batch,
//! multilanguage, dynamic, encoding format).
//!
//! Run: `cargo run --example legacy`

use image::{ImageBuffer, Rgba, RgbaImage};
use qrc::QRCode;

fn main() {
    let qr = QRCode::from_string("https://example.com".to_string());

    // Colorize / resize.
    let colored: RgbaImage = qr.colorize(Rgba([200, 30, 30, 255]));
    let resized: RgbaImage = qr.resize(128, 128);
    println!(
        "colorize {:?} | resize {:?}",
        colored.dimensions(),
        resized.dimensions()
    );

    // Watermark in place (method form) + overlay.
    let mut canvas = qr.to_png(256);
    let watermark: RgbaImage = ImageBuffer::from_pixel(24, 24, Rgba([0, 0, 0, 128]));
    QRCode::add_image_watermark(&mut canvas, &watermark);
    let logo: RgbaImage = ImageBuffer::from_pixel(16, 16, Rgba([0, 120, 255, 255]));
    println!(
        "watermark {:?} | overlay {:?}",
        canvas.dimensions(),
        qr.overlay_image(&logo).dimensions()
    );

    // Combine several codes into a strip.
    let combined = QRCode::combine_qr_codes(vec![
        QRCode::from_string("one".to_string()),
        QRCode::from_string("two".to_string()),
    ])
    .unwrap();
    println!("combine: {} bytes", combined.data.len());

    // Batch / dynamic / multilanguage.
    let batch = QRCode::batch_generate_qr_codes(vec!["a".to_string(), "b".to_string()]);
    let dynamic = QRCode::create_dynamic("promo-1");
    use std::collections::HashMap;
    let mut map = HashMap::new();
    map.insert("en".to_string(), "Hello".to_string());
    let multi = QRCode::create_multilanguage(map);
    println!(
        "batch {} | dynamic {} bytes | multilanguage {:?}",
        batch.len(),
        dynamic.data.len(),
        String::from_utf8_lossy(&multi.data)
    );

    // Encoding format accessors.
    let with_fmt = qr.set_encoding_format("utf-8").unwrap();
    println!("encoding format: {}", with_fmt.get_encoding_format());
    assert!(qr.set_encoding_format("latin-1").is_err());
}
