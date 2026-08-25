// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright (c) 2022-2026 QRC. All rights reserved.

//! Overlay a logo on top of a QR code.
//!
//! Demonstrates placing a brand image at the top-left corner of
//! the QR code. QR codes have error correction, so small overlays
//! remain scannable.
//!
//! Run: `cargo run --example overlay`

#[path = "support.rs"]
mod support;

use image::{ImageBuffer, Rgba};
use qrc::{overlay_image, QRCode};

fn main() {
    support::header("qrc -- overlay");

    let qr = QRCode::from_string("https://example.com/checkout".to_string());

    support::with_temp_dir("overlay", |dir| {
        // ── Create a small overlay image ───────────────────────────────
        let logo = support::task("Create a 10x10 overlay logo", || {
            #[allow(clippy::cast_precision_loss)]
            ImageBuffer::from_fn(10, 10, |x, y| {
                // Blue circle approximation
                let cx = 5.0_f32;
                let cy = 5.0_f32;
                let dist = (x as f32 - cx).hypot(y as f32 - cy);
                if dist < 4.5 {
                    Rgba([0, 102, 204, 255])
                } else {
                    Rgba([255, 255, 255, 0]) // transparent
                }
            })
        });

        // ── Overlay using the method ───────────────────────────────────
        support::task_with_output("Overlay logo via QRCode::overlay_image", || {
            let img = qr.overlay_image(&logo);
            let (w, h) = img.dimensions();
            img.save(dir.join("overlay_method.png")).unwrap();
            vec![
                format!("Output size: {}x{} px (matches QR module grid)", w, h),
                "Logo placed at top-left (0, 0)".to_string(),
                "Use case:    Brand identity on QR codes".to_string(),
            ]
        });

        // ── Overlay using the macro ────────────────────────────────────
        support::task_with_output("Overlay logo via overlay_image! macro", || {
            let img = overlay_image!(qr, &logo);
            let (w, h) = img.dimensions();
            img.save(dir.join("overlay_macro.png")).unwrap();
            vec![
                "Macro:  overlay_image!(qr, &logo)".to_string(),
                format!("Size:   {}x{} px", w, h),
            ]
        });
    });

    support::summary(3);
}
