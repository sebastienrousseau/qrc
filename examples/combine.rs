// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright (c) 2022-2026 QRC. All rights reserved.

//! Combine multiple QR codes into a single composite image.
//!
//! Useful for encoding separate data segments side-by-side,
//! such as a multi-part payload or a visual comparison strip.
//!
//! Run: `cargo run --example combine`

#[path = "support.rs"]
mod support;

use qrc::{combine_qr_codes, QRCode};

fn main() {
    support::header("qrc -- combine");

    // ── Combine three QR codes ─────────────────────────────────────────
    support::task_with_output("Combine 3 QR codes (contact, website, WiFi)", || {
        let contact = QRCode::from_string("tel:+1-555-0100".to_string());
        let website = QRCode::from_string("https://example.com".to_string());
        let wifi = QRCode::from_string("WIFI:T:WPA;S:Guest;P:welcome;;".to_string());

        match QRCode::combine_qr_codes(&[contact, website, wifi]) {
            Ok(combined) => vec![
                format!("Combined data: {} bytes", combined.data.len()),
                format!("Use case:      Multi-info display card"),
            ],
            Err(e) => vec![format!("Error: {e}")],
        }
    });

    // ── Using the macro ────────────────────────────────────────────────
    support::task_with_output("Combine via combine_qr_codes! macro", || {
        let qr1 = QRCode::from_string("Part 1: Header".to_string());
        let qr2 = QRCode::from_string("Part 2: Body".to_string());
        let qr3 = QRCode::from_string("Part 3: Footer".to_string());

        match combine_qr_codes!([qr1, qr2, qr3]) {
            Ok(combined) => vec![
                format!("Combined data: {} bytes", combined.data.len()),
                format!("Macro:         combine_qr_codes!([...])"),
            ],
            Err(e) => vec![format!("Error: {e}")],
        }
    });

    // ── Error handling: empty input ────────────────────────────────────
    support::task_result("Reject empty input gracefully", || {
        QRCode::combine_qr_codes(&[]).map(|_| "should not reach here")
    })
    .ok();

    support::summary(3);
}
