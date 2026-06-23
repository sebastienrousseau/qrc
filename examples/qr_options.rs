//! Element: `QrOptions` and `Ecc` — error correction, forced version, quiet zone.
//!
//! Run: `cargo run --example qr_options`

use qrc::encode::{encode, Ecc, QrOptions};
use qrc::QRCode;

fn main() {
    let data = b"https://example.com";

    // Error-correction level.
    for ecc in [Ecc::Low, Ecc::Medium, Ecc::Quartile, Ecc::High] {
        let m = encode(data, &QrOptions::new().ecc(ecc)).unwrap();
        println!("{ecc:?}: {0}x{0} modules", m.size());
    }

    // Forced version (1..=40) — version 5 is 37x37.
    let m = encode(data, &QrOptions::new().version(5)).unwrap();
    println!("forced version 5: {}x{}", m.size(), m.size());

    // Custom quiet zone width (default 4).
    let m = encode(data, &QrOptions::new().quiet_zone(8)).unwrap();
    println!(
        "quiet zone: {} modules (total {})",
        m.quiet_zone(),
        m.total_size()
    );

    // Builder + defaults.
    let opts = QrOptions::new().ecc(Ecc::High).version(10).quiet_zone(2);
    assert_eq!(
        (opts.ecc, opts.version, opts.quiet_zone),
        (Ecc::High, Some(10), 2)
    );
    assert_eq!(QrOptions::default(), QrOptions::new());
    assert_eq!(Ecc::default(), Ecc::Medium);

    // Invalid / too-small versions are recoverable errors.
    assert!(QRCode::from_string("x".into())
        .encode(&QrOptions::new().version(99))
        .is_err());
    println!("invalid version rejected without panic");
}
