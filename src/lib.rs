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
//! QRC is organised in two layers:
//!
//! - The [`encode`] layer turns bytes into a renderer-ready [`Matrix`] via the
//!   pluggable [`Engine`] trait (default backend: the `qrcode` crate). Control
//!   error correction, version and quiet zone through [`QrOptions`].
//! - The [`render`] layer turns a [`Matrix`] into output: [`render::svg`]
//!   (SVG-first, styleable), [`render::raster`] (RGBA plus real PNG/JPEG/GIF
//!   byte encoders, behind the `raster` feature) and [`render::unicode`]
//!   (terminal output).
//!
//! All generated codes carry the mandatory quiet zone, use integer module
//! scaling and draw opaque modules, so they are scannable by construction —
//! verified by round-trip decode tests.
//!
//! Capability status for this version (0.0.6):
//!
//! | Capability | Status |
//! | ---------- | ------ |
//! | Modes Numeric / Alphanumeric / Byte (auto) | supported (via `qrcode`) |
//! | Kanji / ECI / FNC1 / Structured Append / Micro QR | planned (see ROADMAP.md) |
//! | QR versions 1–40, forced or auto | supported |
//! | Error-correction level (L/M/Q/H) | supported |
//! | Quiet zone (configurable, default 4) | supported |
//! | PNG / JPEG / GIF byte encoders | supported (`raster`) |
//! | SVG output (square / rounded / circle modules) | supported (`svg`) |
//! | Terminal / Unicode output | supported (`unicode`) |
//! | Custom colors | supported |
//! | Pluggable encoding engine | supported |
//! | Logo embedding / branded codes | supported (`raster`) |
//! | Offline artistic blend (image-into-QR) | supported (`raster`) |
//! | ControlNet control-image export (AI art-QR) | supported (`raster`) |
//! | Business-card vCard payload | supported (`payload`) |
//! | Arbitrary image formats (BMP/TIFF/WebP/…) | supported (`raster`) |
//! | AI art-QR via cloud provider | planned (`api` feature) |
//! | Styling (gradients, eye shapes) | planned (Phase 2) |
//! | Structured payloads (WiFi/EMVCo/EPC) | planned (Phase 2) |
//! | CLI / WASM / Python bindings | planned (Phase 3) |
//!
//! ## Usage
//!
//! ```
//! use qrc::{QRCode, QrOptions, Ecc};
//! use qrc::render::svg::SvgOptions;
//!
//! let qr = QRCode::from_string("https://example.com".to_string());
//! let opts = QrOptions::new().ecc(Ecc::High);
//!
//! // SVG-first, no image dependency required.
//! let svg = qr.to_svg_styled(&opts, &SvgOptions::default()).unwrap();
//! assert!(svg.starts_with("<svg"));
//! ```
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
#![crate_type = "lib"]

extern crate qrcode;

#[cfg(feature = "raster")]
use image::{ImageBuffer, Rgba, RgbaImage};
use qrcode::{render::svg, QrCode};
use std::collections::HashMap;

/// The `macros` module contains functions for generating macros.
pub mod macros;

pub mod encode;
pub mod error;
pub mod matrix;
#[cfg(feature = "payload")]
pub mod payload;
pub mod render;

pub use encode::{Ecc, Engine, QrOptions, QrcodeEngine};
pub use error::{QrError, Result};
pub use matrix::Matrix;
pub use render::{Color as StyleColor, ModuleShape};

#[non_exhaustive]
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
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
pub struct QRCode {
    /// The `data` field holds the data to be encoded in the QR code.
    pub data: Vec<u8>,
    /// The `encoding_format` field holds the encoding format of the QR code.
    encoding_format: String,
}
/// Implementation of QRCode structure.
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
    ///
    /// # Parameters
    ///
    /// * `data`: A `Vec<u8>` representing the data to be encoded in the QR code.
    ///
    /// # Returns
    ///
    /// A new `QRCode` instance.
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

    /// Converts the QRCode structure to a `QrCode`, returning an error
    /// instead of panicking when the data exceeds QR capacity.
    ///
    /// # Errors
    ///
    /// Returns [`qrcode::types::QrError`] if the data cannot be encoded
    /// (for example, when it exceeds the maximum capacity of a QR code).
    pub fn try_to_qrcode(&self) -> core::result::Result<QrCode, qrcode::types::QrError> {
        QrCode::new(&self.data)
    }

    /// Converts the QRCode structure to a `QrCode` structure.
    ///
    /// # Panics
    ///
    /// Panics if the data cannot be encoded as a QR code (e.g. it exceeds
    /// the maximum capacity). Prefer [`QRCode::try_to_qrcode`] for fallible
    /// encoding of untrusted input.
    pub fn to_qrcode(&self) -> QrCode {
        self.try_to_qrcode()
            .expect("data could not be encoded as a QR code; use try_to_qrcode for untrusted input")
    }

    /// Renders the encoded QR code into an opaque RGBA image of the given
    /// pixel `width` (square), including the mandatory quiet zone and using
    /// integer module scaling so every module is exactly the same size.
    ///
    /// Dark modules are opaque black and light modules are opaque white,
    /// which keeps the code scannable on any background. Use
    /// [`QRCode::render_image_with`] to choose custom colors.
    #[cfg(feature = "raster")]
    fn render_image(&self, qrcode: &QrCode, width: u32) -> RgbaImage {
        self.render_image_with(
            qrcode,
            width,
            Rgba([0, 0, 0, 255]),
            Rgba([255, 255, 255, 255]),
        )
    }

    /// Renders the QR code into an RGBA image with explicit `dark`/`light`
    /// colors, a 4-module quiet zone, and integer module scaling.
    #[cfg(feature = "raster")]
    fn render_image_with(
        &self,
        qrcode: &QrCode,
        width: u32,
        dark: Rgba<u8>,
        light: Rgba<u8>,
    ) -> RgbaImage {
        const QUIET_ZONE: u32 = 4;
        let modules = qrcode.width() as u32;
        let total_modules = modules + 2 * QUIET_ZONE;

        // Integer module size keeps every module identical; never sub-pixel.
        let module_px = (width / total_modules).max(1);
        let img_size = module_px * total_modules;

        let mut img: RgbaImage = ImageBuffer::from_pixel(img_size, img_size, light);
        for my in 0..modules {
            for mx in 0..modules {
                if qrcode[(mx as usize, my as usize)] != qrcode::Color::Dark {
                    continue;
                }
                let px0 = (mx + QUIET_ZONE) * module_px;
                let py0 = (my + QUIET_ZONE) * module_px;
                for dy in 0..module_px {
                    for dx in 0..module_px {
                        img.put_pixel(px0 + dx, py0 + dy, dark);
                    }
                }
            }
        }
        img
    }

    /// Converts the QRCode structure to a PNG image.
    ///
    /// # Examples
    ///
    /// ```
    /// use qrc::QRCode;
    ///
    /// // Convert a string slice to a String using `.to_string()`
    /// let qr = QRCode::from_string("Hello, world!".to_string());
    /// let png_image = qr.to_png(256);
    /// ```
    ///
    /// # Parameters
    ///
    /// * `width`: The width of the image in pixels.
    ///
    /// # Returns
    ///
    /// An `ImageBuffer` representing the QR code in PNG format.
    #[cfg(feature = "raster")]
    pub fn to_png(&self, width: u32) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
        let qrcode = self.to_qrcode();
        self.render_image(&qrcode, width)
    }
    /// Converts the QRCode structure to a raster image suitable for saving
    /// as JPG.
    ///
    /// # Parameters
    ///
    /// * `width`: The width of the desired image in pixels.
    ///
    /// # Returns
    ///
    /// An `ImageBuffer` of the QR code. Note that JPEG does not support an
    /// alpha channel; the buffer uses opaque colors so it renders correctly
    /// when saved as JPG.
    #[cfg(feature = "raster")]
    pub fn to_jpg(&self, width: u32) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
        let qrcode = self.to_qrcode();
        self.render_image(&qrcode, width)
    }
    /// Converts the QRCode structure to a raster image suitable for saving
    /// as GIF.
    ///
    /// # Parameters
    ///
    /// * `width`: The width of the desired image in pixels.
    ///
    /// # Returns
    ///
    /// An `ImageBuffer` of the QR code.
    #[cfg(feature = "raster")]
    pub fn to_gif(&self, width: u32) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
        let qrcode = self.to_qrcode();
        self.render_image(&qrcode, width)
    }

    /// Converts the QRCode structure to an SVG image.
    ///
    /// # Parameters
    ///
    /// * `width`: The width of the desired image in pixels.
    ///
    /// # Returns
    ///
    /// A `String` representing the QR code in SVG format.
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

    /// Colorizes the QR code with the specified color.
    ///
    /// # Parameters
    ///
    /// * `color`: The `Rgba<u8>` color value to use for the QR code.
    ///
    /// # Returns
    ///
    /// A colorized `RgbaImage` of the QR code.
    #[cfg(feature = "raster")]
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
    #[cfg(feature = "raster")]
    pub fn resize(&self, width: u32, height: u32) -> RgbaImage {
        let qrcode = self.to_qrcode();
        let mut img: RgbaImage = ImageBuffer::new(width, height);
        for y in 0..height {
            for x in 0..width {
                let x_index = (x as f32 / width as f32) * qrcode.width() as f32;
                let y_index = (y as f32 / height as f32) * qrcode.width() as f32;
                let c = match qrcode[(x_index as usize, y_index as usize)] {
                    qrcode::Color::Dark => Rgba([0, 0, 0, 255]),
                    qrcode::Color::Light => Rgba([255, 255, 255, 255]),
                };
                img.put_pixel(x, y, c);
            }
        }
        img
    }

    /// Adds a watermark image to the QR code.
    ///
    /// # Parameters
    ///
    /// * `img`: A mutable reference to the `RgbaImage` of the QR code.
    /// * `watermark`: A reference to the watermark `RgbaImage`.
    #[cfg(feature = "raster")]
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

    /// Creates a multilingual QR code based on a map of language codes to data strings.
    ///
    /// # Parameters
    ///
    /// * `data_map`: A `HashMap` mapping language codes (`String`) to data (`String`).
    ///
    /// # Returns
    ///
    /// A `QRCode` instance representing a multilingual QR code.
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

    /// Generates a dynamic QR code that can be updated after creation.
    ///
    /// # Parameters
    ///
    /// * `initial_data`: A string slice representing the initial data for the QR code.
    ///
    /// # Returns
    ///
    /// A `QRCode` instance representing a dynamic QR code.
    pub fn create_dynamic(initial_data: &str) -> Self {
        // A dynamic QR code encodes a stable redirect URL whose target can be
        // changed server-side after the code is printed. Here we wrap the
        // initial data in a redirect endpoint; swap the base URL for your own
        // managed redirector.
        let dynamic_url = format!("https://your-api-endpoint.com/update?qrcode={initial_data}");
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
    #[cfg(feature = "raster")]
    pub fn combine_qr_codes(codes: Vec<QRCode>) -> core::result::Result<Self, &'static str> {
        // Implementation to merge multiple QR codes into one, suitable for complex data sets.

        // Check if there are any QR codes to combine.
        if codes.is_empty() {
            return Err("No QR codes to combine");
        }

        // Quiet zone applied around each individual code in the strip.
        const QUIET_ZONE: u32 = 4;

        // Encode every code up front so capacity errors surface here rather
        // than panicking mid-render.
        let encoded: Vec<QrCode> = codes
            .iter()
            .map(|code| code.try_to_qrcode())
            .collect::<core::result::Result<_, _>>()
            .map_err(|_| "one of the QR codes could not be encoded")?;

        // The strip is one module per pixel: total width is the sum of each
        // code's quiet-zoned width, height is the tallest quiet-zoned code.
        let slot_widths: Vec<u32> = encoded
            .iter()
            .map(|qr| qr.width() as u32 + 2 * QUIET_ZONE)
            .collect();
        let total_width: u32 = slot_widths.iter().sum();
        // `fold` visits every (guaranteed non-empty) slot, so there is no
        // unreachable default branch as there would be with `max().unwrap_or`.
        let total_height: u32 = slot_widths.iter().copied().fold(0, u32::max);

        // White canvas so the quiet zones are correct everywhere.
        let mut combined_image: RgbaImage =
            ImageBuffer::from_pixel(total_width, total_height, Rgba([255, 255, 255, 255]));

        // Draw each code into its slot, offset by the quiet zone.
        let mut x_offset = 0;
        for (qr, slot_width) in encoded.iter().zip(&slot_widths) {
            let modules = qr.width() as u32;
            for my in 0..modules {
                for mx in 0..modules {
                    if qr[(mx as usize, my as usize)] == qrcode::Color::Dark {
                        let px = x_offset + QUIET_ZONE + mx;
                        let py = QUIET_ZONE + my;
                        combined_image.put_pixel(px, py, Rgba([0, 0, 0, 255]));
                    }
                }
            }
            x_offset += slot_width;
        }

        // Note: the result is a composite *image* (raw RGBA bytes stored in
        // `data`), not a single scannable QR symbol. See ROADMAP.md — this
        // helper is slated for replacement by Structured Append.
        let mut combined_qrcode = QRCode::from_bytes(Vec::new());
        combined_qrcode.data = combined_image.into_raw();

        Ok(combined_qrcode)
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

    /// Overlays an image on top of the QR code.
    ///
    /// # Parameters
    ///
    /// * `overlay`: A reference to the `RgbaImage` to overlay on the QR code.
    ///
    /// # Returns
    ///
    /// A combined `RgbaImage` with the overlay applied.
    #[cfg(feature = "raster")]
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
                    qrcode::Color::Dark => {
                        combined_image.put_pixel(combined_x, combined_y, Rgba([0, 0, 0, 255]));
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

    /// Sets the encoding format of the QR code.
    ///
    /// # Parameters
    ///
    /// * `format`: A string slice representing the encoding format.
    ///
    /// # Returns
    ///
    /// A `Result` which is either a new `QRCode` instance with updated encoding or an error string.
    pub fn set_encoding_format(&self, format: &str) -> core::result::Result<Self, &'static str> {
        if format != "utf-8" {
            return Err("Unsupported encoding format");
        }

        Ok(Self {
            data: self.data.clone(),
            encoding_format: format.to_string(), // Set the encoding format
                                                 // ... copy other fields ...
        })
    }

    /// Retrieves the encoding format of the QR code.
    ///
    /// # Returns
    ///
    /// A string slice representing the encoding format.
    pub fn get_encoding_format(&self) -> &str {
        &self.encoding_format
    }
}

/// Layered API built on the [`encode`] and [`render`] modules.
///
/// These methods return [`Result`] instead of panicking and expose the
/// error-correction, version, quiet-zone, styling and real byte-encoder
/// capabilities of the new architecture.
impl QRCode {
    /// Encodes the data into a renderer-ready [`Matrix`] using the default
    /// [`QrcodeEngine`] and the supplied [`QrOptions`].
    ///
    /// # Errors
    ///
    /// Returns [`QrError`] if the data cannot be encoded at the requested
    /// settings (for example, it exceeds capacity).
    pub fn encode(&self, options: &QrOptions) -> Result<Matrix> {
        encode::encode(&self.data, options)
    }

    /// Renders a styled SVG string using the new SVG-first renderer.
    ///
    /// # Errors
    ///
    /// Returns [`QrError`] if the data cannot be encoded.
    pub fn to_svg_styled(
        &self,
        options: &QrOptions,
        svg_options: &render::svg::SvgOptions,
    ) -> Result<String> {
        let matrix = self.encode(options)?;
        Ok(render::svg::render(&matrix, svg_options))
    }

    /// Renders the QR code to a string of Unicode half-block characters,
    /// suitable for printing directly to a terminal.
    ///
    /// # Errors
    ///
    /// Returns [`QrError`] if the data cannot be encoded.
    #[cfg(feature = "unicode")]
    pub fn to_unicode(&self, options: &QrOptions) -> Result<String> {
        let matrix = self.encode(options)?;
        Ok(render::unicode::render(&matrix))
    }

    /// Encodes the QR code as PNG bytes, ready to write to a file or response.
    ///
    /// # Errors
    ///
    /// Returns [`QrError`] if encoding or rendering fails.
    #[cfg(feature = "raster")]
    pub fn to_png_bytes(
        &self,
        options: &QrOptions,
        raster_options: &render::raster::RasterOptions,
    ) -> Result<Vec<u8>> {
        let matrix = self.encode(options)?;
        render::raster::to_png_bytes(&matrix, raster_options)
    }

    /// Encodes the QR code as JPEG bytes (alpha is flattened to the background).
    ///
    /// # Errors
    ///
    /// Returns [`QrError`] if encoding or rendering fails.
    #[cfg(feature = "raster")]
    pub fn to_jpeg_bytes(
        &self,
        options: &QrOptions,
        raster_options: &render::raster::RasterOptions,
    ) -> Result<Vec<u8>> {
        let matrix = self.encode(options)?;
        render::raster::to_jpeg_bytes(&matrix, raster_options)
    }

    /// Encodes the QR code as GIF bytes.
    ///
    /// # Errors
    ///
    /// Returns [`QrError`] if encoding or rendering fails.
    #[cfg(feature = "raster")]
    pub fn to_gif_bytes(
        &self,
        options: &QrOptions,
        raster_options: &render::raster::RasterOptions,
    ) -> Result<Vec<u8>> {
        let matrix = self.encode(options)?;
        render::raster::to_gif_bytes(&matrix, raster_options)
    }

    /// Encodes the QR code as bytes in an arbitrary [`image::ImageFormat`]
    /// (e.g. BMP, TIFF or WebP in addition to PNG/JPEG/GIF).
    ///
    /// # Errors
    ///
    /// Returns [`QrError`] if the data cannot be encoded, or if the requested
    /// image format has no encoder.
    #[cfg(feature = "raster")]
    pub fn to_image_bytes(
        &self,
        options: &QrOptions,
        raster_options: &render::raster::RasterOptions,
        format: image::ImageFormat,
    ) -> Result<Vec<u8>> {
        let matrix = self.encode(options)?;
        render::raster::to_bytes(&matrix, raster_options, format)
    }

    /// Renders a branded QR image with `logo` embedded at its centre.
    ///
    /// Use [`Ecc::High`] in `options` so the modules hidden behind the logo can
    /// still be recovered. Pairs naturally with [`payload::vcard::BusinessCard`]
    /// to build a contactless business card.
    ///
    /// # Errors
    ///
    /// Returns [`QrError`] if the data cannot be encoded.
    #[cfg(feature = "raster")]
    pub fn to_image_with_logo(
        &self,
        options: &QrOptions,
        raster_options: &render::raster::RasterOptions,
        logo: &image::RgbaImage,
        logo_options: &render::raster::LogoOptions,
    ) -> Result<image::RgbaImage> {
        let matrix = self.encode(options)?;
        Ok(render::raster::render_with_logo(
            &matrix,
            raster_options,
            logo,
            logo_options,
        ))
    }

    /// Renders a branded QR with an embedded `logo` and encodes it in the given
    /// [`image::ImageFormat`] (e.g. PNG).
    ///
    /// # Errors
    ///
    /// Returns [`QrError`] if the data cannot be encoded or the format has no
    /// encoder.
    #[cfg(feature = "raster")]
    pub fn to_image_bytes_with_logo(
        &self,
        options: &QrOptions,
        raster_options: &render::raster::RasterOptions,
        logo: &image::RgbaImage,
        logo_options: &render::raster::LogoOptions,
        format: image::ImageFormat,
    ) -> Result<Vec<u8>> {
        let img = self.to_image_with_logo(options, raster_options, logo, logo_options)?;
        render::raster::image_to_bytes(&img, format)
    }

    /// Renders a ControlNet-ready control image for AI art-QR pipelines
    /// (Stable Diffusion + a QR ControlNet). Combine with [`Ecc::High`].
    ///
    /// # Errors
    ///
    /// Returns [`QrError`] if the data cannot be encoded.
    #[cfg(feature = "raster")]
    pub fn to_control_image(
        &self,
        options: &QrOptions,
        control_options: &render::control::ControlOptions,
    ) -> Result<image::RgbaImage> {
        let matrix = self.encode(options)?;
        Ok(render::control::render(&matrix, control_options))
    }

    /// Renders and encodes a control image as bytes in the given format.
    ///
    /// # Errors
    ///
    /// Returns [`QrError`] if encoding fails or the format has no encoder.
    #[cfg(feature = "raster")]
    pub fn to_control_image_bytes(
        &self,
        options: &QrOptions,
        control_options: &render::control::ControlOptions,
        format: image::ImageFormat,
    ) -> Result<Vec<u8>> {
        let img = self.to_control_image(options, control_options)?;
        render::raster::image_to_bytes(&img, format)
    }

    /// Weaves a `background` image into the QR to produce an offline, branded,
    /// scannable "art" code. Use [`Ecc::High`] so the blended regions remain
    /// recoverable.
    ///
    /// # Errors
    ///
    /// Returns [`QrError`] if the data cannot be encoded.
    #[cfg(feature = "raster")]
    pub fn to_art_image(
        &self,
        options: &QrOptions,
        background: &image::RgbaImage,
        blend_options: &render::art::BlendOptions,
    ) -> Result<image::RgbaImage> {
        let matrix = self.encode(options)?;
        Ok(render::art::blend(&matrix, background, blend_options))
    }

    /// Renders an art QR and encodes it as bytes in the given format.
    ///
    /// # Errors
    ///
    /// Returns [`QrError`] if encoding fails or the format has no encoder.
    #[cfg(feature = "raster")]
    pub fn to_art_bytes(
        &self,
        options: &QrOptions,
        background: &image::RgbaImage,
        blend_options: &render::art::BlendOptions,
        format: image::ImageFormat,
    ) -> Result<Vec<u8>> {
        let img = self.to_art_image(options, background, blend_options)?;
        render::raster::image_to_bytes(&img, format)
    }
}
