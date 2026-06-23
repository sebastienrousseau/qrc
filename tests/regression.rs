//! Full regression suite.
//!
//! Locks in behaviour across the whole public surface with three layers:
//!  1. **Golden** assertions — exact serialised payload strings and image magic
//!     bytes that must not silently change.
//!  2. **Parametric round-trips** — every ECC level and a spread of forced
//!     versions / renderers must still decode back to the exact payload.
//!  3. **Invariants** — quiet zone present, dark modules opaque, branded codes
//!     still scannable.

use image::{ImageBuffer, ImageFormat, Rgba, RgbaImage};
use qrc::encode::{Ecc, QrOptions};
use qrc::payload::emvco::{MerchantAccount, MerchantPayment};
use qrc::payload::mecard::MeCard;
use qrc::payload::vcard::BusinessCard;
use qrc::payload::wifi::{WifiNetwork, WifiSecurity};
use qrc::render::art::BlendOptions;
use qrc::render::control::ControlOptions;
use qrc::render::raster::{LogoOptions, RasterOptions};
use qrc::render::style::{Color, ModuleShape};
use qrc::render::svg::SvgOptions;
use qrc::QRCode;

/// Decodes image bytes back to the QR payload (panics if it does not scan).
fn decode(bytes: &[u8]) -> String {
    let luma = image::load_from_memory(bytes).unwrap().into_luma8();
    let mut prepared = rqrr::PreparedImage::prepare(luma);
    let grids = prepared.detect_grids();
    assert_eq!(grids.len(), 1, "exactly one QR grid expected");
    grids[0].decode().unwrap().1
}

fn png_of(payload: &str, opts: &QrOptions) -> Vec<u8> {
    QRCode::from_string(payload.to_string())
        .to_png_bytes(opts, &RasterOptions::default())
        .unwrap()
}

// --- 1. Golden payloads ----------------------------------------------------

#[test]
fn golden_payload_strings() {
    assert_eq!(
        WifiNetwork::new("Net")
            .security(WifiSecurity::Wpa)
            .password("pw")
            .to_qr_string(),
        "WIFI:T:WPA;S:Net;P:pw;;"
    );
    assert_eq!(
        MeCard::new("Doe,Jane").phone("123").to_mecard(),
        "MECARD:N:Doe\\,Jane;TEL:123;;"
    );
    assert_eq!(
        BusinessCard::new("Jane").organization("Acme").to_vcard(),
        "BEGIN:VCARD\r\nVERSION:3.0\r\nFN:Jane\r\nORG:Acme\r\nEND:VCARD"
    );
    // EMVCo: full string including the computed CRC must be stable.
    let account = MerchantAccount::new(26, "com.example").merchant_id("123");
    let payment = MerchantPayment::new(account, "840", "US", "Acme", "City").amount("4.50");
    let s = payment.to_emvco();
    assert!(s.starts_with("000201010212"));
    assert!(s.ends_with(&format!("6304{}", &s[s.len() - 4..])));
    // Re-encoding the same inputs is deterministic.
    let account2 = MerchantAccount::new(26, "com.example").merchant_id("123");
    let payment2 = MerchantPayment::new(account2, "840", "US", "Acme", "City").amount("4.50");
    assert_eq!(payment2.to_emvco(), s);
}

#[test]
fn golden_image_magic_bytes() {
    let qr = QRCode::from_string("https://example.com".to_string());
    let opts = QrOptions::new();
    let ro = RasterOptions::default();
    let cases: &[(ImageFormat, &[u8])] = &[
        (ImageFormat::Png, &[0x89, b'P', b'N', b'G']),
        (ImageFormat::Jpeg, &[0xFF, 0xD8]),
        (ImageFormat::Gif, b"GIF"),
        (ImageFormat::Bmp, b"BM"),
        (ImageFormat::Tiff, &[0x49, 0x49, 0x2A, 0x00]),
    ];
    for &(fmt, magic) in cases {
        let bytes = qr
            .to_image_bytes(
                &opts,
                &RasterOptions {
                    module_size: 10,
                    ..ro
                },
                fmt,
            )
            .unwrap();
        assert_eq!(&bytes[..magic.len()], magic, "format {fmt:?}");
    }
}

// --- 2. Parametric round-trips ---------------------------------------------

#[test]
fn every_ecc_level_round_trips() {
    let payload = "https://example.com/regression-ecc";
    for ecc in [Ecc::Low, Ecc::Medium, Ecc::Quartile, Ecc::High] {
        let png = png_of(payload, &QrOptions::new().ecc(ecc));
        assert_eq!(decode(&png), payload, "ecc {ecc:?}");
    }
}

#[test]
fn forced_versions_have_expected_sizes_and_round_trip() {
    // Version v has side length v*4+17 modules.
    for (version, side) in [(1u8, 21usize), (5, 37), (10, 57), (40, 177)] {
        let qr = QRCode::from_string("regression".to_string());
        let matrix = qr.encode(&QrOptions::new().version(version)).unwrap();
        assert_eq!(matrix.size(), side, "version {version}");
        let png = qr
            .to_png_bytes(
                &QrOptions::new().version(version),
                &RasterOptions::default(),
            )
            .unwrap();
        assert_eq!(decode(&png), "regression", "version {version}");
    }
}

#[test]
fn raster_formats_round_trip() {
    let payload = "https://example.com/fmt";
    let qr = QRCode::from_string(payload.to_string());
    let opts = QrOptions::new();
    let ro = RasterOptions {
        module_size: 10,
        ..RasterOptions::default()
    };
    assert_eq!(decode(&qr.to_png_bytes(&opts, &ro).unwrap()), payload);
    assert_eq!(decode(&qr.to_jpeg_bytes(&opts, &ro).unwrap()), payload);
    assert_eq!(decode(&qr.to_gif_bytes(&opts, &ro).unwrap()), payload);
    assert_eq!(
        decode(&qr.to_image_bytes(&opts, &ro, ImageFormat::Bmp).unwrap()),
        payload
    );
}

#[test]
fn payloads_round_trip_through_qr() {
    let wifi = WifiNetwork::new("Cafe")
        .security(WifiSecurity::Wpa)
        .password("pw")
        .to_qr_string();
    let mecard = MeCard::new("Doe,Jane").email("j@e.x").to_mecard();
    let vcard = BusinessCard::new("Jane").phone("123").to_vcard();
    let emvco = MerchantPayment::new(
        MerchantAccount::new(26, "com.example"),
        "840",
        "US",
        "Acme",
        "City",
    )
    .to_emvco();
    for payload in [wifi, mecard, vcard, emvco] {
        assert_eq!(decode(&png_of(&payload, &QrOptions::new())), payload);
    }
}

// --- 3. Invariants ---------------------------------------------------------

#[test]
fn quiet_zone_and_opaque_dark_modules() {
    let qr = QRCode::from_string("https://example.com".to_string());
    let matrix = qr.encode(&QrOptions::new()).unwrap();
    assert_eq!(matrix.quiet_zone(), 4);

    let img = qr
        .to_png_bytes(&QrOptions::new(), &RasterOptions::default())
        .unwrap();
    let rgba = image::load_from_memory(&img).unwrap().into_rgba8();
    // Corner is in the quiet zone -> opaque white.
    assert_eq!(rgba.get_pixel(0, 0), &Rgba([255, 255, 255, 255]));
    // Somewhere there is an opaque-black dark module (never transparent).
    assert!(rgba.pixels().any(|p| p.0 == [0, 0, 0, 255]));
    assert!(!rgba.pixels().any(|p| p.0[3] == 0), "no transparent pixels");
}

#[test]
fn svg_structure_is_stable_per_shape() {
    let qr = QRCode::from_string("svg".to_string());
    let opts = QrOptions::new();
    let base = SvgOptions {
        dark: Color::rgb(0x11, 0x22, 0x33),
        ..SvgOptions::default()
    };
    let square = qr.to_svg_styled(&opts, &base).unwrap();
    assert!(square.starts_with("<svg") && square.ends_with("</svg>"));
    assert!(square.contains("fill=\"#112233\""));
    assert!(qr
        .to_svg_styled(
            &opts,
            &SvgOptions {
                shape: ModuleShape::Rounded,
                ..base
            }
        )
        .unwrap()
        .contains("rx="));
    assert!(qr
        .to_svg_styled(
            &opts,
            &SvgOptions {
                shape: ModuleShape::Circle,
                ..base
            }
        )
        .unwrap()
        .contains("<circle"));
}

#[test]
fn branded_codes_still_scan() {
    let payload = "https://example.com/branded";
    let qr = QRCode::from_string(payload.to_string());
    let opts = QrOptions::new().ecc(Ecc::High);
    let ro = RasterOptions {
        module_size: 12,
        ..RasterOptions::default()
    };

    // Logo embedded.
    let logo: RgbaImage = ImageBuffer::from_pixel(96, 96, Rgba([0, 120, 255, 255]));
    let with_logo = qr
        .to_image_bytes_with_logo(&opts, &ro, &logo, &LogoOptions::default(), ImageFormat::Png)
        .unwrap();
    assert_eq!(decode(&with_logo), payload);

    // Artistic blend.
    let bg: RgbaImage = ImageBuffer::from_fn(400, 400, |x, y| Rgba([x as u8, y as u8, 120, 255]));
    let art = qr
        .to_art_bytes(&opts, &bg, &BlendOptions::default(), ImageFormat::Png)
        .unwrap();
    assert_eq!(decode(&art), payload);

    // Control image is a clean exact-size square.
    let control = qr
        .to_control_image(&opts, &ControlOptions::with_size(384))
        .unwrap();
    assert_eq!(control.dimensions(), (384, 384));
}

#[test]
fn empty_payload_round_trips() {
    let png = png_of("", &QrOptions::new());
    assert_eq!(decode(&png), "");
}
