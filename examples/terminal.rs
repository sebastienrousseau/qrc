//! Terminal rendering with Unicode half-block characters — scan it straight
//! off the screen.
//!
//! Run with: `cargo run --example terminal`

use qrc::encode::{Ecc, QrOptions};
use qrc::QRCode;

fn main() {
    let qr = QRCode::from_string("https://example.com".to_string());

    // High ECC keeps the code robust even at terminal resolution.
    let opts = QrOptions::new().ecc(Ecc::High);
    let art = qr.to_unicode(&opts).unwrap();

    println!("Scan this:\n");
    print!("{art}");
}
