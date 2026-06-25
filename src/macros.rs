// Copyright © 2022-2026 QRC. All rights reserved.
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

#[macro_export]
/// Macro to add a watermark image to a QR code.
///
/// # Parameters
/// * `$img` - The main QR code image as a mutable reference.
/// * `$watermark` - The watermark image as an immutable reference.
///
/// # Example
/// ```
/// use qrc::QRCode;
/// use image::{ImageBuffer, Rgba};
///
/// // Create a mock QR code and watermark image for the example
/// let mut img = ImageBuffer::from_pixel(100, 100, Rgba([0, 0, 0, 255]));
/// let watermark = ImageBuffer::from_pixel(50, 50, Rgba([255, 255, 255, 255]));
///
/// qrc::add_image_watermark!(&mut img, &watermark);
/// ```
macro_rules! add_image_watermark {
    ($img:expr, $watermark:expr) => {
        QRCode::add_image_watermark($img, $watermark)
    };
}

#[macro_export]
/// Macro to create a new QR code from the given data.
///
/// # Parameters
/// * `$data` - The data to be encoded in the QR code.
///
/// # Example
/// ```
/// use qrc::{QRCode, qr_code};
/// qr_code!("Hello, world!".into());
/// ```
macro_rules! qr_code {
    ($data:expr) => {
        QRCode::new($data)
    };
}

#[macro_export]
/// Macro to create a QR code with a specific error correction level.
///
/// # Parameters
/// * `$data` - The data to be encoded in the QR code.
/// * `$ec` - The error correction level (`EcLevel`).
///
/// # Example
/// ```
/// use qrc::{QRCode, EcLevel, qr_code_with_ec};
/// let qr = qr_code_with_ec!("Hello".into(), EcLevel::H);
/// assert_eq!(qr.ec_level, EcLevel::H);
/// ```
macro_rules! qr_code_with_ec {
    ($data:expr, $ec:expr) => {
        QRCode::new($data).with_ec_level($ec)
    };
}

#[macro_export]
/// Macro to create a QR code in a specified format with a given width.
///
/// All arms return `Vec<u8>` containing the encoded image bytes.
///
/// # Parameters
/// * `$data` - The data to be encoded in the QR code.
/// * `$format` - The format of the QR code image (e.g., "png", "jpg", "gif").
/// * `$width` - The width of the QR code image.
///
/// # Example
/// ```
/// use qrc::{QRCode, qr_code_to};
/// let bytes = qr_code_to!("Hello, world!".into(), "png", 256);
/// assert!(!bytes.is_empty());
/// ```
macro_rules! qr_code_to {
    ($data:expr, $format:expr, $width:expr) => {
        match $format {
            "png" => QRCode::from_bytes($data).to_png_bytes($width),
            "jpg" => QRCode::from_bytes($data).to_jpg($width),
            "gif" => QRCode::from_bytes($data).to_gif($width),
            _ => panic!("Invalid format"),
        }
    };
}

#[macro_export]
/// Sets the size of the QR code.
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
/// # Parameters
/// - `$qr_code:expr`: An instance of `QRCode`.
/// - `$format:expr`: The encoding format for the QR code data.
///
/// # Example
/// ```
/// use qrc::{QRCode, set_encoding_format};
///
/// let qr_code = QRCode::new("some data".as_bytes().to_vec());
/// let qr_with_format = set_encoding_format!(qr_code, "utf-8");
/// ```
macro_rules! set_encoding_format {
    ($qr_code:expr, $format:expr) => {
        $qr_code.set_encoding_format($format)
    };
}

#[macro_export]
/// Overlays an image (e.g., a logo) at the center of the QR code.
///
/// # Parameters
/// - `$qr_code:expr`: `QRCode` instance to which the image will be overlaid.
/// - `$image_path:expr`: Path to the image file to overlay.
///
/// # Example
/// ```
/// use qrc::{QRCode, overlay_image};
/// use image::{RgbaImage, ImageBuffer, Rgba};
///
/// let qr_code = QRCode::new("some data".as_bytes().to_vec());
/// let logo = ImageBuffer::from_pixel(10, 10, Rgba([255, 0, 0, 255]));
///
/// let qr_with_logo = overlay_image!(qr_code, &logo);
/// ```
macro_rules! overlay_image {
    ($qr_code:expr, $image_path:expr) => {
        $qr_code.overlay_image($image_path)
    };
}

#[macro_export]
/// Generates multiple QR codes in one operation.
///
/// # Parameters
/// - `$data_list:expr`: A vector of data strings for which QR codes are to be generated.
///
/// # Example
/// ```
/// use qrc::QRCode;
/// use qrc::batch_generate_qr;
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
/// # Parameters
/// - `$data:expr`: The data to be compressed and encoded.
///
/// # Example
/// ```
/// use qrc::QRCode;
/// use qrc::compress_data_macro;
/// let compressed_data = compress_data_macro!("Some large string of data");
/// ```
macro_rules! compress_data_macro {
    ($data:expr) => {
        QRCode::compress_data($data)
    };
}

#[macro_export]
/// Combines multiple QR codes into a single QR code.
///
/// # Parameters
/// - An array of `QRCode` instances to combine.
///
/// # Example
/// ```
/// use qrc::QRCode;
/// use qrc::combine_qr_codes;
///
/// let qr_code1 = QRCode::from_string("Data 1".to_string());
/// let qr_code2 = QRCode::from_string("Data 2".to_string());
/// let qr_code3 = QRCode::from_string("Data 3".to_string());
///
/// let combined_qr_code = combine_qr_codes!([qr_code1, qr_code2, qr_code3]);
/// ```
macro_rules! combine_qr_codes {
    ($codes:expr) => {
        QRCode::combine_qr_codes(&$codes)
    };
}

#[macro_export]
/// Generates a dynamic QR code, which can be updated after creation.
///
/// # Parameters
/// - `$initial_data:expr`: The initial data for the QR code.
///
/// # Example
/// ```
/// use qrc::QRCode;
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
/// Use the `$lang_pref; ...` form to specify a preferred language, or omit it
/// to default to `"en"`.
///
/// # Examples
///
/// ```
/// use qrc::QRCode;
/// use qrc::create_multilanguage_qr;
///
/// // Default (uses "en"):
/// let qr = create_multilanguage_qr! {
///     "en" => "Hello",
///     "es" => "Hola",
/// };
///
/// // Explicit language preference:
/// let qr = create_multilanguage_qr! {
///     "es";
///     "en" => "Hello",
///     "es" => "Hola",
/// };
/// ```
macro_rules! create_multilanguage_qr {
    // Arm with explicit language preference
    ($lang_pref:expr; $($lang:expr => $text:expr),* $(,)?) => {{
        use std::collections::HashMap;
        let mut data_map: HashMap<String, String> = HashMap::new();
        $(
            data_map.insert($lang.to_string(), $text.to_string());
        )*
        QRCode::create_multilanguage(&data_map, $lang_pref)
    }};
    // Default arm — falls back to "en"
    ($($lang:expr => $text:expr),* $(,)?) => {{
        use std::collections::HashMap;
        let mut data_map: HashMap<String, String> = HashMap::new();
        $(
            data_map.insert($lang.to_string(), $text.to_string());
        )*
        QRCode::create_multilanguage(&data_map, "en")
    }};
}
