## Benchmark results

- compile by rustc 1.69.0 (84c898d65 2023-04-16)

  0. x86_64-unknown-linux- @i5-4570:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memrmem             |   10.646 µs |   11.440 µs |   10.667 µs |   11.331 µs |
| libc_memrmem            |   39.404 µs |   80.604 µs |   25.872 µs |   55.201 µs |
| memchr_memrmem          |    9.673 µs |   10.075 µs |    9.730 µs |   10.018 µs |
| memx_memrmem            |    0.712 µs |    1.228 µs |    0.675 µs |    1.188 µs |
| memx_memrmem_basic      |    1.803 µs |    3.487 µs |    1.800 µs |    3.493 µs |
| memx_memrmem_sse2       |    0.834 µs |    1.522 µs |    0.832 µs |    1.531 µs |
| memx_memrmem_avx2       |    0.706 µs |    1.223 µs |    0.675 µs |    1.185 µs |

  1. x86_64-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memrmem             |   33.140 µs |   38.388 µs |   33.314 µs |   39.369 µs |
| libc_memrmem            |  127.580 µs |  360.580 µs |   52.124 µs |  110.220 µs |
| memchr_memrmem          |   35.112 µs |   36.081 µs |   34.711 µs |   35.937 µs |
| memx_memrmem            |    2.347 µs |    4.372 µs |    2.515 µs |    4.431 µs |
| memx_memrmem_basic      |    4.951 µs |    9.539 µs |    5.120 µs |    9.406 µs |
| memx_memrmem_sse2       |    2.199 µs |    4.371 µs |    2.319 µs |    4.467 µs |

  2. i686-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memrmem             |   38.452 µs |   45.316 µs |   40.392 µs |   49.508 µs |
| memchr_memrmem          |   40.873 µs |   43.975 µs |   42.112 µs |   46.161 µs |
| memx_memrmem            |    2.339 µs |    4.339 µs |    2.378 µs |    4.780 µs |
| memx_memrmem_basic      |    8.248 µs |   16.257 µs |    8.386 µs |   16.906 µs |
| memx_memrmem_sse2       |    2.211 µs |    4.407 µs |    2.253 µs |    4.772 µs |

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
- @i5-4570: bench on intel i5-4570 @ 3.2GHz
- @Q6600: bench on intel Q6600 @ 2.40GHz or armv7 1.5GHz
