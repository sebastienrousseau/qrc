// Copyright © 2022-2026 QRC. All rights reserved.
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
//!
#![deny(dead_code)]
#![deny(missing_debug_implementations)]
#![deny(missing_docs)]
#![forbid(unsafe_code)]
#![warn(unreachable_pub)]
#![doc(
    html_favicon_url = "https://kura.pro/qrc/favicon.ico",
    html_logo_url = "https://cloudcdn.pro/qrc/v1/logos/qrc.svg",
    html_root_url = "https://docs.rs/qrc"
)]
#![crate_name = "qrc"]

use image::{DynamicImage, ImageBuffer, ImageFormat, Rgba, RgbaImage};
use miniz_oxide::deflate::compress_to_vec_zlib;
use qrcode::{render::svg, Color, QrCode};
use std::collections::HashMap;
use std::fmt::Write as _;
use std::io::Cursor;

pub use qrcode::types::EcLevel;
pub use qrcode::types::QrError;

/// The `macros` module contains functions for generating macros.
pub mod macros;

/// Structured payload builders (vCard, Wi-Fi, MeCard, EMVCo) that turn typed
/// data into the text conventions QR scanners recognise.
pub mod payload;

#[cfg(feature = "wasm")]
/// WASM bindings for the QRC library.
pub mod wasm;

/// Shape used to render each QR code module.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub enum ModuleShape {
    /// Standard square modules (default).
    #[default]
    Square,
    /// Squares with rounded corners.
    RoundedSquare,
    /// Circular modules.
    Circle,
    /// Diamond-shaped modules.
    Diamond,
}

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
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct QRCode {
    /// The `data` field holds the data to be encoded in the QR code.
    pub data: Vec<u8>,
    /// The `encoding_format` field holds the encoding format of the QR code.
    encoding_format: String,
    /// Error correction level for the QR code.
    pub ec_level: EcLevel,
    /// Shape used for rendering individual QR modules.
    pub shape: ModuleShape,
}

impl Default for QRCode {
    fn default() -> Self {
        Self {
            data: Vec::new(),
            encoding_format: "utf-8".to_string(),
            ec_level: EcLevel::M,
            shape: ModuleShape::Square,
        }
    }
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
        Self {
            data,
            encoding_format: "utf-8".to_string(),
            ec_level: EcLevel::M,
            shape: ModuleShape::Square,
        }
    }

    /// Creates a new `QRCode` instance by converting the given string data
    /// into a vector of bytes.
    #[must_use]
    pub fn from_string(data: String) -> Self {
        Self {
            data: data.into_bytes(),
            encoding_format: "utf-8".to_string(),
            ec_level: EcLevel::M,
            shape: ModuleShape::Square,
        }
    }

    /// Creates a new `QRCode` instance from a vector of bytes.
    #[must_use]
    pub fn from_bytes(data: Vec<u8>) -> Self {
        Self {
            data,
            encoding_format: "utf-8".to_string(),
            ec_level: EcLevel::M,
            shape: ModuleShape::Square,
        }
    }

    /// Sets the error correction level (builder pattern).
    ///
    /// # Examples
    ///
    /// ```
    /// use qrc::{QRCode, EcLevel};
    ///
    /// let qr = QRCode::from_string("Hello".to_string())
    ///     .with_ec_level(EcLevel::H);
    /// assert_eq!(qr.ec_level, EcLevel::H);
    /// ```
    #[must_use]
    pub fn with_ec_level(mut self, ec_level: EcLevel) -> Self {
        self.ec_level = ec_level;
        self
    }

    /// Sets the module shape (builder pattern).
    ///
    /// # Examples
    ///
    /// ```
    /// use qrc::{QRCode, ModuleShape};
    ///
    /// let qr = QRCode::from_string("Hello".to_string())
    ///     .with_shape(ModuleShape::Circle);
    /// assert_eq!(qr.shape, ModuleShape::Circle);
    /// ```
    #[must_use]
    pub fn with_shape(mut self, shape: ModuleShape) -> Self {
        self.shape = shape;
        self
    }

    /// Tries to convert the `QRCode` data to a `QrCode` structure.
    ///
    /// # Errors
    ///
    /// Returns `QrError` if the data is too long or otherwise invalid.
    pub fn try_to_qrcode(&self) -> Result<QrCode, QrError> {
        QrCode::with_error_correction_level(&self.data, self.ec_level)
    }

    /// Converts the `QRCode` data to a `QrCode` structure.
    ///
    /// # Panics
    ///
    /// Panics if the data cannot be encoded as a valid QR code.
    /// Use [`try_to_qrcode`](Self::try_to_qrcode) for a fallible alternative.
    #[must_use]
    pub fn to_qrcode(&self) -> QrCode {
        self.try_to_qrcode().expect("Failed to encode QR code")
    }

    /// Renders the QR code into an RGBA image buffer at the given width.
    ///
    /// This is the shared implementation used by `to_png`, `to_jpg`, and `to_gif`.
    #[allow(
        clippy::cast_precision_loss,
        clippy::cast_possible_truncation,
        clippy::cast_sign_loss
    )]
    fn render_image(&self, width: u32) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
        let qrcode = self.to_qrcode();
        let height = width;
        let qr_width = qrcode.width() as f64;
        let module_size = f64::from(width) / qr_width;
        let mut img: RgbaImage = ImageBuffer::from_pixel(width, height, Rgba([255, 255, 255, 255]));

        for (x, y, pixel) in img.enumerate_pixels_mut() {
            let x_index = (f64::from(x) / f64::from(width) * qr_width) as usize;
            let y_index = (f64::from(y) / f64::from(height) * qr_width) as usize;
            if qrcode[(x_index, y_index)] == Color::Dark {
                let mod_x = f64::from(x) - (x_index as f64) * module_size;
                let mod_y = f64::from(y) - (y_index as f64) * module_size;
                if self.is_inside_shape(mod_x, mod_y, module_size) {
                    *pixel = Rgba([0, 0, 0, 255]);
                }
            }
        }
        img
    }

    /// Checks whether a pixel at (`mod_x`, `mod_y`) within a module of the given
    /// size falls inside the current shape.
    #[allow(clippy::cast_precision_loss)]
    fn is_inside_shape(&self, mod_x: f64, mod_y: f64, module_size: f64) -> bool {
        match self.shape {
            ModuleShape::Square => true,
            ModuleShape::RoundedSquare => {
                let radius = module_size * 0.3;
                is_inside_rounded_rect(mod_x, mod_y, module_size, module_size, radius)
            }
            ModuleShape::Circle => {
                let half = module_size / 2.0;
                let dx = mod_x - half;
                let dy = mod_y - half;
                dx * dx + dy * dy <= half * half
            }
            ModuleShape::Diamond => {
                let half = module_size / 2.0;
                (mod_x - half).abs() + (mod_y - half).abs() <= half
            }
        }
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

    /// Returns the PNG-encoded bytes of the QR code.
    ///
    /// # Panics
    ///
    /// Panics if PNG encoding fails (should not happen in practice).
    #[must_use]
    pub fn to_png_bytes(&self, width: u32) -> Vec<u8> {
        let img = self.render_image(width);
        let mut buf = Cursor::new(Vec::new());
        DynamicImage::ImageRgba8(img)
            .write_to(&mut buf, ImageFormat::Png)
            .expect("PNG encoding failed");
        buf.into_inner()
    }

    /// Returns actual JPEG-encoded bytes (quality 85) of the QR code.
    #[must_use]
    pub fn to_jpg(&self, width: u32) -> Vec<u8> {
        self.to_jpg_with_quality(width, 85)
    }

    /// Returns JPEG-encoded bytes at a custom quality (1-100).
    ///
    /// # Panics
    ///
    /// Panics if JPEG encoding fails (should not happen in practice).
    #[must_use]
    pub fn to_jpg_with_quality(&self, width: u32, quality: u8) -> Vec<u8> {
        let img = self.render_image(width);
        let rgb = DynamicImage::ImageRgba8(img).to_rgb8();
        let mut buf = Cursor::new(Vec::new());
        image::codecs::jpeg::JpegEncoder::new_with_quality(&mut buf, quality)
            .encode_image(&rgb)
            .expect("JPEG encoding failed");
        buf.into_inner()
    }

    /// Returns actual GIF-encoded bytes of the QR code.
    ///
    /// # Panics
    ///
    /// Panics if GIF encoding fails (should not happen in practice).
    #[must_use]
    pub fn to_gif(&self, width: u32) -> Vec<u8> {
        let img = self.render_image(width);
        let mut buf = Cursor::new(Vec::new());
        DynamicImage::ImageRgba8(img)
            .write_to(&mut buf, ImageFormat::Gif)
            .expect("GIF encoding failed");
        buf.into_inner()
    }

    /// Returns the raw RGBA image buffer for the QR code.
    #[must_use]
    pub fn to_image(&self, width: u32) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
        self.render_image(width)
    }

    /// Converts the `QRCode` to an SVG image.
    ///
    /// For non-square shapes, a custom SVG renderer is used.
    ///
    /// # Parameters
    ///
    /// * `width`: The width of the desired image in pixels.
    ///
    /// # Returns
    ///
    /// A `String` representing the QR code in SVG format.
    #[must_use]
    #[allow(clippy::cast_precision_loss, clippy::cast_possible_truncation)]
    pub fn to_svg(&self, width: u32) -> String {
        let qrcode = self.to_qrcode();

        if self.shape == ModuleShape::Square {
            return qrcode
                .render::<svg::Color>()
                .min_dimensions(width, width)
                .dark_color(svg::Color("#000000"))
                .light_color(svg::Color("#FFFFFF"))
                .build();
        }

        // Custom SVG for non-square shapes
        let qr_dim = qrcode.width();
        let module_size = f64::from(width) / qr_dim as f64;
        let mut elements = String::new();

        for y in 0..qr_dim {
            for x in 0..qr_dim {
                if qrcode[(x, y)] == Color::Dark {
                    let px = x as f64 * module_size;
                    let py = y as f64 * module_size;
                    match self.shape {
                        ModuleShape::RoundedSquare => {
                            let r = module_size * 0.3;
                            let _ = write!(elements,
                                "<rect x=\"{px}\" y=\"{py}\" width=\"{module_size}\" height=\"{module_size}\" rx=\"{r}\" ry=\"{r}\" fill=\"#000000\"/>"
                            );
                        }
                        ModuleShape::Circle => {
                            let cx = px + module_size / 2.0;
                            let cy = py + module_size / 2.0;
                            let r = module_size / 2.0;
                            let _ = write!(
                                elements,
                                "<circle cx=\"{cx}\" cy=\"{cy}\" r=\"{r}\" fill=\"#000000\"/>"
                            );
                        }
                        ModuleShape::Diamond => {
                            let half = module_size / 2.0;
                            let top_x = px + half;
                            let top_y = py;
                            let right_x = px + module_size;
                            let right_y = py + half;
                            let bot_x = px + half;
                            let bot_y = py + module_size;
                            let left_x = px;
                            let left_y = py + half;
                            let _ = write!(elements,
                                "<polygon points=\"{top_x},{top_y} {right_x},{right_y} {bot_x},{bot_y} {left_x},{left_y}\" fill=\"#000000\"/>"
                            );
                        }
                        ModuleShape::Square => unreachable!(),
                    }
                }
            }
        }

        format!(
            "<?xml version=\"1.0\" standalone=\"yes\"?><svg xmlns=\"http://www.w3.org/2000/svg\" version=\"1.1\" width=\"{width}\" height=\"{width}\"><rect width=\"100%\" height=\"100%\" fill=\"#FFFFFF\"/>{elements}</svg>"
        )
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
    #[allow(clippy::cast_possible_truncation, clippy::cast_precision_loss)]
    pub fn colorize(&self, color: Rgba<u8>) -> RgbaImage {
        let qrcode = self.to_qrcode();
        let qr_dim = qrcode.width() as u32;
        let module_size = 1.0; // 1:1 mapping at native resolution
        let mut img: RgbaImage =
            ImageBuffer::from_pixel(qr_dim, qr_dim, Rgba([255, 255, 255, 255]));
        for (x, y, pixel) in img.enumerate_pixels_mut() {
            if qrcode[(x as usize, y as usize)] == Color::Dark {
                let mod_x = f64::from(x) - f64::from(x) * module_size / module_size;
                let mod_y = f64::from(y) - f64::from(y) * module_size / module_size;
                if self.is_inside_shape(mod_x, mod_y, module_size) {
                    *pixel = color;
                }
            }
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
    #[allow(
        clippy::cast_precision_loss,
        clippy::cast_possible_truncation,
        clippy::cast_sign_loss
    )]
    pub fn resize(&self, width: u32, height: u32) -> RgbaImage {
        let qrcode = self.to_qrcode();
        let qr_width = qrcode.width() as f64;
        let module_size_x = f64::from(width) / qr_width;
        let module_size_y = f64::from(height) / qr_width;
        let mut img: RgbaImage = ImageBuffer::from_pixel(width, height, Rgba([255, 255, 255, 255]));
        for y in 0..height {
            for x in 0..width {
                let x_index = (f64::from(x) / f64::from(width) * qr_width) as usize;
                let y_index = (f64::from(y) / f64::from(height) * qr_width) as usize;
                if qrcode[(x_index, y_index)] == Color::Dark {
                    let mod_x = f64::from(x) - (x_index as f64) * module_size_x;
                    let mod_y = f64::from(y) - (y_index as f64) * module_size_y;
                    // Use average module size for shape check
                    let avg_mod = (module_size_x + module_size_y) / 2.0;
                    if self.is_inside_shape(mod_x, mod_y, avg_mod) {
                        img.put_pixel(x, y, Rgba([0, 0, 0, 255]));
                    }
                }
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
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
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
            let new_r = alpha.mul_add(
                f32::from(watermark_pixel[0]),
                (1.0 - alpha) * f32::from(qr_pixel[0]),
            );
            let new_g = alpha.mul_add(
                f32::from(watermark_pixel[1]),
                (1.0 - alpha) * f32::from(qr_pixel[1]),
            );
            let new_b = alpha.mul_add(
                f32::from(watermark_pixel[2]),
                (1.0 - alpha) * f32::from(qr_pixel[2]),
            );
            let new_a = alpha.mul_add(255.0 - f32::from(qr_pixel[3]), f32::from(qr_pixel[3]));

            img.put_pixel(
                px,
                py,
                Rgba([new_r as u8, new_g as u8, new_b as u8, new_a as u8]),
            );
        }
    }

    /// Creates a multilingual QR code based on a map of language codes to data strings.
    ///
    /// # Parameters
    ///
    /// * `data_map`: A `HashMap` mapping language codes (`String`) to data (`String`).
    /// * `language`: The preferred language code (e.g. `"en"`).
    ///
    /// Falls back to `"en"`, then the first value in the map, then an empty string.
    ///
    /// # Returns
    ///
    /// A `QRCode` instance representing a multilingual QR code.
    #[must_use]
    pub fn create_multilanguage(data_map: &HashMap<String, String>, language: &str) -> Self {
        let selected_data = data_map
            .get(language)
            .or_else(|| data_map.get("en"))
            .or_else(|| data_map.values().next())
            .map_or("", String::as_str);
        Self::from_string(selected_data.to_string())
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
                format!("https://your-api-endpoint.com/update?qrcode={initial_data}")
            }
            _ => return Self::from_string(initial_data.to_string()),
        };

        Self::from_string(dynamic_url)
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
    #[allow(clippy::cast_possible_truncation)]
    pub fn combine_qr_codes(codes: &[Self]) -> Result<Self, &'static str> {
        if codes.is_empty() {
            return Err("No QR codes to combine");
        }

        let total_width: u32 = codes
            .iter()
            .map(|code| code.to_qrcode().width() as u32)
            .sum();

        let mut combined_image: RgbaImage =
            ImageBuffer::from_pixel(total_width, total_width, Rgba([255, 255, 255, 255]));

        let mut x_offset: u32 = 0;

        for code in codes {
            let qrcode = code.to_qrcode();
            let width = qrcode.width() as u32;

            for x in 0..width {
                for y in 0..width {
                    let pixel = qrcode[(x as usize, y as usize)];
                    let combined_x = x + x_offset;

                    if pixel == Color::Dark {
                        combined_image.put_pixel(combined_x, y, Rgba([0, 0, 0, 255]));
                    }
                }
            }

            x_offset += width;
        }

        let mut combined_qrcode = Self::from_bytes(Vec::new());
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
    pub fn batch_generate_qr_codes(data: Vec<String>) -> Vec<Self> {
        data.into_iter().map(Self::from_string).collect()
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
    #[allow(clippy::cast_possible_truncation)]
    pub fn overlay_image(&self, overlay: &RgbaImage) -> RgbaImage {
        let qrcode = self.to_qrcode();
        let qr_dim = qrcode.width() as u32;
        let mut combined_image: RgbaImage =
            ImageBuffer::from_pixel(qr_dim, qr_dim, Rgba([255, 255, 255, 255]));

        for x in 0..qrcode.width() {
            for y in 0..qrcode.width() {
                let pixel = qrcode[(x, y)];
                let cx = x as u32;
                let cy = y as u32;

                if pixel == Color::Dark {
                    combined_image.put_pixel(cx, cy, Rgba([0, 0, 0, 255]));
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
            ec_level: self.ec_level,
            shape: self.shape,
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

/// Checks whether a point (x, y) is inside a rounded rectangle of the
/// given dimensions and corner radius.
#[allow(clippy::many_single_char_names)]
fn is_inside_rounded_rect(x: f64, y: f64, w: f64, h: f64, radius: f64) -> bool {
    let r = radius.min(w / 2.0).min(h / 2.0);
    // Inside the main body (excluding corners)
    if x >= r && x <= w - r {
        return true;
    }
    if y >= r && y <= h - r {
        return true;
    }
    // Check corners
    let corners = [(r, r), (w - r, r), (r, h - r), (w - r, h - r)];
    for (cx, cy) in &corners {
        let dx = x - cx;
        let dy = y - cy;
        if dx * dx + dy * dy <= r * r {
            return true;
        }
    }
    false
}
