## Benchmark results

- compile by rustc 1.68.1 (8460ca823 2023-03-20)

  0. x86_64-unknown-linux- @Broadwell:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memrmem             |   15.445 µs |   16.863 µs |   15.282 µs |   16.571 µs |
| libc_memrmem            |   57.657 µs |  118.290 µs |   49.718 µs |  105.240 µs |
| memchr_memrmem          |   14.150 µs |   14.635 µs |   14.295 µs |   14.313 µs |
| memx_memrmem            |    0.870 µs |    1.475 µs |    0.778 µs |    1.420 µs |
| memx_memrmem_basic      |    2.891 µs |    3.435 µs |    2.845 µs |    3.484 µs |
| memx_memrmem_sse2       |    1.012 µs |    1.885 µs |    1.038 µs |    1.941 µs |
| memx_memrmem_avx2       |    0.855 µs |    1.441 µs |    0.766 µs |    1.400 µs |

  1. x86_64-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memrmem             |   32.437 µs |   38.235 µs |   32.010 µs |   37.789 µs |
| libc_memrmem            |  121.190 µs |  340.790 µs |   57.338 µs |  120.210 µs |
| memchr_memrmem          |   35.172 µs |   35.361 µs |   34.609 µs |   35.386 µs |
| memx_memrmem            |    1.771 µs |    3.662 µs |    1.997 µs |    3.732 µs |
| memx_memrmem_basic      |    5.786 µs |    8.147 µs |    5.267 µs |    8.239 µs |
| memx_memrmem_sse2       |    1.758 µs |    3.657 µs |    1.928 µs |    3.727 µs |

  2. i686-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memrmem             |   38.599 µs |   45.307 µs |   40.774 µs |   50.612 µs |
| memchr_memrmem          |   40.733 µs |   44.172 µs |   42.166 µs |   46.761 µs |
| memx_memrmem            |    2.127 µs |    3.669 µs |    2.201 µs |    4.094 µs |
| memx_memrmem_basic      |    8.680 µs |   11.751 µs |    8.771 µs |   11.470 µs |
| memx_memrmem_sse2       |    2.183 µs |    3.715 µs |    2.170 µs |    4.037 µs |

  3. i586-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memrmem             |   37.647 µs |   49.384 µs |   39.786 µs |   51.267 µs |
| memchr_memrmem          |   41.158 µs |   60.964 µs |   42.408 µs |   60.627 µs |
| memx_memrmem            |    2.190 µs |    3.662 µs |    2.285 µs |    4.107 µs |
| memx_memrmem_basic      |    8.439 µs |   11.450 µs |    8.626 µs |   11.639 µs |


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
