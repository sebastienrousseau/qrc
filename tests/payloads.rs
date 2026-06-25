//! Structured payload builders: Wi-Fi, MeCard and EMVCo merchant payments.

use qrc::payload::emvco::{MerchantAccount, MerchantPayment};
use qrc::payload::mecard::MeCard;
use qrc::payload::wifi::{WifiNetwork, WifiSecurity};
use qrc::QRCode;

/// Asserts that `payload` encodes to a valid QR code (correct version selected,
/// no panic) — i.e. the builder output is within QR capacity and encodable.
fn encodes(payload: &str) {
    assert!(QRCode::from_string(payload.to_string())
        .try_to_qrcode()
        .is_ok());
}

// --- Wi-Fi -----------------------------------------------------------------

#[test]
fn wifi_wpa_with_password_and_hidden() {
    let wifi = WifiNetwork::new("Cafe; Guest")
        .security(WifiSecurity::Wpa)
        .password("p:a,s\"s\\word")
        .hidden(true);
    let s = wifi.to_qr_string();
    assert_eq!(
        s,
        "WIFI:T:WPA;S:Cafe\\; Guest;P:p\\:a\\,s\\\"s\\\\word;H:true;;"
    );
    assert_eq!(wifi.to_string(), s);
    encodes(&s);
}

#[test]
fn wifi_open_network_omits_password() {
    // An open network drops the key even if one was set.
    let s = WifiNetwork::new("Free WiFi")
        .security(WifiSecurity::None)
        .password("ignored")
        .to_qr_string();
    assert_eq!(s, "WIFI:T:nopass;S:Free WiFi;;");
}

#[test]
fn wifi_wep_without_password_or_hidden() {
    let s = WifiNetwork::new("Old")
        .security(WifiSecurity::Wep)
        .to_qr_string();
    assert_eq!(s, "WIFI:T:WEP;S:Old;;");
    assert_eq!(WifiSecurity::default(), WifiSecurity::Wpa);
    assert_eq!(WifiNetwork::default(), WifiNetwork::new(""));
}

// --- MeCard ----------------------------------------------------------------

#[test]
fn mecard_full_and_minimal() {
    let card = MeCard::new("Doe,Jane")
        .reading("doe,jane")
        .phone("+15550100")
        .email("jane@acme.example")
        .url("https://acme.example")
        .address("1 Market St")
        .birthday("19900101")
        .note("hi; there");
    let s = card.to_mecard();
    assert!(s.starts_with("MECARD:N:Doe\\,Jane;"));
    assert!(s.contains("TEL:+15550100;"));
    assert!(s.contains("SOUND:doe\\,jane;"));
    assert!(s.contains("NOTE:hi\\; there;"));
    assert!(s.ends_with(";;"));
    assert_eq!(card.to_string(), s);
    encodes(&s);

    let minimal = MeCard::new("Solo").to_mecard();
    assert_eq!(minimal, "MECARD:N:Solo;;");
    assert_eq!(MeCard::default(), MeCard::new(""));
}

// --- EMVCo -----------------------------------------------------------------

#[test]
fn emvco_dynamic_payment_round_trips_and_has_valid_crc() {
    let account = MerchantAccount::new(26, "com.example.pay").merchant_id("12345678");
    let payment = MerchantPayment::new(account, "840", "US", "Acme Coffee", "Springfield")
        .category_code("5814")
        .amount("4.50");
    let s = payment.to_emvco();

    assert!(s.starts_with("000201")); // payload format indicator
    assert!(s.contains("010212")); // dynamic point-of-initiation
    assert!(s.contains("5303840")); // currency 840
    assert!(s.contains("54044.50")); // amount
    assert!(s.contains("5802US")); // country
    assert_eq!(payment.to_string(), s);
    encodes(&s);

    // The trailing 4 hex chars are a correct CRC over the rest + "6304".
    let (body, crc) = s.split_at(s.len() - 4);
    let mut c: u16 = 0xFFFF;
    for &b in body.as_bytes() {
        c ^= u16::from(b) << 8;
        for _ in 0..8 {
            c = if c & 0x8000 != 0 {
                (c << 1) ^ 0x1021
            } else {
                c << 1
            };
        }
    }
    assert_eq!(crc, format!("{c:04X}"));
}

#[test]
fn emvco_static_payment_and_tag_clamping() {
    // No amount -> static (point of initiation 11), no tag 54.
    let account = MerchantAccount::new(10, "com.example"); // tag clamped up to 26
    let s = MerchantPayment::new(account, "978", "FR", "Boulangerie", "Paris").to_emvco();
    assert!(s.contains("010211")); // static
    assert!(!s.contains("5404"));
    assert!(s.contains("26150011com.example")); // tag 26, len 15, inner 00||11||com.example

    // Tag above range clamps down to 51.
    let high = MerchantAccount::new(99, "g");
    let s = MerchantPayment::new(high, "840", "US", "M", "C").to_emvco();
    assert!(s.contains("51050001g")); // clamped tag 51, inner = 00||01||g
}
