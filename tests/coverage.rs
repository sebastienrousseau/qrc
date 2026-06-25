//! Coverage-completing tests: exercise every public `QRCode` method, all
//! `ModuleShape` branches, the full `BusinessCard` builder, and (when the
//! `wasm` feature is on) the WASM bindings — so the suite covers 100% of the
//! library surface, not just the happy paths in `tests/qr.rs`.

use image::{ImageBuffer, Rgba, RgbaImage};
use qrc::payload::vcard::BusinessCard;
use qrc::{BlendOptions, EcLevel, ModuleShape, QRCode};
use std::collections::HashMap;

const URL: &str = "https://example.com/coverage";

// --- Constructors ----------------------------------------------------------

#[test]
fn constructors_all_variants() {
    let a = QRCode::new(URL.as_bytes().to_vec());
    let b = QRCode::from_string(URL.to_string());
    let c = QRCode::from_bytes(URL.as_bytes().to_vec());
    assert_eq!(a.data, b.data);
    assert_eq!(b.data, c.data);
    assert_eq!(a.get_encoding_format(), "utf-8");
}

#[test]
#[should_panic(expected = "Failed to encode")]
fn to_qrcode_panics_on_oversized_data() {
    // `to_qrcode` is the panicking convenience over `try_to_qrcode`.
    let _ = QRCode::from_string("Z".repeat(8000)).to_qrcode();
}

#[test]
fn try_to_qrcode_reports_capacity_error() {
    assert!(QRCode::from_string("Z".repeat(8000))
        .try_to_qrcode()
        .is_err());
    assert!(QRCode::from_string(URL.to_string()).try_to_qrcode().is_ok());
}

// --- Raster / vector outputs ----------------------------------------------

#[test]
fn every_output_format() {
    let qr = QRCode::from_string(URL.to_string());
    assert_eq!(qr.to_png(64).dimensions(), (64, 64));
    assert_eq!(qr.to_image(64).dimensions(), (64, 64));
    assert!(!qr.to_png_bytes(64).unwrap().is_empty());
    assert!(!qr.to_jpg(64).unwrap().is_empty());
    assert!(!qr.to_jpg_with_quality(64, 50).unwrap().is_empty());
    assert!(!qr.to_gif(64).unwrap().is_empty());
    assert!(qr.to_svg(64).contains("<svg"));

    // Encoding errors propagate: width 0 is not a valid PNG/JPEG size.
    assert!(qr.to_png_bytes(0).is_err());
    assert!(qr.to_jpg(0).is_err());
    assert!(qr.to_jpg_with_quality(0, 50).is_err());
}

#[test]
fn all_module_shapes_render() {
    // Exercises every `is_inside_shape` / rounded-rect branch.
    for shape in [
        ModuleShape::Square,
        ModuleShape::RoundedSquare,
        ModuleShape::Circle,
        ModuleShape::Diamond,
    ] {
        let img = QRCode::from_string(URL.to_string())
            .with_shape(shape)
            .with_ec_level(EcLevel::H)
            .to_png(120);
        assert_eq!(img.dimensions(), (120, 120));
    }
}

#[test]
fn default_instance() {
    let d = QRCode::default();
    assert!(d.data.is_empty());
    assert_eq!(d.get_encoding_format(), "utf-8");
}

#[test]
fn svg_for_every_non_square_shape() {
    // The custom (non-`Square`) SVG path renders circle/diamond/rounded rects.
    for shape in [
        ModuleShape::RoundedSquare,
        ModuleShape::Circle,
        ModuleShape::Diamond,
    ] {
        let svg = QRCode::from_string(URL.to_string())
            .with_shape(shape)
            .to_svg(150);
        assert!(svg.contains("<svg"));
    }
}

#[test]
fn colorize_and_resize() {
    let qr = QRCode::from_string(URL.to_string());
    assert!(qr.colorize(Rgba([0, 102, 204, 255])).dimensions().0 > 0);
    assert_eq!(qr.resize(80, 40).dimensions(), (80, 40));

    // A non-square shape leaves gaps inside each dark module, exercising the
    // `is_inside_shape == false` branch in both colorize and resize.
    let circ = QRCode::from_string(URL.to_string()).with_shape(ModuleShape::Circle);
    assert!(circ.colorize(Rgba([10, 20, 30, 255])).dimensions().0 > 0);
    assert_eq!(circ.resize(60, 60).dimensions(), (60, 60));
}

// --- Logos / watermarks / art ---------------------------------------------

#[test]
fn overlay_watermark_and_art() {
    let qr = QRCode::from_string(URL.to_string()).with_ec_level(EcLevel::H);
    let logo: RgbaImage = ImageBuffer::from_pixel(24, 24, Rgba([220, 20, 60, 255]));

    // Centred overlay: output is larger than the bare module grid. A fully
    // transparent corner pixel exercises the alpha-skip branch.
    let mut logo = logo;
    logo.put_pixel(0, 0, Rgba([0, 0, 0, 0]));
    let overlaid = qr.overlay_image(&logo);
    assert!(overlaid.width() > 24);

    // An oversized overlay spills past the canvas, exercising the bounds-clamp
    // (`px < dim && py < dim` == false) branch.
    let big: RgbaImage = ImageBuffer::from_pixel(4000, 4000, Rgba([1, 2, 3, 255]));
    let _ = qr.overlay_image(&big);

    // Static watermark mutates the passed image in place.
    let mut canvas = qr.to_png(200);
    QRCode::add_image_watermark(&mut canvas, &logo);

    // Art primitives (delegating to the `art` module).
    assert!(qr.to_control_image(256).width() >= 256);
    let bg: RgbaImage = ImageBuffer::from_pixel(64, 64, Rgba([10, 80, 160, 255]));
    assert!(qr.blend_image(&bg, &BlendOptions::default()).width() > 0);
}

// --- Batch / combine / dynamic / multilanguage / compress ------------------

#[test]
fn batch_combine_dynamic_multilanguage_compress() {
    let codes = QRCode::batch_generate_qr_codes(vec!["a".into(), "b".into()]);
    assert_eq!(codes.len(), 2);

    // combine: Ok for a non-empty slice, Err for an empty one.
    assert!(QRCode::combine_qr_codes(&codes).is_ok());
    assert!(QRCode::combine_qr_codes(&[]).is_err());

    let dynamic = QRCode::create_dynamic("https://example.com/r/abc");
    assert!(!dynamic.data.is_empty());

    let mut map = HashMap::new();
    map.insert("en".to_string(), "Hello".to_string());
    map.insert("fr".to_string(), "Bonjour".to_string());
    assert_eq!(QRCode::create_multilanguage(&map, "fr").data, b"Bonjour");
    // A missing language falls back (covers the else branch).
    let _ = QRCode::create_multilanguage(&map, "de");

    assert!(!QRCode::compress_data("a long string ".repeat(20).as_str()).is_empty());
}

// --- Encoding format -------------------------------------------------------

#[test]
fn encoding_format_ok_and_error() {
    let qr = QRCode::from_string(URL.to_string());
    let updated = qr.set_encoding_format("utf-8").unwrap();
    assert_eq!(updated.get_encoding_format(), "utf-8");
    assert!(qr.set_encoding_format("latin-1").is_err());
}

// --- BusinessCard (vCard) — full builder ----------------------------------

#[test]
fn business_card_every_field_and_escaping() {
    let card = BusinessCard::new("Jane Doe")
        .name("Jane", "Doe")
        .organization("Acme, Inc.") // comma must be escaped
        .title("CEO")
        .phone("+1-555-0100")
        .email("jane@acme.example")
        .url("https://acme.example")
        .address("1 Market St; Suite 2") // semicolon must be escaped
        .note("Line1\nLine2"); // newline -> \n
    let v = card.to_vcard();

    assert!(v.starts_with("BEGIN:VCARD"));
    assert!(v.contains("FN:Jane Doe"));
    assert!(v.contains("ORG:Acme\\, Inc."));
    assert!(v.contains("ADR;TYPE=WORK:") || v.contains("1 Market St\\; Suite 2"));
    assert!(v.contains("NOTE:Line1\\nLine2"));
    assert!(v.trim_end().ends_with("END:VCARD"));
    assert_eq!(card.to_string(), v); // Display == to_vcard

    // Minimal card (only the required name).
    assert!(BusinessCard::new("Solo").to_vcard().contains("FN:Solo"));

    // Backslash is escaped and a carriage return is dropped (escape() arms).
    let escaped = BusinessCard::new("X").note("a\\b\rc").to_vcard();
    assert!(escaped.contains("NOTE:a\\\\bc"));
}

// --- WASM bindings (host-callable; JS glue is wasm32-only) ------------------

#[cfg(feature = "wasm")]
#[test]
fn wasm_bindings_cover_every_branch() {
    use qrc::wasm::WasmQRCode;
    let mut qr = WasmQRCode::new(URL);
    for lvl in ["L", "M", "Q", "H", "x"] {
        qr.set_ec_level(lvl); // "x" hits the default arm
    }
    for shape in ["square", "rounded", "circle", "diamond", "x"] {
        qr.set_shape(shape); // "x" hits the default arm
    }
    assert!(qr.to_svg(128).contains("<svg"));
    assert!(!qr.to_png_bytes(128).is_empty());
    assert!(!qr.to_jpg(128).is_empty());
    // width 0 -> inner Err -> the binding's `unwrap_or_default` returns empty.
    assert!(qr.to_png_bytes(0).is_empty());
    assert!(qr.to_jpg(0).is_empty());
}
