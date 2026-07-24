<!-- SPDX-License-Identifier: Apache-2.0 OR MIT -->

<p align="center">
  <img src="https://cloudcdn.pro/qrc/v1/logos/qrc.svg" alt="QRC logo" width="128" />
</p>

<h1 align="center">QR Code Library (QRC)</h1>

<p align="center">
  <strong>Generate and style QR codes as PNG, JPG, GIF, and SVG — fast, dependency-light, and 100% safe Rust.</strong>
</p>

<p align="center">
  <a href="https://github.com/sebastienrousseau/qrc/actions"><img src="https://img.shields.io/github/actions/workflow/status/sebastienrousseau/qrc/ci.yml?style=for-the-badge&logo=github" alt="Build" /></a>
  <a href="https://crates.io/crates/qrc"><img src="https://img.shields.io/crates/v/qrc.svg?style=for-the-badge&color=fc8d62&logo=rust" alt="Crates.io" /></a>
  <a href="https://docs.rs/qrc"><img src="https://img.shields.io/badge/docs.rs-qrc-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" alt="Docs.rs" /></a>
  <a href="https://lib.rs/crates/qrc"><img src="https://img.shields.io/badge/lib.rs-v0.0.7-orange.svg?style=for-the-badge" alt="lib.rs" /></a>
  <a href="https://opensource.org/licenses/MIT"><img src="https://img.shields.io/crates/l/qrc.svg?style=for-the-badge" alt="License" /></a>
</p>

---

## Contents

- [Install](#install) — add it and start generating
- [Quick Start](#quick-start) — a QR code in 4 lines
- [Overview](#overview) — what QRC does
- [Features](#features) — v0.0.6 capability matrix
- [Library Usage](#library-usage) — formats, styling, watermarks
- [Structured Payloads](#structured-payloads) — vCard, Wi-Fi, MeCard, EMVCo
- [Macros](#macros) — 11 convenience macros
- [Examples](#examples) — 20 focused examples
- [Development](#development) — build, test, lint
- [Security](#security) — safety guarantees
- [Documentation](#documentation)
- [License](#license)

---

## Install

```toml
[dependencies]
qrc = "0.0.7"
```

…or from the command line:

```bash
cargo add qrc
```

### Cargo features

| Feature  | Default | Pulls in                         | Adds                                               |
| :------- | :-----: | :------------------------------- | :------------------------------------------------- |
| *(core)* |    ✓    | `image`, `qrcode`, `miniz_oxide` | QR generation, PNG/JPG/GIF/SVG, payloads, macros   |
| `wasm`   |         | `wasm-bindgen`, `js-sys`         | WebAssembly bindings (`qrc::wasm`) for the browser |

The core API needs **no default features**. The `image` dependency is compiled
with only the `png`, `jpeg`, `gif`, and `ico` codecs, keeping the tree lean.

### Build from source

```bash
git clone https://github.com/sebastienrousseau/qrc.git
cd qrc
cargo build --release
```

Requires **Rust 1.75.0+** (MSRV). Tested on Linux, macOS, and Windows.

---

## Quick Start

```rust
use qrc::QRCode;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Build a QR code from any string.
    let qr = QRCode::from_string("https://example.com".to_string());

    // `to_png` returns an `ImageBuffer`, so it has `.save`.
    qr.to_png(512).save("qrcode.png")?;

    // `to_svg` returns a `String` — write it yourself (scales infinitely).
    std::fs::write("qrcode.svg", qr.to_svg(512))?;

    Ok(())
}
```

---

## Overview

QRC turns strings, byte vectors, or raw data into QR code images. It renders to
four formats, lets you pick the error-correction level and module shape, and
ships builders for structured payloads (contacts, Wi-Fi, payments) — all with
zero `unsafe`.

- **4 output formats** — PNG, JPG, GIF, SVG
- **Styling** — 4 error-correction levels and 4 module shapes
- **Structured payloads** — vCard, Wi-Fi, MeCard, EMVCo merchant payments
- **Colour customisation** — any RGBA colour for dark modules
- **Watermarks & overlays** — corner watermark or centre logo
- **Batch generation** — many codes in one call
- **Zero unsafe code** — `#![forbid(unsafe_code)]` crate-wide

---

## Features

| | |
| :--- | :--- |
| **Formats** | PNG, JPG, GIF (raster via `image`), SVG (vector via `qrcode`) |
| **Error correction** | `EcLevel::{L, M, Q, H}` via `with_ec_level` (default `M`) |
| **Module shapes** | `ModuleShape::{Square, RoundedSquare, Circle, Diamond}` via `with_shape` |
| **Payloads** | `payload::{vcard, wifi, mecard, emvco}` — dependency-free string builders |
| **Colours** | Custom RGBA dark modules on a white background |
| **Watermarks / Overlays** | Alpha-blended corner watermark; centre logo overlay |
| **Resizing** | Arbitrary width/height scaling |
| **Batch / Combine** | `Vec<String>` → `Vec<QRCode>`; merge codes side-by-side |
| **Macros** | 11 convenience macros |
| **Safety** | `#![forbid(unsafe_code)]`, `#![deny(missing_docs)]` |
| **MSRV** | Rust 1.75.0 |
| **Runtime deps** | 3 (`image`, `qrcode`, `miniz_oxide`) |

---

## Library Usage

<details>
<summary><b>Generate all four formats</b></summary>

```rust
use qrc::QRCode;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let qr = QRCode::from_string("https://docs.rs/qrc".to_string());

    // `to_png` / `to_image` return an `ImageBuffer` → use `.save`.
    qr.to_png(512).save("qrcode.png")?;

    // `to_jpg` / `to_gif` return ALREADY-ENCODED bytes (`Vec<u8>`) → write them.
    std::fs::write("qrcode.jpg", qr.to_jpg(512)?)?;
    std::fs::write("qrcode.gif", qr.to_gif(512)?)?;

    // `to_svg` returns a `String`.
    std::fs::write("qrcode.svg", qr.to_svg(512))?;

    Ok(())
}
```

</details>

<details>
<summary><b>Pick error-correction level and module shape</b></summary>

```rust
use qrc::{QRCode, EcLevel, ModuleShape};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let qr = QRCode::from_string("https://example.com".to_string())
        .with_ec_level(EcLevel::H)        // ~30% recovery — best for logos/print
        .with_shape(ModuleShape::Circle); // Square | RoundedSquare | Circle | Diamond

    qr.to_png(512).save("styled.png")?;
    Ok(())
}
```

</details>

<details>
<summary><b>Colorize a QR code</b></summary>

```rust
use qrc::QRCode;
use image::Rgba;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let qr = QRCode::from_string("https://example.com".to_string());

    // `colorize` returns an `RgbaImage` → use `.save`.
    qr.colorize(Rgba([0, 102, 204, 255])).save("blue_qrcode.png")?;
    Ok(())
}
```

</details>

<details>
<summary><b>Add a watermark</b></summary>

```rust
use qrc::QRCode;
use image::{ImageBuffer, Rgba};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut img = QRCode::from_string("https://example.com".to_string()).to_png(512);

    // A 20×20 crimson logo, alpha-blended into the corner.
    let logo = ImageBuffer::from_fn(20, 20, |_, _| Rgba([220, 20, 60, 255]));
    QRCode::add_image_watermark(&mut img, &logo);

    img.save("watermarked.png")?;
    Ok(())
}
```

</details>

<details>
<summary><b>Batch generation</b></summary>

```rust
use qrc::QRCode;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let urls = vec![
        "https://example.com/1".to_string(),
        "https://example.com/2".to_string(),
    ];

    for (i, qr) in QRCode::batch_generate_qr_codes(urls).iter().enumerate() {
        qr.to_png(256).save(format!("qr_{i}.png"))?;
    }
    Ok(())
}
```

</details>

---

## Structured Payloads

`qrc::payload` builds the exact text conventions scanners act on — so a scan
offers "Add to Contacts" or "Join Wi-Fi" instead of showing raw text. The
builders are plain strings with no extra dependencies.

```rust
use qrc::QRCode;
use qrc::payload::vcard::BusinessCard;
use qrc::payload::wifi::{WifiNetwork, WifiSecurity};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Contact card (RFC 6350 / vCard 3.0).
    let card = BusinessCard::new("Jane Doe")
        .organization("Acme, Inc.")
        .title("CEO")
        .email("jane@acme.example")
        .url("https://acme.example");
    QRCode::from_string(card.to_vcard()).to_png(512).save("contact.png")?;

    // Wi-Fi join code.
    let wifi = WifiNetwork::new("Acme Guest")
        .security(WifiSecurity::Wpa)
        .password("hunter2");
    QRCode::from_string(wifi.to_qr_string()).to_png(512).save("wifi.png")?;

    Ok(())
}
```

| Builder | Module | Emits |
| :--- | :--- | :--- |
| `BusinessCard` | `payload::vcard` | RFC 6350 vCard 3.0 |
| `WifiNetwork` | `payload::wifi` | `WIFI:` join string |
| `MeCard` | `payload::mecard` | Compact contact |
| `MerchantPayment` | `payload::emvco` | EMVCo MPM + CRC-16/CCITT |

---

## Macros

11 convenience macros for common operations:

| Macro | Description |
| :--- | :--- |
| `qr_code!(data)` | Create a new QR code |
| `qr_code_to!(data, format, width)` | Create in a specific format (png/jpg/gif) |
| `add_image_watermark!(img, watermark)` | Add a watermark to a QR image |
| `resize!(qrcode, size)` | Resize to square dimensions |
| `set_encoding_format!(qr, format)` | Set the encoding format |
| `overlay_image!(qr, image)` | Overlay a logo at the centre |
| `batch_generate_qr!(data_list)` | Generate multiple QR codes |
| `compress_data_macro!(data)` | Compress data via Zlib |
| `combine_qr_codes!(codes)` | Combine codes side-by-side |
| `create_dynamic_qr!(data)` | Create a dynamic (URL-based) QR code |
| `create_multilanguage_qr!("en" => "Hello", ...)` | Multi-language QR code |

See the [`macros` example](examples/macros.rs) for full usage.

---

## Examples

```bash
cargo run --example basic
cargo run --example vcard
```

| Example | Purpose |
| :--- | :--- |
| `basic` | Construction from bytes, strings, and vectors |
| `formats` | Export to PNG, JPG, GIF, and SVG |
| `colorize` | Custom RGBA module colours |
| `resize` | Print, web, and thumbnail sizing |
| `watermark` | Alpha-blended watermark logos |
| `overlay` | Centre-placed logo |
| `compress` | Zlib-compress data before encoding |
| `batch` | Generate many codes from a URL list |
| `combine` | Merge codes into one image |
| `encoding` | Set and validate encoding formats |
| `dynamic` | Updatable URL-based codes |
| `multilingual` | Language-aware codes from a translation map |
| `macros` | All 11 convenience macros |
| `vcard` | vCard contact card |
| `wifi` | Wi-Fi join code |
| `mecard` | Compact MeCard contact |
| `emvco` | EMVCo merchant payment |
| `business_card` | Branded vCard QR — centred logo + quiet zone, stays scannable |
| `control_image` | Export a ControlNet control image (for SD QR art) |
| `art_qr` | Offline image-blended "art QR" — no model needed |

---

## Development

```bash
cargo build                # build the library
cargo test                 # unit, integration, and doc-tests
cargo clippy --all-targets # lint with Clippy
cargo fmt --all            # format with rustfmt
cargo bench                # Criterion benchmarks
cargo xtask ci             # full local CI (fmt + clippy + test)
```

### CI

| Workflow | Trigger | Purpose |
| :--- | :--- | :--- |
| `ci.yml` | push, PR | fmt, clippy, test (3 OS), MSRV, cargo-deny, security audit |
| `document.yml` | push to `main` | Build and deploy API docs |
| `release.yml` | tag `v*` | Cross-platform binaries, crates.io publish |

See [CONTRIBUTING.md](CONTRIBUTING.md) for PR guidelines.

---

## Security

- `#![forbid(unsafe_code)]` across the entire codebase
- `#![deny(missing_docs)]` — every public item is documented
- `cargo audit` (RustSec) and `cargo deny` (licenses, advisories, bans) in CI
- SPDX license headers on all source files
- 3 runtime dependencies — minimal attack surface

---

## Documentation

Full API reference: **[docs.rs/qrc](https://docs.rs/qrc)**.

---

## Changelog

See [CHANGELOG.md](CHANGELOG.md) for release notes, including the breaking changes in 0.0.6.

---

## License

Dual-licensed under [Apache 2.0](https://www.apache.org/licenses/LICENSE-2.0) or [MIT](https://opensource.org/licenses/MIT), at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.

<p align="right"><a href="#contents">Back to Top</a></p>
