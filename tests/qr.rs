#[cfg(test)]
mod tests {
    extern crate image;
    use image::{Rgba, RgbaImage};

    extern crate qrc;
    use qrc::{add_image_watermark, qr_code, qr_code_to, set_encoding_format, QRCode};

    const URL: &str = "https://minifunctions.com/"; // Define a constant for the URL to be encoded

    #[test]
    fn test_new() {
        let data = vec![0x61, 0x62, 0x63];
        let qrcode = QRCode::new(data.clone());
        assert_eq!(qrcode.data, data);
    }

    #[test]
    fn test_from_string() {
        let data = "abc".to_string();
        let qrcode = QRCode::from_string(data.clone());
        assert_eq!(qrcode.data, data.into_bytes());
    }

    #[test]
    fn test_from_bytes() {
        let data = vec![0x61, 0x62, 0x63];
        let qrcode = QRCode::from_bytes(data.clone());
        assert_eq!(qrcode.data, data);
    }

    #[test]
    fn test_to_qrcode() {
        let data = vec![0x61, 0x62, 0x63];
        let qrcode = QRCode::from_bytes(data.clone());
        assert_eq!(qrcode.data, data);
    }

    #[test]
    fn test_to_png() {
        let data = vec![0x61, 0x62, 0x63];
        let qrcode = QRCode::from_bytes(data.clone());
        assert_eq!(qrcode.data, data);

        let qrcode = QRCode::from_string("Hello, world!".to_string());
        let png = qrcode.to_png(21);
        assert_eq!(png.dimensions(), (21, 21));

        let png_data = png.into_raw();
        assert_eq!(png_data.len(), 1764);
    }
    #[test]
    fn test_to_svg() {
        let data = vec![0x61, 0x62, 0x63];
        let qrcode = QRCode::from_bytes(data.clone());
        assert_eq!(qrcode.data, data);

        let qrcode = QRCode::from_string(URL.to_string());
        let qrcode_svg = qrcode.to_svg(512);
        assert_eq!(qrcode_svg.len(), 6918);
    }
    #[test]
    fn test_to_gif() {
        let data = vec![0x61, 0x62, 0x63];
        let qrcode = QRCode::from_bytes(data.clone());
        assert_eq!(qrcode.data, data);

        let qrcode = QRCode::from_string(URL.to_string());
        let qrcode_gif = qrcode.to_gif(512);
        assert_eq!(qrcode_gif.len(), 1048576);
    }
    #[test]
    fn test_to_jpg() {
        let data = vec![0x61, 0x62, 0x63];
        let qrcode = QRCode::from_bytes(data.clone());
        assert_eq!(qrcode.data, data);

        let qrcode = QRCode::from_string(URL.to_string());
        let qrcode_jpg = qrcode.to_jpg(512);
        assert_eq!(qrcode_jpg.len(), 1048576);
    }
    #[test]
    fn test_add_image_watermark() {
        let data = vec![0x61, 0x62, 0x63];
        let qrcode = QRCode::from_bytes(data.clone());
        assert_eq!(qrcode.data, data);

        let qrcode = QRCode::from_string(URL.to_string());
        let mut qrcode_img = qrcode.to_png(512);
        let watermark_img = image::open("bubba.ico").unwrap().into_rgba8();
        add_image_watermark!(&mut qrcode_img, &watermark_img);
        assert_eq!(qrcode_img.dimensions(), (512, 512));
    }
    #[test]
    fn test_colorize() {
        // Create a new QR code with some data.
        let qrcode = QRCode::new(vec![0, 1, 2, 3]);

        // Colorize the QR code with a red color.
        let red_qrcode = qrcode.colorize(Rgba([255, 0, 0, 255]));

        // Convert the QR code to a PNG image and assert that all of the dark cells are red.
        let image: RgbaImage = red_qrcode;
        for (x, y, pixel) in image.enumerate_pixels() {
            let expected_color = if qrcode.to_qrcode()[(x as usize, y as usize)]
                == qrcode::Color::Dark
            {
                Rgba([255, 0, 0, 255])
            } else {
                Rgba([255, 255, 255, 255])
            };
            assert_eq!(*pixel, expected_color);
        }
    }
    #[test]
    fn test_resize() {
        // Create a new QR code with some data.
        let qrcode = QRCode::new(vec![0, 1, 2, 3]);

        // Resize the QR code to 42x42 pixels.
        let resized_qrcode = qrcode.resize(42, 42);

        // Convert the QR code to a PNG image and assert that the dimensions are correct.
        let image: RgbaImage = resized_qrcode;
        assert_eq!(image.dimensions(), (42, 42));
    }

    #[test]
    fn test_qr_code() {
        let data = vec![0x61, 0x62, 0x63];
        let qrcode = qr_code!(data.clone());
        assert_eq!(qrcode.data, data);
    }
    #[test]
    fn test_qr_code_from_png() {
        let data = vec![0x61, 0x62, 0x63];
        let result = qr_code_to!(data.clone(), "png", 512);
        let expected = QRCode::from_bytes(data).to_png(512);
        assert_eq!(result, expected);
    }
    #[test]
    #[should_panic(expected = "Invalid format")]
    fn test_qr_code_from_invalid_format() {
        let data = vec![0u8, 1, 2, 3];
        let _result = qr_code_to!(data, "jpeg", 512);
    }

    #[test]
    fn test_empty_string() {
        let data = "".to_string();
        let qrcode = QRCode::from_string(data.clone());
        assert_eq!(qrcode.data, data.into_bytes());
    }
    #[test]
    fn test_set_encoding_format() {
        let qrcode = QRCode::new("some data".as_bytes().to_vec());
        let qr_with_format = set_encoding_format!(qrcode, "utf-8").unwrap(); // Unwrap the Result
        assert_eq!(qr_with_format.get_encoding_format(), "utf-8");
    }

    // #[test]
    // fn test_overlay_image() {
    //     let qr_code = QRCode::new("some data".as_bytes().to_vec());
    //     let logo = ImageBuffer::from_pixel(10, 10, Rgba([255, 0, 0, 255]));
    //     let qr_with_logo = overlay_image!(qr_code, &logo);

    //     // Perform an appropriate assertion, such as checking the dimensions of the resulting image
    //     assert_eq!(qr_with_logo.dimensions(), (qr_code.width() as u32, qr_code.width() as u32));
    // }

    // #[test]
    // fn test_batch_generate_qr() {
    //     let qr_codes = batch_generate_qr!(vec!["https://example.com".to_string(), "https://example2.com".to_string()]);
    //     // Assert the number of QR codes generated matches the input vector
    //     assert_eq!(qr_codes.len(), 2);
    // }

    // #[test]
    // fn test_create_dynamic_qr() {
    //     let qr_code = create_dynamic_qr!("Initial Data");
    //     // Perform an appropriate assertion based on your implementation of dynamic QR codes
    // }

    // #[test]
    // fn test_create_multilanguage_qr() {
    //     let qr_code = create_multilanguage_qr! {
    //         "en" => "Hello",
    //         "es" => "Hola",
    //         "fr" => "Bonjour"
    //     };
    //     // Perform an appropriate assertion, such as checking if the correct language data is set
    // }
}
