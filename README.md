# memx

`memx` minics libc.

This crate is implemented memory functions like libc memcmp(), memchr(),
memmem(), memcpy(), memset().

### Features

* Rewriting with rust lang.
* minimum support rustc 1.46.0 (04488afe3 2020-08-24)

### Todo

- [ ] Support the zero overhead trait.
- [x] Support more fast routine on armv7-android
- [x] Support more fast routine on x86_64
- [x] Support #!\[no_std\]


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
