//! Element: the `Matrix` — the backend-agnostic module grid renderers consume.
//!
//! Run: `cargo run --example matrix`

use qrc::encode::QrOptions;
use qrc::QRCode;

fn main() {
    let matrix = QRCode::from_string("https://example.com".to_string())
        .encode(&QrOptions::new())
        .unwrap();

    println!("size (modules per side, no quiet zone): {}", matrix.size());
    println!("quiet zone width: {}", matrix.quiet_zone());
    println!("total size (incl. quiet zone): {}", matrix.total_size());

    // Data-coordinate access (excludes quiet zone). The top-left finder
    // pattern's corner module is dark.
    println!("module (0,0) is dark: {}", matrix.is_dark(0, 0));
    // Out-of-range data coordinates read as light.
    println!(
        "module (size,0) is dark: {}",
        matrix.is_dark(matrix.size(), 0)
    );

    // Quiet-zone-aware access: (0,0) is the top-left of the quiet zone (light).
    println!(
        "module (0,0) incl. quiet zone is dark: {}",
        matrix.is_dark_with_quiet_zone(0, 0)
    );

    // Count dark modules as a quick sanity check.
    let mut dark = 0;
    for y in 0..matrix.size() {
        for x in 0..matrix.size() {
            if matrix.is_dark(x, y) {
                dark += 1;
            }
        }
    }
    println!("dark modules: {dark} / {}", matrix.size() * matrix.size());
}
