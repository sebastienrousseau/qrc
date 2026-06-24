//! Element: EMVCo merchant-payment payloads — `MerchantAccount` /
//! `MerchantPayment` (TLV + CRC-16/CCITT).
//!
//! Run: `cargo run --example emvco`

use qrc::payload::emvco::{MerchantAccount, MerchantPayment};
use qrc::QRCode;

fn main() {
    let account = MerchantAccount::new(26, "com.example.pay").merchant_id("12345678");

    // Dynamic payment (with amount).
    let dynamic = MerchantPayment::new(account.clone(), "840", "US", "Acme Coffee", "Springfield")
        .category_code("5814")
        .amount("4.50");
    println!("dynamic: {}", dynamic.to_emvco());

    // Static payment (no amount — payer enters it).
    let static_payment = MerchantPayment::new(account, "840", "US", "Acme Coffee", "Springfield");
    println!("static:  {}", static_payment.to_emvco());

    assert!(QRCode::from_string(dynamic.to_emvco())
        .try_to_qrcode()
        .is_ok());
}
