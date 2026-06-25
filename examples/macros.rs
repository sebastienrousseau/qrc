// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright (c) 2022-2026 QRC. All rights reserved.

//! All QRC macros demonstrated in one place.
//!
//! Shows every macro exported by the `qrc` crate with practical
//! examples, making it easy to discover the convenience API.
//!
//! Run: `cargo run --example macros`

#[path = "support.rs"]
mod support;

use image::{ImageBuffer, Rgba};
use qrc::{
    add_image_watermark, batch_generate_qr, combine_qr_codes, compress_data_macro,
    create_dynamic_qr, create_multilanguage_qr, overlay_image, qr_code, qr_code_to,
    qr_code_with_ec, resize, set_encoding_format, EcLevel, QRCode,
};

fn main() {
    support::header("qrc -- macros");

    // ── qr_code! ───────────────────────────────────────────────────────
    support::task_with_output("qr_code! — Quick QR creation", || {
        let qr = qr_code!("https://crates.io/crates/qrc".into());
        vec![
            format!("Data: {}", String::from_utf8_lossy(&qr.data)),
            format!("Modules: {}", qr.to_qrcode().width()),
        ]
    });

    // ── qr_code_with_ec! ──────────────────────────────────────────────
    support::task_with_output("qr_code_with_ec! — EC level selection", || {
        let qr = qr_code_with_ec!("High EC".into(), EcLevel::H);
        vec![
            format!("EC Level: {:?}", qr.ec_level),
            format!("Modules: {}", qr.to_qrcode().width()),
        ]
    });

    // ── qr_code_to! ───────────────────────────────────────────────────
    support::task_with_output("qr_code_to! — Create and convert in one step", || {
        let png = qr_code_to!("Hello QRC".into(), "png", 128).unwrap();
        let jpg = qr_code_to!("Hello QRC".into(), "jpg", 128).unwrap();
        let gif = qr_code_to!("Hello QRC".into(), "gif", 128).unwrap();
        vec![
            format!("PNG: {} bytes", png.len()),
            format!("JPG: {} bytes", jpg.len()),
            format!("GIF: {} bytes", gif.len()),
        ]
    });

    // ── resize! ────────────────────────────────────────────────────────
    support::task_with_output("resize! — Square resize shorthand", || {
        let qr = qr_code!("Resize me".into());
        let img = resize!(qr, 256);
        vec![format!(
            "resize!(qr, 256) => {}x{}",
            img.width(),
            img.height()
        )]
    });

    // ── set_encoding_format! ───────────────────────────────────────────
    support::task_with_output("set_encoding_format! — Set encoding", || {
        let qr = qr_code!("Encode me".into());
        let updated = set_encoding_format!(qr, "utf-8").unwrap();
        vec![format!("Encoding: {}", updated.get_encoding_format())]
    });

    // ── add_image_watermark! ───────────────────────────────────────────
    support::task_with_output("add_image_watermark! — Brand your QR", || {
        let qr = QRCode::from_string("https://example.com".to_string());
        let mut img = qr.to_png(128);
        let logo = ImageBuffer::from_pixel(10, 10, Rgba([255, 0, 0, 255]));
        add_image_watermark!(&mut img, &logo);
        vec![format!("Watermarked: {}x{}", img.width(), img.height())]
    });

    // ── overlay_image! ─────────────────────────────────────────────────
    support::task_with_output("overlay_image! — Overlay a logo", || {
        let qr = QRCode::from_string("Overlay test".to_string());
        let logo = ImageBuffer::from_pixel(5, 5, Rgba([0, 0, 255, 255]));
        let img = overlay_image!(qr, &logo);
        vec![format!("Overlaid: {}x{}", img.width(), img.height())]
    });

    // ── compress_data_macro! ───────────────────────────────────────────
    support::task_with_output("compress_data_macro! — Zlib compression", || {
        let data = "Repeated data for compression test. ".repeat(10);
        let compressed = compress_data_macro!(&data);
        vec![
            format!("Original:   {} bytes", data.len()),
            format!("Compressed: {} bytes", compressed.len()),
        ]
    });

    // ── batch_generate_qr! ─────────────────────────────────────────────
    support::task_with_output("batch_generate_qr! — Multiple QR codes", || {
        let codes = batch_generate_qr!(vec![
            "https://a.example.com".to_string(),
            "https://b.example.com".to_string(),
        ]);
        vec![format!("Generated: {} QR codes", codes.len())]
    });

    // ── combine_qr_codes! ──────────────────────────────────────────────
    support::task_with_output("combine_qr_codes! — Merge QR codes", || {
        let qr1 = QRCode::from_string("Left".to_string());
        let qr2 = QRCode::from_string("Right".to_string());
        match combine_qr_codes!([qr1, qr2]) {
            Ok(combined) => vec![format!("Combined data: {} bytes", combined.data.len())],
            Err(e) => vec![format!("Error: {e}")],
        }
    });

    // ── create_dynamic_qr! ─────────────────────────────────────────────
    support::task_with_output("create_dynamic_qr! — Updatable QR", || {
        let qr = create_dynamic_qr!("campaign-2026");
        vec![format!("URL: {}", String::from_utf8_lossy(&qr.data))]
    });

    // ── create_multilanguage_qr! ───────────────────────────────────────
    support::task_with_output("create_multilanguage_qr! — i18n QR", || {
        let qr = create_multilanguage_qr! {
            "en" => "Hello",
            "es" => "Hola",
            "fr" => "Bonjour",
        };
        vec![format!("Selected: {}", String::from_utf8_lossy(&qr.data))]
    });

    support::summary(12);
}
