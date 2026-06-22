//! SVG-first rendering: module shapes, custom colors, translucency, module
//! size, and the `Color` helpers.
//!
//! Run with: `cargo run --example svg_styling`

use qrc::encode::QrOptions;
use qrc::render::style::{Color, ModuleShape};
use qrc::render::svg::SvgOptions;
use qrc::QRCode;

fn main() {
    let qr = QRCode::from_string("https://example.com/styled".to_string());
    let opts = QrOptions::new();

    // Each module shape, with a branded foreground color.
    for shape in [
        ModuleShape::Square,
        ModuleShape::Rounded,
        ModuleShape::Circle,
    ] {
        let svg = qr
            .to_svg_styled(
                &opts,
                &SvgOptions {
                    module_size: 10,
                    dark: Color::rgb(0x1E, 0x90, 0xFF),
                    light: Color::WHITE,
                    shape,
                },
            )
            .unwrap();
        println!("{shape:?}: {} bytes of SVG", svg.len());
    }

    // Translucent dark modules add a `fill-opacity` attribute.
    let translucent = qr
        .to_svg_styled(
            &opts,
            &SvgOptions {
                dark: Color::rgba(0, 0, 0, 128),
                ..SvgOptions::with_module_size(8)
            },
        )
        .unwrap();
    println!(
        "translucent SVG has opacity attr: {}",
        translucent.contains("fill-opacity=")
    );

    // The free renderer function, driven from a `Matrix` directly.
    let matrix = qr.encode(&opts).unwrap();
    let free = qrc::render::svg::render(&matrix, &SvgOptions::default());
    println!("free svg::render produced {} bytes", free.len());

    // The legacy quiet-zoned SVG renderer (qrcode-backed).
    let legacy = qr.to_svg(256);
    println!("legacy to_svg produced {} bytes", legacy.len());

    // Color helpers.
    let c = Color::rgb(0x11, 0x22, 0x33);
    println!(
        "color hex {}, rgba {:?}, opacity {:.2}",
        c.to_hex(),
        c.to_array(),
        c.opacity()
    );
    println!("defaults: dark={:?} light={:?}", Color::BLACK, Color::WHITE);
}
