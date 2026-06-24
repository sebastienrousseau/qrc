// Copyright © 2022-2026 QR Code Library (QRC). All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Structured payload builders.
//!
//! These turn structured data into the exact text conventions that QR scanners
//! recognise (contacts, etc.), so the decoded code triggers a rich action
//! rather than showing raw text. They are plain string builders with no extra
//! dependencies.

pub mod emvco;
pub mod mecard;
pub mod vcard;
pub mod wifi;

pub use emvco::{MerchantAccount, MerchantPayment};
pub use mecard::MeCard;
pub use vcard::BusinessCard;
pub use wifi::{WifiNetwork, WifiSecurity};
