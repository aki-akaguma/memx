## Benchmark results

- compile by rustc 1.66.0 (69f9c33d7 2022-12-12)

  1. x86_64-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memrmem             |  533.220 µs |  719.900 µs |  529.920 µs |  758.690 µs |
| libc_memrmem            | 1635.500 µs | 1709.100 µs |  677.460 µs |  731.650 µs |
| memchr_memrmem          |  567.880 µs |  603.540 µs |  566.890 µs |  595.790 µs |
| memx_memrmem            |  117.140 µs |  115.130 µs |  116.640 µs |  118.710 µs |
| memx_memrmem_basic      |  106.800 µs |  106.310 µs |  106.150 µs |  106.940 µs |
| memx_memrmem_sse2       |  116.490 µs |  115.230 µs |  117.270 µs |  117.510 µs |

  2. i686-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memrmem             |  711.530 µs |  880.330 µs |  866.880 µs |  945.290 µs |
| memchr_memrmem          |  798.910 µs |  809.580 µs |  853.150 µs |  851.050 µs |
| memx_memrmem            |  118.910 µs |  116.880 µs |  131.880 µs |  131.570 µs |
| memx_memrmem_basic      |  131.940 µs |  127.540 µs |  129.170 µs |  127.360 µs |
| memx_memrmem_sse2       |  118.880 µs |  116.840 µs |  131.700 µs |  131.340 µs |

- compile by rustc 1.65.0 (897e37553 2022-11-02)

  3. i586-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memrmem             |  786.700 µs |  981.280 µs |  754.460 µs |  700.420 µs |
| memchr_memrmem          |  879.130 µs | 1008.200 µs |  903.960 µs | 1044.900 µs |
| memx_memrmem            |  131.160 µs |  129.880 µs |  126.770 µs |  127.520 µs |
| memx_memrmem_basic      |  131.120 µs |  129.760 µs |  127.360 µs |  128.080 µs |


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
