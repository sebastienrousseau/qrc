//! Element: the `Engine` trait and the default `QrcodeEngine`.
//!
//! The engine abstraction lets the backend be swapped without touching the
//! rendering layer or public API.
//!
//! Run: `cargo run --example engine`

use qrc::encode::{encode, Engine, QrOptions, QrcodeEngine};

fn main() {
    let opts = QrOptions::new();

    // The convenience free function uses the default engine.
    let m = encode(b"https://example.com", &opts).unwrap();
    println!("encode(): {}x{} modules", m.size(), m.size());

    // The same engine used directly...
    let engine = QrcodeEngine;
    let m = engine.encode(b"via QrcodeEngine", &opts).unwrap();
    println!("QrcodeEngine: {} modules", m.size());

    // ...and through a trait object, so alternative backends are pluggable.
    let dynamic: &dyn Engine = &QrcodeEngine;
    let m = dynamic.encode(b"via &dyn Engine", &opts).unwrap();
    println!("&dyn Engine: {} modules", m.size());
}
