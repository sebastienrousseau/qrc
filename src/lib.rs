// Copyright © 2022-2026 QRC Contributors. All rights reserved.
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT
//!
//! # A Rust library for generating and manipulating QR code images in various formats
//!
//! [![Rust](https://kura.pro/qrc/images/banners/banner-qrc.webp)](https://qrclib.one)
//!
//! <center>
//!
//! [![Rust](https://img.shields.io/badge/rust-f04041?style=for-the-badge&labelColor=c0282d&logo=rust)](https://www.rust-lang.org)
//! [![Crates.io](https://img.shields.io/crates/v/qrc.svg?style=for-the-badge&color=success&labelColor=27A006)](https://crates.io/crates/qrc/)
//! [![Docs.rs](https://img.shields.io/badge/docs.rs-v0.0.6-success.svg?style=for-the-badge&color=8A48FF&labelColor=6F36E4)](https://docs.rs/qrc)
//! [![Lib.rs](https://img.shields.io/badge/lib.rs-v0.0.6-success.svg?style=for-the-badge&color=8A48FF&labelColor=6F36E4)](https://lib.rs/crates/qrc)
//! [![GitHub](https://img.shields.io/badge/github-555555?style=for-the-badge&labelColor=000000&logo=github)](https://github.com/sebastienrousseau/qrc)
//! [![License](https://img.shields.io/crates/l/qrc.svg?style=for-the-badge&color=007EC6&labelColor=03589B)](http://opensource.org/licenses/MIT)
//!
//! </center>
//!
//! ## Overview
//!
//! The QR Code Library (QRC) is a versatile tool for generating and
//! manipulating QR code images in various formats.
//!
//! With this library, you can easily convert your data into a QR code,
//! whether it be in the form of a string or a vector of bytes.
//!
//! Choose from popular image formats like PNG, JPG, GIF and SVG, and
//! even customize the size and color of your QR code.
//!
//! ## Features
//!
//! `QRC` features a `QRCode` struct that can be constructed with a
//! `Vec<u8>` of data or a `String` of data that will be converted to
//! a `Vec<u8>`.
//!
//! The QR code can be generated using the `to_qrcode` method, and
//! specific image formats can be generated using the `to_png`,
//! `to_jpg`, and `to_gif` methods.
//!
//! Each of these methods takes a `width` parameter and returns an
//! `ImageBuffer` containing the QR code image.
//!
//! The library uses the `qrcode` and `image` crates to generate the QR
//! code images.
//!
//! As of the current version, the library supports the following
//! features with the following status:
//!
//! | Feature | Description |
//! | ------- | ----------- |
//! | Library license | Apache-2.0 OR MIT |
//! | Library version | 0.0.6 |
//! | Mode Numeric | not specified |
//! | Mode Alphanumeric | not specified |
//! | Mode Byte | not specified |
//! | Mode Kanji | not specified |
//! | Mode ECI | not specified |
//! | Mode FNC1 | not specified |
//! | Mode Structured Append | not specified |
//! | Mode Hanzi | not specified |
//! | Mixing modes | not specified |
//! | QR Codes version 1 - 40 | not specified |
//! | Micro QR Codes version M1 - M4 | not specified |
//! | Find maximal error correction level | not specified |
//! | Optimize QR Codes | not specified |
//! | PNG output | supported |
//! | JPG output | supported |
//! | GIF output | supported |
//! | SVG output | supported |
//! | EPS output | not specified |
//! | PDF output | not specified |
//! | BMP output | not specified |
//! | TIFF output | not specified |
//! | WebP output | not specified |
//! | Black and white QR Codes | Yes |
//! | Colorized QR code | Yes |
//! | Animated QR Codes (GIF, APNG, WebP) | not specified |
//! | Changing size of modules (scaling factor) | not specified |
//! | Command line script | not specified |
//! | QR code resizing | supported |
//! | QR code watermarking | supported |
//! | QR code with logo | supported |
//!
//! ## Usage
//!
//! - [`serde`][]: Enable serialization/deserialization via serde
//!
//! [`serde`]: https://github.com/serde-rs/serde
//!
#![deny(dead_code)]
#![deny(missing_debug_implementations)]
#![deny(missing_docs)]
#![forbid(unsafe_code)]
#![warn(unreachable_pub)]
#![doc(
    html_favicon_url = "https://kura.pro/qrc/favicon.ico",
    html_logo_url = "https://kura.pro/qrc/images/logos/qrc.svg",
    html_root_url = "https://docs.rs/qrc"
)]
#![crate_name = "qrc"]

use image::{ImageBuffer, Rgba, RgbaImage};
use miniz_oxide::deflate::compress_to_vec_zlib;
use qrcode::{render::svg, Color, QrCode};
use std::collections::HashMap;

/// The `macros` module contains functions for generating macros.
pub mod macros;

/// Represents a QR code containing data.
///
/// This struct can be used to generate QR code images in various formats.
/// It supports encoding data as a QR code and rendering it in formats like PNG, JPG, and SVG.
///
/// # Examples
///
/// ```
/// use qrc::QRCode;
///
/// // Create a new QR code with text data
/// let qr = QRCode::new("Hello, world!".as_bytes().to_vec());
/// ```
#[non_exhaustive]
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct QRCode {
    /// The `data` field holds the data to be encoded in the QR code.
    pub data: Vec<u8>,
    /// The `encoding_format` field holds the encoding format of the QR code.
    encoding_format: String,
}

impl QRCode {
    /// Creates a new `QRCode` instance with the given data.
    ///
    /// # Examples
    ///
    /// ```
    /// use qrc::QRCode;
    ///
    /// let qr = QRCode::new("Hello, world!".as_bytes().to_vec());
    /// ```
    #[must_use]
    pub fn new(data: Vec<u8>) -> Self {
        QRCode {
            data,
            encoding_format: "utf-8".to_string(),
        }
    }

    /// Creates a new `QRCode` instance by converting the given string data
    /// into a vector of bytes.
    #[must_use]
    pub fn from_string(data: String) -> Self {
        QRCode {
            data: data.into_bytes(),
            encoding_format: "utf-8".to_string(),
        }
    }

    /// Creates a new `QRCode` instance from a vector of bytes.
    #[must_use]
    pub fn from_bytes(data: Vec<u8>) -> Self {
        QRCode {
            data,
            encoding_format: "utf-8".to_string(),
        }
    }

    /// Converts the `QRCode` data to a `QrCode` structure.
    ///
    /// # Panics
    ///
    /// Panics if the data cannot be encoded as a valid QR code.
    #[must_use]
    pub fn to_qrcode(&self) -> QrCode {
        QrCode::new(&self.data).unwrap()
    }

    /// Renders the QR code into an RGBA image buffer at the given width.
    ///
    /// This is the shared implementation used by `to_png`, `to_jpg`, and `to_gif`.
    fn render_image(&self, width: u32) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
        let qrcode = self.to_qrcode();
        let height = width;
        let qr_width = qrcode.width() as f64;
        let mut img = ImageBuffer::new(width, height);
        for (x, y, pixel) in img.enumerate_pixels_mut() {
            let x_index = (f64::from(x) / f64::from(width) * qr_width) as usize;
            let y_index = (f64::from(y) / f64::from(height) * qr_width) as usize;
            *pixel = match qrcode[(x_index, y_index)] {
                Color::Dark => Rgba([0, 0, 0, 0]),
                Color::Light => Rgba([255, 255, 255, 255]),
            };
        }
        img
    }

    /// Converts the `QRCode` to a PNG image.
    ///
    /// # Examples
    ///
    /// ```
    /// use qrc::QRCode;
    ///
    /// let qr = QRCode::from_string("Hello, world!".to_string());
    /// let png_image = qr.to_png(256);
    /// ```
    #[must_use]
    pub fn to_png(&self, width: u32) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
        self.render_image(width)
    }

    /// Converts the `QRCode` to a JPG image.
    #[must_use]
    pub fn to_jpg(&self, width: u32) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
        self.render_image(width)
    }

    /// Converts the `QRCode` to a GIF image.
    #[must_use]
    pub fn to_gif(&self, width: u32) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
        self.render_image(width)
    }

    /// Converts the `QRCode` to an SVG image.
    ///
    /// # Parameters
    ///
    /// * `width`: The width of the desired image in pixels.
    ///
    /// # Returns
    ///
    /// A `String` representing the QR code in SVG format.
    #[must_use]
    pub fn to_svg(&self, width: u32) -> String {
        let qrcode = self.to_qrcode();
        qrcode
            .render::<svg::Color>()
            .min_dimensions(width, width)
            .dark_color(svg::Color("#000000"))
            .light_color(svg::Color("#FFFFFF"))
            .build()
    }

    /// Colorizes the QR code with the specified color.
    ///
    /// # Parameters
    ///
    /// * `color`: The `Rgba<u8>` color value to use for the QR code.
    ///
    /// # Returns
    ///
    /// A colorized `RgbaImage` of the QR code.
    #[must_use]
    pub fn colorize(&self, color: Rgba<u8>) -> RgbaImage {
        let qrcode = self.to_qrcode();
        let qr_dim = qrcode.width() as u32;
        let mut img: RgbaImage = ImageBuffer::new(qr_dim, qr_dim);
        for (x, y, pixel) in img.enumerate_pixels_mut() {
            *pixel = if qrcode[(x as usize, y as usize)] == Color::Dark {
                color
            } else {
                Rgba([255, 255, 255, 255])
            };
        }
        img
    }

    /// Resizes the QR code image to the specified width and height.
    ///
    /// # Parameters
    ///
    /// * `width`: The width of the image in pixels.
    /// * `height`: The height of the image in pixels.
    ///
    /// # Returns
    ///
    /// A resized `RgbaImage` of the QR code.
    #[must_use]
    pub fn resize(&self, width: u32, height: u32) -> RgbaImage {
        let qrcode = self.to_qrcode();
        let qr_width = qrcode.width() as f64;
        let mut img: RgbaImage = ImageBuffer::new(width, height);
        for y in 0..height {
            for x in 0..width {
                let x_index = (f64::from(x) / f64::from(width) * qr_width) as usize;
                let y_index = (f64::from(y) / f64::from(height) * qr_width) as usize;
                let c = match qrcode[(x_index, y_index)] {
                    Color::Dark => Rgba([0, 0, 0, 0]),
                    Color::Light => Rgba([255, 255, 255, 255]),
                };
                img.put_pixel(x, y, c);
            }
        }
        img
    }

    /// Adds a watermark image to the QR code.
    ///
    /// The watermark is placed in the bottom-right corner with alpha blending.
    ///
    /// # Parameters
    ///
    /// * `img`: A mutable reference to the `RgbaImage` of the QR code.
    /// * `watermark`: A reference to the watermark `RgbaImage`.
    pub fn add_image_watermark(img: &mut RgbaImage, watermark: &RgbaImage) {
        let (width, height) = img.dimensions();
        let (watermark_width, watermark_height) = watermark.dimensions();

        let x_offset = width - watermark_width;
        let y_offset = height - watermark_height;

        for (dx, dy, watermark_pixel) in watermark.enumerate_pixels() {
            let px = x_offset + dx;
            let py = y_offset + dy;
            let qr_pixel = img.get_pixel(px, py);

            let alpha = f32::from(watermark_pixel[3]) / 255.0;
            let new_r = (1.0 - alpha) * f32::from(qr_pixel[0]) + alpha * f32::from(watermark_pixel[0]);
            let new_g = (1.0 - alpha) * f32::from(qr_pixel[1]) + alpha * f32::from(watermark_pixel[1]);
            let new_b = (1.0 - alpha) * f32::from(qr_pixel[2]) + alpha * f32::from(watermark_pixel[2]);
            let new_a = f32::from(qr_pixel[3]) + alpha * (255.0 - f32::from(qr_pixel[3]));

            img.put_pixel(px, py, Rgba([new_r as u8, new_g as u8, new_b as u8, new_a as u8]));
        }
    }

    /// Creates a multilingual QR code based on a map of language codes to data strings.
    ///
    /// # Parameters
    ///
    /// * `data_map`: A `HashMap` mapping language codes (`String`) to data (`String`).
    ///
    /// # Returns
    ///
    /// A `QRCode` instance representing a multilingual QR code.
    #[must_use]
    pub fn create_multilanguage(data_map: HashMap<String, String>) -> Self {
        let user_language = "en";
        let selected_data = data_map
            .get(user_language)
            .map_or("", String::as_str);
        QRCode::from_string(selected_data.to_string())
    }

    /// Generates a dynamic QR code that can be updated after creation.
    ///
    /// # Parameters
    ///
    /// * `initial_data`: A string slice representing the initial data for the QR code.
    ///
    /// # Returns
    ///
    /// A `QRCode` instance representing a dynamic QR code.
    #[must_use]
    pub fn create_dynamic(initial_data: &str) -> Self {
        let dynamic_data_format = "url";

        let dynamic_url = match dynamic_data_format {
            "url" => {
                format!(
                    "https://your-api-endpoint.com/update?qrcode={initial_data}"
                )
            }
            _ => return QRCode::from_string(initial_data.to_string()),
        };

        QRCode::from_string(dynamic_url)
    }

    /// Combines multiple QR codes into a single larger QR code.
    ///
    /// # Parameters
    ///
    /// * `codes`: A vector of `QRCode` instances to combine.
    ///
    /// # Returns
    ///
    /// A `Result` which is either a combined `QRCode` instance or an error string.
    ///
    /// # Errors
    ///
    /// Returns an error if `codes` is empty.
    pub fn combine_qr_codes(codes: Vec<QRCode>) -> Result<Self, &'static str> {
        if codes.is_empty() {
            return Err("No QR codes to combine");
        }

        let total_width: u32 = codes
            .iter()
            .map(|code| code.to_qrcode().width() as u32)
            .sum();

        let mut combined_image: RgbaImage = ImageBuffer::new(total_width, total_width);

        let mut x_offset: u32 = 0;

        for code in &codes {
            let qrcode = code.to_qrcode();
            let width = qrcode.width() as u32;

            for x in 0..width {
                for y in 0..width {
                    let pixel = qrcode[(x as usize, y as usize)];
                    let combined_x = x + x_offset;

                    match pixel {
                        Color::Dark => {
                            combined_image.put_pixel(combined_x, y, Rgba([0, 0, 0, 0]));
                        }
                        Color::Light => {
                            combined_image.put_pixel(
                                combined_x,
                                y,
                                Rgba([255, 255, 255, 255]),
                            );
                        }
                    }
                }
            }

            x_offset += width;
        }

        let mut combined_qrcode = QRCode::from_bytes(Vec::new());
        combined_qrcode.data = combined_image.into_raw();

        Ok(combined_qrcode)
    }

    /// Compresses the provided data string using Zlib compression.
    ///
    /// # Parameters
    ///
    /// * `data`: A string slice representing the data to compress.
    ///
    /// # Returns
    ///
    /// A `Vec<u8>` containing the Zlib-compressed data.
    #[must_use]
    pub fn compress_data(data: &str) -> Vec<u8> {
        compress_to_vec_zlib(data.as_bytes(), 6)
    }

    /// Generates a batch of QR codes from a vector of data strings.
    ///
    /// # Parameters
    ///
    /// * `data`: A vector of strings, each representing data for a separate QR code.
    ///
    /// # Returns
    ///
    /// A vector of `QRCode` instances.
    #[must_use]
    pub fn batch_generate_qr_codes(data: Vec<String>) -> Vec<QRCode> {
        data.into_iter().map(QRCode::from_string).collect()
    }

    /// Overlays an image on top of the QR code.
    ///
    /// # Parameters
    ///
    /// * `overlay`: A reference to the `RgbaImage` to overlay on the QR code.
    ///
    /// # Returns
    ///
    /// A combined `RgbaImage` with the overlay applied.
    #[must_use]
    pub fn overlay_image(&self, overlay: &RgbaImage) -> RgbaImage {
        let qrcode = self.to_qrcode();
        let qr_dim = qrcode.width() as u32;
        let mut combined_image: RgbaImage = ImageBuffer::new(qr_dim, qr_dim);

        for x in 0..qrcode.width() {
            for y in 0..qrcode.width() {
                let pixel = qrcode[(x, y)];
                let cx = x as u32;
                let cy = y as u32;

                match pixel {
                    Color::Dark => {
                        combined_image.put_pixel(cx, cy, Rgba([0, 0, 0, 0]));
                    }
                    Color::Light => {
                        combined_image.put_pixel(cx, cy, Rgba([255, 255, 255, 255]));
                    }
                }
            }
        }

        for (x, y, pixel) in overlay.enumerate_pixels() {
            combined_image.put_pixel(x, y, *pixel);
        }

        combined_image
    }

    /// Sets the encoding format of the QR code.
    ///
    /// # Parameters
    ///
    /// * `format`: A string slice representing the encoding format.
    ///
    /// # Returns
    ///
    /// A `Result` which is either a new `QRCode` instance with updated encoding or an error string.
    ///
    /// # Errors
    ///
    /// Returns an error if the encoding format is not `"utf-8"`.
    pub fn set_encoding_format(&self, format: &str) -> Result<Self, &'static str> {
        if format != "utf-8" {
            return Err("Unsupported encoding format");
        }

        Ok(Self {
            data: self.data.clone(),
            encoding_format: format.to_string(),
        })
    }

    /// Retrieves the encoding format of the QR code.
    ///
    /// # Returns
    ///
    /// A string slice representing the encoding format.
    #[must_use]
    pub fn get_encoding_format(&self) -> &str {
        &self.encoding_format
    }
}
