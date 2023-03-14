## Benchmark results

- compile by rustc 1.68.0 (2c8cc3432 2023-03-06)

  0. x86_64-unknown-linux- @Broadwell:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memrmem             |  342.100 µs |  324.430 µs |  338.440 µs |  326.180 µs |
| libc_memrmem            |  836.610 µs |  840.070 µs |  566.210 µs |  590.150 µs |
| memchr_memrmem          |  374.270 µs |  359.970 µs |  376.120 µs |  361.820 µs |
| memx_memrmem            |   51.367 µs |   52.722 µs |   54.466 µs |   54.490 µs |
| memx_memrmem_basic      |   52.436 µs |   51.100 µs |   58.923 µs |   65.466 µs |
| memx_memrmem_sse2       |   51.595 µs |   53.343 µs |   54.024 µs |   55.180 µs |

  1. x86_64-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memrmem             |  526.290 µs |  703.500 µs |  522.620 µs |  749.550 µs |
| libc_memrmem            | 1672.900 µs | 1739.400 µs |  663.110 µs |  717.070 µs |
| memchr_memrmem          |  574.700 µs |  601.440 µs |  587.580 µs |  598.060 µs |
| memx_memrmem            |  105.680 µs |  104.530 µs |  106.370 µs |  104.910 µs |
| memx_memrmem_basic      |   96.177 µs |   92.227 µs |   97.046 µs |   92.570 µs |
| memx_memrmem_sse2       |  104.630 µs |  103.870 µs |  106.270 µs |  104.320 µs |

  2. i686-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memrmem             |  698.820 µs |  827.480 µs |  842.160 µs |  934.410 µs |
| memchr_memrmem          |  775.830 µs |  807.030 µs |  806.560 µs |  806.140 µs |
| memx_memrmem            |  112.590 µs |  112.450 µs |  114.870 µs |  114.760 µs |
| memx_memrmem_basic      |  120.560 µs |  120.650 µs |  116.100 µs |  116.600 µs |
| memx_memrmem_sse2       |  112.580 µs |  112.480 µs |  116.160 µs |  114.720 µs |

  3. i586-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memrmem             |  767.240 µs |  916.100 µs |  751.680 µs |  682.120 µs |
| memchr_memrmem          |  821.820 µs |  942.950 µs |  833.960 µs |  969.990 µs |
| memx_memrmem            |  112.680 µs |  114.910 µs |  118.450 µs |  119.770 µs |
| memx_memrmem_basic      |  112.510 µs |  114.870 µs |  118.770 µs |  120.470 µs |


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


- `us` is micro seconds
- `:en` is english haystack.
- `:ja` is japanese haystack.
- `musl` is x86_64-unknown-linux-musl
- bench on intel Q6600 @ 2.40GHz or armv7 1.5GHz
