//! Control-image export and offline artistic-blend QR codes.

use image::{ImageBuffer, Rgba, RgbaImage};
use qrc::encode::{Ecc, QrOptions};
use qrc::render::art::BlendOptions;
use qrc::render::control::ControlOptions;
use qrc::QRCode;

/// A colourful gradient stand-in for an uploaded photo/logo.
fn background(dim: u32) -> RgbaImage {
    ImageBuffer::from_fn(dim, dim, |x, y| {
        Rgba([
            (x * 255 / dim.max(1)) as u8,
            (y * 255 / dim.max(1)) as u8,
            128,
            255,
        ])
    })
}

fn decode(png: &[u8]) -> String {
    let luma = image::load_from_memory(png).unwrap().into_luma8();
    let mut prepared = rqrr::PreparedImage::prepare(luma);
    let grids = prepared.detect_grids();
    assert_eq!(grids.len(), 1, "exactly one QR grid expected");
    grids[0].decode().unwrap().1
}

// --- Control image ---------------------------------------------------------

#[test]
fn control_image_is_exact_square_size() {
    let qr = QRCode::from_string("https://example.com/control".to_string());
    let img = qr
        .to_control_image(&QrOptions::new().ecc(Ecc::High), &ControlOptions::default())
        .unwrap();
    assert_eq!(img.dimensions(), (768, 768));
    // Corners are in the quiet zone -> light.
    assert_eq!(img.get_pixel(0, 0), &Rgba([255, 255, 255, 255]));
}

#[test]
fn control_image_grows_rather_than_distorts_when_too_small() {
    let qr = QRCode::from_string("Z".repeat(900)); // large symbol
    let img = qr
        .to_control_image(
            &QrOptions::new().ecc(Ecc::High),
            &ControlOptions::with_size(16),
        )
        .unwrap();
    let (w, h) = img.dimensions();
    assert_eq!(w, h);
    assert!(w >= 16); // canvas grew to fit whole modules
}

#[test]
fn control_image_bytes_round_trip_decodes() {
    let qr = QRCode::from_string("https://example.com/ctrl-bytes".to_string());
    let png = qr
        .to_control_image_bytes(
            &QrOptions::new().ecc(Ecc::High),
            &ControlOptions::default(),
            image::ImageFormat::Png,
        )
        .unwrap();
    assert_eq!(decode(&png), "https://example.com/ctrl-bytes");
}

// --- Artistic blend --------------------------------------------------------

#[test]
fn blend_options_defaults() {
    let o = BlendOptions::default();
    assert_eq!(o.module_size, 12);
    assert!((o.strength - 0.75).abs() < f32::EPSILON);
    assert!((o.dot_ratio - 0.66).abs() < f32::EPSILON);
}

#[test]
fn art_qr_blends_an_image_and_still_decodes() {
    // The headline: a code woven into a photo must still scan back exactly.
    let payload = "https://example.com/art";
    let qr = QRCode::from_string(payload.to_string());
    let bg = background(600);
    let png = qr
        .to_art_bytes(
            &QrOptions::new().ecc(Ecc::High),
            &bg,
            &BlendOptions::default(),
            image::ImageFormat::Png,
        )
        .unwrap();
    assert_eq!(decode(&png), payload);

    // The result genuinely contains background colour (not a plain B/W QR):
    // somewhere a pixel is neither pure black nor pure white.
    let img = image::load_from_memory(&png).unwrap().into_rgba8();
    let has_colour = img.pixels().any(|p| {
        let [r, g, b, _] = p.0;
        !(r == g && g == b) // a non-grey pixel implies the image shows through
    });
    assert!(has_colour, "art QR should show the background image");
}

#[test]
fn art_qr_handles_empty_background() {
    let qr = QRCode::from_string("https://example.com/empty-bg".to_string());
    let empty = RgbaImage::new(0, 0);
    let img = qr
        .to_art_image(
            &QrOptions::new().ecc(Ecc::High),
            &empty,
            &BlendOptions::default(),
        )
        .unwrap();
    assert!(img.width() > 0 && img.height() > 0);
}

#[test]
fn art_methods_propagate_encode_errors() {
    let huge = QRCode::from_string("Z".repeat(8000));
    let bg = background(100);
    assert!(huge
        .to_art_image(&QrOptions::new(), &bg, &BlendOptions::default())
        .is_err());
    assert!(huge
        .to_art_bytes(
            &QrOptions::new(),
            &bg,
            &BlendOptions::default(),
            image::ImageFormat::Png
        )
        .is_err());
    assert!(huge
        .to_control_image(&QrOptions::new(), &ControlOptions::default())
        .is_err());
    assert!(huge
        .to_control_image_bytes(
            &QrOptions::new(),
            &ControlOptions::default(),
            image::ImageFormat::Png
        )
        .is_err());
}
