// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright (c) 2022-2026 QRC. All rights reserved.

//! Multi-language QR codes for international audiences.
//!
//! Encodes language-specific content selected by user preference.
//! Ideal for multilingual product packaging, museum guides, or
//! international event materials.
//!
//! Run: `cargo run --example multilingual`

#[path = "support.rs"]
mod support;

use qrc::{create_multilanguage_qr, QRCode};
use std::collections::HashMap;

fn main() {
    support::header("qrc -- multilingual");

    // ── Using the method directly ──────────────────────────────────────
    support::task_with_output("Create multilingual QR via HashMap", || {
        let mut translations = HashMap::new();
        translations.insert("en".to_string(), "Welcome to our store!".to_string());
        translations.insert("es".to_string(), "Bienvenido a nuestra tienda!".to_string());
        translations.insert(
            "fr".to_string(),
            "Bienvenue dans notre magasin!".to_string(),
        );
        translations.insert("de".to_string(), "Willkommen in unserem Laden!".to_string());

        let qr = QRCode::create_multilanguage(translations);
        vec![
            format!("Selected:   \"{}\"", String::from_utf8_lossy(&qr.data)),
            format!("Language:   en (default)"),
            format!("Use case:   International product packaging"),
        ]
    });

    // ── Using the macro ────────────────────────────────────────────────
    support::task_with_output("Create via create_multilanguage_qr! macro", || {
        let qr = create_multilanguage_qr! {
            "en" => "https://example.com/en/guide",
            "es" => "https://example.com/es/guia",
            "fr" => "https://example.com/fr/guide",
            "ja" => "https://example.com/ja/gaido",
        };
        vec![
            format!("Encoded: {}", String::from_utf8_lossy(&qr.data)),
            format!("Macro:   create_multilanguage_qr! {{ \"en\" => ..., ... }}"),
        ]
    });

    // ── Museum exhibit example ─────────────────────────────────────────
    support::task_with_output("Museum exhibit: audio guide links", || {
        let qr = create_multilanguage_qr! {
            "en" => "https://museum.example.com/audio/mona-lisa/en",
            "fr" => "https://museum.example.com/audio/la-joconde/fr",
            "it" => "https://museum.example.com/audio/la-gioconda/it",
        };
        let internal = qr.to_qrcode();
        vec![
            format!("Content:  {}", String::from_utf8_lossy(&qr.data)),
            format!("Modules:  {}x{}", internal.width(), internal.width()),
            format!("Scenario: Visitor scans, gets audio in their language"),
        ]
    });

    support::summary(3);
}
