<!-- SPDX-License-Identifier: Apache-2.0 OR MIT -->

# Changelog

All notable changes to **QRC** are documented here.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).
While the crate is pre-1.0, minor breaking changes may land in patch releases.

## [0.0.6] - Unreleased

### Added

- **Structured payload builders** (`payload` module): `vcard::BusinessCard`
  (RFC 6350), `wifi::WifiNetwork`, `mecard::MeCard`, and
  `emvco::MerchantPayment` (EMVCo MPM + CRC-16/CCITT) — dependency-free string
  builders for codes that scanners act on.
- **Offline "art QR" primitives**: `QRCode::to_control_image` (a clean,
  high-contrast control image for a Stable Diffusion QR ControlNet) and
  `QRCode::blend_image` (weaves a background image through the data modules
  while staying scannable), tuned via `BlendOptions`.
- **Module shapes**: `ModuleShape::{Square, RoundedSquare, Circle, Diamond}`
  via `QRCode::with_shape`, for raster and SVG output.
- **Error-correction selection**: `QRCode::with_ec_level` (`EcLevel::{L,M,Q,H}`).
- **WASM bindings** (`wasm` feature): `WasmQRCode` for use from JavaScript.
- Branded business-card example (vCard + centred logo) and a set of focused,
  per-element examples.
- Supply-chain CI: `cargo audit` + `cargo deny`, plus pre-commit hooks for
  secret and large-file scanning.

### Changed

- **BREAKING:** `QRCode::to_png_bytes`, `to_jpg`, `to_jpg_with_quality`, and
  `to_gif` now return `Result<Vec<u8>, image::ImageError>` instead of panicking
  via `.expect(...)`. The `qr_code_to!` macro therefore also yields a `Result`.
  Migration: append `?` or `.unwrap()` at the call site.
- Trimmed the `image` dependency to the required codecs
  (`png`, `jpeg`, `gif`, `ico`), substantially shrinking the dependency tree
  for downstream consumers.
- Minimum Supported Rust Version is now **1.75.0**.

### Fixed

- `QRCode::overlay_image` now renders the code at a usable scale with the
  mandatory 4-module quiet zone and **centres** the overlay, instead of
  rendering at 1px/module and pasting the logo over the top-left finder pattern
  ([#41]). The result is scannable.

### Quality

- 100% test coverage (lines, functions, and regions) and 100% public-item
  documentation; the README was rewritten and verified against the API.

## [0.0.5] - 2024

- Prior releases generated PNG/JPG/GIF/SVG QR codes with colour customisation,
  watermarks, logo overlays, batch generation, and convenience macros.

[0.0.6]: https://github.com/sebastienrousseau/qrc/compare/v0.0.5...main
[0.0.5]: https://github.com/sebastienrousseau/qrc/releases/tag/v0.0.5
[#41]: https://github.com/sebastienrousseau/qrc/issues/41
