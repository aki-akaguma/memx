# memx
memory functions like a libc memcmp(), memchr(), memmem(), memcpy(), memset()

## Features

* Rewriting with rust lang.
* minimum support: rustc 1.41.1 (f3e1a954d 2020-02-24)

## Todo

- [ ] Support the zero overhead trait.
- [x] Support more fast routine on armv7-android
- [x] Support more fast routine on x86_64
- [x] Support #!\[no_std\]

## Benchmark results

- [`memchr()`](https://github.com/aki-akaguma/memx/blob/main/docs/README.memchr.md)
- [`memcmp()`](https://github.com/aki-akaguma/memx/blob/main/docs/README.memcmp.md)
- [`memcpy()`](https://github.com/aki-akaguma/memx/blob/main/docs/README.memcpy.md)
- [`memeq()`](https://github.com/aki-akaguma/memx/blob/main/docs/README.memeq.md)
- [`memmem()`](https://github.com/aki-akaguma/memx/blob/main/docs/README.memmem.md)
- [`memset()`](https://github.com/aki-akaguma/memx/blob/main/docs/README.memset.md)

## Changelogs

[This crate's changelog here.](https://github.com/aki-akaguma/memx/blob/main/CHANGELOG.md)


## References

 * [making-a-char-searcher-in-c](https://pzemtsov.github.io/2019/09/26/making-a-char-searcher-in-c.html)
 * [bithacks-ZeroInWord](https://graphics.stanford.edu/~seander/bithacks.html#ZeroInWord)
