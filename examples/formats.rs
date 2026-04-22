// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright (c) 2022-2026 QRC. All rights reserved.

//! Export QR codes in every supported image format.
//!
//! Generates PNG, JPG, GIF, and SVG files for a real-world URL,
//! showing file sizes and dimensions for each format.
//!
//! Run: `cargo run --example formats`

#[path = "support.rs"]
mod support;

use image::DynamicImage;
use qrc::QRCode;
use std::fs;

fn main() {
    support::header("qrc -- formats");

    let url = "https://docs.rs/qrc";
    let qr = QRCode::from_string(url.to_string());
    let size: u32 = 512;

    support::with_temp_dir("formats", |dir| {
        // ── PNG output ─────────────────────────────────────────────────
        support::task_with_output("Export as PNG (lossless, web-ready)", || {
            let img = qr.to_png(size);
            let path = dir.join("qrcode.png");
            img.save(&path).unwrap();
            let file_size = fs::metadata(&path).unwrap().len();
            vec![
                format!("Dimensions: {}x{} px", size, size),
                format!("File size:  {} bytes", file_size),
                format!("Path:       {}", path.display()),
            ]
        });

        // ── JPG output (requires RGB, not RGBA) ──────────────────────────
        support::task_with_output("Export as JPG (compressed, print-ready)", || {
            let img = qr.to_jpg(size);
            let path = dir.join("qrcode.jpg");
            // JPEG doesn't support alpha — convert RGBA to RGB
            DynamicImage::ImageRgba8(img).to_rgb8().save(&path).unwrap();
            let file_size = fs::metadata(&path).unwrap().len();
            vec![
                format!("Dimensions: {}x{} px", size, size),
                format!("File size:  {} bytes", file_size),
                format!("Use case:   Print materials, email attachments"),
                format!("Note:       RGBA converted to RGB for JPEG compatibility"),
            ]
        });

        // ── GIF output ─────────────────────────────────────────────────
        support::task_with_output("Export as GIF (small palette, universal)", || {
            let img = qr.to_gif(size);
            let path = dir.join("qrcode.gif");
            img.save(&path).unwrap();
            let file_size = fs::metadata(&path).unwrap().len();
            vec![
                format!("Dimensions: {}x{} px", size, size),
                format!("File size:  {} bytes", file_size),
                format!("Use case:   Legacy systems, email signatures"),
            ]
        });

        // ── SVG output ─────────────────────────────────────────────────
        support::task_with_output("Export as SVG (vector, infinite scaling)", || {
            let svg = qr.to_svg(size);
            let path = dir.join("qrcode.svg");
            fs::write(&path, &svg).unwrap();
            let file_size = fs::metadata(&path).unwrap().len();
            vec![
                format!("SVG length: {} chars", svg.len()),
                format!("File size:  {} bytes", file_size),
                format!("Use case:   Logos, responsive web, high-DPI print"),
            ]
        });
    });

    support::summary(4);
}
