# QRC — Deep-Dive Analysis & Road to a Top-10 QR Library (v0.0.6 → v0.3.0)

> Status: planning artifact for the `feat/v0.0.6` line. Updated 2026-06-22.
> Scope: audit of the current crate, 2026 market/competitor research, gap analysis,
> and a phased implementation plan to make `qrc` a credible top-10 Rust QR library.

---

## 1. Executive summary

`qrc` v0.0.5 is a thin, partly-broken wrapper over the `qrcode` + `image` + `flate2`
crates. It ships **critical correctness bugs that make its raster QR codes unscannable**,
several "features" that are non-functional stubs, and no differentiation against the
incumbents (`qrcode`, `fast_qr`, `qrcode-generator`). Total downloads are ~11k (≈745
recent) vs. millions for the leaders.

To become a top-10 crate it needs, in order: (1) **correctness** — produce scannable
codes; (2) **a real, ergonomic, typed API** with error handling; (3) **differentiation**
the incumbents lack — first-class **styling/branding** (gradients, dot/eye shapes, logo
embedding, SVG-first) plus a **structured-payload builder** (WiFi/vCard/MeCard/geo/EMVCo/
EPC-SEPA); (4) **reach** — `no_std`, WASM/npm, and a CLI. Decoding is a separate crowded
lane (`rqrr`/`rxing`/`zedbar`) and is explicitly *out of scope* for the generator core.

---

## 1a. Progress on `feat/v0.0.6`

**Phase 0 (correctness) — landed.** Opaque dark modules + real quiet zone +
integer scaling across all renderers; fallible `try_to_qrcode`/`encode`;
panic-free `combine_qr_codes`; fixed example; metadata cleanup; an independent
**round-trip decode test (`rqrr`) proves output is now scannable**.

**Phase 1 (layered API + render core) — landed (first increment).**
- `error` (`QrError`/`Result`), `matrix` (`Matrix`), `encode` (`QrOptions`,
  `Ecc`, `Engine` trait, `QrcodeEngine`), `render` (`style`, `svg`, `raster`,
  `unicode`).
- New API on `QRCode`: `encode`, `to_svg_styled`, `to_unicode`,
  `to_png_bytes`/`to_jpeg_bytes`/`to_gif_bytes` (real format encoders).
- SVG-first renderer with square/rounded/circle modules and custom colors.
- Feature flags: `default = [std, raster, svg, unicode]`; `image`/`flate2` are
  optional — the **core (encode + SVG + unicode) compiles with no `image`
  dependency**, confirmed by `--no-default-features --features svg,unicode`.
- Integration tests (incl. PNG/JPEG/GIF round-trip decode), styled-SVG and
  unicode tests, ECC/version option tests; new criterion benches for the
  layered API. Clippy clean on all-features/all-targets; `cargo fmt` clean.

**Quality gates — met.**
- **100% test coverage** (`cargo llvm-cov --all-features`): regions, functions
  and lines all 100% across every source file. Unreachable defensive branches
  were either refactored away (`create_dynamic` dead arm, `combine`'s
  `unwrap_or`) or made reachable through public API (`to_image_bytes` + a
  no-encoder format triggers the image-error path); `compress_data` and its
  macro were removed (irreducibly-uncoverable `Vec` I/O branches + flagged
  stub), dropping the `flate2` dependency.
- **100% documentation**: public items enforced by `deny(missing_docs)`;
  private items verified clean via `clippy::missing_docs_in_private_items`;
  `cargo doc` builds with `-D warnings` and intra-doc-link checks.
- **One example per element**: ~20 focused examples under `examples/` (one per
  public type/feature — `qrcode`, `qr_options`, `engine`, `matrix`, `color`,
  `svg_options`, `raster_options`, `unicode`, `logo_options`, `blend_options`,
  `control_options`, `wifi`/`mecard`/`vcard`/`emvco`, `macros`, `legacy`, plus
  the `business_card`/`art_qr`/`ai_art` recipes); every one runs without
  panicking and collectively they exercise the full public surface.
- **Benchmarks for every area** (`benches/qrc.rs`) and a **regression suite**
  (`tests/regression.rs`: golden payloads + parametric round-trips + invariants).

**Phase 2 (differentiation) — started.**
- **Branded codes with logo embedding** (`render::raster::{LogoOptions,
  embed_logo, render_with_logo}` + `QRCode::to_image_with_logo` /
  `to_image_bytes_with_logo`): centred, aspect-preserving, padded knockout,
  ECC-High aware. A round-trip decode test proves a logo'd code still scans.
- **vCard business-card payload** (`payload::vcard::BusinessCard`, feature
  `payload`, no deps): RFC-6350 vCard 3.0 with proper escaping.
- `business_card` example ties them together (vCard + uploadable logo → PNG).
- Maintained at 100% coverage / 100% docs.

**AI / artistic QR (per architecture decision) — core landed.**
The work is split to keep the core tight and the heavy lifting modular:
- **Core (this crate, offline, no new deps):**
  - `render::control` + `QRCode::to_control_image{,_bytes}` — export a clean,
    exact-square, high-contrast **ControlNet-ready control image** for AI
    pipelines (Option 2).
  - `render::art` + `QRCode::to_art_image{,_bytes}` — **offline artistic blend**
    weaving a supplied image into the code (dot + solid-finder enforcement),
    deterministic and rqrr-verified scannable (Option 3). `art_qr` example.
- **`feature = "api"` add-on — landed:** `api::generate` / `QRCode::to_ai_art`
  take the control image + a prompt and call a cloud generator behind the
  injectable `api::Provider` trait, verify the result still scans, and retry
  (Option 1). `api::replicate::ReplicateProvider` is the included back end; its
  HTTP is behind an injectable `HttpClient` (real `UreqClient`, mockable for
  tests). 100% coverage incl. a localhost-server test of the real client; the
  `ai_art` example runs against Replicate with a token.
- **Demo — landed:** `demos/qrc-candle`, a standalone CLI (its own `[workspace]`,
  detached from the core's build/CI/coverage) piping the control image into
  `candle` Stable-Diffusion for fully-local generation (Option 4). Uses a
  **true ControlNet** (`src/controlnet.rs` — a diffusers `ControlNetModel`
  port: conditioning embedding + UNet-mirrored encoder + zero-conv outputs,
  injected via the UNet's `forward_with_additional_residuals`). Loads any SD1.5
  ControlNet safetensors (default: a QR-code ControlNet). Compile-verified
  against candle 0.8; weights download at run time.

**Deferred with reason:**
- *Full `no_std`*: blocked on the `qrcode`/`image` backends requiring `std`.
  The architecture is `no_std`-ready (core logic is `alloc`-based behind the
  `Engine` trait); revisit when an `alloc`-only backend lands.
- *`fast_qr` head-to-head benchmark*: benches exist for our API; the comparative
  table is a follow-up (adds a dev-dep + publishes numbers in the README).

---

## 2. Current-state audit

### 2.1 Critical correctness bugs (P0 — codes are broken)

| # | Location | Bug | Impact |
|---|----------|-----|--------|
| C1 | `to_png/to_jpg/to_gif` (`lib.rs:213,236,259`) | Dark modules written as `Rgba([0,0,0,0])` — **alpha 0 = fully transparent**, not black | On any white/transparent canvas the data modules vanish → **unscannable** |
| C2 | raster paths | **No quiet zone**. `qrcode.width()` excludes the mandatory 4-module margin; raster output adds none | Most scanners reject codes with no quiet zone |
| C3 | `to_png`, `to_jpg`, `to_gif` | All three are **byte-identical** and none actually encode PNG/JPEG/GIF — they return a raw `ImageBuffer`. JPEG can't even carry alpha | "Formats" are an illusion; misleading API |
| C4 | `to_qrcode()` (`lib.rs:183`) | `QrCode::new(&self.data).unwrap()` | Any payload over capacity (~2953 B byte mode) **panics** — public API DoS |
| C5 | nearest-neighbour float scaling | Non-integer `width / module_count` makes some modules 1px wider than others | Module-size jitter degrades/destroys scannability |
| C6 | README/example (`example.rs`, `README.md`) | `ImageBuffer::from_raw(21, 21, png_data)` with a 512×512 buffer → `None` → `.unwrap()` **panics** | The headline example crashes |

### 2.2 Non-functional / misleading features (P1)

- `create_dynamic` — returns a hard-coded fake URL `https://your-api-endpoint.com/...`; no real dynamic-QR mechanism.
- `create_multilanguage` — ignores the map except the hard-coded `"en"` key.
- `combine_qr_codes` — calls `resize(...)` and discards the result (no-op); pixel copy can write out of bounds; produces a non-decodable blob, not a valid QR.
- `compress_data` (zlib) — output is not something any scanner will decompress; encoding it yields an unreadable code.
- `set_encoding_format` / `get_encoding_format` — only accepts `"utf-8"`; the `encoding_format` field is otherwise inert (no ECI).
- `overlay_image` — copies the overlay from (0,0) at full size; larger overlays `put_pixel` out of bounds → panic; no quiet-zone/ECC-aware placement.

### 2.3 API & quality gaps (P1/P2)

- No control over **error-correction level, version, mask, or quiet-zone width**.
- No `Result`-returning constructors; panics instead of `Error`.
- No real **encoders** to PNG/JPEG/GIF/WebP bytes; no writer that takes a `Write`/path.
- No **Micro QR / rMQR / ECI / Kanji / structured-append / FNC1** despite the doc table listing them.
- No `no_std`, no WASM, no CLI, no language bindings.
- Stale metadata: docs say v0.0.1, `html_root_url = "https://docs.rs/mini-functions"`, badges pinned to 0.0.1, copyright 2022-2023.
- Tests assert trivia (e.g. `data` round-trips) and **hard-code output byte lengths**; no scannability/round-trip decode test, no property tests.
- `to_jpg`/`to_gif` benches and tests don't verify the format is actually that format.

### 2.4 Security / robustness

- `#![forbid(unsafe_code)]` — good, keep it.
- Multiple **panic-on-untrusted-input** paths (C4, overlay, combine) = availability risk for any service generating codes from user data.
- No input-size guardrails; no fuzzing; `audit.yml`/`deny.toml` exist but supply chain not gated on advisory severity.

---

## 3. 2026 market & competitor landscape (research summary)

**Rust encoders.** `qrcode` 0.14.1 is the de-facto engine (15.5M dl) but slow-moving and
render-rich; `fast_qr` 0.13.1 is the **performance + styling + WASM leader** (~6–7×
faster than `qrcode`, official npm package, shapes/logo/colors, SVG); `qrcode-generator`
wraps Nayuki (segments, Kanji). `rxing` is the broadest (encode+decode, Micro QR, rMQR,
Aztec, DataMatrix, …). Decoding: `rqrr`, `rxing`, new `zedbar`.

**Cross-language best-in-class to benchmark against.** Python **segno** (widest output,
Micro QR, structured-append, Hanzi, CLI), **qr-code-styling** (JS — 6 dot types, separate
eye/eye-ball styling, linear+radial gradients, logo), Go **yeqown/go-qrcode** (shapes,
gradient, logo, halftone, WASM), **zxing-cpp** (decode + Micro/rMQR, bindings everywhere).

**2026 trends that matter for a generator.**
- **Styled/branded codes are the dominant market demand** (gradients, custom dot & eye
  shapes, embedded logos, SVG-first) — leans on level-H ECC to stay scannable.
- **Structured payloads** (WiFi, vCard/MeCard, geo, calendar, **EMVCo** payment MPM/CPM,
  **EPC/SEPA** credit transfer, UPI/PIX) — *scanner conventions, not in ISO 18004*, so a
  clean builder layer is a real, ownable value-add.
- **rMQR (ISO/IEC 23941)** and **Micro QR (ISO/IEC 18004:2024)** — rMQR generation is
  essentially unserved in pure Rust.
- **SVG-first** rendering for print/branding; **WASM/npm** distribution is table-stakes.
- **Quishing / QR-phishing** awareness → libraries should make safe, well-formed,
  high-ECC, quiet-zoned codes the easy default and document scan-safety.

### 3.1 Where the defensible gaps are (priority order)

1. **Styling/branding in Rust** — only `fast_qr`/`yeqown` really compete; nobody matches
   `qr-code-styling`'s designer feature set with SVG-first output. **Highest value.**
2. **Structured-payload builders** — no polished Rust crate owns this. **Low effort, high differentiation.**
3. **Performance** — adopt or match `fast_qr`; publish honest benchmarks.
4. **WASM/npm + Python (PyO3) bindings** — a true PyO3 binding to a pure-Rust encoder is an open niche.
5. **Standards completeness** — Micro QR, then rMQR generation, ECI/Kanji, structured append.

---

## 4. Target architecture

Move from one 644-line `lib.rs` to a layered, feature-gated crate:

```
qrc/
  src/
    lib.rs            // re-exports, crate docs, #![no_std]-compatible core
    error.rs          // QrError (thiserror-free, no_std), Result alias
    encode/           // engine abstraction
      mod.rs          // QrOptions{ ecc, version, mask, mode, quiet_zone }
      engine.rs       // trait Engine -> matrix; default impl over `qrcode`
    matrix.rs         // Module matrix (bit grid) + quiet-zone aware accessors
    render/
      mod.rs
      raster.rs       // correct, integer-scaled RGBA; PNG/JPEG/GIF/WebP byte encoders
      svg.rs          // SVG-first renderer: shapes, eyes, gradients, logo slot
      unicode.rs      // terminal/ASCII (half-block) output
      style.rs        // ModuleShape, EyeShape, Gradient, Logo, Colors, QuietZone
    payload/          // structured payload builders (feature = "payload")
      mod.rs  url.rs  wifi.rs  vcard.rs  mecard.rs  geo.rs  email.rs  sms.rs
      tel.rs  calendar.rs  emvco.rs  epc.rs        // payments behind feature flags
    macros.rs         // keep ergonomic macros, fix to real API
  cli/                // feature = "cli": `qrc` binary (clap)
  bindings/
    wasm/             // feature = "wasm": wasm-bindgen + npm package
    python/           // PyO3 crate (separate workspace member)
  fuzz/               // cargo-fuzz targets
  benches/qrc.rs      // criterion vs qrcode + fast_qr
```

**Feature flags:** `default = ["std", "png", "svg"]`; optional `jpeg`, `gif`, `webp`,
`payload`, `payments`, `styling`, `cli`, `wasm`, `serde`, `unicode`. Core encode/SVG path
builds under `no_std + alloc`.

**Engine abstraction:** keep `qrcode` as the default backend behind an `Engine` trait so a
`fast_qr` backend (or an in-house encoder) can be swapped in for speed without breaking the
public API.

---

## 5. Phased implementation plan

### Phase 0 — Correctness & honesty (v0.0.6) — *foundation, ships first*
- [ ] Fix C1: dark = opaque black `[0,0,0,255]`, light = white; configurable colors.
- [ ] Fix C2: add a real quiet zone (default 4 modules; 2 for Micro) in every renderer.
- [ ] Fix C3: real byte encoders — `to_png_bytes/to_jpeg_bytes/to_gif_bytes/to_webp_bytes`
      and a `write_*` family; deprecate the three identical RGBA methods or make them
      honest (`to_image() -> RgbaImage`).
- [ ] Fix C4: fallible API — `QrError`, `QrCode::encode(...) -> Result<...>`; keep
      `*_unchecked`/`expect`-style only where documented.
- [ ] Fix C5: integer module scaling (`module_px = width / (modules + 2*quiet)`), center
      the grid; never sub-pixel a module.
- [ ] Fix C6: correct the README + example; add a doctest that **decodes** its own output
      (round-trip via `rqrr` as a dev-dependency) to prove scannability.
- [ ] Add ECC/version/mask/quiet-zone options (`QrOptions`).
- [ ] Quarantine misleading stubs: remove or `#[deprecated]` `create_dynamic`,
      `create_multilanguage`, `combine_qr_codes`, `compress_data`; gate behind clearly
      experimental names if kept.
- [ ] Metadata cleanup: version, `html_root_url`, badges, copyright years, doc feature table reflects reality.
- [ ] CI: deny warnings, add MSRV check, add a round-trip decode test + `cargo-fuzz` smoke target.

### Phase 1 — Real API & rendering core (v0.1.0)
- [ ] Land the layered architecture (`encode`/`render`/`matrix`/`error`).
- [ ] SVG-first renderer with configurable module/eye shapes and quiet zone.
- [ ] Correct raster renderer with all real byte encoders behind format features.
- [ ] Unicode/terminal renderer (half-block) for CLI/dev UX.
- [ ] `no_std + alloc` support for the core encode + SVG path; `std` feature default-on.
- [ ] Property tests (proptest) + round-trip decode tests across versions/ECC/modes.
- [ ] Honest criterion benchmarks vs `qrcode` and `fast_qr`, published in README.

### Phase 2 — Differentiation: styling + payloads (v0.2.0)
- [ ] Styling: dot shapes (square, rounded, dot, classy), eye/eye-ball shapes, **linear &
      radial gradients**, background, **logo embedding** with auto ECC bump + safe knockout.
- [ ] Structured-payload builders: `url`, `wifi`, `vcard`, `mecard`, `geo`, `email`, `sms`,
      `tel`, `calendar` — all with proper escaping and tests against real scanner formats.
- [ ] Payments behind `payments` feature: **EMVCo MPM**, **EPC/SEPA** (QR v13, ECC-M),
      with CRC/checksum where the standard requires it.
- [ ] `serde` for `QrOptions`/`Style`; a config-driven builder.

### Phase 3 — Reach: CLI, WASM, bindings, standards (v0.3.0)
- [ ] `qrc` CLI (clap): stdin/args → file/stdout, all formats + styling + payload subcommands.
- [ ] WASM (`wasm-bindgen`) + published **npm** package; live browser demo.
- [ ] **PyO3** Python bindings (the open niche) + wheels via `maturin`.
- [ ] Micro QR generation; then **rMQR** (ISO/IEC 23941) generation — pure-Rust first mover.
- [ ] ECI / Kanji / structured-append support via the engine abstraction.

### Phase 4 — Polish & adoption
- [ ] Docs site / `mdBook`, cookbook of payloads & styles, scan-safety guide (quishing).
- [ ] Fuzzing in CI, `cargo-mutants`, coverage gate, semver-checks.
- [ ] Benchmark page; comparison matrix vs competitors kept current.

---

## 6. Success metrics (definition of "top 10")

- Correctness: 100% of generated codes round-trip-decode in the test matrix.
- Performance: within ~1.5× of `fast_qr`, faster than `qrcode` on styled output.
- Reach: published on crates.io + npm + PyPI; `no_std` green; CLI in releases.
- Differentiation: only Rust crate offering SVG-first styling **and** a payload builder
  **and** payment-QR support.
- Traction: docs.rs all-features green, growing recent-download trend, issues triaged.

---

## 7. Risks & decisions to confirm

- **Engine choice:** stay on `qrcode` (stable, Micro QR, low risk) vs. switch default to
  `fast_qr` (speed/styling, but fewer modes) vs. build in-house. *Recommendation:* keep
  `qrcode` as default backend behind an `Engine` trait; add `fast_qr` backend later.
- **Bindings priority:** WASM/npm first (table-stakes) then PyO3 (open niche).
- **Scope discipline:** no decoding in the generator core; document `rqrr`/`rxing` for that.
- **Backwards-compat:** v0.0.x allows breaking changes; do the API reset now, pre-1.0.
