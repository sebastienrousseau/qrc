//! Business-card QR: vCard payload builder + branded logo embedding.

use image::{ImageBuffer, Rgba, RgbaImage};
use qrc::encode::{Ecc, QrOptions};
use qrc::error::QrError;
use qrc::payload::vcard::BusinessCard;
use qrc::render::raster::{LogoOptions, RasterOptions};
use qrc::QRCode;

// ---------------------------------------------------------------------------
// vCard builder
// ---------------------------------------------------------------------------

#[test]
fn full_card_serialises_every_property() {
    let card = BusinessCard::new("Jane Doe")
        .name("Jane", "Doe")
        .organization("Acme")
        .title("CEO")
        .phone("+1-555-0100")
        .email("jane@acme.example")
        .url("https://acme.example")
        .address("1 Market St")
        .note("VIP");
    let v = card.to_vcard();

    assert!(v.starts_with("BEGIN:VCARD\r\nVERSION:3.0\r\n"));
    assert!(v.ends_with("END:VCARD"));
    assert!(v.contains("N:Doe;Jane;;;\r\n"));
    assert!(v.contains("FN:Jane Doe\r\n"));
    assert!(v.contains("ORG:Acme\r\n"));
    assert!(v.contains("TITLE:CEO\r\n"));
    assert!(v.contains("TEL;TYPE=CELL:+1-555-0100\r\n"));
    assert!(v.contains("EMAIL:jane@acme.example\r\n"));
    assert!(v.contains("URL:https://acme.example\r\n"));
    assert!(v.contains("ADR;TYPE=WORK:;;1 Market St;;;;\r\n"));
    assert!(v.contains("NOTE:VIP\r\n"));
    // Display matches to_vcard.
    assert_eq!(card.to_string(), v);
}

#[test]
fn minimal_card_only_emits_fn() {
    let v = BusinessCard::new("Solo").to_vcard();
    assert!(v.contains("FN:Solo\r\n"));
    assert!(!v.contains("\r\nN:"));
    assert!(!v.contains("ORG:"));
    assert_eq!(BusinessCard::default(), BusinessCard::new(""));
}

#[test]
fn special_characters_are_escaped() {
    let v = BusinessCard::new("Doe, Jane; \\CEO\\")
        .note("line1\nline2\rX")
        .to_vcard();
    assert!(v.contains("FN:Doe\\, Jane\\; \\\\CEO\\\\\r\n"));
    // \n is escaped, the bare \r is stripped.
    assert!(v.contains("NOTE:line1\\nline2X\r\n"));
}

// ---------------------------------------------------------------------------
// Logo embedding (branded QR)
// ---------------------------------------------------------------------------

fn logo(w: u32, h: u32) -> RgbaImage {
    ImageBuffer::from_pixel(w, h, Rgba([0, 120, 255, 255]))
}

#[test]
fn logo_options_defaults() {
    let o = LogoOptions::default();
    assert!((o.size_ratio - 0.22).abs() < f32::EPSILON);
    assert_eq!(o.padding, 6);
    assert!(o.background.is_some());
}

#[test]
fn branded_business_card_still_decodes() {
    // The headline use case: a vCard, branded with a centred logo, must still
    // scan back to the exact vCard payload.
    let card = BusinessCard::new("Jane Doe")
        .organization("Acme")
        .email("jane@acme.example");
    let vcard = card.to_vcard();
    let qr = QRCode::from_string(vcard.clone());

    let png = qr
        .to_image_bytes_with_logo(
            &QrOptions::new().ecc(Ecc::High),
            &RasterOptions {
                module_size: 12,
                ..RasterOptions::default()
            },
            &logo(96, 96),
            &LogoOptions {
                size_ratio: 0.18,
                ..LogoOptions::default()
            },
            image::ImageFormat::Png,
        )
        .unwrap();

    let luma = image::load_from_memory(&png).unwrap().into_luma8();
    let mut prepared = rqrr::PreparedImage::prepare(luma);
    let grids = prepared.detect_grids();
    assert_eq!(grids.len(), 1);
    assert_eq!(grids[0].decode().unwrap().1, vcard);
}

#[test]
fn embed_logo_honours_background_some_and_none() {
    let qr = QRCode::from_string("bg test".to_string());
    let opts = QrOptions::new().ecc(Ecc::High);
    let raster = RasterOptions::default();

    let with_pad = qr
        .to_image_with_logo(
            &opts,
            &raster,
            &logo(40, 40),
            &LogoOptions::default(), // background = Some(WHITE)
        )
        .unwrap();
    // Centre pixel falls inside the white knockout.
    let (cw, ch) = with_pad.dimensions();
    assert_eq!(
        with_pad.get_pixel(cw / 2, ch / 2),
        &Rgba([0, 120, 255, 255])
    );

    let no_pad = qr
        .to_image_with_logo(
            &opts,
            &raster,
            &logo(40, 40),
            &LogoOptions {
                background: None,
                ..LogoOptions::default()
            },
        )
        .unwrap();
    assert_eq!(no_pad.dimensions(), with_pad.dimensions());
}

#[test]
fn embed_logo_ignores_empty_logo_and_clamps_ratio() {
    let qr = QRCode::from_string("clamp".to_string());
    let opts = QrOptions::new();
    let raster = RasterOptions::default();

    // Empty logo leaves the rendered image unchanged and does not panic.
    let empty = RgbaImage::new(0, 0);
    let out = qr
        .to_image_with_logo(&opts, &raster, &empty, &LogoOptions::default())
        .unwrap();
    let plain = qrc::render::raster::render(&qr.encode(&opts).unwrap(), &raster);
    assert_eq!(out.dimensions(), plain.dimensions());
    assert_eq!(out.into_raw(), plain.into_raw());

    // An out-of-range size_ratio is clamped (no panic / no overflow).
    let oversized_ratio = qr
        .to_image_with_logo(
            &opts,
            &raster,
            &logo(50, 50),
            &LogoOptions {
                size_ratio: 5.0,
                padding: 2,
                background: Some(qrc::render::style::Color::WHITE),
            },
        )
        .unwrap();
    assert!(oversized_ratio.width() > 0);
}

#[test]
fn logo_methods_propagate_errors() {
    let huge = QRCode::from_string("Z".repeat(8000));
    let opts = QrOptions::new();
    let raster = RasterOptions::default();
    assert!(huge
        .to_image_with_logo(&opts, &raster, &logo(10, 10), &LogoOptions::default())
        .is_err());

    // Over-capacity data fails before rendering.
    assert!(huge
        .to_image_bytes_with_logo(
            &opts,
            &raster,
            &logo(10, 10),
            &LogoOptions::default(),
            image::ImageFormat::Png,
        )
        .is_err());

    // A valid code but an unsupported output format maps to a Render error.
    let qr = QRCode::from_string("ok".to_string());
    let err = qr
        .to_image_bytes_with_logo(
            &opts,
            &raster,
            &logo(10, 10),
            &LogoOptions::default(),
            image::ImageFormat::Dds,
        )
        .unwrap_err();
    assert!(matches!(err, QrError::Render(_)));
}

#[test]
fn image_to_bytes_round_trips_a_branded_image() {
    let qr = QRCode::from_string("https://example.com/brand".to_string());
    let opts = QrOptions::new().ecc(Ecc::High);
    let raster = RasterOptions {
        module_size: 10,
        ..RasterOptions::default()
    };
    let img = qr
        .to_image_with_logo(&opts, &raster, &logo(60, 60), &LogoOptions::default())
        .unwrap();
    let bytes = qrc::render::raster::image_to_bytes(&img, image::ImageFormat::Png).unwrap();
    assert_eq!(
        &bytes[..8],
        &[0x89, b'P', b'N', b'G', 0x0D, 0x0A, 0x1A, 0x0A]
    );
}
