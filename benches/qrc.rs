//! # QRC benchmark suite
//!
//! Criterion benchmarks covering every public area of the crate: construction,
//! encoding (per error-correction level), each renderer (SVG, raster byte
//! encoders, unicode, control image, artistic blend, logo embedding), the
//! payload builders, and the legacy helpers.

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use image::{ImageBuffer, ImageFormat, Rgba, RgbaImage};

use qrc::encode::{encode, Ecc, QrOptions};
use qrc::payload::emvco::{MerchantAccount, MerchantPayment};
use qrc::payload::mecard::MeCard;
use qrc::payload::vcard::BusinessCard;
use qrc::payload::wifi::{WifiNetwork, WifiSecurity};
use qrc::render::art::{self, BlendOptions};
use qrc::render::control::{self, ControlOptions};
use qrc::render::raster::{self, LogoOptions, RasterOptions};
use qrc::render::style::{Color, ModuleShape};
use qrc::render::svg::{self, SvgOptions};
use qrc::QRCode;

/// Representative payload encoded across the benchmarks.
const URL: &str = "https://example.com/benchmark-payload";

/// A QR code over the benchmark payload.
fn qr() -> QRCode {
    QRCode::from_string(URL.to_string())
}

/// A small synthetic logo image for branding benchmarks.
fn logo() -> RgbaImage {
    ImageBuffer::from_pixel(96, 96, Rgba([0x12, 0x6E, 0xE0, 255]))
}

/// A gradient background image for blend benchmarks.
fn background() -> RgbaImage {
    ImageBuffer::from_fn(256, 256, |x, y| Rgba([x as u8, y as u8, 128, 255]))
}

// --- construction & encoding ----------------------------------------------

/// Benchmarks the `QRCode` constructors.
fn construction(c: &mut Criterion) {
    c.bench_function("QRCode::from_string", |b| {
        b.iter(|| QRCode::from_string(black_box(URL.to_string())))
    });
    c.bench_function("QRCode::from_bytes", |b| {
        b.iter(|| QRCode::from_bytes(black_box(URL.as_bytes().to_vec())))
    });
}

/// Benchmarks encoding at each error-correction level.
fn encoding(c: &mut Criterion) {
    let q = qr();
    for ecc in [Ecc::Low, Ecc::Medium, Ecc::Quartile, Ecc::High] {
        let opts = QrOptions::new().ecc(ecc);
        c.bench_function(&format!("encode::{ecc:?}"), |b| {
            b.iter(|| encode(black_box(URL.as_bytes()), black_box(&opts)).unwrap())
        });
    }
    c.bench_function("QRCode::encode", |b| {
        let opts = QrOptions::new();
        b.iter(|| q.encode(black_box(&opts)).unwrap())
    });
    c.bench_function("QRCode::to_qrcode", |b| b.iter(|| q.to_qrcode()));
}

// --- renderers -------------------------------------------------------------

/// Benchmarks the SVG renderers.
fn svg(c: &mut Criterion) {
    let q = qr();
    let opts = QrOptions::new();
    for shape in [
        ModuleShape::Square,
        ModuleShape::Rounded,
        ModuleShape::Circle,
    ] {
        let s = SvgOptions {
            shape,
            ..SvgOptions::default()
        };
        c.bench_function(&format!("to_svg_styled::{shape:?}"), |b| {
            b.iter(|| q.to_svg_styled(black_box(&opts), black_box(&s)).unwrap())
        });
    }
    c.bench_function("to_svg (legacy)", |b| b.iter(|| q.to_svg(black_box(256))));
    let matrix = q.encode(&opts).unwrap();
    c.bench_function("svg::render", |b| {
        b.iter(|| svg::render(black_box(&matrix), black_box(&SvgOptions::default())))
    });
}

/// Benchmarks the raster byte encoders.
fn raster(c: &mut Criterion) {
    let q = qr();
    let opts = QrOptions::new();
    let ro = RasterOptions::default();
    c.bench_function("to_png_bytes", |b| {
        b.iter(|| q.to_png_bytes(black_box(&opts), black_box(&ro)).unwrap())
    });
    c.bench_function("to_jpeg_bytes", |b| {
        b.iter(|| q.to_jpeg_bytes(black_box(&opts), black_box(&ro)).unwrap())
    });
    c.bench_function("to_gif_bytes", |b| {
        b.iter(|| q.to_gif_bytes(black_box(&opts), black_box(&ro)).unwrap())
    });
    c.bench_function("to_image_bytes::Bmp", |b| {
        b.iter(|| {
            q.to_image_bytes(black_box(&opts), black_box(&ro), ImageFormat::Bmp)
                .unwrap()
        })
    });
    let matrix = q.encode(&opts).unwrap();
    c.bench_function("raster::render", |b| {
        b.iter(|| raster::render(black_box(&matrix), black_box(&ro)))
    });
}

/// Benchmarks the terminal renderer.
fn unicode(c: &mut Criterion) {
    let q = qr();
    let opts = QrOptions::new();
    c.bench_function("to_unicode", |b| {
        b.iter(|| q.to_unicode(black_box(&opts)).unwrap())
    });
}

/// Benchmarks logo embedding, artistic blend and control image.
fn branding(c: &mut Criterion) {
    let q = qr();
    let opts = QrOptions::new().ecc(Ecc::High);
    let ro = RasterOptions::default();
    let logo = logo();
    let bg = background();
    c.bench_function("to_image_with_logo", |b| {
        b.iter(|| {
            q.to_image_with_logo(&opts, &ro, black_box(&logo), &LogoOptions::default())
                .unwrap()
        })
    });
    c.bench_function("to_art_image", |b| {
        b.iter(|| {
            q.to_art_image(&opts, black_box(&bg), &BlendOptions::default())
                .unwrap()
        })
    });
    let matrix = q.encode(&opts).unwrap();
    c.bench_function("art::blend", |b| {
        b.iter(|| art::blend(&matrix, black_box(&bg), &BlendOptions::default()))
    });
    c.bench_function("to_control_image", |b| {
        b.iter(|| {
            q.to_control_image(&opts, black_box(&ControlOptions::with_size(256)))
                .unwrap()
        })
    });
    c.bench_function("control::render", |b| {
        b.iter(|| control::render(&matrix, black_box(&ControlOptions::with_size(256))))
    });
}

// --- payloads & misc -------------------------------------------------------

/// Benchmarks the structured payload builders.
fn payloads(c: &mut Criterion) {
    c.bench_function("BusinessCard::to_vcard", |b| {
        let card = BusinessCard::new("Jane Doe")
            .organization("Acme")
            .email("jane@acme.example");
        b.iter(|| black_box(&card).to_vcard())
    });
    c.bench_function("MeCard::to_mecard", |b| {
        let card = MeCard::new("Doe,Jane").phone("+15550100");
        b.iter(|| black_box(&card).to_mecard())
    });
    c.bench_function("WifiNetwork::to_qr_string", |b| {
        let w = WifiNetwork::new("Cafe")
            .security(WifiSecurity::Wpa)
            .password("pw");
        b.iter(|| black_box(&w).to_qr_string())
    });
    c.bench_function("MerchantPayment::to_emvco", |b| {
        let account = MerchantAccount::new(26, "com.example").merchant_id("123");
        let p = MerchantPayment::new(account, "840", "US", "Acme", "City").amount("4.50");
        b.iter(|| black_box(&p).to_emvco())
    });
}

/// Benchmarks legacy helpers and colour formatting.
fn misc(c: &mut Criterion) {
    let q = qr();
    c.bench_function("colorize", |b| {
        b.iter(|| q.colorize(black_box(Rgba([255, 0, 0, 255]))))
    });
    c.bench_function("resize", |b| {
        b.iter(|| q.resize(black_box(128), black_box(128)))
    });
    c.bench_function("Color::to_hex", |b| {
        let color = Color::rgb(0x11, 0x22, 0x33);
        b.iter(|| black_box(&color).to_hex())
    });
}

criterion_group!(
    benches,
    construction,
    encoding,
    svg,
    raster,
    unicode,
    branding,
    payloads,
    misc,
);
criterion_main!(benches);
