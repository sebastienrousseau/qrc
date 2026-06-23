//! # Branded business-card QR — template
//!
//! Generates a scannable vCard business-card QR with your **logo** centered in a
//! clean knockout, in your brand color. It writes two files:
//!
//! - `branded_card.svg` — **vector**, razor-sharp at any print size (your logo
//!   is embedded, so it scales perfectly). Open it in a browser/Preview, or use
//!   it directly on business cards / posters.
//! - `branded_card.png` — a raster version (logo composited if you supply a
//!   raster logo; for an SVG logo the PNG is logo-less — open the SVG instead).
//!
//! ## Make it your own
//!
//! Edit the `CONFIG` constants below — your name, contact details, logo path and
//! colors — then run:
//!
//! ```sh
//! cargo run --example branded_card --all-features
//! ```
//!
//! Tips:
//! - Keep the payload compact (name + email + URL) so the QR stays clean.
//! - Supply your logo in the color you want (e.g. matching `INK` for a
//!   monochrome look). Any SVG/PNG works for the SVG output.
//! - The center logo relies on the QR's high error correction; keeping
//!   `KNOCKOUT_FRAC` at ~0.16 or below stays comfortably within tolerance.

use qrc::encode::{Ecc, QrOptions};
use qrc::payload::vcard::BusinessCard;
use qrc::render::raster::{self, RasterOptions};
use qrc::render::style::Color;
use qrc::QRCode;

// ───────────────────────────── CONFIG (edit me) ─────────────────────────────
// — Identity (required) —
const FULL_NAME: &str = "Sebastien Rousseau"; // your preferred professional name
const FIRST: &str = "Sebastien";
const LAST: &str = "Rousseau";
const JOB_TITLE: &str = "Founder & Software Engineer"; // a clear, specific title
const COMPANY: &str = ""; // your brand / company name; "" to skip

// — Contact (keep to one each to avoid clutter) —
const MOBILE: &str = ""; // one primary number, e.g. "+44 20 7946 0000"; "" to skip
const EMAIL: &str = "contact@sebastienrousseau.com"; // domain-based business email

// — Web presence (optional — include only if central to your work) —
const WEBSITE: &str = "https://sebastienrousseau.com"; // "" to skip
const SOCIAL: &str = "https://www.linkedin.com/in/sebastienrousseau/"; // "" to skip

// — Branding & output —
const LOGO_PATH: &str = "examples/assets/logo.svg"; // your logo (SVG/PNG/JPG); "" for none
const INK: &str = "#1d1d1f"; // QR + logo color (background is white)
/// Error correction / density: "low" | "medium" | "quartile" | "high".
/// Higher protects against damage but makes a *denser, busier* code. A centred
/// logo only needs the knockout covered, so "quartile" keeps it clean and
/// premium while staying robust; use "high" only for harsh print conditions.
const ECC: &str = "quartile";
const OUT: &str = "branded_card"; // writes <OUT>.svg and <OUT>.png
const MODULE_PX: u32 = 24;
const KNOCKOUT_FRAC: f32 = 0.16; // knockout radius as a fraction of the QR width
const LOGO_FRAC: f32 = 0.82; // logo size as a fraction of the knockout diameter
                             // ─────────────────────────────────────────────────────────────────────────────

/// Minimal base64 (standard alphabet) for embedding the logo as a data URI.
fn base64(data: &[u8]) -> String {
    const A: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut out = String::with_capacity((data.len() + 2) / 3 * 4);
    for chunk in data.chunks(3) {
        let b = [
            chunk[0],
            *chunk.get(1).unwrap_or(&0),
            *chunk.get(2).unwrap_or(&0),
        ];
        let n = (b[0] as usize) << 16 | (b[1] as usize) << 8 | b[2] as usize;
        out.push(A[(n >> 18) & 63] as char);
        out.push(A[(n >> 12) & 63] as char);
        out.push(if chunk.len() > 1 {
            A[(n >> 6) & 63] as char
        } else {
            '='
        });
        out.push(if chunk.len() > 2 {
            A[n & 63] as char
        } else {
            '='
        });
    }
    out
}

fn hex_rgb(hex: &str) -> (u8, u8, u8) {
    let h = hex.trim_start_matches('#');
    let v = u32::from_str_radix(h, 16).unwrap_or(0x1d1d1f);
    ((v >> 16) as u8, (v >> 8) as u8, v as u8)
}

fn logo_data_uri() -> Option<String> {
    let bytes = std::fs::read(LOGO_PATH).ok()?;
    let mime = match LOGO_PATH.rsplit('.').next() {
        Some("svg") => "image/svg+xml",
        Some("png") => "image/png",
        Some("jpg") | Some("jpeg") => "image/jpeg",
        _ => "application/octet-stream",
    };
    Some(format!("data:{mime};base64,{}", base64(&bytes)))
}

fn main() {
    // Build the vCard from whichever fields are filled in.
    let mut card = BusinessCard::new(FULL_NAME).name(FIRST, LAST);
    if !COMPANY.is_empty() {
        card = card.organization(COMPANY);
    }
    if !JOB_TITLE.is_empty() {
        card = card.title(JOB_TITLE);
    }
    if !MOBILE.is_empty() {
        card = card.phone(MOBILE);
    }
    if !EMAIL.is_empty() {
        card = card.email(EMAIL);
    }
    if !WEBSITE.is_empty() {
        card = card.url(WEBSITE);
    }
    let mut payload = card.to_vcard();
    if !SOCIAL.is_empty() {
        // The builder carries one URL; add the social profile as a second
        // URL line so it shows as a proper, tappable link in Contacts.
        payload = payload.replace("END:VCARD", &format!("URL:{SOCIAL}\r\nEND:VCARD"));
    }
    let ecc = match ECC {
        "low" => Ecc::Low,
        "medium" => Ecc::Medium,
        "high" => Ecc::High,
        _ => Ecc::Quartile,
    };
    let m = QRCode::from_string(payload.clone())
        .encode(&QrOptions::new().ecc(ecc))
        .expect("payload too large — trim the card fields");
    let (ir, ig, ib) = hex_rgb(INK);
    let n = m.size();
    let qz = 4u32;
    let total = n as u32 + 2 * qz;
    let dim = total * MODULE_PX;
    let (cx, cy) = (dim as f32 / 2.0, dim as f32 / 2.0);
    let kr = dim as f32 * KNOCKOUT_FRAC;

    // ---- Vector SVG ----
    let mut svg = format!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{dim}\" height=\"{dim}\" viewBox=\"0 0 {dim} {dim}\">\
<rect width=\"{dim}\" height=\"{dim}\" fill=\"#ffffff\"/><g fill=\"{INK}\">"
    );
    for y in 0..n {
        for x in 0..n {
            if m.is_dark(x, y) {
                let px = (qz + x as u32) * MODULE_PX;
                let py = (qz + y as u32) * MODULE_PX;
                svg.push_str(&format!(
                    "<rect x=\"{px}\" y=\"{py}\" width=\"{MODULE_PX}\" height=\"{MODULE_PX}\"/>"
                ));
            }
        }
    }
    svg.push_str("</g>");
    if let Some(uri) = logo_data_uri() {
        let lside = 2.0 * kr * LOGO_FRAC;
        svg.push_str(&format!(
            "<circle cx=\"{cx:.1}\" cy=\"{cy:.1}\" r=\"{kr:.1}\" fill=\"#ffffff\"/>\
<image x=\"{:.1}\" y=\"{:.1}\" width=\"{lside:.1}\" height=\"{lside:.1}\" href=\"{uri}\"/>",
            cx - lside / 2.0,
            cy - lside / 2.0
        ));
    }
    svg.push_str("</svg>");
    std::fs::write(format!("{OUT}.svg"), &svg).unwrap();

    // ---- Raster PNG (qrc renders crisp, scannable modules) ----
    let mut img = raster::render(
        &m,
        &RasterOptions {
            module_size: MODULE_PX,
            dark: Color::rgb(ir, ig, ib),
            light: Color::WHITE,
        },
    );
    let (w, h) = img.dimensions();
    let logo_raster = if LOGO_PATH.is_empty() {
        None
    } else {
        image::open(LOGO_PATH).ok()
    };
    if let Some(logo) = &logo_raster {
        // white circular knockout, then composite the (raster) logo
        let (icx, icy, ikr) = (cx as i32, cy as i32, kr as i32);
        for yy in (icy - ikr)..(icy + ikr) {
            for xx in (icx - ikr)..(icx + ikr) {
                if xx >= 0 && yy >= 0 && (xx as u32) < w && (yy as u32) < h {
                    let (dx, dy) = ((xx - icx) as f32, (yy - icy) as f32);
                    if dx * dx + dy * dy <= (ikr * ikr) as f32 {
                        img.put_pixel(xx as u32, yy as u32, image::Rgba([255, 255, 255, 255]));
                    }
                }
            }
        }
        let target = (2.0 * kr * LOGO_FRAC) as u32;
        let lr = image::imageops::resize(
            &logo.to_rgba8(),
            target,
            target,
            image::imageops::FilterType::Lanczos3,
        );
        let (ox, oy) = (icx - target as i32 / 2, icy - target as i32 / 2);
        for (px, py, p) in lr.enumerate_pixels() {
            let lum = (p[0] as u32 + p[1] as u32 + p[2] as u32) / 3;
            if lum < 250 && p[3] > 40 {
                let (xx, yy) = (ox + px as i32, oy + py as i32);
                if xx >= 0 && yy >= 0 && (xx as u32) < w && (yy as u32) < h {
                    img.put_pixel(xx as u32, yy as u32, *p);
                }
            }
        }
    }
    img.save(format!("{OUT}.png")).unwrap();

    // ---- Verify the code is valid (decode the raster back) ----
    let bytes = std::fs::read(format!("{OUT}.png")).unwrap();
    let luma = image::load_from_memory(&bytes).unwrap().into_luma8();
    let mut prep = rqrr::PreparedImage::prepare(luma);
    let ok = prep
        .detect_grids()
        .first()
        .and_then(|g| g.decode().ok())
        .is_some_and(|(_, s)| s == payload);

    println!("Wrote {OUT}.svg (vector) and {OUT}.png ({n}x{n} modules)");
    println!("Payload scans back correctly: {ok}");
    if logo_raster.is_none() && !LOGO_PATH.is_empty() {
        println!("Note: '{LOGO_PATH}' isn't a raster image, so the PNG has no centered logo — open {OUT}.svg for the logo'd vector version.");
    }
}
