//! Element: every exported macro.
//!
//! Run: `cargo run --example macros`

use image::{ImageBuffer, Rgba, RgbaImage};
use qrc::{
    add_image_watermark, batch_generate_qr, combine_qr_codes, create_dynamic_qr,
    create_multilanguage_qr, overlay_image, qr_code, qr_code_to, resize, set_encoding_format,
    QRCode,
};

fn main() {
    // Construction and format.
    let qr = qr_code!(b"https://example.com".to_vec());
    let png = qr_code_to!(b"https://example.com".to_vec(), "png", 256);
    let jpg = qr_code_to!(b"https://example.com".to_vec(), "jpg", 256);
    let gif = qr_code_to!(b"https://example.com".to_vec(), "gif", 256);
    println!(
        "qr_code_to png/jpg/gif: {:?} {:?} {:?}",
        png.dimensions(),
        jpg.dimensions(),
        gif.dimensions()
    );

    // Resize and encoding format.
    println!("resize!: {:?}", resize!(qr, 64).dimensions());
    let with_fmt = set_encoding_format!(qr, "utf-8").unwrap();
    println!("encoding: {}", with_fmt.get_encoding_format());

    // Watermark and overlay (synthetic images, no files needed).
    let mut canvas = qr.to_png(256);
    let watermark: RgbaImage = ImageBuffer::from_pixel(24, 24, Rgba([0, 0, 0, 128]));
    add_image_watermark!(&mut canvas, &watermark);
    let logo: RgbaImage = ImageBuffer::from_pixel(16, 16, Rgba([0, 120, 255, 255]));
    println!(
        "overlay_image!: {:?}",
        overlay_image!(qr, &logo).dimensions()
    );

    // Batch, combine, dynamic, multilanguage.
    println!(
        "batch: {}",
        batch_generate_qr!(vec!["a".to_string(), "b".to_string()]).len()
    );
    let combined = combine_qr_codes!(vec![
        QRCode::from_string("one".to_string()),
        QRCode::from_string("two".to_string()),
    ])
    .unwrap();
    println!("combine: {} bytes", combined.data.len());
    println!("dynamic: {} bytes", create_dynamic_qr!("promo").data.len());
    let ml = create_multilanguage_qr! { "en" => "Hello", "fr" => "Bonjour" };
    println!("multilanguage: {:?}", String::from_utf8_lossy(&ml.data));
}
