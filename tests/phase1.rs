//! Integration tests for the Phase 1 layered API (encode + render).

use qrc::encode::{Ecc, Engine, QrOptions, QrcodeEngine};
use qrc::render::raster::RasterOptions;
use qrc::render::svg::SvgOptions;
use qrc::render::{Color, ModuleShape};
use qrc::QRCode;

const PAYLOAD: &str = "https://example.com/phase1";

/// Decodes a raster-encoded byte blob back to its payload using an independent
/// decoder, proving the format encoder produced a scannable image.
fn decode_bytes(bytes: &[u8]) -> String {
    let luma = image::load_from_memory(bytes)
        .expect("bytes should be a valid image")
        .into_luma8();
    let mut prepared = rqrr::PreparedImage::prepare(luma);
    let grids = prepared.detect_grids();
    assert_eq!(grids.len(), 1, "exactly one QR grid expected");
    grids[0].decode().expect("grid should decode").1
}

#[test]
fn encode_produces_matrix_with_quiet_zone() {
    let qr = QRCode::from_string(PAYLOAD.to_string());
    let matrix = qr.encode(&QrOptions::new()).unwrap();
    assert_eq!(matrix.quiet_zone(), 4);
    assert_eq!(matrix.total_size(), matrix.size() + 8);
    // Out-of-range / quiet-zone coordinates are light.
    assert!(!matrix.is_dark(matrix.size(), 0));
    assert!(!matrix.is_dark_with_quiet_zone(0, 0));
}

#[test]
fn forced_version_sets_matrix_size() {
    // Version 1 is 21x21, version 5 is 37x37 (v * 4 + 17).
    let qr = QRCode::from_string("hi".to_string());
    let m1 = qr.encode(&QrOptions::new().version(1)).unwrap();
    assert_eq!(m1.size(), 21);
    let m5 = qr.encode(&QrOptions::new().version(5)).unwrap();
    assert_eq!(m5.size(), 37);
}

#[test]
fn invalid_version_is_an_error_not_a_panic() {
    let qr = QRCode::from_string("hi".to_string());
    assert!(qr.encode(&QrOptions::new().version(99)).is_err());
}

#[test]
fn oversized_payload_is_an_error_not_a_panic() {
    // Far beyond byte-mode capacity at the smallest version.
    let big = "A".repeat(8000);
    let qr = QRCode::from_string(big);
    assert!(qr.encode(&QrOptions::new()).is_err());
}

#[test]
fn higher_ecc_needs_more_or_equal_modules() {
    let qr = QRCode::from_string(PAYLOAD.to_string());
    let low = qr.encode(&QrOptions::new().ecc(Ecc::Low)).unwrap();
    let high = qr.encode(&QrOptions::new().ecc(Ecc::High)).unwrap();
    assert!(high.size() >= low.size());
}

#[test]
fn engine_trait_is_object_usable() {
    let engine: &dyn Engine = &QrcodeEngine;
    let matrix = engine
        .encode(PAYLOAD.as_bytes(), &QrOptions::new())
        .unwrap();
    assert!(matrix.size() >= 21);
}

#[test]
fn png_bytes_round_trip_decodes() {
    let qr = QRCode::from_string(PAYLOAD.to_string());
    let opts = QrOptions::new();
    let bytes = qr.to_png_bytes(&opts, &RasterOptions::default()).unwrap();
    // PNG magic number.
    assert_eq!(
        &bytes[..8],
        &[0x89, b'P', b'N', b'G', 0x0D, 0x0A, 0x1A, 0x0A]
    );
    assert_eq!(decode_bytes(&bytes), PAYLOAD);
}

#[test]
fn jpeg_bytes_round_trip_decodes() {
    let qr = QRCode::from_string(PAYLOAD.to_string());
    // Larger modules keep JPEG artefacts from breaking the decode.
    let raster = RasterOptions {
        module_size: 10,
        ..RasterOptions::default()
    };
    let bytes = qr.to_jpeg_bytes(&QrOptions::new(), &raster).unwrap();
    assert_eq!(&bytes[..2], &[0xFF, 0xD8]); // JPEG SOI marker
    assert_eq!(decode_bytes(&bytes), PAYLOAD);
}

#[test]
fn gif_bytes_round_trip_decodes() {
    let qr = QRCode::from_string(PAYLOAD.to_string());
    let bytes = qr
        .to_gif_bytes(&QrOptions::new(), &RasterOptions::default())
        .unwrap();
    assert_eq!(&bytes[..3], b"GIF");
    assert_eq!(decode_bytes(&bytes), PAYLOAD);
}

#[test]
fn styled_svg_is_well_formed() {
    let qr = QRCode::from_string(PAYLOAD.to_string());
    let svg = qr
        .to_svg_styled(
            &QrOptions::new(),
            &SvgOptions {
                module_size: 8,
                dark: Color::rgb(0x11, 0x22, 0x33),
                light: Color::WHITE,
                shape: ModuleShape::Rounded,
            },
        )
        .unwrap();
    assert!(svg.starts_with("<svg"));
    assert!(svg.ends_with("</svg>"));
    assert!(svg.contains("fill=\"#112233\""));
    assert!(svg.contains("rx=")); // rounded modules
}

#[test]
fn circle_svg_uses_circles() {
    let qr = QRCode::from_string(PAYLOAD.to_string());
    let svg = qr
        .to_svg_styled(
            &QrOptions::new(),
            &SvgOptions {
                shape: ModuleShape::Circle,
                ..SvgOptions::default()
            },
        )
        .unwrap();
    assert!(svg.contains("<circle"));
}

#[test]
fn unicode_output_is_non_empty_and_blocky() {
    let qr = QRCode::from_string(PAYLOAD.to_string());
    let text = qr.to_unicode(&QrOptions::new()).unwrap();
    assert!(!text.is_empty());
    assert!(text.contains('\u{2588}') || text.contains('\u{2580}') || text.contains('\u{2584}'));
}

#[test]
fn fit_width_never_exceeds_target() {
    let qr = QRCode::from_string(PAYLOAD.to_string());
    let matrix = qr.encode(&QrOptions::new()).unwrap();
    let raster = RasterOptions::fit_width(&matrix, 300);
    let dim = matrix.total_size() as u32 * raster.module_size;
    assert!(dim <= 300);
    assert!(raster.module_size >= 1);
}
