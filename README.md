# memx

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

## Support status of miri :: rustc 1.56.0-nightly (a6ece5615 2021-08-03)

Ok lists:

- cargo miri test --target=i586-unknown-linux-gnu
- cargo miri test --target=aarch64-unknown-linux-gnu
- cargo miri test --target=armv7-unknown-linux-gnueabihf

Failed lists:

- cargo miri test --target=x86_64-unknown-linux-gnu
- cargo miri test --target=i686-unknown-linux-gnu

miri error: `unimplemented intrinsic: simd_eq`


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
