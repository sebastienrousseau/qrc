<!-- markdownlint-disable MD033 MD041 -->
<img
    src="https://kura.pro/qrc/images/logos/qrc.svg"
    alt="RustLogs (RLG) logo"
    title="RustLogs (RLG) logo"
    height="261"
    width="261"
    align="right"
/>
<!-- markdownlint-enable MD033 MD041 -->

# QRC

A Rust library for generating and manipulating QR code images in various formats 🦀

[![Made With Love][made-with-rust]][05]
[![Crates.io][crates-badge]][07]
[![Lib.rs][libs-badge]][06]
[![Docs.rs][docs-badge]][9]
[![License][license-badge]][02]

![divider][divider]

## Welcome to QRC

![QRC Banner][banner]

<!-- markdownlint-disable MD033 -->
<center>

**[Website][00]
• [Documentation][9]
• [Report Bug][03]
• [Request Feature][03]
• [Contributing Guidelines][04]**

</center>

<!-- markdownlint-enable MD033 -->

## Overview

The QR Code Library (QRC) is a comprehensive Rust library designed to create and
manipulate QR codes. It offers a wide range of functionalities including
generating QR codes in various formats, customizing color schemes, adding image
watermarks, and more.

## Features

- Generate QR code images in multiple formats like PNG, JPG, GIF, and SVG,
- Customize color schemes,
- Add image watermarks,
- Easy integration with Rust projects.

### Usage Examples

Here are some basic examples of how to use QRC in your Rust projects:

```rust
// Create a new QRCode using the QRCode::from_string() function and convert it to a PNG representation
let qrcode = QRCode::from_string(URL.to_string());

// Convert the QRCode into a PNG representation
let png = qrcode.to_png(512);

// Convert the PNG representation of the QRCode into a vector of bytes
let png_data = png.into_raw();
let png_image = ImageBuffer::<Rgba<u8>, Vec<u8>>::from_raw(21, 21, png_data).unwrap();

// Print the PNG representation of the QRCode
println!("fn to_png(): {:?}", png_image.save("qrcode.png"));
match png_image.save("qrcode.png") {
    // Print the path to the PNG representation of the QRCode that was saved to a file called "qrcode.png"
    Ok(_) => println!("png file created: qrcode.png"),
    // Print the path to the PNG representation of the QRCode that was saved to a file called "qrcode.png"
    Err(e) => println!("png file created: qrcode.png: {e}"),
}
```

## Macros in QRC

The QR Code Library (QRC) offers a set of macros for easy manipulation and
generation of QR codes in Rust. Below is the documentation for each macro
available in the library.

### 1. `add_image_watermark`

- **Description**: This macro creates a new instance of the QRCode struct with a watermark image added to an existing image.
- **Usage**:

```rust
add_image_watermark!(img, watermark);
```

- `img`: An expression representing the main image.
- `watermark`: An expression representing the watermark image.

### 2. `qr_code`

- **Description**: Generates a new QR code instance with the provided data.
- **Usage**:

```rust
qr_code!(data);
```

- `data`: An expression representing the data to encode in the QR code.

### 3. `qr_code_to`

- **Description**: Creates a QR code in a specified format (PNG, JPG, or GIF) and size.
- **Usage**:

```rust
qr_code_to!(data, format, width);
```

- `data`: The data to encode.
- `format`: The desired output format ("png", "jpg", or "gif").
- `width`: The width of the QR code.

### 4. `resize`

- **Description**: Sets the size of the QR code.
- **Usage**:

```rust
resize!(qrcode, size);
```

- `qrcode`: An instance of `QRCode`.
- `size`: The desired size for the QR code.

### 5. `set_encoding_format`

- **Description**: Sets the encoding format for the data in a QR code.
- **Usage**:

```rust
set_encoding_format!(qr_code, format);
```

- `qr_code`: An instance of `QRCode`.
- `format`: The encoding format for the QR code data.

### 6. `overlay_image`

- **Description**: Overlays an image at the center of the QR code.
- **Usage**:

```rust
overlay_image!(qr_code, image_path);
```

- `qr_code`: QRCode instance to which the image will be overlaid.
- `image_path`: Path to the image file to overlay.

### 7. `batch_generate_qr`

- **Description**: Generates multiple QR codes in one operation.
- **Usage**:

```rust
batch_generate_qr!(data_list);
```

- `data_list`: A vector of data strings for QR code generation.

### 8. `compress_data_macro`

- **Description**: Compresses data before encoding it into a QR code.
- **Usage**:

```rust
compress_data_macro!(data);
```

- `data`: The data to be compressed and encoded.

### 9. `combine_qr_codes`

- **Description**: Combines multiple QR codes into a single QR code.
- **Usage**:

```rust
combine_qr_codes!(codes);
```

- `codes`: An array of QRCode instances to combine.

### 10. `create_dynamic_qr`

- **Description**: Generates a dynamic QR code that can be updated after creation.
- **Usage**:

```rust
create_dynamic_qr!(initial_data);
```

- `initial_data`: The initial data for the QR code.

### 11. `create_multilanguage_qr`

- **Description**: Generates QR codes with multi-language support.
- **Usage**:

```rust
create_multilanguage_qr! {
    "en" => "Hello",
    "es" => "Hola",
    "fr" => "Bonjour",
};
```

- Language-data pairs for the QR code.

## Installation

It takes just a few minutes to get up and running with `qrc`.

### Requirements

`qrc` requires Rust **1.71.1** or later.

## Documentation

You can find detailed documentation for QRC on [lib.rs][06], [crates.io][07] and
[docs.rs][08].

## Usage 📖

To use `qrc` in your project, add the following to your
`Cargo.toml` file:

```toml
[dependencies]
qrc = "0.0.4"
```

Add the following to your `main.rs` file:

```rust
extern crate qrc;
use qrc::*;
```

then you can use the functions in your application code.

### Examples

`QRC` comes with a set of examples that you can use to get started. The
examples are located in the `examples` directory of the project. To run
the examples, clone the repository and run the following command in your
terminal from the project root directory.

```shell
cargo run --example example
```

## Semantic Versioning Policy

For transparency into our release cycle and in striving to maintain
backward compatibility, `QRC` follows [semantic versioning][09].

## License

The project is licensed under the terms of both the MIT license and the
Apache License (Version 2.0).

- [Apache License, Version 2.0][01]
- [MIT license][02]

## Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms
or conditions.

![divider][divider]

## Acknowledgements

A big thank you to all the awesome contributors of [QRC][05] for their help and
support. A special thank you goes to the
[Rust Reddit](https://www.reddit.com/r/rust/) community for providing a lot of
useful suggestions on how to improve this project.

[00]: https://qrclib.one
[01]: http://www.apache.org/licenses/LICENSE-2.0
[02]: http://opensource.org/licenses/MIT
[03]: https://github.com/sebastienrousseau/qrc/issues
[04]: https://raw.githubusercontent.com/sebastienrousseau/qrc/main/CONTRIBUTING.md
[05]: https://github.com/sebastienrousseau/qrc/graphs/contributors
[06]: https://lib.rs/crates/qrc
[07]: https://crates.io/crates/qrc
[08]: https://docs.rs/qrc
[09]: http://semver.org/

[banner]: https://kura.pro/qrc/images/titles/title-qrc.svg "QRC Banner"
[crates-badge]: https://img.shields.io/crates/v/qrc.svg?style=for-the-badge "QRC on Crates.io"
[divider]: https://kura.pro/common/images/elements/divider.svg "divider"
[docs-badge]: https://img.shields.io/docsrs/qrc.svg?style=for-the-badge "QRC on Docs.rs"
[libs-badge]: https://img.shields.io/badge/lib.rs-v0.0.2-orange.svg?style=for-the-badge "QRC on Lib.rs"
[license-badge]: https://img.shields.io/crates/l/qrc.svg?style=for-the-badge "QRC License"
[made-with-rust]: https://img.shields.io/badge/rust-f04041?style=for-the-badge&labelColor=c0282d&logo=rust "Made With Rust"
