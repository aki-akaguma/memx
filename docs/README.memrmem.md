## Benchmark results

- compile by rustc 1.68.0 (2c8cc3432 2023-03-06)

  0. x86_64-unknown-linux- @Broadwell:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memrmem             |  338.460 µs |  335.350 µs |  332.440 µs |  324.600 µs |
| libc_memrmem            |  887.470 µs |  886.430 µs |  427.190 µs |  435.270 µs |
| memchr_memrmem          |  370.080 µs |  360.630 µs |  370.170 µs |  356.150 µs |
| memx_memrmem            |   54.428 µs |   55.647 µs |   56.894 µs |   57.728 µs |
| memx_memrmem_basic      |   52.604 µs |   54.276 µs |   60.470 µs |   63.835 µs |
| memx_memrmem_sse2       |   53.249 µs |   55.395 µs |   57.481 µs |   54.494 µs |

  1. x86_64-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memrmem             |  529.150 µs |  728.500 µs |  529.900 µs |  718.600 µs |
| libc_memrmem            | 1643.600 µs | 1715.500 µs |  678.470 µs |  735.270 µs |
| memchr_memrmem          |  588.630 µs |  630.180 µs |  593.780 µs |  619.830 µs |
| memx_memrmem            |  122.690 µs |  119.430 µs |  122.440 µs |  119.590 µs |
| memx_memrmem_basic      |  107.250 µs |  111.660 µs |  111.740 µs |  113.930 µs |
| memx_memrmem_sse2       |  122.170 µs |  118.820 µs |  121.710 µs |  118.450 µs |

  2. i686-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memrmem             |  715.880 µs |  883.580 µs |  867.320 µs |  956.740 µs |
| memchr_memrmem          |  795.510 µs |  815.060 µs |  834.790 µs |  842.360 µs |
| memx_memrmem            |  123.740 µs |  126.260 µs |  134.700 µs |  135.410 µs |
| memx_memrmem_basic      |  123.970 µs |  122.530 µs |  133.850 µs |  126.340 µs |
| memx_memrmem_sse2       |  123.780 µs |  126.100 µs |  134.940 µs |  135.550 µs |

  3. i586-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memrmem             |  788.290 µs |  941.630 µs |  778.570 µs |  705.460 µs |
| memchr_memrmem          |  842.130 µs |  974.050 µs |  856.750 µs |  998.350 µs |
| memx_memrmem            |  158.850 µs |  147.870 µs |  145.610 µs |  144.980 µs |
| memx_memrmem_basic      |  151.330 µs |  147.890 µs |  145.780 µs |  144.920 µs |


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
