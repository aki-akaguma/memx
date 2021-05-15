# memx
memory functions like a libc memcmp(), memchr(), memmem(), memcpy(), memset()

## Features

* Rewriting with rust lang.
* minimum support: rustc 1.41.1 (f3e1a954d 2020-02-24)

## Todo

- [ ] Support the zero overhead trait.

## Benchmark

- compile by rustc 1.52.1 (9bc8c42bb 2021-05-09)

  1. x86_64:

|         `name`          | `bench:en`  | `bench:ja`  |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memchr              |  225.480 uc |  225.440 uc |  226.240 uc |  226.010 uc |
| memx_memchr             |  258.560 uc |   81.691 uc |  262.690 uc |   81.100 uc |
| memx_memchr_basic       |  255.280 uc |   94.201 uc |  261.980 uc |   94.397 uc |
| memx_memchr_libc        |  290.440 uc |   83.619 uc |  614.890 uc |  209.530 uc |
| memchr_memchr           |  220.390 uc |   65.749 uc |  226.130 uc |   64.531 uc |

|         `name`          | `bench:en`  | `bench:ja`  |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memcmp              |  253.010 uc |  334.350 uc |  419.660 uc |  372.390 uc |
| memx_memcmp             |  231.610 uc |  282.000 uc |  229.550 uc |  282.410 uc |
| memx_memcmp_basic       |  568.900 uc |  496.000 uc |  567.050 uc |  495.880 uc |
| memx_memcmp_libc        |  350.580 uc |  425.370 uc |  537.150 uc |  476.790 uc |

|         `name`          | `bench:en`  | `bench:ja`  |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memeq               |  277.660 uc |  330.270 uc |  570.310 uc |  523.730 uc |
| memx_memeq              |  190.760 uc |  252.530 uc |  188.910 uc |  253.260 uc |
| memx_memeq_basic        |  569.100 uc |  494.500 uc |  568.900 uc |  494.340 uc |
| memx_memeq_libc         |  338.200 uc |  404.930 uc |  604.540 uc |  658.000 uc |

  2. i686:

|         `name`          | `bench:en`  | `bench:ja`  |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memchr              |   83.806 uc |   83.824 uc |   85.359 uc |   85.323 uc |
| memx_memchr             |  335.410 uc |  132.370 uc |  336.920 uc |  138.760 uc |
| memx_memchr_basic       |  302.390 uc |  131.960 uc |  339.340 uc |  139.240 uc |
| memx_memchr_libc        |  372.080 uc |  100.040 uc |  580.580 uc |  153.270 uc |
| memchr_memchr           |  488.860 uc |  194.320 uc |  533.760 uc |  199.280 uc |

|         `name`          | `bench:en`  | `bench:ja`  |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memcmp              |  262.970 uc |  365.830 uc |  562.350 uc |  526.390 uc |
| memx_memcmp             |  308.270 uc |  416.620 uc |  320.760 uc |  444.170 uc |
| memx_memcmp_basic       |  610.440 uc |  842.000 uc |  621.930 uc |  863.560 uc |
| memx_memcmp_libc        |  402.930 uc |  529.090 uc |  646.850 uc |  728.580 uc |

|         `name`          | `bench:en`  | `bench:ja`  |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memeq               |  285.640 uc |  350.920 uc |  632.690 uc |  657.960 uc |
| memx_memeq              |  289.600 uc |  383.810 uc |  307.630 uc |  419.700 uc |
| memx_memeq_basic        |  587.200 uc |  525.210 uc |  578.270 uc |  531.900 uc |
| memx_memeq_libc         |  367.890 uc |  482.970 uc |  720.300 uc |  857.800 uc |

- std is std::str::find()
- `us` is micro seconds
- `:en` is english haystack.
- `:ja` is japanese haystack.
- `musl` is x86_64-unknown-linux-musl
- bench on intel Q6600 @ 2.40GHz


## Changelogs

[This crate's changelog here.](https://github.com/aki-akaguma/memx/blob/main/CHANGELOG.md)


## References
