//! Every legacy `QRCode` helper and every exported macro, exercised in memory
//! (no external asset files required).
//!
//! Run with: `cargo run --example legacy_and_macros`

use image::{ImageBuffer, Rgba, RgbaImage};
use qrc::{
    add_image_watermark, batch_generate_qr, combine_qr_codes, create_dynamic_qr,
    create_multilanguage_qr, overlay_image, qr_code, qr_code_to, resize, set_encoding_format,
    QRCode,
};

fn main() {
    // --- Constructors + fallible/infallible conversion -----------------
    let qr = QRCode::from_string("https://example.com".to_string());
    let _ = QRCode::new(b"bytes".to_vec());
    let _ = QRCode::from_bytes(vec![1, 2, 3]);
    println!("try_to_qrcode ok? {}", qr.try_to_qrcode().is_ok());
    println!("module width: {}", qr.to_qrcode().width());

    // --- Raster helpers ------------------------------------------------
    let colorized: RgbaImage = qr.colorize(Rgba([200, 30, 30, 255]));
    println!("colorized to {:?}", colorized.dimensions());

    let resized: RgbaImage = qr.resize(128, 128);
    println!("resized to {:?}", resized.dimensions());

    // Watermark (in place) using a synthetic semi-transparent logo.
    let mut canvas = qr.to_png(256);
    let watermark: RgbaImage = ImageBuffer::from_pixel(32, 32, Rgba([0, 0, 0, 128]));
    add_image_watermark!(&mut canvas, &watermark);
    QRCode::add_image_watermark(&mut canvas, &watermark);
    println!("watermarked {:?}", canvas.dimensions());

    // Overlay a logo at the top-left (method + macro).
    let logo: RgbaImage = ImageBuffer::from_pixel(16, 16, Rgba([0, 120, 255, 255]));
    let overlaid = qr.overlay_image(&logo);
    let overlaid_macro = overlay_image!(qr, &logo);
    println!(
        "overlay {:?} (macro {:?})",
        overlaid.dimensions(),
        overlaid_macro.dimensions()
    );

    // Combine several codes into one strip (method + macro).
    let combined = QRCode::combine_qr_codes(vec![
        QRCode::from_string("one".to_string()),
        QRCode::from_string("two".to_string()),
    ])
    .unwrap();
    println!("combined data is {} bytes", combined.data.len());
    let combined_macro = combine_qr_codes!(vec![
        QRCode::from_string("a".to_string()),
        QRCode::from_string("b".to_string()),
    ])
    .unwrap();
    println!("combined via macro: {} bytes", combined_macro.data.len());

    // --- Batch, dynamic, multilanguage ---------------------------------
    let batch = QRCode::batch_generate_qr_codes(vec!["x".to_string(), "y".to_string()]);
    let batch_macro = batch_generate_qr!(vec!["p".to_string(), "q".to_string()]);
    println!("batch {} / macro {}", batch.len(), batch_macro.len());

    let dynamic = QRCode::create_dynamic("promo-1");
    let dynamic_macro = create_dynamic_qr!("promo-2");
    println!(
        "dynamic urls: {} bytes / {} bytes",
        dynamic.data.len(),
        dynamic_macro.data.len()
    );

    let multilang = create_multilanguage_qr! {
        "en" => "Hello",
        "es" => "Hola",
        "fr" => "Bonjour",
    };
    println!(
        "multilanguage selected: {:?}",
        String::from_utf8_lossy(&multilang.data)
    );

    // --- Encoding format accessors (method + macro) --------------------
    let with_fmt = set_encoding_format!(qr, "utf-8").unwrap();
    println!("encoding format: {}", with_fmt.get_encoding_format());
    assert!(qr.set_encoding_format("latin-1").is_err());

    // --- Construction + format macros ----------------------------------
    let _ = qr_code!(b"made with a macro".to_vec());
    let png = qr_code_to!(b"https://example.com".to_vec(), "png", 256);
    let jpg = qr_code_to!(b"https://example.com".to_vec(), "jpg", 256);
    let gif = qr_code_to!(b"https://example.com".to_vec(), "gif", 256);
    println!(
        "qr_code_to png {:?} jpg {:?} gif {:?}",
        png.dimensions(),
        jpg.dimensions(),
        gif.dimensions()
    );

    // resize! macro.
    let resized_macro = resize!(qr, 64);
    println!("resize! macro -> {:?}", resized_macro.dimensions());
}
