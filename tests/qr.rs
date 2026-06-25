#[cfg(test)]
mod tests {
    use image::{Rgba, RgbaImage};
    use qrc::{
        add_image_watermark, qr_code, qr_code_to, qr_code_with_ec, set_encoding_format, EcLevel,
        ModuleShape, QRCode,
    };

    const URL: &str = "https://minifunctions.com/";

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
        assert!(!qrcode_svg.is_empty());
        assert!(qrcode_svg.contains("svg"));
    }

    #[test]
    fn test_to_gif() {
        let qrcode = QRCode::from_string(URL.to_string());
        let qrcode_gif = qrcode.to_gif(512).unwrap();
        // GIF magic bytes: GIF89a or GIF87a
        assert!(qrcode_gif.len() > 6);
        assert_eq!(&qrcode_gif[..3], b"GIF");
    }

    #[test]
    fn test_to_jpg() {
        let qrcode = QRCode::from_string(URL.to_string());
        let qrcode_jpg = qrcode.to_jpg(512).unwrap();
        // JPEG magic bytes: FF D8 FF
        assert!(qrcode_jpg.len() > 3);
        assert_eq!(&qrcode_jpg[..2], &[0xFF, 0xD8]);
    }

    #[test]
    fn test_add_image_watermark() {
        let data = vec![0x61, 0x62, 0x63];
        let qrcode = QRCode::from_bytes(data.clone());
        assert_eq!(qrcode.data, data);

        let qrcode = QRCode::from_string(URL.to_string());
        let mut qrcode_img = qrcode.to_png(512);
        let watermark_img = image::open("tests/fixtures/bubba.ico")
            .unwrap()
            .into_rgba8();
        add_image_watermark!(&mut qrcode_img, &watermark_img);
        assert_eq!(qrcode_img.dimensions(), (512, 512));
    }

    #[test]
    fn test_colorize() {
        let qrcode = QRCode::new(vec![0, 1, 2, 3]);
        let red_qrcode = qrcode.colorize(Rgba([255, 0, 0, 255]));

        let image: RgbaImage = red_qrcode;
        for (x, y, pixel) in image.enumerate_pixels() {
            let expected_color =
                if qrcode.to_qrcode()[(x as usize, y as usize)] == qrcode::Color::Dark {
                    Rgba([255, 0, 0, 255])
                } else {
                    Rgba([255, 255, 255, 255])
                };
            assert_eq!(*pixel, expected_color);
        }
    }

    #[test]
    fn test_resize() {
        let qrcode = QRCode::new(vec![0, 1, 2, 3]);
        let resized_qrcode = qrcode.resize(42, 42);

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
        let result = qr_code_to!(data.clone(), "png", 512).unwrap();
        // qr_code_to! now returns Result<Vec<u8>, _> (PNG-encoded bytes)
        assert!(!result.is_empty());
        // Verify PNG magic bytes
        assert_eq!(&result[..4], &[0x89, 0x50, 0x4E, 0x47]);
    }

    #[test]
    #[should_panic(expected = "Invalid format")]
    fn test_qr_code_from_invalid_format() {
        let data = vec![0u8, 1, 2, 3];
        let _result = qr_code_to!(data, "jpeg", 512);
    }

    #[test]
    fn test_empty_string() {
        let data = String::new();
        let qrcode = QRCode::from_string(data.clone());
        assert_eq!(qrcode.data, data.into_bytes());
    }

    #[test]
    fn test_set_encoding_format() {
        let qrcode = QRCode::new(b"some data".to_vec());
        let qr_with_format = set_encoding_format!(qrcode, "utf-8").unwrap();
        assert_eq!(qr_with_format.get_encoding_format(), "utf-8");
    }

    // ── EC level tests ──────────────────────────────────────────────────

    #[test]
    fn test_default_ec_level() {
        let qr = QRCode::new(b"test".to_vec());
        assert_eq!(qr.ec_level, EcLevel::M);
    }

    #[test]
    fn test_with_ec_level() {
        let qr = QRCode::from_string("test".to_string()).with_ec_level(EcLevel::H);
        assert_eq!(qr.ec_level, EcLevel::H);
    }

    #[test]
    fn test_ec_level_affects_output() {
        let data = "Hello, world!".to_string();
        let svg_m = QRCode::from_string(data.clone())
            .with_ec_level(EcLevel::L)
            .to_svg(256);
        let svg_h = QRCode::from_string(data)
            .with_ec_level(EcLevel::H)
            .to_svg(256);
        // Higher EC produces a larger (more modules) QR code
        assert_ne!(svg_m, svg_h);
    }

    #[test]
    fn test_qr_code_with_ec_macro() {
        let qr = qr_code_with_ec!(b"macro test".to_vec(), EcLevel::Q);
        assert_eq!(qr.ec_level, EcLevel::Q);
    }

    // ── Fallible API tests ──────────────────────────────────────────────

    #[test]
    fn test_try_to_qrcode_success() {
        let qr = QRCode::from_string("Hello".to_string());
        assert!(qr.try_to_qrcode().is_ok());
    }

    #[test]
    fn test_try_to_qrcode_too_long() {
        // QR codes can hold at most ~2953 bytes at EC level L; 7089 numeric chars.
        // Feeding 3000 bytes at EC H should fail.
        let qr = QRCode::from_bytes(vec![0u8; 3000]).with_ec_level(EcLevel::H);
        assert!(qr.try_to_qrcode().is_err());
    }

    // ── Multilanguage tests ─────────────────────────────────────────────

    #[test]
    fn test_multilanguage_selects_requested() {
        use std::collections::HashMap;
        let mut map = HashMap::new();
        map.insert("en".to_string(), "Hello".to_string());
        map.insert("es".to_string(), "Hola".to_string());
        let qr = QRCode::create_multilanguage(&map, "es");
        assert_eq!(String::from_utf8_lossy(&qr.data), "Hola");
    }

    #[test]
    fn test_multilanguage_fallback_en() {
        use std::collections::HashMap;
        let mut map = HashMap::new();
        map.insert("en".to_string(), "Hello".to_string());
        map.insert("es".to_string(), "Hola".to_string());
        let qr = QRCode::create_multilanguage(&map, "fr");
        assert_eq!(String::from_utf8_lossy(&qr.data), "Hello");
    }

    #[test]
    fn test_multilanguage_fallback_first() {
        use std::collections::HashMap;
        let mut map = HashMap::new();
        map.insert("ja".to_string(), "Konnichiwa".to_string());
        let qr = QRCode::create_multilanguage(&map, "fr");
        assert_eq!(String::from_utf8_lossy(&qr.data), "Konnichiwa");
    }

    // ── Format magic byte tests ─────────────────────────────────────────

    #[test]
    fn test_jpg_magic_bytes() {
        let qr = QRCode::from_string("test".to_string());
        let jpg = qr.to_jpg(64).unwrap();
        assert_eq!(&jpg[..2], &[0xFF, 0xD8]);
    }

    #[test]
    fn test_gif_magic_bytes() {
        let qr = QRCode::from_string("test".to_string());
        let gif = qr.to_gif(64).unwrap();
        assert_eq!(&gif[..3], b"GIF");
    }

    #[test]
    fn test_png_bytes_magic() {
        let qr = QRCode::from_string("test".to_string());
        let png = qr.to_png_bytes(64).unwrap();
        assert_eq!(&png[..4], &[0x89, 0x50, 0x4E, 0x47]);
    }

    // ── Shape tests ─────────────────────────────────────────────────────

    #[test]
    fn test_shape_circle() {
        let qr = QRCode::from_string("shapes".to_string()).with_shape(ModuleShape::Circle);
        let img = qr.to_png(128);
        assert_eq!(img.dimensions(), (128, 128));
    }

    #[test]
    fn test_shape_svg_rounded() {
        let qr = QRCode::from_string("shapes".to_string()).with_shape(ModuleShape::RoundedSquare);
        let svg = qr.to_svg(256);
        assert!(svg.contains("rx="));
        assert!(svg.contains("ry="));
    }
}
