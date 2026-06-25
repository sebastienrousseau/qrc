//! Element: MeCard contact payloads — `MeCard`.
//!
//! Run: `cargo run --example mecard`

use qrc::payload::mecard::MeCard;
use qrc::QRCode;

fn main() {
    let card = MeCard::new("Doe,Jane")
        .reading("doe,jane")
        .phone("+1-555-0100")
        .email("jane@acme.example")
        .url("https://acme.example")
        .address("1 Market St")
        .birthday("19900101")
        .note("hi there");
    println!("{}", card.to_mecard());

    let minimal = MeCard::new("Solo");
    println!("minimal: {}", minimal.to_mecard());

    assert!(QRCode::from_string(card.to_mecard())
        .try_to_qrcode()
        .is_ok());
}
