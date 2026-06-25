//! Export a ControlNet-ready control image.
//!
//! Feed `control.png` to Stable Diffusion + a QR ControlNet (e.g. *QR Code
//! Monster*) as the control input to generate AI "art QR" codes. High error
//! correction gives the model the most room to hide art behind.
//!
//!   cargo run --example control_image

use qrc::{EcLevel, QRCode};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let qr = QRCode::from_string("https://docs.rs/qrc".to_string()).with_ec_level(EcLevel::H);

    // Square, centred, high-contrast, with a generous quiet zone.
    let control = qr.to_control_image(768);
    control.save("control.png")?;

    println!(
        "Wrote control.png ({}x{}) — use it as the ControlNet control image.",
        control.width(),
        control.height()
    );
    Ok(())
}
