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

- [`memchr()`](https://github.com/aki-akaguma/memx/blob/main/docs/README.memchr.md)
- [`memcmp()`](https://github.com/aki-akaguma/memx/blob/main/docs/README.memcmp.md)
- [`memcpy()`](https://github.com/aki-akaguma/memx/blob/main/docs/README.memcpy.md)
- [`memeq()`](https://github.com/aki-akaguma/memx/blob/main/docs/README.memeq.md)
- [`memmem()`](https://github.com/aki-akaguma/memx/blob/main/docs/README.memmem.md)
- [`memset()`](https://github.com/aki-akaguma/memx/blob/main/docs/README.memset.md)

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
