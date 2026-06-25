// Copyright © 2022-2026 QR Code Library (QRC). All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Business-card (vCard) payloads.
//!
//! [`BusinessCard`] builds an [RFC 6350](https://www.rfc-editor.org/rfc/rfc6350)
//! / vCard 3.0 string that QR scanners recognise as a contact, prompting "add
//! to contacts" rather than showing raw text. Encode the result like any other
//! string:
//!
//! ```
//! use qrc::payload::vcard::BusinessCard;
//! use qrc::QRCode;
//!
//! let card = BusinessCard::new("Jane Doe")
//!     .organization("Acme, Inc.")
//!     .title("CEO")
//!     .phone("+1-555-0100")
//!     .email("jane@acme.example");
//! let qr = QRCode::from_string(card.to_vcard());
//! assert!(qr.try_to_qrcode().is_ok());
//! ```

/// Escapes a value for inclusion in a vCard property per RFC 6350 §3.4:
/// backslash, comma and semicolon are escaped, and newlines become `\n`.
fn escape(value: &str) -> String {
    let mut out = String::with_capacity(value.len());
    for ch in value.chars() {
        match ch {
            '\\' => out.push_str("\\\\"),
            ',' => out.push_str("\\,"),
            ';' => out.push_str("\\;"),
            '\n' => out.push_str("\\n"),
            '\r' => {}
            _ => out.push(ch),
        }
    }
    out
}

/// A contact / business card that serialises to a vCard 3.0 string.
///
/// Only [`BusinessCard::new`] (the formatted name) is required; every other
/// field is optional and omitted from the output when unset.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct BusinessCard {
    /// Formatted display name (vCard `FN`, required).
    full_name: String,
    /// Given (first) name, used to build the structured `N` property.
    first_name: Option<String>,
    /// Family (last) name, used to build the structured `N` property.
    last_name: Option<String>,
    /// Organisation (`ORG`).
    organization: Option<String>,
    /// Job title (`TITLE`).
    title: Option<String>,
    /// Telephone number (`TEL;TYPE=CELL`).
    phone: Option<String>,
    /// Email address (`EMAIL`).
    email: Option<String>,
    /// Website URL (`URL`).
    url: Option<String>,
    /// Free-form street/city address (`ADR;TYPE=WORK`).
    address: Option<String>,
    /// Free-form note (`NOTE`).
    note: Option<String>,
}

impl BusinessCard {
    /// Creates a card with the given formatted (display) name.
    #[must_use]
    pub fn new(full_name: impl Into<String>) -> Self {
        BusinessCard {
            full_name: full_name.into(),
            ..Default::default()
        }
    }

    /// Sets the structured given/family name (`N` property).
    #[must_use]
    pub fn name(mut self, first: impl Into<String>, last: impl Into<String>) -> Self {
        self.first_name = Some(first.into());
        self.last_name = Some(last.into());
        self
    }

    /// Sets the organisation.
    #[must_use]
    pub fn organization(mut self, org: impl Into<String>) -> Self {
        self.organization = Some(org.into());
        self
    }

    /// Sets the job title.
    #[must_use]
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
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

    /// Sets a free-form address line.
    #[must_use]
    pub fn address(mut self, address: impl Into<String>) -> Self {
        self.address = Some(address.into());
        self
    }

    /// Sets a free-form note.
    #[must_use]
    pub fn note(mut self, note: impl Into<String>) -> Self {
        self.note = Some(note.into());
        self
    }

    /// Serialises the card to a vCard 3.0 string.
    #[must_use]
    pub fn to_vcard(&self) -> String {
        let mut s = String::from("BEGIN:VCARD\r\nVERSION:3.0\r\n");

        // Structured name (N): Family;Given;;;
        if self.first_name.is_some() || self.last_name.is_some() {
            let last = self.last_name.as_deref().unwrap_or_default();
            let first = self.first_name.as_deref().unwrap_or_default();
            s.push_str(&format!("N:{};{};;;\r\n", escape(last), escape(first)));
        }

        s.push_str(&format!("FN:{}\r\n", escape(&self.full_name)));

        for (prop, value) in [
            ("ORG", &self.organization),
            ("TITLE", &self.title),
            ("URL", &self.url),
            ("NOTE", &self.note),
        ] {
            if let Some(v) = value {
                s.push_str(&format!("{prop}:{}\r\n", escape(v)));
            }
        }
        if let Some(phone) = &self.phone {
            s.push_str(&format!("TEL;TYPE=CELL:{}\r\n", escape(phone)));
        }
        if let Some(email) = &self.email {
            s.push_str(&format!("EMAIL:{}\r\n", escape(email)));
        }
        if let Some(address) = &self.address {
            s.push_str(&format!("ADR;TYPE=WORK:;;{};;;;\r\n", escape(address)));
        }

        s.push_str("END:VCARD");
        s
    }
}

impl core::fmt::Display for BusinessCard {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(&self.to_vcard())
    }
}
