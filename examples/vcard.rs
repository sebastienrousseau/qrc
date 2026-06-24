//! Element: vCard business-card payloads — `BusinessCard`.
//!
//! Run: `cargo run --example vcard`

use qrc::payload::vcard::BusinessCard;
use qrc::QRCode;

fn main() {
    let card = BusinessCard::new("Jane Doe")
        .name("Jane", "Doe")
        .organization("Acme, Inc.")
        .title("CEO")
        .phone("+1-555-0100")
        .email("jane@acme.example")
        .url("https://acme.example")
        .address("1 Market Street")
        .note("Scan to add me");
    println!("{}", card.to_vcard());

    // Only the formatted name is required.
    println!("\nminimal:\n{}", BusinessCard::new("Solo").to_vcard());

    assert!(QRCode::from_string(card.to_vcard()).try_to_qrcode().is_ok());
}
