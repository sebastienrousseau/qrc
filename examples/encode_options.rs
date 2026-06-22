//! Encoding layer: `QrOptions`, error-correction levels, forced versions,
//! quiet zones, the pluggable `Engine` trait, `Matrix` inspection, and the
//! fallible API.
//!
//! Run with: `cargo run --example encode_options`

use qrc::encode::{encode, Ecc, Engine, QrOptions, QrcodeEngine};
use qrc::QRCode;

fn main() {
    // --- Error-correction levels ---------------------------------------
    for ecc in [Ecc::Low, Ecc::Medium, Ecc::Quartile, Ecc::High] {
        let m = encode(b"https://example.com", &QrOptions::new().ecc(ecc)).unwrap();
        println!("ecc {ecc:?}: {0}x{0} modules", m.size());
    }

    // --- Forced version + custom quiet zone (builder style) ------------
    let opts = QrOptions::new().ecc(Ecc::High).version(5).quiet_zone(6);
    let matrix = encode(b"forced version 5", &opts).unwrap();
    println!(
        "version 5 -> {}x{} modules, quiet zone {}, total {}",
        matrix.size(),
        matrix.size(),
        matrix.quiet_zone(),
        matrix.total_size()
    );

    // --- Matrix inspection ---------------------------------------------
    println!("top-left module dark? {}", matrix.is_dark(0, 0));
    println!(
        "module including quiet zone at (0,0) dark? {}",
        matrix.is_dark_with_quiet_zone(0, 0)
    );

    // --- The Engine trait (swap the backend without touching the API) --
    let engine: &dyn Engine = &QrcodeEngine;
    let via_engine = engine
        .encode(b"via trait object", &QrOptions::new())
        .unwrap();
    println!(
        "engine produced {}x{} modules",
        via_engine.size(),
        via_engine.size()
    );

    // --- Fallible vs infallible ----------------------------------------
    let qr = QRCode::from_string("safe for untrusted input".to_string());
    match qr.encode(&QrOptions::new()) {
        Ok(m) => println!("encoded ok: {} modules", m.size()),
        Err(e) => println!("encode failed: {e}"),
    }

    // Over-capacity data returns an error instead of panicking.
    let huge = QRCode::from_string("Z".repeat(8000));
    assert!(huge.encode(&QrOptions::new()).is_err());
    println!("over-capacity payload correctly rejected (no panic)");

    // Forcing too small a version is also a recoverable error.
    assert!(encode(&vec![b'A'; 2000], &QrOptions::new().version(1)).is_err());
    println!("under-sized forced version correctly rejected");
}
