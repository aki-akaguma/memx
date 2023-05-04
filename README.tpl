# {{crate}}

[![crate][crate-image]][crate-link]
[![Docs][docs-image]][docs-link]
![Rust Version][rustc-image]
![Apache2/MIT licensed][license-image]
[![Test ubu][test-ubuntu-image]][test-ubuntu-link]
[![Test mac][test-windows-image]][test-windows-link]
[![Test win][test-macos-image]][test-macos-link]

{{readme}}

# Benchmark results

| forward search   | backward search   |
|:-----------------|:------------------|
| [`memchr()`]     | [`memrchr()`]     |
| [`memnechr()`]   | [`memrnechr()`]   |
| [`memmem()`]     | [`memrmem()`]     |
| [`memchr_dbl()`] | [`memrchr_dbl()`] |

| memory op      |
|:---------------|
| [`memcmp()`]   |
| [`memeq()`]    |
| [`memcpy()`]   |
| [`memset()`]   |

[`memchr()`]: https://github.com/aki-akaguma/memx/blob/main/docs/README.memchr.md
[`memcmp()`]: https://github.com/aki-akaguma/memx/blob/main/docs/README.memcmp.md
[`memcpy()`]: https://github.com/aki-akaguma/memx/blob/main/docs/README.memcpy.md
[`memeq()`]: https://github.com/aki-akaguma/memx/blob/main/docs/README.memeq.md
[`memmem()`]: https://github.com/aki-akaguma/memx/blob/main/docs/README.memmem.md
[`memnechr()`]: https://github.com/aki-akaguma/memx/blob/main/docs/README.memnechr.md
[`memrchr()`]: https://github.com/aki-akaguma/memx/blob/main/docs/README.memrchr.md
[`memrmem()`]: https://github.com/aki-akaguma/memx/blob/main/docs/README.memrmem.md
[`memrnechr()`]: https://github.com/aki-akaguma/memx/blob/main/docs/README.memrnechr.md
[`memset()`]: https://github.com/aki-akaguma/memx/blob/main/docs/README.memset.md
[`memchr_dbl()`]: https://github.com/aki-akaguma/memx/blob/main/docs/README.memchr_dbl.md
[`memrchr_dbl()`]: https://github.com/aki-akaguma/memx/blob/main/docs/README.memrchr_dbl.md

# Changelogs

[This crate's changelog here.](https://github.com/aki-akaguma/{{crate}}/blob/main/CHANGELOG.md)

# License

This project is licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   https://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   https://opensource.org/licenses/MIT)

at your option.

[//]: # (badges)

[crate-image]: https://img.shields.io/crates/v/{{crate}}.svg
[crate-link]: https://crates.io/crates/{{crate}}
[docs-image]: https://docs.rs/{{crate}}/badge.svg
[docs-link]: https://docs.rs/{{crate}}/
[rustc-image]: https://img.shields.io/badge/rustc-1.56+-blue.svg
[license-image]: https://img.shields.io/badge/license-Apache2.0/MIT-blue.svg
[test-ubuntu-image]: https://github.com/aki-akaguma/{{crate}}/actions/workflows/test-ubuntu.yml/badge.svg
[test-ubuntu-link]: https://github.com/aki-akaguma/{{crate}}/actions/workflows/test-ubuntu.yml
[test-macos-image]: https://github.com/aki-akaguma/{{crate}}/actions/workflows/test-macos.yml/badge.svg
[test-macos-link]: https://github.com/aki-akaguma/{{crate}}/actions/workflows/test-macos.yml
[test-windows-image]: https://github.com/aki-akaguma/{{crate}}/actions/workflows/test-windows.yml/badge.svg
[test-windows-link]: https://github.com/aki-akaguma/{{crate}}/actions/workflows/test-windows.yml
