//! Element: `Color` and `ModuleShape` styling primitives.
//!
//! Run: `cargo run --example color`

use qrc::render::style::{Color, ModuleShape};

fn main() {
    // Constructors.
    let black = Color::BLACK;
    let white = Color::WHITE;
    let brand = Color::rgb(0x1E, 0x90, 0xFF);
    let translucent = Color::rgba(0, 0, 0, 128);

    // Accessors.
    println!("brand rgba array: {:?}", brand.to_array());
    println!("brand hex: {}", brand.to_hex());
    println!("translucent opacity: {:.3}", translucent.opacity());
    println!(
        "black opacity: {:.3}, white hex: {}",
        black.opacity(),
        white.to_hex()
    );

    // Default color is opaque black.
    assert_eq!(Color::default(), Color::BLACK);

    // Module shapes available to the SVG renderer.
    for shape in [
        ModuleShape::Square,
        ModuleShape::Rounded,
        ModuleShape::Circle,
    ] {
        println!("module shape: {shape:?}");
    }
    assert_eq!(ModuleShape::default(), ModuleShape::Square);
}
