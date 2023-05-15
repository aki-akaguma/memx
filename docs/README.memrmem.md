## Benchmark results

- compile by rustc 1.69.0 (84c898d65 2023-04-16)

  0. x86_64-unknown-linux- @i5-4570:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memrmem             |   10.615 µs |   11.433 µs |   10.610 µs |   11.398 µs |
| libc_memrmem            |   39.367 µs |   80.704 µs |   25.829 µs |   55.172 µs |
| memchr_memrmem          |   10.418 µs |   10.644 µs |    9.633 µs |   10.017 µs |
| memx_memrmem            |    0.721 µs |    1.300 µs |    0.707 µs |    1.237 µs |
| memx_memrmem_basic      |    2.171 µs |    4.316 µs |    2.181 µs |    4.422 µs |
| memx_memrmem_sse2       |    1.009 µs |    1.885 µs |    0.998 µs |    1.867 µs |
| memx_memrmem_avx2       |    0.715 µs |    1.302 µs |    0.701 µs |    1.235 µs |

  1. x86_64-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memrmem             |   32.608 µs |   38.152 µs |   32.825 µs |   38.481 µs |
| libc_memrmem            |  125.440 µs |  353.490 µs |   51.224 µs |  108.320 µs |
| memchr_memrmem          |   35.166 µs |   35.402 µs |   34.239 µs |   35.341 µs |
| memx_memrmem            |    2.653 µs |    5.458 µs |    2.777 µs |    5.197 µs |
| memx_memrmem_basic      |    5.914 µs |   11.977 µs |    5.893 µs |   11.939 µs |
| memx_memrmem_sse2       |    2.508 µs |    5.452 µs |    2.652 µs |    5.219 µs |

  2. i686-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memrmem             |   38.417 µs |   44.786 µs |   40.566 µs |   48.517 µs |
| memchr_memrmem          |   41.105 µs |   44.202 µs |   42.344 µs |   45.985 µs |
| memx_memrmem            |    3.004 µs |    5.273 µs |    3.088 µs |    5.771 µs |
| memx_memrmem_basic      |   10.626 µs |   20.918 µs |   11.152 µs |   22.065 µs |
| memx_memrmem_sse2       |    2.900 µs |    5.252 µs |    2.934 µs |    5.792 µs |

  3. i586-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memrmem             |   38.281 µs |   50.303 µs |   40.584 µs |   52.088 µs |
| memchr_memrmem          |   40.560 µs |   61.418 µs |   41.814 µs |   60.823 µs |
| memx_memrmem            |    3.226 µs |    5.470 µs |    3.267 µs |    6.003 µs |
| memx_memrmem_basic      |   11.026 µs |   21.526 µs |   11.289 µs |   22.543 µs |


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
- @i5-4570: bench on intel i5-4570 @ 3.2GHz
- @Q6600: bench on intel Q6600 @ 2.40GHz or armv7 1.5GHz
