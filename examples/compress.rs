// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright (c) 2022-2026 QRC. All rights reserved.

//! Compress data before encoding into a QR code.
//!
//! When encoding large text payloads (e.g., vCards, JSON configs),
//! compression reduces the data size so it fits within QR capacity.
//!
//! Run: `cargo run --example compress`

#[path = "support.rs"]
mod support;

use qrc::{compress_data_macro, QRCode};

fn main() {
    support::header("qrc -- compress");

    // ── Compress a vCard ───────────────────────────────────────────────
    support::task_with_output("Compress a vCard contact", || {
        let vcard = "\
BEGIN:VCARD\n\
VERSION:3.0\n\
FN:Jane Smith\n\
ORG:Acme Corp\n\
TEL:+1-555-0199\n\
EMAIL:jane.smith@acme.com\n\
ADR:;;123 Main St;Springfield;IL;62701;US\n\
END:VCARD";

        let compressed = QRCode::compress_data(vcard);
        #[allow(clippy::cast_precision_loss)]
        let ratio = compressed.len() as f64 / vcard.len() as f64 * 100.0;
        vec![
            format!("Original:   {} bytes", vcard.len()),
            format!("Compressed: {} bytes", compressed.len()),
            format!("Ratio:      {ratio:.1}%"),
            "Use case:   Fit more contact info in a QR code".to_string(),
        ]
    });

    // ── Compress JSON configuration ────────────────────────────────────
    support::task_with_output("Compress a JSON configuration payload", || {
        let json = r#"{"wifi":{"ssid":"CorpNet","password":"s3cur3!","type":"WPA2"},"proxy":{"host":"10.0.0.1","port":8080}}"#;
        let compressed = compress_data_macro!(json);
        #[allow(clippy::cast_precision_loss)]
        let ratio = compressed.len() as f64 / json.len() as f64 * 100.0;
        vec![
            format!("Original:   {} bytes", json.len()),
            format!("Compressed: {} bytes", compressed.len()),
            format!("Ratio:      {ratio:.1}%"),
            "Use case:   Device provisioning via QR scan".to_string(),
        ]
    });

    // ── Create QR from compressed data ─────────────────────────────────
    support::task_with_output("Create QR code from compressed data", || {
        let large_text = "A".repeat(500);
        let compressed = QRCode::compress_data(&large_text);
        let qr = QRCode::from_bytes(compressed.clone());
        let internal = qr.to_qrcode();
        vec![
            format!("Original:     {} bytes", large_text.len()),
            format!("Compressed:   {} bytes", compressed.len()),
            format!("QR modules:   {}x{}", internal.width(), internal.width()),
            "Tip:          Decompress on the receiving end".to_string(),
        ]
    });

    support::summary(3);
}
