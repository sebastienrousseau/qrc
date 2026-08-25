// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright (c) 2022-2026 QRC. All rights reserved.

//! Batch-generate QR codes for multiple items at once.
//!
//! Ideal for e-commerce product pages, event tickets, or
//! inventory labels where many QR codes are needed in one pass.
//!
//! Run: `cargo run --example batch`

#[path = "support.rs"]
mod support;

use qrc::{batch_generate_qr, QRCode};
use std::fs;

fn main() {
    support::header("qrc -- batch");

    // ── Product catalogue URLs ─────────────────────────────────────────
    let products = vec![
        "https://shop.example.com/product/1001".to_string(),
        "https://shop.example.com/product/1002".to_string(),
        "https://shop.example.com/product/1003".to_string(),
        "https://shop.example.com/product/1004".to_string(),
        "https://shop.example.com/product/1005".to_string(),
    ];

    support::task_with_output("Batch-generate 5 product QR codes", || {
        let codes = QRCode::batch_generate_qr_codes(products.clone());
        codes
            .iter()
            .enumerate()
            .map(|(i, qr)| {
                let internal = qr.to_qrcode();
                format!(
                    "#{}: {} ({} modules)",
                    i + 1,
                    String::from_utf8_lossy(&qr.data),
                    internal.width()
                )
            })
            .collect()
    });

    // ── Using the macro ────────────────────────────────────────────────
    support::task_with_output("Same batch via batch_generate_qr! macro", || {
        let urls = vec![
            "https://event.example.com/ticket/A1".to_string(),
            "https://event.example.com/ticket/A2".to_string(),
            "https://event.example.com/ticket/A3".to_string(),
        ];
        let codes = batch_generate_qr!(urls);
        vec![
            format!("Generated:  {} QR codes", codes.len()),
            "Use case:   Event ticket batch".to_string(),
        ]
    });

    // ── Save batch to files ────────────────────────────────────────────
    support::with_temp_dir("batch", |dir| {
        support::task_with_output("Export batch as individual PNG files", || {
            let codes = QRCode::batch_generate_qr_codes(products.clone());
            let mut lines = Vec::new();
            for (i, qr) in codes.iter().enumerate() {
                let img = qr.to_png(256);
                let filename = format!("product_{}.png", i + 1);
                img.save(dir.join(&filename)).unwrap();
                let size = fs::metadata(dir.join(&filename)).unwrap().len();
                lines.push(format!("{filename}: {size} bytes"));
            }
            lines.push(format!("Total: {} files written", codes.len()));
            lines
        });
    });

    support::summary(3);
}
