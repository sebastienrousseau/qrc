// Copyright © 2022-2026 QR Code Library (QRC). All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! MeCard contact payloads.
//!
//! MeCard is a compact contact format (originating with NTT DoCoMo) that many
//! scanners support alongside vCard; it is shorter, so it fits in a smaller QR.
//! For a richer business card use [`crate::payload::vcard`].
//!
//! ```
//! use qrc::payload::mecard::MeCard;
//! use qrc::QRCode;
//!
//! let card = MeCard::new("Doe,Jane").phone("+15550100").email("jane@acme.example");
//! let qr = QRCode::from_string(card.to_mecard());
//! assert!(qr.try_to_qrcode().is_ok());
//! ```

/// Escapes a value for a MeCard field: `\`, `;`, `:` and `,` are
/// backslash-escaped.
fn escape(value: &str) -> String {
    let mut out = String::with_capacity(value.len());
    for ch in value.chars() {
        if matches!(ch, '\\' | ';' | ':' | ',') {
            out.push('\\');
        }
        out.push(ch);
    }
    out
}

/// A MeCard contact that serialises to a `MECARD:...;;` string.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct MeCard {
    /// Name, conventionally `Last,First` (`N:`).
    name: String,
    /// Phonetic reading of the name (`SOUND:`).
    reading: Option<String>,
    /// Telephone number (`TEL:`).
    phone: Option<String>,
    /// Email address (`EMAIL:`).
    email: Option<String>,
    /// Website URL (`URL:`).
    url: Option<String>,
    /// Address (`ADR:`).
    address: Option<String>,
    /// Birthday as `YYYYMMDD` (`BDAY:`).
    birthday: Option<String>,
    /// Free-form note (`NOTE:`).
    note: Option<String>,
}

impl MeCard {
    /// Creates a card with the given name (conventionally `Last,First`).
    #[must_use]
    pub fn new(name: impl Into<String>) -> Self {
        MeCard {
            name: name.into(),
            ..Default::default()
        }
    }

    /// Sets the phonetic reading of the name.
    #[must_use]
    pub fn reading(mut self, reading: impl Into<String>) -> Self {
        self.reading = Some(reading.into());
        self
    }

    /// Sets the telephone number.
    #[must_use]
    pub fn phone(mut self, phone: impl Into<String>) -> Self {
        self.phone = Some(phone.into());
        self
    }

    /// Sets the email address.
    #[must_use]
    pub fn email(mut self, email: impl Into<String>) -> Self {
        self.email = Some(email.into());
        self
    }

    /// Sets the website URL.
    #[must_use]
    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.url = Some(url.into());
        self
    }

    /// Sets the address.
    #[must_use]
    pub fn address(mut self, address: impl Into<String>) -> Self {
        self.address = Some(address.into());
        self
    }

    /// Sets the birthday (`YYYYMMDD`).
    #[must_use]
    pub fn birthday(mut self, birthday: impl Into<String>) -> Self {
        self.birthday = Some(birthday.into());
        self
    }

    /// Sets a free-form note.
    #[must_use]
    pub fn note(mut self, note: impl Into<String>) -> Self {
        self.note = Some(note.into());
        self
    }

    /// Serialises the contact to its `MECARD:...;;` payload string.
    #[must_use]
    pub fn to_mecard(&self) -> String {
        let mut s = format!("MECARD:N:{};", escape(&self.name));
        for (tag, value) in [
            ("SOUND", &self.reading),
            ("TEL", &self.phone),
            ("EMAIL", &self.email),
            ("URL", &self.url),
            ("ADR", &self.address),
            ("BDAY", &self.birthday),
            ("NOTE", &self.note),
        ] {
            if let Some(v) = value {
                s.push_str(&format!("{tag}:{};", escape(v)));
            }
        }
        s.push(';');
        s
    }
}

impl core::fmt::Display for MeCard {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(&self.to_mecard())
    }
}
