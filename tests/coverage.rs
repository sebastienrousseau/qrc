//! Exhaustive coverage tests for every public function and code path.
//!
//! Split by area: legacy `QRCode` helpers, the `encode` layer, the `error`
//! types, and renderer edge cases. Round-trip / behavioural assertions live in
//! `qr.rs` and `phase1.rs`; this file fills the remaining branches.

use std::collections::HashMap;

use qrc::encode::{encode, Ecc, Engine, QrOptions, QrcodeEngine};
use qrc::error::QrError;
use qrc::render::style::{Color, ModuleShape};
use qrc::render::svg::SvgOptions;
use qrc::QRCode;

// ---------------------------------------------------------------------------
// Legacy QRCode helpers
// ---------------------------------------------------------------------------

#[test]
fn create_multilanguage_selects_english_when_present() {
    let mut map = HashMap::new();
    map.insert("en".to_string(), "Hello".to_string());
    map.insert("fr".to_string(), "Bonjour".to_string());
    let qr = QRCode::create_multilanguage(map);
    assert_eq!(qr.data, b"Hello");
}

#[test]
fn create_multilanguage_is_empty_without_english() {
    let mut map = HashMap::new();
    map.insert("fr".to_string(), "Bonjour".to_string());
    let qr = QRCode::create_multilanguage(map);
    assert!(qr.data.is_empty());
}

#[test]
fn create_dynamic_wraps_a_redirect_url() {
    let qr = QRCode::create_dynamic("promo-42");
    let s = String::from_utf8(qr.data).unwrap();
    assert!(s.starts_with("https://your-api-endpoint.com/update?qrcode="));
    assert!(s.ends_with("promo-42"));
}

#[test]
fn batch_generate_produces_one_code_per_item() {
    let codes = QRCode::batch_generate_qr_codes(vec![
        "https://a.example".to_string(),
        "https://b.example".to_string(),
        "https://c.example".to_string(),
    ]);
    assert_eq!(codes.len(), 3);
    assert_eq!(codes[1].data, b"https://b.example");
}

#[test]
fn set_encoding_format_accepts_utf8_and_rejects_others() {
    let qr = QRCode::new(b"data".to_vec());
    let ok = qr.set_encoding_format("utf-8").unwrap();
    assert_eq!(ok.get_encoding_format(), "utf-8");
    assert!(qr.set_encoding_format("latin-1").is_err());
}

#[test]
#[should_panic(expected = "could not be encoded")]
fn to_qrcode_panics_on_oversized_data() {
    // Exercises the documented panic branch of the infallible `to_qrcode`.
    let qr = QRCode::from_string("Z".repeat(8000));
    let _ = qr.to_qrcode();
}

#[cfg(feature = "raster")]
mod raster_helpers {
    use super::*;
    use image::{ImageBuffer, Rgba, RgbaImage};

    #[test]
    fn combine_qr_codes_concatenates_codes() {
        let combined = QRCode::combine_qr_codes(vec![
            QRCode::from_string("one".to_string()),
            QRCode::from_string("two".to_string()),
        ])
        .unwrap();
        assert!(!combined.data.is_empty());
    }

    #[test]
    fn combine_qr_codes_rejects_empty_input() {
        assert_eq!(
            QRCode::combine_qr_codes(vec![]).unwrap_err(),
            "No QR codes to combine"
        );
    }

    #[test]
    fn combine_qr_codes_surfaces_encode_errors() {
        let err = QRCode::combine_qr_codes(vec![
            QRCode::from_string("ok".to_string()),
            QRCode::from_string("Z".repeat(8000)),
        ])
        .unwrap_err();
        assert_eq!(err, "one of the QR codes could not be encoded");
    }

    #[test]
    fn overlay_image_places_overlay_on_top() {
        let qr = QRCode::from_string("logo me".to_string());
        let logo: RgbaImage = ImageBuffer::from_pixel(8, 8, Rgba([10, 20, 30, 255]));
        let out = qr.overlay_image(&logo);
        assert_eq!(out.get_pixel(0, 0), &Rgba([10, 20, 30, 255]));
    }
}

// ---------------------------------------------------------------------------
// encode layer
// ---------------------------------------------------------------------------

#[test]
fn every_ecc_level_encodes() {
    for ecc in [Ecc::Low, Ecc::Medium, Ecc::Quartile, Ecc::High] {
        let m = encode(b"ecc-coverage", &QrOptions::new().ecc(ecc)).unwrap();
        assert!(m.size() >= 21);
    }
}

#[test]
fn qr_options_builder_and_defaults() {
    let opts = QrOptions::new().ecc(Ecc::Quartile).version(3).quiet_zone(6);
    assert_eq!(opts.ecc, Ecc::Quartile);
    assert_eq!(opts.version, Some(3));
    assert_eq!(opts.quiet_zone, 6);
    // Default derives.
    assert_eq!(QrOptions::default(), QrOptions::new());
    assert_eq!(Ecc::default(), Ecc::Medium);

    let m = encode(b"hi", &opts).unwrap();
    assert_eq!(m.quiet_zone(), 6);
}

#[test]
fn forcing_too_small_a_version_is_an_invalid_version_error() {
    // Version 1 cannot hold this much data → InvalidVersion (not a panic).
    let err = encode(&vec![b'A'; 2000], &QrOptions::new().version(1)).unwrap_err();
    assert_eq!(err, QrError::InvalidVersion(1));
}

#[test]
fn out_of_range_version_is_rejected() {
    assert_eq!(
        encode(b"x", &QrOptions::new().version(0)).unwrap_err(),
        QrError::InvalidVersion(0)
    );
    assert_eq!(
        encode(b"x", &QrOptions::new().version(41)).unwrap_err(),
        QrError::InvalidVersion(41)
    );
}

#[test]
fn engine_default_and_clone_are_usable() {
    let engine = QrcodeEngine;
    let copy = engine; // Copy
    let m = copy.encode(b"engine", &QrOptions::default()).unwrap();
    assert!(m.size() >= 21);
}

// ---------------------------------------------------------------------------
// error types
// ---------------------------------------------------------------------------

#[test]
fn every_error_variant_displays() {
    assert_eq!(
        QrError::DataTooLong.to_string(),
        "data is too long to encode as a QR code"
    );
    assert_eq!(
        QrError::InvalidVersion(7).to_string(),
        "invalid or insufficient QR version: 7"
    );
    assert_eq!(
        QrError::Encode("boom").to_string(),
        "QR encoding failed: boom"
    );
    assert_eq!(
        QrError::Render("boom").to_string(),
        "QR rendering failed: boom"
    );
}

#[test]
fn error_derives_and_trait_object() {
    let e = QrError::DataTooLong;
    let cloned = e.clone();
    assert_eq!(e, cloned);
    assert!(format!("{e:?}").contains("DataTooLong"));
    let dyn_err: &dyn std::error::Error = &e;
    assert!(!dyn_err.to_string().is_empty());
}

#[test]
fn from_qrcode_error_maps_each_variant() {
    use qrcode::types::QrError as E;
    assert_eq!(QrError::from(E::DataTooLong), QrError::DataTooLong);
    assert_eq!(
        QrError::from(E::InvalidVersion),
        QrError::Encode("invalid version")
    );
    assert_eq!(
        QrError::from(E::UnsupportedCharacterSet),
        QrError::Encode("unsupported character set")
    );
    assert_eq!(
        QrError::from(E::InvalidEciDesignator),
        QrError::Encode("invalid ECI designator")
    );
    assert_eq!(
        QrError::from(E::InvalidCharacter),
        QrError::Encode("invalid character")
    );
}

// ---------------------------------------------------------------------------
// style + svg renderer edge cases
// ---------------------------------------------------------------------------

#[test]
fn color_constructors_and_accessors() {
    assert_eq!(Color::BLACK.to_array(), [0, 0, 0, 255]);
    assert_eq!(Color::WHITE.to_array(), [255, 255, 255, 255]);
    assert_eq!(Color::rgb(1, 2, 3).to_array(), [1, 2, 3, 255]);
    assert_eq!(Color::rgba(1, 2, 3, 4).to_array(), [1, 2, 3, 4]);
    assert_eq!(Color::rgb(0x11, 0x22, 0x33).to_hex(), "#112233");
    assert_eq!(Color::default(), Color::BLACK);
    assert!((Color::rgba(0, 0, 0, 0).opacity() - 0.0).abs() < f32::EPSILON);
    assert!((Color::BLACK.opacity() - 1.0).abs() < f32::EPSILON);
    // Debug + equality on ModuleShape.
    assert_eq!(ModuleShape::default(), ModuleShape::Square);
    assert!(format!("{:?}", ModuleShape::Circle).contains("Circle"));
}

#[test]
fn svg_options_with_module_size_and_translucent_fill() {
    let opts = SvgOptions::with_module_size(12);
    assert_eq!(opts.module_size, 12);

    let qr = QRCode::from_string("translucent".to_string());
    // A semi-transparent dark color exercises the fill-opacity branch.
    let svg = qr
        .to_svg_styled(
            &QrOptions::new(),
            &SvgOptions {
                dark: Color::rgba(0, 0, 0, 128),
                ..SvgOptions::default()
            },
        )
        .unwrap();
    assert!(svg.contains("fill-opacity="));
}

#[test]
fn renderers_propagate_encode_errors() {
    // An over-capacity payload makes the inner `encode(...)?` fail, exercising
    // the error-propagation branch of every high-level renderer method.
    let qr = QRCode::from_string("Z".repeat(8000));
    let opts = QrOptions::new();
    assert!(qr.to_svg_styled(&opts, &SvgOptions::default()).is_err());
    assert!(qr.to_unicode(&opts).is_err());
}

#[cfg(feature = "raster")]
#[test]
fn raster_methods_propagate_encode_errors() {
    use qrc::render::raster::RasterOptions;
    let qr = QRCode::from_string("Z".repeat(8000));
    let opts = QrOptions::new();
    let raster = RasterOptions::default();
    assert!(qr.to_png_bytes(&opts, &raster).is_err());
    assert!(qr.to_jpeg_bytes(&opts, &raster).is_err());
    assert!(qr.to_gif_bytes(&opts, &raster).is_err());
    assert!(qr
        .to_image_bytes(&opts, &raster, image::ImageFormat::Png)
        .is_err());
}

#[cfg(feature = "raster")]
#[test]
fn to_image_bytes_supports_extra_formats_and_maps_failures() {
    use qrc::error::QrError;
    use qrc::render::raster::RasterOptions;
    let qr = QRCode::from_string("https://example.com/bmp".to_string());
    let opts = QrOptions::new();
    let raster = RasterOptions::default();

    // A supported format (BMP) produces bytes.
    let bmp = qr
        .to_image_bytes(&opts, &raster, image::ImageFormat::Bmp)
        .unwrap();
    assert_eq!(&bmp[..2], b"BM");

    // A format with no encoder maps to a Render error rather than panicking.
    let err = qr
        .to_image_bytes(&opts, &raster, image::ImageFormat::Dds)
        .unwrap_err();
    assert!(matches!(err, QrError::Render(_)));
}

#[test]
fn svg_clamps_zero_module_size_to_one() {
    let qr = QRCode::from_string("x".to_string());
    let svg = qr
        .to_svg_styled(
            &QrOptions::new(),
            &SvgOptions {
                module_size: 0,
                ..SvgOptions::default()
            },
        )
        .unwrap();
    assert!(svg.starts_with("<svg"));
}
