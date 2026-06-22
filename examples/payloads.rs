//! Structured payload builders: Wi-Fi join, MeCard contact, and EMVCo payment.
//!
//! Each produces a string that scanners recognise as a rich action. Pair them
//! with the styling/branding renderers for a polished, on-brand code.
//!
//! Run with: `cargo run --example payloads`

use qrc::encode::{Ecc, QrOptions};
use qrc::payload::emvco::{MerchantAccount, MerchantPayment};
use qrc::payload::mecard::MeCard;
use qrc::payload::wifi::{WifiNetwork, WifiSecurity};
use qrc::render::svg::SvgOptions;
use qrc::QRCode;

fn main() {
    // Wi-Fi: "join network" prompt.
    let wifi = WifiNetwork::new("Cafe Guest")
        .security(WifiSecurity::Wpa)
        .password("latte123")
        .hidden(false);
    println!("WiFi:   {}", wifi.to_qr_string());

    // MeCard: compact contact (fits a smaller QR than vCard).
    let card = MeCard::new("Doe,Jane")
        .phone("+1-555-0100")
        .email("jane@acme.example")
        .url("https://acme.example");
    println!("MeCard: {}", card.to_mecard());

    // EMVCo: merchant-presented payment with amount and CRC.
    let account = MerchantAccount::new(26, "com.example.pay").merchant_id("12345678");
    let payment = MerchantPayment::new(account, "840", "US", "Acme Coffee", "Springfield")
        .category_code("5814")
        .amount("4.50");
    println!("EMVCo:  {}", payment.to_emvco());

    // Any payload encodes like a normal string — here, a branded SVG of the
    // Wi-Fi code at high error correction (good for adding a logo later).
    let svg = QRCode::from_string(wifi.to_qr_string())
        .to_svg_styled(&QrOptions::new().ecc(Ecc::High), &SvgOptions::default())
        .unwrap();
    println!("\nWi-Fi code as SVG: {} bytes", svg.len());
}
