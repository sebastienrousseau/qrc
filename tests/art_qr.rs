//! Offline art-QR primitives: ControlNet control image + image blend.

use image::{ImageBuffer, Rgba, RgbaImage};
use qrc::{BlendOptions, EcLevel, QRCode};

#[test]
fn control_image_is_square_and_at_least_requested_size() {
    let img = QRCode::from_string("https://example.com".to_string())
        .with_ec_level(EcLevel::H)
        .to_control_image(768);
    assert_eq!(img.width(), img.height());
    assert!(img.width() >= 768);
}

#[test]
fn control_image_grows_to_whole_modules_when_too_small() {
    // A tiny requested size can't fit the modules → the canvas grows to whole
    // modules rather than distorting them.
    let img = QRCode::from_string("hello".to_string()).to_control_image(8);
    assert_eq!(img.width(), img.height());
    assert!(img.width() >= 8);
}

#[test]
fn blend_dimensions_match_total_modules_times_module_size() {
    let qr = QRCode::from_string("https://example.com".to_string()).with_ec_level(EcLevel::H);
    let n = qr.try_to_qrcode().unwrap().width() as u32;
    let opts = BlendOptions {
        module_size: 10,
        ..BlendOptions::default()
    };
    let bg = ImageBuffer::from_pixel(50, 50, Rgba([10, 20, 30, 255]));
    let art = qr.blend_image(&bg, &opts);
    let expected = (n + 8) * 10; // n data modules + 2×4 quiet-zone modules
    assert_eq!(art.dimensions(), (expected, expected));
}

#[test]
fn blend_accepts_empty_background() {
    // An empty background is treated as a blank light canvas (no panic).
    let empty: RgbaImage = ImageBuffer::new(0, 0);
    let art = QRCode::from_string("x".to_string()).blend_image(&empty, &BlendOptions::default());
    assert!(art.width() > 0);
}
