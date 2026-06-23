//! Element: the `QRCode` type — construction and conversion.
//!
//! Run: `cargo run --example qrcode`

use qrc::QRCode;

fn main() {
    // Three ways to construct.
    let from_str = QRCode::from_string("https://example.com".to_string());
    let from_bytes = QRCode::from_bytes(vec![0x61, 0x62, 0x63]);
    let from_new = QRCode::new(b"raw bytes".to_vec());
    println!(
        "data lengths: {} {} {}",
        from_str.data.len(),
        from_bytes.data.len(),
        from_new.data.len()
    );

    // Fallible conversion (use for untrusted input).
    match from_str.try_to_qrcode() {
        Ok(code) => println!("encoded: {} modules wide", code.width()),
        Err(e) => println!("could not encode: {e}"),
    }

    // Infallible conversion (panics on over-capacity data — see the docs).
    let code = from_str.to_qrcode();
    println!("to_qrcode(): {} modules wide", code.width());

    // Over-capacity data is a recoverable error, not a panic.
    assert!(QRCode::from_string("Z".repeat(8000))
        .try_to_qrcode()
        .is_err());
    println!("over-capacity payload rejected without panic");
}
