# memx

[![crate][crate-image]][crate-link]
[![Docs][docs-image]][docs-link]
![Rust Version][rustc-image]
![Apache2/MIT licensed][license-image]
[![Test ubu][test-ubuntu-image]][test-ubuntu-link]
[![Test mac][test-windows-image]][test-windows-link]
[![Test win][test-macos-image]][test-macos-link]

`memx` minics libc.

This crate is implemented memory functions like libc memcmp(), memchr(),
memmem(), memcpy(), memset().

## Features

- Rewriting with rust lang.
- minimum support rustc 1.56.1 (59eed8a2a 2021-11-01)

## Todo

- [ ] Support the zero overhead trait.
- [x] Support more fast routine on armv7-android
- [x] Support more fast routine on x86_64
- [x] Support #!\[no_std\]

## Support status of miri :: rustc 1.68.0-nightly (77429957a 2023-01-01)

Ok lists:

- cargo +nightly miri test --target=x86_64-unknown-linux-gnu
- cargo +nightly miri test --target=i686-unknown-linux-gnu
- cargo +nightly miri test --target=i586-unknown-linux-gnu
- cargo +nightly miri test --target=aarch64-unknown-linux-gnu
- cargo +nightly miri test --target=armv7-unknown-linux-gnueabihf

Failed lists:

- nothing


# Benchmark results

| forward search     | backward search     |
|:-------------------|:--------------------|
| [`memchr()`]       | [`memrchr()`]       |
| [`memnechr()`]     | [`memrnechr()`]     |
| [`memmem()`]       | [`memrmem()`]       |
|                    |                     |
| [`memchr_dbl()`]   | [`memrchr_dbl()`]   |
| [`memchr_tpl()`]   | [`memrchr_tpl()`]   |
| [`memchr_qpl()`]   | [`memrchr_qpl()`]   |
|                    |                     |
| [`memnechr_dbl()`] | [`memrnechr_dbl()`] |
| [`memnechr_tpl()`] | [`memrnechr_tpl()`] |
| [`memnechr_qpl()`] | [`memrnechr_qpl()`] |

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
[`memchr_tpl()`]: https://github.com/aki-akaguma/memx/blob/main/docs/README.memchr_tpl.md
[`memrchr_tpl()`]: https://github.com/aki-akaguma/memx/blob/main/docs/README.memrchr_tpl.md
[`memchr_qpl()`]: https://github.com/aki-akaguma/memx/blob/main/docs/README.memchr_qpl.md
[`memrchr_qpl()`]: https://github.com/aki-akaguma/memx/blob/main/docs/README.memrchr_qpl.md

[`memnechr_dbl()`]: https://github.com/aki-akaguma/memx/blob/main/docs/README.memnechr_dbl.md
[`memrnechr_dbl()`]: https://github.com/aki-akaguma/memx/blob/main/docs/README.memrnechr_dbl.md
[`memnechr_tpl()`]: https://github.com/aki-akaguma/memx/blob/main/docs/README.memnechr_tpl.md
[`memrnechr_tpl()`]: https://github.com/aki-akaguma/memx/blob/main/docs/README.memrnechr_tpl.md
[`memnechr_qpl()`]: https://github.com/aki-akaguma/memx/blob/main/docs/README.memnechr_qpl.md
[`memrnechr_qpl()`]: https://github.com/aki-akaguma/memx/blob/main/docs/README.memrnechr_qpl.md

# Changelogs

[This crate's changelog here.](https://github.com/aki-akaguma/memx/blob/main/CHANGELOG.md)

# License

This project is licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   https://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   https://opensource.org/licenses/MIT)

at your option.

[//]: # (badges)

[crate-image]: https://img.shields.io/crates/v/memx.svg
[crate-link]: https://crates.io/crates/memx
[docs-image]: https://docs.rs/memx/badge.svg
[docs-link]: https://docs.rs/memx/
[rustc-image]: https://img.shields.io/badge/rustc-1.56+-blue.svg
[license-image]: https://img.shields.io/badge/license-Apache2.0/MIT-blue.svg
[test-ubuntu-image]: https://github.com/aki-akaguma/memx/actions/workflows/test-ubuntu.yml/badge.svg
[test-ubuntu-link]: https://github.com/aki-akaguma/memx/actions/workflows/test-ubuntu.yml
[test-macos-image]: https://github.com/aki-akaguma/memx/actions/workflows/test-macos.yml/badge.svg
[test-macos-link]: https://github.com/aki-akaguma/memx/actions/workflows/test-macos.yml
[test-windows-image]: https://github.com/aki-akaguma/memx/actions/workflows/test-windows.yml/badge.svg
[test-windows-link]: https://github.com/aki-akaguma/memx/actions/workflows/test-windows.yml
