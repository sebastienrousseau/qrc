//! Branded business-card QR: a vCard contact code with a logo centred in a
//! clean white knockout — the kind of code you put on a business card.
//!
//! Scannability is preserved by construction:
//!   * highest error correction (`EcLevel::H`, ~30% recovery) absorbs the logo,
//!   * a 4-module quiet zone is added (the raster renderer omits it), and
//!   * the knockout stays small (~8% of the code area).
//!
//! Run with your own logo (PNG/JPG/GIF):
//!   cargo run --example business_card -- path/to/logo.png
//! …or with no argument to use a placeholder mark.

use image::{imageops, ImageBuffer, Rgba, RgbaImage};
use qrc::payload::vcard::BusinessCard;
use qrc::{EcLevel, QRCode};

const MODULE_PX: u32 = 12; // pixels per QR module
const QUIET_MODULES: u32 = 4; // mandatory quiet-zone width
const INK: Rgba<u8> = Rgba([0x1d, 0x1d, 0x1f, 255]); // brand ink for the placeholder

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1 — The contact card. Scanners offer "Add to Contacts" for a vCard.
    let card = BusinessCard::new("Jane Doe")
        .organization("Acme, Inc.")
        .title("Principal Engineer")
        .email("jane@acme.example")
        .url("https://acme.example");

    // 2 — Encode at the highest error-correction level so the centred logo
    //     doesn't push the code below the decode threshold.
    let qr = QRCode::from_string(card.to_vcard()).with_ec_level(EcLevel::H);
    let modules = qr.try_to_qrcode()?.width() as u32; // QR side, in modules
    let core = qr.to_png(modules * MODULE_PX); // crisp, but no quiet zone

    // 3 — Paste the bare code onto a white canvas with a 4-module quiet zone.
    let quiet = QUIET_MODULES * MODULE_PX;
    let dim = modules * MODULE_PX + 2 * quiet;
    let mut img: RgbaImage = ImageBuffer::from_pixel(dim, dim, Rgba([255, 255, 255, 255]));
    imageops::overlay(&mut img, &core, quiet.into(), quiet.into());

    // 4 — White circular knockout at the centre.
    let (cx, cy) = (dim as i32 / 2, dim as i32 / 2);
    let kr = (dim as f32 * 0.16) as i32;
    for y in (cy - kr)..(cy + kr) {
        for x in (cx - kr)..(cx + kr) {
            let (dx, dy) = ((x - cx) as f32, (y - cy) as f32);
            if dx * dx + dy * dy <= (kr * kr) as f32 {
                img.put_pixel(x as u32, y as u32, Rgba([255, 255, 255, 255]));
            }
        }
    }

    // 5 — The logo: a real file if supplied, else a placeholder ring. Scaled to
    //     ~80% of the knockout and alpha-composited (transparent pixels skipped).
    let logo = match std::env::args().nth(1) {
        Some(path) => image::open(path)?.to_rgba8(),
        None => placeholder_logo(),
    };
    let target = (2.0 * kr as f32 * 0.8) as u32;
    let logo = imageops::resize(&logo, target, target, imageops::FilterType::Lanczos3);
    let (ox, oy) = (cx - target as i32 / 2, cy - target as i32 / 2);
    for (x, y, p) in logo.enumerate_pixels() {
        if p[3] > 40 {
            let (px, py) = (ox + x as i32, oy + y as i32);
            if px >= 0 && py >= 0 && (px as u32) < dim && (py as u32) < dim {
                img.put_pixel(px as u32, py as u32, *p);
            }
        }
    }

    img.save("business_card.png")?;
    println!("Wrote business_card.png ({dim}x{dim}) — scan to add Jane Doe to your contacts.");
    Ok(())
}

/// A simple brand mark (a coloured ring) used when no logo file is supplied.
fn placeholder_logo() -> RgbaImage {
    let size: u32 = 256;
    let c = size as i32 / 2;
    let outer = (size as f32 * 0.46) as i32;
    let inner = (size as f32 * 0.30) as i32;
    ImageBuffer::from_fn(size, size, |x, y| {
        let (dx, dy) = (x as i32 - c, y as i32 - c);
        let d2 = dx * dx + dy * dy;
        if d2 <= outer * outer && d2 >= inner * inner {
            INK
        } else {
            Rgba([0, 0, 0, 0]) // transparent
        }
    })
}
