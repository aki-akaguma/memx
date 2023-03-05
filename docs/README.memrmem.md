## Benchmark results

- compile by rustc 1.67.1 (d5a82bbd2 2023-02-07)

  1. x86_64-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memrmem             |  527.030 µs |  725.140 µs |  535.010 µs |  712.460 µs |
| libc_memrmem            | 1702.600 µs | 1772.300 µs |  675.360 µs |  730.210 µs |
| memchr_memrmem          |  607.250 µs |  612.140 µs |  608.380 µs |  615.660 µs |
| memx_memrmem            |  120.460 µs |  118.350 µs |  120.850 µs |  118.330 µs |
| memx_memrmem_basic      |  134.450 µs |  111.980 µs |  107.230 µs |  112.930 µs |
| memx_memrmem_sse2       |  120.510 µs |  118.350 µs |  121.010 µs |  118.120 µs |

  2. i686-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memrmem             |  746.890 µs |  890.370 µs |  783.410 µs |  888.780 µs |
| memchr_memrmem          |  863.120 µs |  883.090 µs |  861.680 µs |  845.020 µs |
| memx_memrmem            |  123.360 µs |  126.070 µs |  134.870 µs |  140.470 µs |
| memx_memrmem_basic      |  128.050 µs |  125.070 µs |  127.410 µs |  130.410 µs |
| memx_memrmem_sse2       |  123.420 µs |  126.360 µs |  135.110 µs |  140.570 µs |

  3. i586-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memrmem             |  780.520 µs |  893.370 µs |  755.140 µs |  692.380 µs |
| memchr_memrmem          |  865.260 µs |  997.990 µs |  861.240 µs | 1208.200 µs |
| memx_memrmem            |  153.470 µs |  149.620 µs |  145.540 µs |  144.070 µs |
| memx_memrmem_basic      |  153.210 µs |  149.290 µs |  146.900 µs |  142.460 µs |


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
