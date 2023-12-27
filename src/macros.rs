// Copyright © 2022-2024 QR Code Library (QRC). All rights reserved.
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

// These lines declare the copyright and licensing for the QR Code Library (QRC).
// The library is protected by copyright law from 2022 to 2024.
// It is dual-licensed under the Apache License 2.0 and the MIT License.
// SPDX (Software Package Data Exchange) identifiers are used for clarity in licensing.

#[macro_export]
/// The `add_image_watermark` macro creates a new instance of the QRCode struct
/// with the given data.
// This attribute exports the macro so it can be used in other modules or crates.
macro_rules! add_image_watermark {
    // Defines the macro `add_image_watermark`.
    ($img:expr, $watermark:expr) => {
        // The macro takes two parameters: `$img` and `$watermark`.
        // `$img` is an expression representing the main image.
        // `$watermark` is an expression representing the watermark image.
        QRCode::add_image_watermark($img, $watermark)
        // Calls the `add_image_watermark` method on the `QRCode` struct,
        // passing the provided image and watermark.
    };
}

#[macro_export]
/// The `qr_code` macro creates a new instance of the QRCode struct
/// with the given data.
// This attribute exports the macro for external use.
macro_rules! qr_code {
    // Defines the macro `qr_code`.
    ($data:expr) => {
        // The macro takes one parameter: `$data`.
        // `$data` is an expression representing the data to encode in the QR code.
        QRCode::new($data)
        // Calls the `new` method on the `QRCode` struct with the provided data.
    };
}

#[macro_export]
/// Define a macro named `qr_code_to`
// Export the macro for use in other modules.
macro_rules! qr_code_to {
    // Define the macro `qr_code_to`.
    // This macro takes three expressions: `$data`, `$format`, and `$width`.
    ($data:expr, $format:expr, $width:expr) => {
        // Match the value of `$format`.
        match $format {
            // If `$format` is equal to "png", generate a PNG format QR code.
            "png" => QRCode::from_bytes($data).to_png($width),
            // If `$format` is equal to "jpg", generate a JPG format QR code.
            "jpg" => QRCode::from_bytes($data).to_jpg($width),
            // If `$format` is equal to "gif", generate a GIF format QR code.
            "gif" => QRCode::from_bytes($data).to_gif($width),
            // For any other value, cause a panic with the message "Invalid format".
            _ => panic!("Invalid format"),
            // The underscore `_` is a catch-all pattern; if `$format` doesn't match
            // any of the specified formats, this block will execute.
        }
    };
}

#[macro_export]
/// Sets the size of the QR code.
///
/// This macro allows the user to specify the size of the QR code.
/// The size is typically defined in terms of pixels or modules (the small squares that make up a QR code).
///
/// # Parameters
/// - `$qrcode:expr`: An instance of `QRCode`.
/// - `$size:expr`: The desired size for the QR code.
///
/// # Example
/// ```
/// use qrc::QRCode;
/// use qrc::resize;
///
/// let qrcode = QRCode::new("Hello, world!".as_bytes().to_vec());
/// let resized_qrcode = resize!(qrcode, 256);
/// ```
macro_rules! resize {
    ($qrcode:expr, $size:expr) => {
        $qrcode.resize($size, $size)
    };
}

#[macro_export]
/// Sets the encoding format for the data in a QR code.
///
/// QR codes can encode data in several formats, such as numeric, alphanumeric,
/// or binary. This macro allows setting the preferred encoding format for the data.
///
/// # Parameters
/// - `$qr_code:expr`: An instance of `QRCode`.
/// - `$format:expr`: The encoding format for the QR code data.
///
/// # Example
/// ```
/// use qrc::{QRCode, set_encoding_format}; // Import QRCode and the macro
///
/// let qr_code = QRCode::new("some data".as_bytes().to_vec()); // Create a QRCode instance
/// let qr_with_format = set_encoding_format!(qr_code, "utf-8"); // Use the macro to set the encoding format
/// ```
macro_rules! set_encoding_format {
    ($qr_code:expr, $format:expr) => {
        $qr_code.set_encoding_format($format)
    };
}

#[macro_export]
/// Overlays an image (e.g., a logo) at the center of the QR code.
///
/// This is particularly useful for branding purposes, allowing the inclusion
/// of a company logo within the QR code.
///
/// # Parameters
/// - `$qr_code:expr`: QRCode instance to which the image will be overlaid.
/// - `$image_path:expr`: Path to the image file to overlay.
///
/// # Example
/// ```
/// # // The following is a mock example, as actual file loading cannot be done in doctests
/// use qrc::{QRCode, overlay_image};
/// use image::{RgbaImage, ImageBuffer, Rgba};
///
/// let qr_code = QRCode::new("some data".as_bytes().to_vec());
/// // Mock an image (e.g., a small red square)
/// let logo = ImageBuffer::from_pixel(10, 10, Rgba([255, 0, 0, 255]));
///
/// let qr_with_logo = overlay_image!(qr_code, &logo); // Use the macro for overlaying the image
/// ```
macro_rules! overlay_image {
    ($qr_code:expr, $image_path:expr) => {
        $qr_code.overlay_image($image_path)
    };
}

#[macro_export]
/// Generates multiple QR codes in one operation.
///
/// This macro is useful for batch processing, such as generating QR codes
/// for a list of URLs or serial numbers.
///
/// # Parameters
/// - `$data_list:expr`: A vector of data strings for which QR codes are to be generated.
///
/// # Example
/// ```
/// use qrc::QRCode; // Import QRCode type
/// use qrc::batch_generate_qr; // Import the macro
/// let qr_codes = batch_generate_qr!(vec!["https://example.com".to_string(), "https://example2.com".to_string()]);
/// ```
macro_rules! batch_generate_qr {
    ($data_list:expr) => {
        QRCode::batch_generate_qr_codes($data_list)
    };
}

#[macro_export]
/// Compresses data before encoding it into a QR code.
///
/// This is beneficial for encoding large amounts of data into a QR code
/// by reducing the size of the data.
///
/// # Parameters
/// - `$data:expr`: The data to be compressed and encoded.
///
/// # Example
/// ```
/// use qrc::QRCode; // Import QRCode type
/// use qrc::compress_data_macro; // Corrected import to use the macro
/// let compressed_data = compress_data_macro!("Some large string of data"); // Correct usage
/// ```
macro_rules! compress_data_macro {
    ($data:expr) => {
        QRCode::compress_data($data)
    };
}

#[macro_export]
/// Combines multiple QR codes into a single QR code.
///
/// This macro is useful for scenarios where multiple QR codes need to be combined,
/// such as creating a composite QR code with several data sources.
///
/// # Parameters
/// - An array of QRCode instances to combine.
///
/// # Example
/// ```
/// use qrc::QRCode; // Import QRCode type
/// use qrc::combine_qr_codes;
///
/// let qr_code1 = QRCode::from_string("Data 1".to_string());
/// let qr_code2 = QRCode::from_string("Data 2".to_string());
/// let qr_code3 = QRCode::from_string("Data 3".to_string());
///
/// let combined_qr_code = combine_qr_codes!(vec![qr_code1, qr_code2, qr_code3]);
/// ```
macro_rules! combine_qr_codes {
    ($codes:expr) => {
        QRCode::combine_qr_codes($codes)
    };
}

#[macro_export]
/// Generates a dynamic QR code, which can be updated after creation.
///
/// Useful for scenarios where the data linked to the QR code might change
/// over time, such as promotional offers or event details.
///
/// # Parameters
/// - `$initial_data:expr`: The initial data for the QR code.
///
/// # Example
/// ```
/// use qrc::QRCode; // Import QRCode type
/// use qrc::create_dynamic_qr;
/// create_dynamic_qr!("Initial Data");
/// ```
macro_rules! create_dynamic_qr {
    ($initial_data:expr) => {
        QRCode::create_dynamic($initial_data)
    };
}

#[macro_export]
/// Generates QR codes with multi-language support.
///
/// The QR code displays different data based on the user's language preference.
///
/// # Parameters
/// - Pairs of language codes and corresponding data.
///
/// # Example
/// ```
/// use qrc::QRCode; // Import QRCode type
/// use qrc::create_multilanguage_qr;
/// create_multilanguage_qr! {
///     "en" => "Hello",
///     "es" => "Hola",
///     "fr" => "Bonjour"
/// };
/// ```
macro_rules! create_multilanguage_qr {
    ($($lang:expr => $text:expr),* $(,)?) => {{
        use std::collections::HashMap;
        let mut data_map: HashMap<String, String> = HashMap::new();

        $(
            data_map.insert($lang.to_string(), $text.to_string());
        )*

        QRCode::create_multilanguage(data_map)
    }};
}

