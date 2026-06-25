// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright (c) 2022-2026 QRC. All rights reserved.

//! Encoding format management for QR codes.
//!
//! Demonstrates inspecting and setting the encoding format,
//! and how unsupported formats are rejected gracefully.
//!
//! Run: `cargo run --example encoding`

#[path = "support.rs"]
mod support;

use qrc::{set_encoding_format, QRCode};

fn main() {
    support::header("qrc -- encoding");

    // ── Default encoding ───────────────────────────────────────────────
    support::task_with_output("Inspect default encoding format", || {
        let qr = QRCode::from_string("Hello, world!".to_string());
        vec![
            format!("Format: {}", qr.get_encoding_format()),
            format!("Data:   {} bytes", qr.data.len()),
        ]
    });

    // ── Set encoding explicitly ────────────────────────────────────────
    support::task_with_output("Set encoding to utf-8 (supported)", || {
        let qr = QRCode::from_string("Bonjour le monde!".to_string());
        match qr.set_encoding_format("utf-8") {
            Ok(updated) => vec![
                format!("Format: {}", updated.get_encoding_format()),
                format!("Status: accepted"),
            ],
            Err(e) => vec![format!("Error: {e}")],
        }
    });

    // ── Using the macro ────────────────────────────────────────────────
    support::task_with_output("Set encoding via set_encoding_format! macro", || {
        let qr = QRCode::new(b"data".to_vec());
        match set_encoding_format!(qr, "utf-8") {
            Ok(updated) => vec![
                format!("Macro:  set_encoding_format!(qr, \"utf-8\")"),
                format!("Format: {}", updated.get_encoding_format()),
            ],
            Err(e) => vec![format!("Error: {e}")],
        }
    });

    // ── Reject unsupported encoding ────────────────────────────────────
    support::task_result("Reject unsupported encoding (latin-1)", || {
        let qr = QRCode::from_string("Test".to_string());
        qr.set_encoding_format("latin-1")
    })
    .ok();

    support::summary(4);
}
