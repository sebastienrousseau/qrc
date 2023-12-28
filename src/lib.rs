// Copyright © 2022-2023 Mini Functions. All rights reserved.
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT
//!
//! # A Rust library for generating and manipulating QR code images in various formats
//!
//! [![Rust](https://kura.pro/qrc/images/banners/banner-qrc.webp)](https://minifunctions.com)
//!
//! <center>
//!
//! [![Rust](https://img.shields.io/badge/rust-f04041?style=for-the-badge&labelColor=c0282d&logo=rust)](https://www.rust-lang.org)
//! [![Crates.io](https://img.shields.io/crates/v/qrc.svg?style=for-the-badge&color=success&labelColor=27A006)](https://crates.io/crates/qrc/)
//! [![Docs.rs](https://img.shields.io/badge/docs.rs-v0.0.1-success.svg?style=for-the-badge&color=8A48FF&labelColor=6F36E4)](https://docs.rs/qrc)
//! [![Lib.rs](https://img.shields.io/badge/lib.rs-v0.0.1-success.svg?style=for-the-badge&color=8A48FF&labelColor=6F36E4)](https://lib.rs/crates/qrc)
//! [![GitHub](https://img.shields.io/badge/github-555555?style=for-the-badge&labelColor=000000&logo=github)](https://github.com/sebastienrousseau/mini-functions/tree/main/qrc)
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
//! The QR code can be generated using the zto_qrcode` method, and
//! specific image formats can be generated using the `to_png`,
//! `to_jpg`, and `to_gif` methods.
//!
//! Each of these methods takes a `width` parameter and returns an
//! `ImageBuffer` containing the QR code image.
//!
//! The library uses the qrcode and image crates to generate the QR
//! code images.
//!
//! As of the current version, the library supports the following
//! features with the following status:
//!
//! | Feature | Description |
//! | ------- | ----------- |
//! | Library license | Apache-2.0 OR MIT |
//! | Library version | 0.0.1 |
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
#![cfg_attr(feature = "bench", feature(test))]
#![deny(dead_code)]
#![deny(missing_debug_implementations)]
#![deny(missing_docs)]
#![forbid(unsafe_code)]
#![warn(unreachable_pub)]
#![doc(
    html_favicon_url = "https://kura.pro/qrc/favicon.ico",
    html_logo_url = "https://kura.pro/qrc/images/logos/qrc.svg",
    html_root_url = "https://docs.rs/mini-functions"
)]
#![crate_name = "qrc"]
#![crate_type = "lib"]

extern crate image;
extern crate qrcode;

use flate2::{write::ZlibEncoder, Compression};
use image::{ImageBuffer, Rgba, RgbaImage};
use qrcode::{render::svg, Color, QrCode};
use std::{collections::HashMap, io::Write};

/// The `macros` module contains functions for generating macros.
pub mod macros;

#[non_exhaustive]
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
/// QRCode is a structure that contains data in the form of a vector of
/// bytes.
pub struct QRCode {
    /// The `data` field holds the data to be encoded in the QR code.
    pub data: Vec<u8>,
    /// The `encoding_format` field holds the encoding format of the QR code.
    encoding_format: String,
}
/// Implementation of QRCode structure.
impl QRCode {
    /// Creates a new QRCode structure with the given data.
    pub fn new(data: Vec<u8>) -> Self {
        QRCode {
            data,
            encoding_format: "utf-8".to_string(),
        }
    }

    /// The `from_string` method creates a new instance of the QRCode
    /// struct by converting the given string data into a vector of
    /// bytes
    pub fn from_string(data: String) -> Self {
        QRCode {
            data: data.into_bytes(),
            encoding_format: "utf-8".to_string(),
        }
    }

    /// Creates a new QRCode structure from a vector of bytes.
    pub fn from_bytes(data: Vec<u8>) -> Self {
        QRCode {
            data,
            encoding_format: "utf-8".to_string(),
        }
    }

    /// Converts the QRCode structure to a QrCode structure.
    pub fn to_qrcode(&self) -> QrCode {
        QrCode::new(&self.data).unwrap()
    }

    /// Converts the QRCode structure to a PNG image.
    pub fn to_png(&self, width: u32) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
        let qrcode = self.to_qrcode();
        let height = width;
        let mut img = ImageBuffer::new(width, height);
        for (x, y, pixel) in img.enumerate_pixels_mut() {
            let x_index = (x as f32 / width as f32) * qrcode.width() as f32;
            let y_index = (y as f32 / height as f32) * qrcode.width() as f32;
            *pixel = match qrcode[(x_index as usize, y_index as usize)] {
                qrcode::Color::Dark => Rgba([0, 0, 0, 0]),
                qrcode::Color::Light => Rgba([255, 255, 255, 255]),
            };
        }
        img
    }
    /// Converts the QRCode structure to a JPG image.
    pub fn to_jpg(&self, width: u32) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
        let qrcode = self.to_qrcode();
        let height = width;
        let mut img = ImageBuffer::new(width, height);
        for (x, y, pixel) in img.enumerate_pixels_mut() {
            let x_index = (x as f32 / width as f32) * qrcode.width() as f32;
            let y_index = (y as f32 / height as f32) * qrcode.width() as f32;
            *pixel = match qrcode[(x_index as usize, y_index as usize)] {
                qrcode::Color::Dark => Rgba([0, 0, 0, 0]),
                qrcode::Color::Light => Rgba([255, 255, 255, 255]),
            };
        }
        img
    }
    /// Converts the QRCode structure to a GIF image.
    pub fn to_gif(&self, width: u32) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
        let qrcode = self.to_qrcode();
        let height = width;
        let mut img = ImageBuffer::new(width, height);
        for (x, y, pixel) in img.enumerate_pixels_mut() {
            let x_index = (x as f32 / width as f32) * qrcode.width() as f32;
            let y_index = (y as f32 / height as f32) * qrcode.width() as f32;
            *pixel = match qrcode[(x_index as usize, y_index as usize)] {
                qrcode::Color::Dark => Rgba([0, 0, 0, 0]),
                qrcode::Color::Light => Rgba([255, 255, 255, 255]),
            };
        }
        img
    }

    /// Converts the QRCode structure to an SVG image.
    pub fn to_svg(&self, width: u32) -> String {
        let qrcode = self.to_qrcode();
        let svg_string = qrcode
            .render::<svg::Color>()
            .min_dimensions(width, width)
            .dark_color(svg::Color("#000000"))
            .light_color(svg::Color("#FFFFFF"))
            .build();
        svg_string
    }

    /// The `colorize` method creates a new PNG image of the QR code
    /// using the data stored in the QRCode and the given color value to
    /// colorize the QR code.
    pub fn colorize(&self, color: Rgba<u8>) -> RgbaImage {
        let qrcode = self.to_qrcode();
        let mut img: RgbaImage = ImageBuffer::new(qrcode.width() as u32, qrcode.width() as u32);
        for (x, y, pixel) in img.enumerate_pixels_mut() {
            let c = if qrcode[(x as usize, y as usize)] == qrcode::Color::Dark {
                color
            } else {
                Rgba([255, 255, 255, 255])
            };
            *pixel = c;
        }
        img
    }

    /// The `resize` method creates a new PNG image of the QR code using
    /// the data stored in the QRCode and the given width and height
    /// values to resize the QR code.
    pub fn resize(&self, width: u32, height: u32) -> RgbaImage {
        let qrcode = self.to_qrcode();
        let mut img: RgbaImage = ImageBuffer::new(width, height);
        for y in 0..height {
            for x in 0..width {
                let x_index = (x as f32 / width as f32) * qrcode.width() as f32;
                let y_index = (y as f32 / height as f32) * qrcode.width() as f32;
                let c = match qrcode[(x_index as usize, y_index as usize)] {
                    qrcode::Color::Dark => Rgba([0, 0, 0, 0]),
                    qrcode::Color::Light => Rgba([255, 255, 255, 255]),
                };
                img.put_pixel(x, y, c);
            }
        }
        img
    }
    /// The `add_image_watermark` method adds a watermark to the given image.
    pub fn add_image_watermark(img: &mut RgbaImage, watermark: &RgbaImage) {
        let (width, height) = img.dimensions();
        let (watermark_width, watermark_height) = watermark.dimensions();

        // position the watermark in the bottom right corner
        let x = width - watermark_width;
        let y = height - watermark_height;

        // draw the watermark on the QR code image
        for (dx, dy, watermark_pixel) in watermark.enumerate_pixels() {
            let x = x + dx;
            let y = y + dy;
            let qr_pixel = img.get_pixel(x, y);

            let alpha = (watermark_pixel[3] as f32) / 255.0;
            let new_r = (1.0 - alpha) * (qr_pixel[0] as f32) + alpha * (watermark_pixel[0] as f32);
            let new_g = (1.0 - alpha) * (qr_pixel[1] as f32) + alpha * (watermark_pixel[1] as f32);
            let new_b = (1.0 - alpha) * (qr_pixel[2] as f32) + alpha * (watermark_pixel[2] as f32);
            let new_a = (qr_pixel[3] as f32) + alpha * (255.0 - qr_pixel[3] as f32);

            let new_pixel = [new_r as u8, new_g as u8, new_b as u8, new_a as u8];
            img.put_pixel(x, y, image::Rgba(new_pixel));
        }
    }

    /// Creates a QR code with multi-language support.
    pub fn create_multilanguage(data_map: HashMap<String, String>) -> Self {
        // Implementation to generate a QR code that can display different data
        // based on the user's language preference.

        // You can choose the user's language preference based on their settings,
        // or use a default language if no preference is available.
        let user_language = "en"; // Replace with the actual user's language or a default value.

        // Determine the data to be encoded based on the user's language preference.
        let mut selected_data = "";
        if let Some(language_data) = data_map.get(user_language) {
            selected_data = language_data;
        }

        // Create a QRCode instance with the selected data.
        QRCode::from_string(selected_data.to_string())
    }

    /// Generates a dynamic QR code, which can be updated after creation.
    pub fn create_dynamic(initial_data: &str) -> Self {
        // Implementation for creating a QR code whose content can be updated post-creation.

        // You can choose a specific format or protocol for dynamic QR codes, such as URL encoding.
        let dynamic_data_format = "url"; // Replace with your chosen format.

        // Create a dynamic QR code URL based on the initial data and format.
        let dynamic_url = match dynamic_data_format {
            "url" => {
                format!(
                    "https://your-api-endpoint.com/update?qrcode={}",
                    initial_data
                )
            }
            // Add more format cases as needed.
            _ => return QRCode::from_string(initial_data.to_string()), // Default to the initial data.
        };

        // Create a QRCode instance with the dynamic URL.
        QRCode::from_string(dynamic_url)

    }

    /// Combines multiple QR codes into a single larger QR code.
    pub fn combine_qr_codes(codes: Vec<QRCode>) -> Result<Self, &'static str> {
        // Implementation to merge multiple QR codes into one, suitable for complex data sets.

        // Check if there are any QR codes to combine.
        if codes.is_empty() {
            return Err("No QR codes to combine");
        }

        // Calculate the total width and height needed for the combined QR code.
        let total_width = codes
            .iter()
            .map(|code| code.to_qrcode().width() as u32)
            .sum();

        // Create a new QRCode instance with a placeholder empty data.
        let mut combined_qrcode = QRCode::from_bytes(Vec::new());

        // Set the width and height of the combined QR code.
        combined_qrcode.resize(total_width, total_width);

        // Create an image buffer to hold the combined QR code.
        let mut combined_image = combined_qrcode.to_png(total_width);

        // Initialize the x-coordinate for drawing the QR codes.
        let mut x_offset = 0;

        // Iterate through the QR codes and draw them onto the combined image.
        for code in codes {
            let qrcode = code.to_qrcode();
            let width = qrcode.width() as u32;
            let height = qrcode.width() as u32;

            // Copy each QR code's pixels to the combined image at the appropriate x-coordinate.
            for x in 0..width {
                for y in 0..height {
                    let pixel = qrcode[(x as usize, y as usize)];
                    let combined_x = x + x_offset;
                    let combined_y = y;

                    // Set the pixel color on the combined image.
                    match pixel {
                        qrcode::Color::Dark => {
                            combined_image.put_pixel(combined_x, combined_y, Rgba([0, 0, 0, 0]));
                        }
                        qrcode::Color::Light => {
                            combined_image.put_pixel(
                                combined_x,
                                combined_y,
                                Rgba([255, 255, 255, 255]),
                            );
                        }
                    }
                }
            }

            // Update the x-coordinate for the next QR code.
            x_offset += width;
        }

        // Update the data of the combined QR code with the image buffer.
        combined_qrcode.data = combined_image.into_raw();

        Ok(combined_qrcode)
    }

    /// Compresses data before encoding it into a QR code.
    pub fn compress_data(data: &str) -> Vec<u8> {
        // Implementation for data compression to reduce the size of data before QR code generation.

        // Encode the input data into bytes.
        let input_bytes = data.as_bytes();

        // Create a buffer to store the compressed data.
        let mut compressed_data = Vec::new();

        // Initialize a Zlib encoder with compression settings.
        let mut encoder = ZlibEncoder::new(&mut compressed_data, Compression::default());

        // Compress the input data and check for errors.
        if encoder.write_all(input_bytes).is_err()  {
            // Compression failed, return the original data.
            return input_bytes.to_vec();
        }

        // Finish the compression process and retrieve the compressed data.
        if encoder.finish().is_err() {
            // Compression failed, return the original data.
            return input_bytes.to_vec();
        }

        compressed_data
    }

    /// Decompresses data after decoding it from a QR code.
    pub fn batch_generate_qr_codes(data: Vec<String>) -> Vec<QRCode> {
        // Implementation for batch generating QR codes from a list of data.

        // Create a vector to store the generated QR codes.
        let mut qr_codes = Vec::new();

        // Iterate through the data and generate a QR code for each item.
        for item in data {
            // Create a QR code for the current item.
            let qr_code = QRCode::from_string(item);

            // Add the QR code to the vector.
            qr_codes.push(qr_code);
        }

        qr_codes
    }

    /// Implementation for overlaying an image on top of a QR code.
    pub fn overlay_image(&self, overlay: &RgbaImage) -> RgbaImage {
        // Create a QR code image.
        let qrcode = self.to_qrcode();

        // Create an image buffer to hold the combined image.
        let mut combined_image = ImageBuffer::new(qrcode.width() as u32, qrcode.width() as u32);

        // Copy the QR code pixels to the combined image.
        for x in 0..qrcode.width() {
            for y in 0..qrcode.width() {
                let pixel = qrcode[(x, y)];
                let combined_x = x as u32; // Convert usize to u32
                let combined_y = y as u32; // Convert usize to u32

                // Set the pixel color on the combined image.
                match pixel {
                    Color::Dark => {
                        combined_image.put_pixel(combined_x, combined_y, Rgba([0, 0, 0, 0]));
                    }
                    Color::Light => {
                        combined_image.put_pixel(
                            combined_x,
                            combined_y,
                            Rgba([255, 255, 255, 255]),
                        );
                    }
                }
            }
        }

        // Overlay the image on top of the QR code.
        for x in 0..overlay.width() {
            for y in 0..overlay.height() {
                let pixel = overlay.get_pixel(x, y);
                let combined_x = x; // No need to convert as `x` and `y` are already u32
                let combined_y = y; // No need to convert as `x` and `y` are already u32

                // Set the pixel color on the combined image.
                combined_image.put_pixel(combined_x, combined_y, *pixel);
            }
        }
        combined_image
    }

    /// Sets the encoding format for the QR code and returns a new instance.
    pub fn set_encoding_format(&self, format: &str) -> Result<Self, &'static str> {
        if format != "utf-8" {
            return Err("Unsupported encoding format");
        }

        Ok(Self {
            data: self.data.clone(),
            encoding_format: format.to_string(), // Set the encoding format
                                                 // ... copy other fields ...
        })
    }

    /// Gets the encoding format of the QR code.
    pub fn get_encoding_format(&self) -> &str {
        &self.encoding_format
    }
}
