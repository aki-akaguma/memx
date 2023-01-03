## Benchmark results

- compile by rustc 1.65.0 (897e37553 2022-11-02)

  1. x86_64-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memrmem             |  527.970 µs | 1094.200 µs |  521.910 µs |  719.320 µs |
| libc_memrmem            | 1703.600 µs | 2004.200 µs |  610.880 µs |  684.360 µs |
| memchr_memrmem          |  661.440 µs |  647.690 µs |  652.130 µs |  662.680 µs |
| memx_memrmem            |  119.910 µs |  118.020 µs |  119.200 µs |  117.100 µs |
| memx_memrmem_basic      |  109.040 µs |  108.980 µs |  107.530 µs |  107.610 µs |
| memx_memrmem_sse2       |  119.500 µs |  117.420 µs |  118.860 µs |  117.040 µs |

  2. i686-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memrmem             |  696.600 µs |  851.210 µs |  752.360 µs |  922.350 µs |
| memchr_memrmem          |  898.960 µs |  882.900 µs |  948.240 µs |  917.910 µs |
| memx_memrmem            |  140.810 µs |  142.140 µs |  134.400 µs |  141.550 µs |
| memx_memrmem_basic      |  134.090 µs |  128.400 µs |  146.020 µs |  138.240 µs |
| memx_memrmem_sse2       |  136.140 µs |  136.990 µs |  129.380 µs |  135.740 µs |

  3. i586-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memrmem             |  775.320 µs |  689.950 µs |  792.560 µs |  730.160 µs |
| memchr_memrmem          |  854.250 µs | 1256.400 µs |  904.840 µs | 1238.600 µs |
| memx_memrmem            |  135.780 µs |  134.430 µs |  152.810 µs |  139.730 µs |
| memx_memrmem_basic      |  135.840 µs |  134.410 µs |  156.640 µs |  140.560 µs |


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
