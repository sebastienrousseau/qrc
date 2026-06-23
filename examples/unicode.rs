//! Element: terminal rendering with `to_unicode` (half-block characters).
//!
//! Run: `cargo run --example unicode`

use qrc::encode::{Ecc, QrOptions};
use qrc::QRCode;

fn main() {
    let qr = QRCode::from_string("https://example.com".to_string());
    // High ECC keeps the code robust at terminal resolution.
    let art = qr.to_unicode(&QrOptions::new().ecc(Ecc::High)).unwrap();
    println!("Scan this:\n");
    print!("{art}");
}
