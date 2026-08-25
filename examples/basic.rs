// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright (c) 2022-2026 QRC. All rights reserved.

//! Basic QR code creation from different data sources.
//!
//! Demonstrates the three constructors: `new`, `from_string`, `from_bytes`.
//!
//! Run: `cargo run --example basic`

#[path = "support.rs"]
mod support;

use qrc::QRCode;

fn main() {
    support::header("qrc -- basic");

    // ── From raw bytes ─────────────────────────────────────────────────
    support::task_with_output("Create QR code from raw bytes", || {
        let qr = QRCode::new(vec![0x48, 0x65, 0x6C, 0x6C, 0x6F]); // "Hello"
        vec![
            format!("Data length: {} bytes", qr.data.len()),
            format!("Encoding:    {}", qr.get_encoding_format()),
        ]
    });

    // ── From a String ──────────────────────────────────────────────────
    support::task_with_output("Create QR code from a URL string", || {
        let url = "https://example.com/products/12345".to_string();
        let qr = QRCode::from_string(url);
        vec![
            format!("Data length: {} bytes", qr.data.len()),
            format!("Content:     {}", String::from_utf8_lossy(&qr.data)),
        ]
    });

    // ── From a byte vector ─────────────────────────────────────────────
    support::task_with_output("Create QR code from byte vector", || {
        let wifi = "WIFI:T:WPA;S:MyNetwork;P:secret123;;";
        let qr = QRCode::from_bytes(wifi.as_bytes().to_vec());
        vec![
            format!("Data length: {} bytes", qr.data.len()),
            "Use case:    Wi-Fi auto-connect QR code".to_string(),
        ]
    });

    // ── Convert to internal QrCode ─────────────────────────────────────
    support::task_with_output("Access underlying QrCode structure", || {
        let qr = QRCode::from_string("Hello, QRC!".to_string());
        let internal = qr.to_qrcode();
        vec![
            format!("QR version:  {:?}", internal.version()),
            format!("Module width: {} modules", internal.width()),
        ]
    });

    support::summary(4);
}
