// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright (c) 2022-2026 QRC. All rights reserved.

//! Add a watermark image to a QR code.
//!
//! Demonstrates placing a small logo or watermark in the bottom-right
//! corner of a QR code, useful for branding while keeping scannability.
//!
//! Run: `cargo run --example watermark`

#[path = "support.rs"]
mod support;

use image::{ImageBuffer, Rgba};
use qrc::{add_image_watermark, QRCode};

fn main() {
    support::header("qrc -- watermark");

    let qr = QRCode::from_string("https://example.com/product/42".to_string());

    support::with_temp_dir("watermark", |dir| {
        // ── Create a small logo programmatically ───────────────────────
        let logo = support::task("Create a 20x20 brand logo (red square)", || {
            ImageBuffer::from_fn(20, 20, |x, y| {
                // Red square with a 2px white border
                if !(2..18).contains(&x) || !(2..18).contains(&y) {
                    Rgba([255, 255, 255, 255])
                } else {
                    Rgba([220, 20, 60, 255]) // crimson
                }
            })
        });

        // ── Apply watermark using the method ───────────────────────────
        support::task_with_output("Apply watermark via QRCode::add_image_watermark", || {
            let mut img = qr.to_png(256);
            QRCode::add_image_watermark(&mut img, &logo);
            let (w, h) = img.dimensions();
            img.save(dir.join("watermarked_method.png")).unwrap();
            vec![
                format!("QR size:    {}x{} px", w, h),
                "Logo size:  20x20 px".to_string(),
                "Position:   Bottom-right corner (alpha blended)".to_string(),
            ]
        });

        // ── Apply watermark using the macro ────────────────────────────
        support::task_with_output("Apply watermark via add_image_watermark! macro", || {
            let mut img = qr.to_png(256);
            add_image_watermark!(&mut img, &logo);
            img.save(dir.join("watermarked_macro.png")).unwrap();
            vec![
                "Macro:   add_image_watermark!(&mut img, &logo)".to_string(),
                "Result:  Identical to method call".to_string(),
            ]
        });

        // ── Larger watermark on high-res QR ────────────────────────────
        support::task_with_output("Larger logo on 512px QR code", || {
            let big_logo = ImageBuffer::from_fn(40, 40, |_, _| Rgba([0, 102, 204, 200]));
            let mut img = qr.to_png(512);
            QRCode::add_image_watermark(&mut img, &big_logo);
            img.save(dir.join("watermarked_large.png")).unwrap();
            vec![
                "QR size:    512x512 px".to_string(),
                "Logo size:  40x40 px".to_string(),
                "Tip:        Keep logo < 10%% of QR area for scannability".to_string(),
            ]
        });
    });

    support::summary(4);
}
