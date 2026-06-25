//! Element: Wi-Fi join payloads — `WifiNetwork` / `WifiSecurity`.
//!
//! Run: `cargo run --example wifi`

use qrc::payload::wifi::{WifiNetwork, WifiSecurity};
use qrc::QRCode;

fn main() {
    // WPA network with a password, hidden SSID.
    let wpa = WifiNetwork::new("Cafe; Guest")
        .security(WifiSecurity::Wpa)
        .password("p@ss,word")
        .hidden(true);
    println!("WPA:  {}", wpa.to_qr_string()); // special chars are escaped

    // Open network (the password is omitted).
    let open = WifiNetwork::new("Free WiFi").security(WifiSecurity::None);
    println!("Open: {}", open.to_qr_string());

    // Encodes like any string.
    assert!(QRCode::from_string(wpa.to_qr_string())
        .try_to_qrcode()
        .is_ok());
    println!("encodable: yes");
}
