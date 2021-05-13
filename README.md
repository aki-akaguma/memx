# memx
memory functions like a libc memcmp(), memchr(), memmem(), memcpy(), memset()

## Features

* Rewriting with rust lang.
* Support the zero overhead trait.
* minimum support: rustc 1.41.1 (f3e1a954d 2020-02-24)

## Benchmark

- compile by rustc 1.52.0 (88f19c6da 2021-05-03)

  1. x86_64:

|         `name`          | `bench:en`  | `bench:ja`  |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| plain_memeq             |  559.110 uc |  491.800 uc |  550.460 uc |  494.600 uc |
| std_memeq               |  318.740 uc |  344.230 uc |  571.000 uc |  519.360 uc |
| memx_memeq              |  189.210 uc |  246.930 uc |  183.880 uc |  247.880 uc |
| memx_memeq_basic        |  554.850 uc |  492.360 uc |  550.540 uc |  493.680 uc |
| memx_memeq_libc         |  328.350 uc |  399.950 uc |  603.270 uc |  644.310 uc |

  2. i686:

|         `name`          | `bench:en`  | `bench:ja`  |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| plain_memeq             |  567.840 uc |  504.920 uc |  564.000 uc |  517.670 uc |
| std_memeq               |  322.190 uc |  369.080 uc |  606.880 uc |  632.140 uc |
| memx_memeq              |  281.720 uc |  371.920 uc |  300.970 uc |  409.950 uc |
| memx_memeq_basic        |  552.390 uc |  505.240 uc |  564.210 uc |  523.520 uc |
| memx_memeq_libc         |  368.910 uc |  468.050 uc |  707.210 uc |  839.970 uc |

- std is std::str::find()
- `us` is micro seconds
- `:en` is english haystack.
- `:ja` is japanese haystack.
- `musl` is x86_64-unknown-linux-musl
- bench on intel Q6600 @ 2.40GHz


## Changelogs

[This crate's changelog here.](https://github.com/aki-akaguma/memx/blob/main/CHANGELOG.md)


## References
