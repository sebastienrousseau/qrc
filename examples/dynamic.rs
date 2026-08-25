// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright (c) 2022-2026 QRC. All rights reserved.

//! Dynamic QR codes: updateable content via redirect URLs.
//!
//! Dynamic QR codes encode a redirect URL instead of static data.
//! The destination can be changed server-side without reprinting
//! the physical QR code.
//!
//! Run: `cargo run --example dynamic`

#[path = "support.rs"]
mod support;

use qrc::{create_dynamic_qr, QRCode};

fn main() {
    support::header("qrc -- dynamic");

    // ── Create a dynamic QR code ───────────────────────────────────────
    support::task_with_output("Create dynamic QR for a campaign", || {
        let qr = QRCode::create_dynamic("summer-sale-2026");
        let encoded_url = String::from_utf8_lossy(&qr.data);
        vec![
            format!("Encoded URL: {encoded_url}"),
            "Use case:    Print once, update destination anytime".to_string(),
            "Tip:         Host a redirect service at the encoded URL".to_string(),
        ]
    });

    // ── Using the macro ────────────────────────────────────────────────
    support::task_with_output("Create via create_dynamic_qr! macro", || {
        let qr = create_dynamic_qr!("product-launch-q4");
        let encoded_url = String::from_utf8_lossy(&qr.data);
        vec![
            format!("Encoded URL: {encoded_url}"),
            "Macro:       create_dynamic_qr!(\"product-launch-q4\")".to_string(),
        ]
    });

    // ── Real-world scenario: event ticket ──────────────────────────────
    support::task_with_output("Event ticket (updateable venue info)", || {
        let qr = QRCode::create_dynamic("event-ticket-8842");
        let internal = qr.to_qrcode();
        vec![
            format!("QR modules: {}x{}", internal.width(), internal.width()),
            "Scenario:   Ticket printed weeks before event".to_string(),
            "Benefit:    Venue change? Update the redirect, not the ticket".to_string(),
        ]
    });

    support::summary(3);
}
