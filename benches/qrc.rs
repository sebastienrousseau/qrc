//! # QRC Benchmarking
//!
//! Benchmarks for the `qrc` crate, testing various functionalities like QR code generation,
//! colourisation, and performance under different scenarios.

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use image::Rgba;
extern crate qrc;
use self::qrc::encode::QrOptions;
use self::qrc::render::raster::RasterOptions;
use self::qrc::render::svg::SvgOptions;
use self::qrc::QRCode;

/// Representative payload encoded across the benchmarks.
const URL: &str = "https://example.com/benchmark-payload";

/// Benchmark for the layered encode path (bytes -> Matrix).
fn encode_benchmark(c: &mut Criterion) {
    let qrcode = QRCode::from_string(URL.to_string());
    let opts = QrOptions::new();
    c.bench_function("QRCode::encode", |b| {
        b.iter(|| qrcode.encode(black_box(&opts)).unwrap())
    });
}

/// Benchmark for real PNG byte encoding via the new API.
fn to_png_bytes_benchmark(c: &mut Criterion) {
    let qrcode = QRCode::from_string(URL.to_string());
    let opts = QrOptions::new();
    let raster = RasterOptions::default();
    c.bench_function("QRCode::to_png_bytes", |b| {
        b.iter(|| {
            qrcode
                .to_png_bytes(black_box(&opts), black_box(&raster))
                .unwrap()
        })
    });
}

/// Benchmark for the SVG-first styled renderer.
fn to_svg_styled_benchmark(c: &mut Criterion) {
    let qrcode = QRCode::from_string(URL.to_string());
    let opts = QrOptions::new();
    let svg = SvgOptions::default();
    c.bench_function("QRCode::to_svg_styled", |b| {
        b.iter(|| {
            qrcode
                .to_svg_styled(black_box(&opts), black_box(&svg))
                .unwrap()
        })
    });
}

/// Benchmark for the terminal/unicode renderer.
fn to_unicode_benchmark(c: &mut Criterion) {
    let qrcode = QRCode::from_string(URL.to_string());
    let opts = QrOptions::new();
    c.bench_function("QRCode::to_unicode", |b| {
        b.iter(|| qrcode.to_unicode(black_box(&opts)).unwrap())
    });
}

/// Benchmark for QRCode::new
fn new_benchmark(c: &mut Criterion) {
    c.bench_function("QRCode::new", |b| {
        b.iter(|| QRCode::new(black_box(vec![1, 2, 3])))
    });
}
/// Benchmark for QRCode::to_png
fn to_png_benchmark(c: &mut Criterion) {
    let qrcode = QRCode::new(vec![1, 2, 3]);
    c.bench_function("QRCode::to_png", |b| b.iter(|| qrcode.to_png(512)));
}

/// Benchmark for QRCode::from_string
fn from_string_benchmark(c: &mut Criterion) {
    c.bench_function("QRCode::from_string", |b| {
        b.iter(|| QRCode::from_string(black_box("Hello, world!".to_string())))
    });
}

/// Benchmark for QRCode::from_bytes
fn from_bytes_benchmark(c: &mut Criterion) {
    c.bench_function("QRCode::from_bytes", |b| {
        b.iter(|| QRCode::from_bytes(black_box(vec![1, 2, 3])))
    });
}

/// Benchmark for QRCode::to_svg
fn to_svg_benchmark(c: &mut Criterion) {
    let qrcode = QRCode::new(vec![1, 2, 3]);
    c.bench_function("QRCode::to_svg", |b| {
        b.iter(|| qrcode.to_svg(black_box(100)))
    });
}

/// Benchmark for QRCode::colorize
fn colorize_benchmark(c: &mut Criterion) {
    let qrcode = QRCode::new(vec![1, 2, 3]);
    let color = Rgba([0, 0, 0, 0]);
    c.bench_function("QRCode::colorize", |b| {
        b.iter(|| qrcode.colorize(black_box(color)))
    });
}

/// Benchmark for QRCode::resize
fn resize_benchmark(c: &mut Criterion) {
    let qrcode = QRCode::new(vec![1, 2, 3]);
    c.bench_function("QRCode::resize", |b| {
        b.iter(|| qrcode.resize(black_box(100), black_box(100)))
    });
}

criterion_group!(
    benches,
    colorize_benchmark,
    encode_benchmark,
    from_bytes_benchmark,
    from_string_benchmark,
    new_benchmark,
    resize_benchmark,
    to_png_benchmark,
    to_png_bytes_benchmark,
    to_svg_benchmark,
    to_svg_styled_benchmark,
    to_unicode_benchmark,
);
criterion_main!(benches);
