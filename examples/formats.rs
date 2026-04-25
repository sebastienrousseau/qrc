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

        // ── JPG output ──────────────────────────────────────────────────
        support::task_with_output("Export as JPG (compressed, print-ready)", || {
            let jpg_bytes = qr.to_jpg(size);
            let path = dir.join("qrcode.jpg");
            fs::write(&path, &jpg_bytes).unwrap();
            vec![
                format!("Dimensions: {}x{} px", size, size),
                format!("File size:  {} bytes", jpg_bytes.len()),
                format!("Use case:   Print materials, email attachments"),
            ]
        });

        // ── GIF output ─────────────────────────────────────────────────
        support::task_with_output("Export as GIF (small palette, universal)", || {
            let gif_bytes = qr.to_gif(size);
            let path = dir.join("qrcode.gif");
            fs::write(&path, &gif_bytes).unwrap();
            vec![
                format!("Dimensions: {}x{} px", size, size),
                format!("File size:  {} bytes", gif_bytes.len()),
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
