// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright (c) 2022-2026 QRC. All rights reserved.

//! Resize QR codes for different media: icons, social, print.
//!
//! Demonstrates producing multiple sizes from a single QR code
//! for responsive design across digital and physical media.
//!
//! Run: `cargo run --example resize`

#[path = "support.rs"]
mod support;

use qrc::QRCode;

fn main() {
    support::header("qrc -- resize");

    let qr = QRCode::from_string("https://example.com".to_string());

    // ── Thumbnail (favicon / app icon) ─────────────────────────────────
    support::task_with_output("Thumbnail for app icons (64x64)", || {
        let img = qr.resize(64, 64);
        let (w, h) = img.dimensions();
        vec![
            format!("Dimensions: {}x{} px", w, h),
            format!("Pixels:     {}", w as usize * h as usize),
            format!("Use case:   Favicon, notification badge"),
        ]
    });

    // ── Social media (Instagram, Twitter) ──────────────────────────────
    support::task_with_output("Social media post (512x512)", || {
        let img = qr.resize(512, 512);
        let (w, h) = img.dimensions();
        vec![
            format!("Dimensions: {}x{} px", w, h),
            format!("Use case:   Instagram story, Twitter post"),
        ]
    });

    // ── Print (300 DPI, ~4 inches) ─────────────────────────────────────
    support::task_with_output("Print-ready (1200x1200 for 300 DPI)", || {
        let img = qr.resize(1200, 1200);
        let (w, h) = img.dimensions();
        vec![
            format!("Dimensions: {}x{} px", w, h),
            format!("At 300 DPI: {:.1}x{:.1} inches", w as f64 / 300.0, h as f64 / 300.0),
            format!("Use case:   Flyers, posters, packaging"),
        ]
    });

    // ── Non-square (banner style) ──────────────────────────────────────
    support::task_with_output("Banner aspect ratio (800x400)", || {
        let img = qr.resize(800, 400);
        let (w, h) = img.dimensions();
        vec![
            format!("Dimensions: {}x{} px", w, h),
            format!("Ratio:      {}:{}", w / 400, h / 400),
            format!("Use case:   Email banner, horizontal placement"),
        ]
    });

    // ── Using the resize! macro ────────────────────────────────────────
    support::task_with_output("Square resize via resize! macro (256x256)", || {
        let img = qrc::resize!(qr, 256);
        let (w, h) = img.dimensions();
        vec![
            format!("Dimensions: {}x{} px", w, h),
            format!("Macro:      resize!(qr, 256)"),
        ]
    });

    support::summary(5);
}
