## Benchmark results

- compile by rustc 1.68.1 (8460ca823 2023-03-20)

  0. x86_64-unknown-linux- @Broadwell:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memrmem             |   15.409 µs |   16.358 µs |   15.436 µs |   16.372 µs |
| libc_memrmem            |   51.359 µs |  105.180 µs |   37.579 µs |   80.683 µs |
| memchr_memrmem          |   14.002 µs |   14.483 µs |   14.323 µs |   14.296 µs |
| memx_memrmem            |    0.920 µs |    1.499 µs |    0.829 µs |    1.441 µs |
| memx_memrmem_basic      |    2.868 µs |    3.434 µs |    2.844 µs |    3.412 µs |
| memx_memrmem_sse2       |    1.070 µs |    1.892 µs |    1.027 µs |    1.911 µs |
| memx_memrmem_avx2       |    0.851 µs |    1.448 µs |    0.775 µs |    1.423 µs |

  1. x86_64-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memrmem             |   32.259 µs |   37.834 µs |   32.433 µs |   37.247 µs |
| libc_memrmem            |  124.980 µs |  353.530 µs |   57.325 µs |  120.110 µs |
| memchr_memrmem          |   34.713 µs |   35.521 µs |   33.605 µs |   35.362 µs |
| memx_memrmem            |    1.951 µs |    3.776 µs |    2.108 µs |    3.873 µs |
| memx_memrmem_basic      |    5.663 µs |    8.242 µs |    5.383 µs |    8.413 µs |
| memx_memrmem_sse2       |    1.799 µs |    3.656 µs |    1.974 µs |    3.759 µs |

  2. i686-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memrmem             |   38.581 µs |   45.835 µs |   41.301 µs |   49.025 µs |
| memchr_memrmem          |   40.109 µs |   43.965 µs |   42.732 µs |   45.831 µs |
| memx_memrmem            |    2.111 µs |    3.754 µs |    2.292 µs |    4.255 µs |
| memx_memrmem_basic      |    8.702 µs |   11.677 µs |    8.756 µs |   11.528 µs |
| memx_memrmem_sse2       |    2.074 µs |    3.691 µs |    2.185 µs |    3.993 µs |

  3. i586-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memrmem             |   37.930 µs |   48.551 µs |   39.645 µs |   49.753 µs |
| memchr_memrmem          |   41.509 µs |   60.789 µs |   42.017 µs |   60.946 µs |
| memx_memrmem            |    2.232 µs |    3.821 µs |    2.385 µs |    4.321 µs |
| memx_memrmem_basic      |    8.390 µs |   11.492 µs |    8.604 µs |   11.665 µs |


- compile by rustc 1.53.0 (53cb7b09b 2021-06-17)

  1. x86_64-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memrmem             |  543.460 us |  759.250 us |  536.440 us |  759.370 us |
| libc_memrmem            | 1687.500 us | 1749.600 us |  667.440 us |  720.360 us |
| memchr_memrmem          |  643.720 us |  657.040 us |  654.540 us |  669.780 us |
| memx_memrmem            |  113.400 us |  113.700 us |  114.040 us |  113.620 us |
| memx_memrmem_basic      |  110.120 us |  108.700 us |  110.110 us |  110.010 us |
| memx_memrmem_sse2       |  113.460 us |  112.400 us |  113.560 us |  112.770 us |

  2. i686-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memrmem             |  730.030 us |  886.480 us |  768.880 us |  853.430 us |
| memchr_memrmem          |  820.390 us |  824.070 us |  881.630 us |  851.500 us |
| memx_memrmem            |  126.400 us |  127.140 us |  139.320 us |  135.380 us |
| memx_memrmem_basic      |  125.310 us |  125.180 us |  136.860 us |  128.350 us |
| memx_memrmem_sse2       |  122.240 us |  123.570 us |  131.970 us |  131.880 us |

  3. i586-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memrmem             |  720.990 us |  838.830 us |  762.540 us |  835.690 us |
| memchr_memrmem          |  868.280 us | 1279.700 us |  868.220 us | 1257.300 us |
| memx_memrmem            |  130.680 us |  126.340 us |  144.500 us |  145.400 us |
| memx_memrmem_basic      |  130.960 us |  128.030 us |  144.730 us |  131.690 us |

  4. armv7-linux-androideabi:

|         `name`          |  `andr:en`  |  `andr:ja`  |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memrmem             | 5735.900 us | 3668.800 us | 3054.100 us | 2006.900 us |
| memchr_memrmem          | 4526.400 us | 4486.400 us | 3291.500 us | 3245.300 us |
| memx_memrmem            | 2013.100 us | 2139.300 us |  544.170 us |  534.090 us |
| memx_memrmem_basic      | 2005.800 us | 2027.800 us |  556.750 us |  542.600 us |

- `µs` is micro seconds
- `:en` is english haystack or short length.
- `:ja` is japanese haystack or long length.
- `gnu` is x86_64-unknown-linux-gnu
- `musl` is x86_64-unknown-linux-musl
- bench on intel Q6600 @ 2.40GHz or armv7 1.5GHz
