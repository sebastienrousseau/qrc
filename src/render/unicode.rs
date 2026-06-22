// Copyright © 2022-2026 QR Code Library (QRC). All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Terminal rendering using Unicode half-block characters.
//!
//! Two vertical modules are packed into one character row, so the output keeps
//! roughly square proportions in a typical monospace terminal. The quiet zone
//! is included so the result is scannable straight off the screen.

use crate::matrix::Matrix;

const FULL: char = '\u{2588}'; // █ both halves dark
const UPPER: char = '\u{2580}'; // ▀ top dark
const LOWER: char = '\u{2584}'; // ▄ bottom dark
const BLANK: char = ' '; // both light

/// Renders `matrix` to a string of half-block characters (dark on light).
#[must_use]
pub fn render(matrix: &Matrix) -> String {
    let total = matrix.total_size();
    let mut out = String::with_capacity((total / 2 + 1) * (total + 1));
    let mut y = 0;
    while y < total {
        for x in 0..total {
            let top = matrix.is_dark_with_quiet_zone(x, y);
            let bottom = matrix.is_dark_with_quiet_zone(x, y + 1);
            out.push(match (top, bottom) {
                (true, true) => FULL,
                (true, false) => UPPER,
                (false, true) => LOWER,
                (false, false) => BLANK,
            });
        }
        out.push('\n');
        y += 2;
    }
    out
}
