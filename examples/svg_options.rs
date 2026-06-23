//! Element: SVG rendering — `SvgOptions`, `to_svg_styled`, the free renderer,
//! and the legacy `to_svg`.
//!
//! Run: `cargo run --example svg_options`

use qrc::encode::QrOptions;
use qrc::render::style::{Color, ModuleShape};
use qrc::render::svg::{self, SvgOptions};
use qrc::QRCode;

fn main() {
    let qr = QRCode::from_string("https://example.com".to_string());
    let opts = QrOptions::new();

    // Each module shape with a branded color.
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
                    dark: Color::rgb(0x10, 0x10, 0x2A),
                    light: Color::WHITE,
                    shape,
                },
            )
            .unwrap();
        println!("{shape:?}: {} bytes", svg.len());
    }

    // Translucent dark adds a fill-opacity attribute.
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
        "translucent contains opacity: {}",
        translucent.contains("fill-opacity=")
    );

    // The free renderer works directly from a Matrix.
    let matrix = qr.encode(&opts).unwrap();
    println!(
        "free svg::render: {} bytes",
        svg::render(&matrix, &SvgOptions::default()).len()
    );

    // The legacy qrcode-backed SVG renderer.
    println!("legacy to_svg: {} bytes", qr.to_svg(256).len());
}
