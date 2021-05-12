# memx
memory functions like a libc memcmp(), memchr(), memmem(), memcpy(), memset()

## Features

* Rewriting with rust lang.
* Support the zero overhead trait.
* minimum support: rustc 1.41.1 (f3e1a954d 2020-02-24)

## Benchmark

- compile by rustc 1.52.0 (88f19c6da 2021-05-03)

- std is std::str::find()
- `us` is micro seconds
- `:en` is english haystack.
- `:ja` is japanese haystack.
- `musl` is x86_64-unknown-linux-musl
- bench on intel Q6600 @ 2.40GHz


## Changelogs

[This crate's changelog here.](https://github.com/aki-akaguma/memx/blob/main/CHANGELOG.md)


## References
