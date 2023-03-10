## Benchmark results

- compile by rustc 1.68.0 (2c8cc3432 2023-03-06)

  0. x86_64-unknown-linux- @Broadwell:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memmem              |  277.310 µs |  205.810 µs |  269.650 µs |  202.180 µs |
| libc_memmem             |  102.510 µs |   98.584 µs |  154.610 µs |  168.120 µs |
| memchr_memmem           |   90.105 µs |   90.674 µs |   93.144 µs |   93.493 µs |
| memx_memmem             |   50.684 µs |   55.409 µs |   55.514 µs |   51.777 µs |
| memx_memmem_basic       |   53.415 µs |   58.230 µs |   63.483 µs |   58.490 µs |
| memx_memmem_sse2        |   52.115 µs |   50.989 µs |   60.884 µs |   53.446 µs |

  1. x86_64-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memmem              |  482.280 µs |  451.730 µs |  475.460 µs |  452.670 µs |
| libc_memmem             |  127.980 µs |  118.560 µs |  223.010 µs |  263.560 µs |
| memchr_memmem           |  164.380 µs |  179.110 µs |  163.110 µs |  180.410 µs |
| memx_memmem             |  110.210 µs |  110.350 µs |  108.930 µs |  112.550 µs |
| memx_memmem_basic       |  111.220 µs |  110.860 µs |  112.600 µs |  110.770 µs |
| memx_memmem_sse2        |  109.590 µs |  110.030 µs |  108.980 µs |  112.860 µs |

  2. i686-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memmem              |  660.990 µs |  559.300 µs |  730.980 µs |  542.530 µs |
| libc_memmem             |  134.440 µs |  123.840 µs |  236.610 µs |  253.060 µs |
| memchr_memmem           |  447.560 µs |  438.670 µs |  445.330 µs |  439.640 µs |
| memx_memmem             |  114.100 µs |  116.850 µs |  124.920 µs |  123.740 µs |
| memx_memmem_basic       |  129.990 µs |  137.070 µs |  135.530 µs |  142.360 µs |
| memx_memmem_sse2        |  114.040 µs |  116.370 µs |  124.880 µs |  123.610 µs |

  3. i586-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memmem              |  761.790 µs |  634.170 µs |  734.140 µs |  589.500 µs |
| libc_memmem             |  133.950 µs |  123.970 µs |  252.960 µs |  266.070 µs |
| memchr_memmem           |  459.960 µs |  501.420 µs |  505.900 µs |  540.650 µs |
| memx_memmem             |  145.320 µs |  137.450 µs |  145.610 µs |  144.470 µs |
| memx_memmem_basic       |  145.070 µs |  137.590 µs |  145.760 µs |  144.520 µs |


- compile by rustc 1.53.0 (53cb7b09b 2021-06-17)

  1. x86_64-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memmem              |  513.410 us |  472.390 us |  509.130 us |  447.490 us |
| libc_memmem             |  131.950 us |  121.220 us |  219.310 us |  273.500 us |
| memchr_memmem           |  196.830 us |  225.500 us |  195.990 us |  213.690 us |
| memx_memmem             |  103.520 us |  104.140 us |  102.050 us |  103.850 us |
| memx_memmem_basic       |  105.840 us |  104.520 us |  105.040 us |  104.540 us |
| memx_memmem_sse2        |  103.510 us |  103.680 us |  104.710 us |  103.340 us |

  2. i686-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memmem              |  689.260 us |  555.490 us |  714.560 us |  583.750 us |
| libc_memmem             |  131.130 us |  120.530 us |  241.670 us |  252.080 us |
| memchr_memmem           |  472.230 us |  482.810 us |  503.740 us |  491.810 us |
| memx_memmem             |  117.360 us |  112.680 us |  128.280 us |  127.470 us |
| memx_memmem_basic       |  121.310 us |  120.250 us |  133.310 us |  121.630 us |
| memx_memmem_sse2        |  112.680 us |  109.490 us |  123.990 us |  122.480 us |

  3. i586-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memmem              |  751.850 us |  601.410 us |  719.720 us |  576.260 us |
| libc_memmem             |  131.200 us |  120.900 us |  241.750 us |  245.320 us |
| memchr_memmem           |  502.970 us |  569.400 us |  496.330 us |  564.220 us |
| memx_memmem             |  127.740 us |  124.620 us |  147.610 us |  129.860 us |
| memx_memmem_basic       |  123.320 us |  126.560 us |  142.830 us |  123.180 us |

  4. armv7-linux-androideabi:

|         `name`          |  `andr:en`  |  `andr:ja`  |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memmem              | 7409.000 us | 3891.700 us | 3709.500 us | 1918.400 us |
| libc_memmem             | 3591.800 us | 3580.600 us | 1554.300 us | 1453.900 us |
| memchr_memmem           | 3747.600 us | 3391.700 us | 2182.600 us | 1897.900 us |
| memx_memmem             | 2243.900 us | 2005.800 us |  546.910 us |  582.020 us |
| memx_memmem_basic       | 2244.100 us | 2122.200 us |  570.210 us |  549.610 us |


- `us` is micro seconds
- `:en` is english haystack.
- `:ja` is japanese haystack.
- `musl` is x86_64-unknown-linux-musl
- bench on intel Q6600 @ 2.40GHz or armv7 1.5GHz
