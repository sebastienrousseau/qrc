// Copyright © 2022-2026 QR Code Library (QRC). All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Wi-Fi network join payloads.
//!
//! [`WifiNetwork`] builds the `WIFI:...;;` string that phone cameras recognise
//! to offer "join network". Encode it like any other string:
//!
//! ```
//! use qrc::payload::wifi::{WifiNetwork, WifiSecurity};
//! use qrc::QRCode;
//!
//! let wifi = WifiNetwork::new("Cafe Guest")
//!     .security(WifiSecurity::Wpa)
//!     .password("latte123");
//! let qr = QRCode::from_string(wifi.to_qr_string());
//! assert!(qr.try_to_qrcode().is_ok());
//! ```

/// The authentication type of a Wi-Fi network.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum WifiSecurity {
    /// WPA/WPA2/WPA3 personal (the common case).
    #[default]
    Wpa,
    /// Legacy WEP.
    Wep,
    /// Open network (no password).
    None,
}

impl WifiSecurity {
    /// The `T:` token value used in the payload.
    fn token(self) -> &'static str {
        match self {
            WifiSecurity::Wpa => "WPA",
            WifiSecurity::Wep => "WEP",
            WifiSecurity::None => "nopass",
        }
    }
}

/// Escapes a value for a Wi-Fi payload: `\`, `;`, `,`, `:` and `"` are
/// backslash-escaped per the de-facto MeCard-style convention.
fn escape(value: &str) -> String {
    let mut out = String::with_capacity(value.len());
    for ch in value.chars() {
        if matches!(ch, '\\' | ';' | ',' | ':' | '"') {
            out.push('\\');
        }
        out.push(ch);
    }
    out
}

/// A Wi-Fi network that serialises to a `WIFI:...;;` join string.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct WifiNetwork {
    /// Network name (SSID).
    ssid: String,
    /// Pre-shared key / password (omitted for open networks).
    password: Option<String>,
    /// Authentication type.
    security: WifiSecurity,
    /// Whether the SSID is hidden (not broadcast).
    hidden: bool,
}

impl WifiNetwork {
    /// Creates a network with the given SSID (WPA by default).
    #[must_use]
    pub fn new(ssid: impl Into<String>) -> Self {
        WifiNetwork {
            ssid: ssid.into(),
            ..Default::default()
        }
    }

    /// Sets the authentication type.
    #[must_use]
    pub fn security(mut self, security: WifiSecurity) -> Self {
        self.security = security;
        self
    }

    /// Sets the password / pre-shared key.
    #[must_use]
    pub fn password(mut self, password: impl Into<String>) -> Self {
        self.password = Some(password.into());
        self
    }

    /// Marks the SSID as hidden.
    #[must_use]
    pub fn hidden(mut self, hidden: bool) -> Self {
        self.hidden = hidden;
        self
    }

    /// Serialises the network to its `WIFI:...;;` payload string.
    #[must_use]
    pub fn to_qr_string(&self) -> String {
        let mut s = format!("WIFI:T:{};S:{};", self.security.token(), escape(&self.ssid));
        // Open networks carry no key.
        if self.security != WifiSecurity::None {
            if let Some(password) = &self.password {
                s.push_str(&format!("P:{};", escape(password)));
            }
        }
        if self.hidden {
            s.push_str("H:true;");
        }
        s.push(';');
        s
    }
}

impl core::fmt::Display for WifiNetwork {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(&self.to_qr_string())
    }
}
