// Copyright © 2022-2026 QRC. All rights reserved.
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

//! WASM bindings for the QRC library.

use wasm_bindgen::prelude::*;

use crate::{EcLevel, ModuleShape, QRCode};

/// A QR code generator for use from JavaScript / WASM.
#[wasm_bindgen]
#[derive(Debug)]
pub struct WasmQRCode {
    inner: QRCode,
}

#[wasm_bindgen]
impl WasmQRCode {
    /// Creates a new `WasmQRCode` from a string.
    #[wasm_bindgen(constructor)]
    pub fn new(data: &str) -> Self {
        Self {
            inner: QRCode::from_string(data.to_string()),
        }
    }

    /// Sets the error correction level.
    ///
    /// Valid values: `"L"`, `"M"`, `"Q"`, `"H"`.
    #[wasm_bindgen(js_name = "setEcLevel")]
    pub fn set_ec_level(&mut self, level: &str) {
        let ec = match level {
            "L" => EcLevel::L,
            "Q" => EcLevel::Q,
            "H" => EcLevel::H,
            _ => EcLevel::M,
        };
        self.inner.ec_level = ec;
    }

    /// Sets the module shape.
    ///
    /// Valid values: `"square"`, `"rounded"`, `"circle"`, `"diamond"`.
    #[wasm_bindgen(js_name = "setShape")]
    pub fn set_shape(&mut self, shape: &str) {
        let s = match shape {
            "rounded" => ModuleShape::RoundedSquare,
            "circle" => ModuleShape::Circle,
            "diamond" => ModuleShape::Diamond,
            _ => ModuleShape::Square,
        };
        self.inner.shape = s;
    }

    /// Returns the QR code as an SVG string.
    #[wasm_bindgen(js_name = "toSvg")]
    pub fn to_svg(&self, width: u32) -> String {
        self.inner.to_svg(width)
    }

    /// Returns PNG-encoded bytes.
    #[wasm_bindgen(js_name = "toPngBytes")]
    pub fn to_png_bytes(&self, width: u32) -> Vec<u8> {
        self.inner.to_png_bytes(width).unwrap_or_default()
    }

    /// Returns JPEG-encoded bytes (quality 85).
    #[wasm_bindgen(js_name = "toJpg")]
    pub fn to_jpg(&self, width: u32) -> Vec<u8> {
        self.inner.to_jpg(width).unwrap_or_default()
    }
}
