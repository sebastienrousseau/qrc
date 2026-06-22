// Copyright © 2022-2026 QR Code Library (QRC). All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! SVG-first rendering.
//!
//! Produces resolution-independent SVG with configurable colors and module
//! shapes, always emitting a correct quiet zone. SVG is the recommended output
//! for print and branded codes.

use crate::matrix::Matrix;
use crate::render::style::{Color, ModuleShape};

/// Options for [`render`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct SvgOptions {
    /// Size of a single module in SVG user units. The final image is
    /// `module_size * matrix.total_size()` units square.
    pub module_size: u32,
    /// Dark-module color.
    pub dark: Color,
    /// Light/background color.
    pub light: Color,
    /// Shape used for each dark module.
    pub shape: ModuleShape,
}

impl Default for SvgOptions {
    fn default() -> Self {
        SvgOptions {
            module_size: 8,
            dark: Color::BLACK,
            light: Color::WHITE,
            shape: ModuleShape::Square,
        }
    }
}

impl SvgOptions {
    /// Default options at the given module size.
    #[must_use]
    pub fn with_module_size(module_size: u32) -> Self {
        SvgOptions {
            module_size,
            ..Self::default()
        }
    }
}

/// Renders `matrix` to an SVG document string.
#[must_use]
pub fn render(matrix: &Matrix, opts: &SvgOptions) -> String {
    let m = opts.module_size.max(1);
    let total = matrix.total_size() as u32;
    let dim = total * m;
    let qz = matrix.quiet_zone() as u32;

    let mut svg = String::with_capacity(128 + matrix.size() * matrix.size() * 8);
    svg.push_str(&format!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{dim}\" height=\"{dim}\" \
viewBox=\"0 0 {dim} {dim}\" shape-rendering=\"crispEdges\">"
    ));
    // Background (covers the quiet zone too).
    svg.push_str(&format!(
        "<rect width=\"{dim}\" height=\"{dim}\" fill=\"{}\"/>",
        opts.light.to_hex()
    ));

    let fill = opts.dark.to_hex();
    let opacity = opts.dark.opacity();
    let opacity_attr = if opacity < 1.0 {
        format!(" fill-opacity=\"{opacity:.3}\"")
    } else {
        String::new()
    };

    for y in 0..matrix.size() {
        for x in 0..matrix.size() {
            if !matrix.is_dark(x, y) {
                continue;
            }
            let px = (x as u32 + qz) * m;
            let py = (y as u32 + qz) * m;
            match opts.shape {
                ModuleShape::Square => svg.push_str(&format!(
                    "<rect x=\"{px}\" y=\"{py}\" width=\"{m}\" height=\"{m}\" fill=\"{fill}\"{opacity_attr}/>"
                )),
                ModuleShape::Rounded => {
                    let r = m / 4;
                    svg.push_str(&format!(
                        "<rect x=\"{px}\" y=\"{py}\" width=\"{m}\" height=\"{m}\" rx=\"{r}\" ry=\"{r}\" fill=\"{fill}\"{opacity_attr}/>"
                    ));
                }
                ModuleShape::Circle => {
                    let cx = px + m / 2;
                    let cy = py + m / 2;
                    let rad = m / 2;
                    svg.push_str(&format!(
                        "<circle cx=\"{cx}\" cy=\"{cy}\" r=\"{rad}\" fill=\"{fill}\"{opacity_attr}/>"
                    ));
                }
            }
        }
    }
    svg.push_str("</svg>");
    svg
}
