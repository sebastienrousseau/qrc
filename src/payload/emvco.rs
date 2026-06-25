// Copyright © 2022-2026 QR Code Library (QRC). All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! EMVCo Merchant-Presented Mode (MPM) payment payloads.
//!
//! Builds the TLV-encoded, CRC-checked string defined by the
//! [EMVCo QR Code Specification for Payment Systems (MPM)][emvco] that banking
//! apps scan to pay a merchant. Field values are assumed to be ASCII and under
//! 100 characters (the spec's two-digit length).
//!
//! ```
//! use qrc::payload::emvco::{MerchantAccount, MerchantPayment};
//! use qrc::QRCode;
//!
//! let account = MerchantAccount::new(26, "com.example.pay").merchant_id("12345678");
//! let payment = MerchantPayment::new(account, "840", "US", "Acme Coffee", "Springfield")
//!     .amount("4.50");
//! let qr = QRCode::from_string(payment.to_emvco());
//! assert!(qr.try_to_qrcode().is_ok());
//! ```
//!
//! [emvco]: https://www.emvco.com/emv-technologies/qrcodes/

/// CRC-16/CCITT-FALSE (poly `0x1021`, init `0xFFFF`) over `data`, as required by
/// EMVCo tag 63.
fn crc16(data: &[u8]) -> u16 {
    let mut crc: u16 = 0xFFFF;
    for &byte in data {
        crc ^= u16::from(byte) << 8;
        for _ in 0..8 {
            crc = if crc & 0x8000 != 0 {
                (crc << 1) ^ 0x1021
            } else {
                crc << 1
            };
        }
    }
    crc
}

/// Encodes one EMVCo data object as `ID || LEN || VALUE` (two-digit length).
fn tlv(id: &str, value: &str) -> String {
    format!("{id}{:02}{value}", value.len())
}

/// A merchant account information object (tags 26–51): a globally-unique
/// identifier plus an optional scheme-specific merchant id.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MerchantAccount {
    /// Tag in the 26–51 range that identifies the payment scheme.
    tag: u8,
    /// Globally unique identifier (sub-tag 00), e.g. a reverse-DNS string.
    guid: String,
    /// Scheme-specific merchant identifier (sub-tag 01).
    merchant_id: Option<String>,
}

impl MerchantAccount {
    /// Creates account information under `tag` (clamped to 26–51) with the given
    /// globally-unique identifier.
    #[must_use]
    pub fn new(tag: u8, guid: impl Into<String>) -> Self {
        MerchantAccount {
            tag: tag.clamp(26, 51),
            guid: guid.into(),
            merchant_id: None,
        }
    }

    /// Sets the scheme-specific merchant id (sub-tag 01).
    #[must_use]
    pub fn merchant_id(mut self, id: impl Into<String>) -> Self {
        self.merchant_id = Some(id.into());
        self
    }

    /// Serialises to the outer `tag || len || (sub-TLVs)` object.
    fn to_tlv(&self) -> String {
        let mut inner = tlv("00", &self.guid);
        if let Some(id) = &self.merchant_id {
            inner.push_str(&tlv("01", id));
        }
        tlv(&format!("{:02}", self.tag), &inner)
    }
}

/// An EMVCo merchant-presented payment that serialises to a scannable string.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MerchantPayment {
    /// Merchant account information (tags 26–51).
    account: MerchantAccount,
    /// Merchant Category Code (tag 52).
    category_code: String,
    /// Transaction currency, ISO 4217 numeric (tag 53), e.g. `840` for USD.
    currency: String,
    /// Transaction amount (tag 54); when present the code is dynamic.
    amount: Option<String>,
    /// Country code, ISO 3166-1 alpha-2 (tag 58).
    country_code: String,
    /// Merchant name (tag 59).
    merchant_name: String,
    /// Merchant city (tag 60).
    merchant_city: String,
}

impl MerchantPayment {
    /// Creates a static payment with the required fields. `currency` is the
    /// ISO 4217 numeric code and `country_code` the ISO 3166-1 alpha-2 code.
    #[must_use]
    pub fn new(
        account: MerchantAccount,
        currency: impl Into<String>,
        country_code: impl Into<String>,
        merchant_name: impl Into<String>,
        merchant_city: impl Into<String>,
    ) -> Self {
        MerchantPayment {
            account,
            category_code: "0000".to_string(),
            currency: currency.into(),
            amount: None,
            country_code: country_code.into(),
            merchant_name: merchant_name.into(),
            merchant_city: merchant_city.into(),
        }
    }

    /// Sets the Merchant Category Code (tag 52).
    #[must_use]
    pub fn category_code(mut self, mcc: impl Into<String>) -> Self {
        self.category_code = mcc.into();
        self
    }

    /// Sets the transaction amount (tag 54), making the code dynamic.
    #[must_use]
    pub fn amount(mut self, amount: impl Into<String>) -> Self {
        self.amount = Some(amount.into());
        self
    }

    /// Serialises to the full EMVCo MPM payload, including the trailing CRC.
    #[must_use]
    pub fn to_emvco(&self) -> String {
        let mut s = String::new();
        s.push_str(&tlv("00", "01")); // payload format indicator
                                      // point of initiation: 11 static, 12 dynamic
        s.push_str(&tlv("01", if self.amount.is_some() { "12" } else { "11" }));
        s.push_str(&self.account.to_tlv());
        s.push_str(&tlv("52", &self.category_code));
        s.push_str(&tlv("53", &self.currency));
        if let Some(amount) = &self.amount {
            s.push_str(&tlv("54", amount));
        }
        s.push_str(&tlv("58", &self.country_code));
        s.push_str(&tlv("59", &self.merchant_name));
        s.push_str(&tlv("60", &self.merchant_city));
        // CRC is computed over everything including the tag+length "6304".
        s.push_str("6304");
        s.push_str(&format!("{:04X}", crc16(s.as_bytes())));
        s
    }
}

impl core::fmt::Display for MerchantPayment {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(&self.to_emvco())
    }
}
