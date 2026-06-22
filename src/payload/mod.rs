// Copyright © 2022-2026 QR Code Library (QRC). All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Structured payload builders.
//!
//! These turn structured data into the exact text conventions that QR scanners
//! recognise (contacts, etc.), so the decoded code triggers a rich action
//! rather than showing raw text. They are plain string builders with no extra
//! dependencies, gated behind the `payload` feature.

pub mod vcard;

pub use vcard::BusinessCard;
