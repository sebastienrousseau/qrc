//! # QRC Example
//!
//! This example demonstrates the usage of the `qrc` crate for generating and manipulating QR codes.

use image::{imageops, ImageBuffer, Rgba, RgbaImage};
use qrc::{add_image_watermark, qr_code, qr_code_to, QRCode};
use std::fs;

const URL: &str = "https://minifunctions.com/";

fn main() {
    // Create a QR code and convert it to a PNG representation
    let qrcode = QRCode::from_string(URL.to_string());
    let png = qrcode.to_png(512);
    let png_data = png.into_raw();
    let png_image = ImageBuffer::<Rgba<u8>, Vec<u8>>::from_raw(21, 21, png_data).unwrap();
    println!(
        "fn to_png():                ✅ {:?}",
        png_image.save("qrcode.png")
    );
    match png_image.save("qrcode.png") {
        Ok(()) => println!("png file created:           ✅ qrcode.png"),
        Err(e) => println!("png file created:           ❌ qrcode.png: {e}"),
    }
    match fs::remove_file("qrcode.png") {
        Ok(()) => println!("png file removed:           ✅ qrcode.png"),
        Err(e) => println!("png file removed:           ❌ qrcode.png: {e}"),
    }

    // Colorized QR code
    let qrcode = QRCode::from_string(URL.to_string());
    let red = Rgba([255, 0, 0, 255]);
    let red_qrcode = qrcode.colorize(red);
    let img: RgbaImage = red_qrcode;
    let resized_img = imageops::resize(&img, 512, 512, imageops::FilterType::Nearest);
    let image: ImageBuffer<Rgba<u8>, Vec<u8>> = resized_img;
    println!(
        "fn colorize():              ✅ {:?}",
        image.save("qrcode_colorized.png")
    );
    match image.save("qrcode_colorized.png") {
        Ok(()) => println!("colorized png file created: ✅ qrcode_colorized.png"),
        Err(e) => println!("colorized png file created: ❌ qrcode_colorized.png: {e}"),
    }
    match fs::remove_file("qrcode_colorized.png") {
        Ok(()) => println!("colorized png file removed: ✅ qrcode_colorized.png"),
        Err(e) => println!("colorized png file removed: ❌ qrcode_colorized.png: {e}"),
    }

    // SVG output
    let qrcode = QRCode::from_string(URL.to_string());
    let qrcode_svg = qrcode.to_svg(512);
    match fs::write("qrcode.svg", qrcode_svg) {
        Ok(()) => println!("svg file created:           ✅ qrcode.svg"),
        Err(e) => println!("svg file created:           ❌ qrcode.svg: {e}"),
    }
    match fs::remove_file("qrcode.svg") {
        Ok(()) => println!("svg file removed:           ✅ qrcode.svg"),
        Err(e) => println!("svg file removed:           ❌ qrcode.svg: {e}"),
    }

    // Resize
    let qrcode = QRCode::new(vec![0x61, 0x62, 0x63]);
    let resized_image: RgbaImage = qrcode.resize(512, 512);
    println!(
        "fn resize():                ✅ {:?}",
        resized_image.save("qrcode_resized.png")
    );
    match resized_image.save("qrcode_resized.png") {
        Ok(()) => println!("resized file created:       ✅ qrcode_resized.png"),
        Err(e) => println!("resized file created:       ❌ qrcode_resized.png: {e}"),
    }
    match fs::remove_file("qrcode_resized.png") {
        Ok(()) => println!("resized file removed:       ✅ qrcode_resized.png"),
        Err(e) => println!("resized file removed:       ❌ qrcode_resized.png: {e}"),
    }

    // Macro: qr_code! to SVG
    let qrcode = qr_code!(URL.into());
    let qrcode_svg = qrcode.to_svg(512);
    match fs::write("qrcode.svg", qrcode_svg) {
        Ok(()) => println!("svg file created:           ✅ qrcode.svg"),
        Err(e) => println!("svg file created:           ❌ qrcode.svg: {e}"),
    }
    match fs::remove_file("qrcode.svg") {
        Ok(()) => println!("svg file removed:           ✅ qrcode.svg"),
        Err(e) => println!("svg file removed:           ❌ qrcode.svg: {e}"),
    }

    // Macro: qr_code_to! PNG
    let qrcode = qr_code_to!(URL.into(), "png", 512);
    match qrcode.save("qrcode.png") {
        Ok(()) => println!("png file created:           ✅ qrcode.png"),
        Err(e) => println!("png file created:           ❌ qrcode.png: {e}"),
    }
    match fs::remove_file("qrcode.png") {
        Ok(()) => println!("png file removed:           ✅ qrcode.png"),
        Err(e) => println!("png file removed:           ❌ qrcode.png: {e}"),
    }

    // Macro: qr_code_to! GIF
    let qrcode = qr_code_to!(URL.into(), "gif", 512);
    match qrcode.save("qrcode.gif") {
        Ok(()) => println!("gif file created:           ✅ qrcode.gif"),
        Err(e) => println!("gif file created:           ❌ qrcode.gif: {e}"),
    }
    match fs::remove_file("qrcode.gif") {
        Ok(()) => println!("gif file removed:           ✅ qrcode.gif"),
        Err(e) => println!("gif file removed:           ❌ qrcode.gif: {e}"),
    }

    // Macro: qr_code_to! JPG
    let qrcode = qr_code_to!(URL.into(), "jpg", 512);
    match qrcode.save("qrcode.jpg") {
        Ok(()) => println!("jpg file created:           ✅ qrcode.jpg"),
        Err(e) => println!("jpg file created:           ❌ qrcode.jpg: {e}"),
    }
    match fs::remove_file("qrcode.jpg") {
        Ok(()) => println!("jpg file removed:           ✅ qrcode.jpg"),
        Err(e) => println!("jpg file removed:           ❌ qrcode.jpg: {e}"),
    }

    // Watermark
    let qrcode = QRCode::from_string(URL.to_string());
    let mut qrcode_img = qrcode.to_png(512);
    let watermark_img = image::open("bubba.ico").unwrap().into_rgba8();
    add_image_watermark!(&mut qrcode_img, &watermark_img);
    match qrcode_img.save("qrcode_watermarked.png") {
        Ok(()) => println!("png file with watermark:    ✅ qrcode_watermarked.png"),
        Err(e) => println!("png file with watermark:    ❌ qrcode_watermarked.png: {e}"),
    }
    match fs::remove_file("qrcode_watermarked.png") {
        Ok(()) => println!("watermark file removed:     ✅ qrcode_watermarked.png"),
        Err(e) => println!("watermark file removed:     ❌ qrcode_watermarked.png: {e}"),
    }
}
