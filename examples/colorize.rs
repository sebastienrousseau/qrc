// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright (c) 2022-2026 QRC. All rights reserved.

//! Brand-coloured QR codes for marketing and identity.
//!
//! Shows how to generate QR codes in custom colours matching
//! brand guidelines while keeping scan reliability.
//!
//! Run: `cargo run --example colorize`

#[path = "support.rs"]
mod support;

use image::Rgba;
use qrc::QRCode;

fn main() {
    support::header("qrc -- colorize");

    let url = "https://example.com/signup";
    let qr = QRCode::from_string(url.to_string());

    support::with_temp_dir("colorize", |dir| {
        // ── Corporate blue ─────────────────────────────────────────────
        support::task_with_output("Corporate blue (#0066CC)", || {
            let blue = Rgba([0x00, 0x66, 0xCC, 0xFF]);
            let img = qr.colorize(blue);
            let (w, h) = img.dimensions();
            img.save(dir.join("blue.png")).unwrap();
            vec![
                "Colour: rgba(0, 102, 204, 255)".to_string(),
                format!("Size:   {}x{} px", w, h),
                "Use:    Corporate website, business cards".to_string(),
            ]
        });

        // ── Crimson red ────────────────────────────────────────────────
        support::task_with_output("Crimson red (#DC143C)", || {
            let red = Rgba([0xDC, 0x14, 0x3C, 0xFF]);
            let img = qr.colorize(red);
            img.save(dir.join("red.png")).unwrap();
            vec![
                "Colour: rgba(220, 20, 60, 255)".to_string(),
                "Use:    Sale promotions, urgent call-to-action".to_string(),
            ]
        });

        // ── Forest green ───────────────────────────────────────────────
        support::task_with_output("Forest green (#228B22)", || {
            let green = Rgba([0x22, 0x8B, 0x22, 0xFF]);
            let img = qr.colorize(green);
            img.save(dir.join("green.png")).unwrap();
            vec![
                "Colour: rgba(34, 139, 34, 255)".to_string(),
                "Use:    Eco-friendly campaigns, sustainability".to_string(),
            ]
        });

        // ── Dark grey (high contrast) ──────────────────────────────────
        support::task_with_output("Dark grey for print (#333333)", || {
            let grey = Rgba([0x33, 0x33, 0x33, 0xFF]);
            let img = qr.colorize(grey);
            img.save(dir.join("grey.png")).unwrap();
            vec![
                "Colour: rgba(51, 51, 51, 255)".to_string(),
                "Use:    Professional print, subtle branding".to_string(),
                "Tip:    Dark colours on white ensure scan reliability".to_string(),
            ]
        });
    });

    support::summary(4);
}
