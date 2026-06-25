//! Offline "art QR": blend an image through the code — no AI model needed.
//!
//! A background (a procedural gradient here, or a file you pass in) shows
//! through the data modules while finder patterns, the quiet zone, and a centre
//! dot per module keep it scannable. High error correction keeps the blended
//! regions recoverable.
//!
//!   cargo run --example art_qr                 # procedural gradient background
//!   cargo run --example art_qr -- photo.png    # your own background

use image::{ImageBuffer, Rgba, RgbaImage};
use qrc::{BlendOptions, EcLevel, QRCode};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let qr = QRCode::from_string("https://docs.rs/qrc".to_string()).with_ec_level(EcLevel::H);

    // A real background if supplied, else a diagonal blue→magenta gradient.
    let background: RgbaImage = match std::env::args().nth(1) {
        Some(path) => image::open(path)?.to_rgba8(),
        None => gradient(512),
    };

    // strength: higher = scans better; lower = more image shows through.
    let opts = BlendOptions {
        strength: 0.7,
        ..BlendOptions::default()
    };
    let art = qr.blend_image(&background, &opts);
    art.save("art_qr.png")?;

    println!(
        "Wrote art_qr.png ({}x{}) — a scannable, image-blended QR code.",
        art.width(),
        art.height()
    );
    Ok(())
}

/// A simple diagonal gradient used when no background image is supplied.
fn gradient(size: u32) -> RgbaImage {
    ImageBuffer::from_fn(size, size, |x, y| {
        let t = (x + y) as f32 / (2.0 * size as f32); // 0.0 → 1.0 across the diagonal
        let r = (t * 220.0) as u8;
        let b = (220.0 - t * 80.0) as u8;
        Rgba([r, 40, b, 255])
    })
}
