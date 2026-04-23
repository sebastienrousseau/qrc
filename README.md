<!-- SPDX-License-Identifier: Apache-2.0 OR MIT -->

<p align="center">
  <img src="https://cloudcdn.pro/qrc/v1/logos/qrc.svg" alt="QRC logo" width="128" />
</p>

<h1 align="center">QR Code Library (QRC)</h1>

<p align="center">
  <strong>Generate and manipulate QR code images in PNG, JPG, GIF, and SVG -- built in Rust.</strong>
</p>

<p align="center">
  <a href="https://github.com/sebastienrousseau/qrc/actions"><img src="https://img.shields.io/github/actions/workflow/status/sebastienrousseau/qrc/ci.yml?style=for-the-badge&logo=github" alt="Build" /></a>
  <a href="https://crates.io/crates/qrc"><img src="https://img.shields.io/crates/v/qrc.svg?style=for-the-badge&color=fc8d62&logo=rust" alt="Crates.io" /></a>
  <a href="https://docs.rs/qrc"><img src="https://img.shields.io/badge/docs.rs-qrc-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" alt="Docs.rs" /></a>
  <a href="https://lib.rs/crates/qrc"><img src="https://img.shields.io/badge/lib.rs-v0.0.6-orange.svg?style=for-the-badge" alt="lib.rs" /></a>
  <a href="https://opensource.org/licenses/MIT"><img src="https://img.shields.io/crates/l/qrc.svg?style=for-the-badge" alt="License" /></a>
</p>

---

## Contents

- [Install](#install) -- add to Cargo.toml and start generating
- [Quick Start](#quick-start) -- create a QR code in 4 lines
- [Overview](#overview) -- what QRC does
- [Features](#features) -- v0.0.6 capability matrix
- [Library Usage](#library-usage) -- formats, colours, watermarks, macros
- [Macros](#macros) -- 11 convenience macros
- [Examples](#examples) -- 13 focused examples
- [Development](#development) -- build, test, lint
- [Security](#security) -- safety guarantees
- [License](#license)

---

## Install

```toml
[dependencies]
qrc = "0.0.6"
```

### Build from source

```bash
git clone https://github.com/sebastienrousseau/qrc.git
cd qrc
cargo build --release
```

Requires **Rust 1.75.0+**. Tested on Linux, macOS, and Windows.

---

## Quick Start

```rust
use qrc::QRCode;

// 1 -- Create a QR code from any string
let qr = QRCode::from_string("https://example.com".to_string());

// 2 -- Export as PNG (512x512)
let png = qr.to_png(512);
png.save("qrcode.png").unwrap();

// 3 -- Export as SVG (infinite scaling)
let svg = qr.to_svg(512);
std::fs::write("qrcode.svg", &svg).unwrap();

// 4 -- Customise with colour
let coloured = qr.colorize(image::Rgba([0, 102, 204, 255]));
```

---

## Overview

QRC generates QR code images from strings, byte vectors, or raw data. It renders to four image formats, supports colour customisation, watermarking, logo overlays, batch generation, and data compression -- all with zero unsafe code.

- **4 output formats** -- PNG, JPG, GIF, SVG
- **Colour customisation** -- any RGBA colour for dark modules
- **Image watermarks** -- alpha-blended logos in the corner
- **Logo overlays** -- centre-placed images on the QR code
- **Batch generation** -- generate hundreds of QR codes in one call
- **Multi-language** -- language-aware QR codes from a translation map
- **Data compression** -- Zlib-compress data before encoding
- **Dynamic QR codes** -- URL-based codes that can be updated after creation
- **Zero unsafe code** -- `#![forbid(unsafe_code)]` across the entire codebase

---

## Features

| | |
| :--- | :--- |
| **Formats** | PNG, JPG, GIF (raster via `image` crate), SVG (vector via `qrcode` crate) |
| **Colours** | Custom RGBA colour for dark modules, white background |
| **Watermarks** | Alpha-blended watermark placement in bottom-right corner |
| **Overlays** | Centre-placed logo overlay on any QR code |
| **Resizing** | Arbitrary width/height scaling with pixel-level control |
| **Compression** | Zlib compression via `miniz_oxide` (level 6) |
| **Batch** | Generate a `Vec<QRCode>` from a `Vec<String>` in one call |
| **Combine** | Merge multiple QR codes side-by-side into a single image |
| **Dynamic** | URL-based QR codes for post-creation updates |
| **Multi-language** | HashMap-driven language selection with BCP 47 codes |
| **Encoding** | UTF-8 encoding format with validation |
| **Macros** | 11 convenience macros for every operation |
| **Safety** | `#![forbid(unsafe_code)]`, `#![deny(missing_docs)]` |
| **MSRV** | Rust 1.75.0 |
| **Dependencies** | 3 runtime (`image`, `qrcode`, `miniz_oxide`) |

### Metrics

| Metric | Value |
| :--- | :--- |
| **Test suite** | 16 unit tests + 14 doc-tests |
| **Examples** | 13 focused examples |
| **Benchmarks** | 7 Criterion benchmarks |
| **Dependencies** | 3 runtime, 1 dev |

---

## Library Usage

<details>
<summary><b>Generate all four formats</b></summary>

```rust
use qrc::QRCode;
use image::DynamicImage;

let qr = QRCode::from_string("https://docs.rs/qrc".to_string());

// PNG -- lossless, web-ready
let png = qr.to_png(512);
png.save("qrcode.png").unwrap();

// JPG -- convert RGBA to RGB for JPEG compatibility
let jpg = qr.to_jpg(512);
DynamicImage::ImageRgba8(jpg).to_rgb8().save("qrcode.jpg").unwrap();

// GIF -- small palette, universal
let gif = qr.to_gif(512);
gif.save("qrcode.gif").unwrap();

// SVG -- vector, infinite scaling
let svg = qr.to_svg(512);
std::fs::write("qrcode.svg", &svg).unwrap();
```

</details>

<details>
<summary><b>Colorize a QR code</b></summary>

```rust
use qrc::QRCode;
use image::Rgba;

let qr = QRCode::from_string("https://example.com".to_string());
let blue_qr = qr.colorize(Rgba([0, 102, 204, 255]));
blue_qr.save("blue_qrcode.png").unwrap();
```

</details>

<details>
<summary><b>Add a watermark</b></summary>

```rust
use qrc::QRCode;
use image::{ImageBuffer, Rgba};

let qr = QRCode::from_string("https://example.com".to_string());
let mut img = qr.to_png(512);

// Create a 20x20 red logo
let logo = ImageBuffer::from_fn(20, 20, |_, _| Rgba([220, 20, 60, 255]));
QRCode::add_image_watermark(&mut img, &logo);
img.save("watermarked.png").unwrap();
```

</details>

<details>
<summary><b>Batch generation</b></summary>

```rust
use qrc::QRCode;

let urls = vec![
    "https://example.com/1".to_string(),
    "https://example.com/2".to_string(),
    "https://example.com/3".to_string(),
];
let codes = QRCode::batch_generate_qr_codes(urls);

for (i, qr) in codes.iter().enumerate() {
    qr.to_png(256).save(format!("qr_{i}.png")).unwrap();
}
```

</details>

<details>
<summary><b>Compress data before encoding</b></summary>

```rust
use qrc::QRCode;

let data = "A very long string that benefits from compression...";
let compressed = QRCode::compress_data(data);
let qr = QRCode::from_bytes(compressed);
```

</details>

---

## Macros

11 convenience macros for every QRC operation:

| Macro | Description |
| :--- | :--- |
| `qr_code!(data)` | Create a new QR code |
| `qr_code_to!(data, format, width)` | Create in a specific format (png/jpg/gif) |
| `add_image_watermark!(img, watermark)` | Add a watermark to a QR code image |
| `resize!(qrcode, size)` | Resize a QR code to square dimensions |
| `set_encoding_format!(qr, format)` | Set the encoding format |
| `overlay_image!(qr, image)` | Overlay a logo at the centre |
| `batch_generate_qr!(data_list)` | Generate multiple QR codes |
| `compress_data_macro!(data)` | Compress data via Zlib |
| `combine_qr_codes!(codes)` | Combine multiple QR codes side-by-side |
| `create_dynamic_qr!(data)` | Create a dynamic (URL-based) QR code |
| `create_multilanguage_qr!("en" => "Hello", ...)` | Multi-language QR code |

See the [macros example](examples/macros.rs) for detailed usage.

---

## Examples

Run any example:

```bash
cargo run --example basic
cargo run --example formats
```

| Example | Purpose |
| :--- | :--- |
| `basic` | QR code construction from bytes, strings, and vectors |
| `formats` | Export to PNG, JPG, GIF, and SVG with file size comparison |
| `colorize` | Apply custom RGBA colours to QR code modules |
| `resize` | Resize QR codes for print, web, and thumbnail use cases |
| `watermark` | Add watermark logos with alpha blending |
| `overlay` | Centre-place a logo on a QR code |
| `compress` | Zlib-compress data before QR encoding |
| `batch` | Generate multiple QR codes from a URL list |
| `combine` | Merge multiple QR codes into a single image |
| `encoding` | Set and validate encoding formats |
| `dynamic` | Create dynamic QR codes with updatable URLs |
| `multilingual` | Language-aware QR codes from a translation map |
| `macros` | Demonstrate all 11 convenience macros |

---

## Development

```bash
cargo build               # build the library
cargo test                 # run all tests (16 unit + 14 doc-tests)
cargo clippy --all-targets # lint with Clippy
cargo fmt --all            # format with rustfmt
cargo bench                # run Criterion benchmarks
cargo xtask ci             # full CI pipeline (fmt + clippy + test)
```

### CI

| Workflow | Trigger | Purpose |
| :--- | :--- | :--- |
| `ci.yml` | push, PR | fmt, clippy, test (3 OS), MSRV, cargo-deny, security audit |
| `document.yml` | push to main | Build and deploy API docs to GitHub Pages |
| `release.yml` | tag `v*` | Cross-platform binaries (21 targets), crates.io publish |

See [CONTRIBUTING.md](CONTRIBUTING.md) for PR guidelines.

---

## Security

<details>
<summary><b>Safety guarantees</b></summary>

- `#![forbid(unsafe_code)]` across the entire codebase
- `#![deny(missing_docs)]` -- every public item is documented
- `cargo audit` with zero advisories
- `cargo deny` -- license, advisory, and ban checks in CI
- SPDX license headers on all source files
- 3 runtime dependencies -- minimal attack surface

</details>

---

## License

Dual-licensed under [Apache 2.0](https://www.apache.org/licenses/LICENSE-2.0) or [MIT](https://opensource.org/licenses/MIT), at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.

<p align="right"><a href="#contents">Back to Top</a></p>
