<!-- markdownlint-disable MD033 MD041 -->

<img src="https://kura.pro/qrc/images/logos/rlg.svg"
alt="QR Code (QRC) logo" height="261" width="261" align="right" />

<!-- markdownlint-enable MD033 MD041 -->

# RLG

A Rust library that implements application-level logging with a simple, readable output format.

[![Made With Love][made-with-rust]][05]
[![Crates.io][crates-badge]][07]
[![Lib.rs][libs-badge]][09]
[![Docs.rs][docs-badge]][08]
[![License][license-badge]][02]

![divider][divider]

## Welcome to QR Code (QRC) 👋

![RLG Banner][banner]

<!-- markdownlint-disable MD033 -->
<center>

**[Website][00]
• [Documentation][08]
• [Report Bug][03]
• [Request Feature][03]
• [Contributing Guidelines][04]**

</center>

<!-- markdownlint-enable MD033 -->

## Overview 📖

`QR Code (QRC)` is a Rust library that provides application-level logging with
a simple, readable output format. It offers logging APIs and various helper
macros to simplify common logging tasks.

## Features ✨

- Supports many log levels: `ALL`, `DEBUG`, `DISABLED`, `ERROR`,
  `FATAL`, `INFO`, `NONE`, `TRACE`, `VERBOSE` and `WARNING`,
- Provides structured log formats that are easy to parse and filter,
- Compatible with multiple output formats including:
  - Common Event Format (CEF),
  - Extended Log Format (ELF),
  - Graylog Extended Log Format (GELF),
  - JavaScript Object Notation (JSON)
  - NCSA Common Log Format (CLF),
  - W3C Extended Log File Format (W3C),
  - and more.

[00]: https://rustlogs.com
[02]: http://opensource.org/licenses/MIT
[03]: https://github.com/sebastienrousseau/qrc/issues
[04]: https://raw.githubusercontent.com/sebastienrousseau/qrc/main/.github/CONTRIBUTING.md
[05]: https://github.com/sebastienrousseau/qrc/graphs/contributors
[07]: https://crates.io/crates/rlg
[08]: https://docs.rs/rlg
[09]: https://lib.rs/crates/rlg

[banner]: https://kura.pro/qrc/images/titles/title-rlg.svg "RLG Banner"
[crates-badge]: https://img.shields.io/crates/v/rlg.svg?style=for-the-badge 'Crates.io'
[divider]: https://raw.githubusercontent.com/sebastienrousseau/vault/main/assets/elements/divider.svg "divider"
[docs-badge]: https://img.shields.io/docsrs/rlg.svg?style=for-the-badge 'Docs.rs'
[libs-badge]: https://img.shields.io/badge/lib.rs-v0.0.2-orange.svg?style=for-the-badge 'Lib.rs'
[license-badge]: https://img.shields.io/crates/l/rlg.svg?style=for-the-badge 'License'
[made-with-rust]: https://img.shields.io/badge/rust-f04041?style=for-the-badge&labelColor=c0282d&logo=rust 'Made With Rust'

## Changelog 📚