## Benchmark results

- compile by rustc 1.68.0 (2c8cc3432 2023-03-06)

  0. x86_64-unknown-linux- @Broadwell:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memnechr            |  194.370 µs |  115.540 µs |  210.270 µs |  124.780 µs |
| memx_memnechr           |  190.390 µs |   98.596 µs |  200.710 µs |  110.450 µs |
| memx_memnechr_basic     |  191.760 µs |   99.658 µs |  216.470 µs |  115.580 µs |
| memx_memnechr_sse2      |  176.930 µs |   87.416 µs |  190.310 µs |   94.458 µs |

  1. x86_64-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memnechr            |  379.190 µs |  211.770 µs |  367.690 µs |  210.860 µs |
| memx_memnechr           |  317.890 µs |  143.870 µs |  316.710 µs |  145.170 µs |
| memx_memnechr_basic     |  311.880 µs |  158.930 µs |  312.450 µs |  161.080 µs |
| memx_memnechr_sse2      |  295.140 µs |  133.040 µs |  291.220 µs |  135.010 µs |

  2. i686-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memnechr            |  389.590 µs |  239.400 µs |  393.910 µs |  229.200 µs |
| memx_memnechr           |  346.650 µs |  152.870 µs |  348.460 µs |  152.610 µs |
| memx_memnechr_basic     |  307.930 µs |  160.480 µs |  311.940 µs |  162.180 µs |
| memx_memnechr_sse2      |  331.930 µs |  150.050 µs |  333.790 µs |  155.440 µs |

  3. i586-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memnechr            |  346.010 µs |  214.610 µs |  356.240 µs |  224.160 µs |
| memx_memnechr           |  266.330 µs |  158.130 µs |  290.040 µs |  171.650 µs |
| memx_memnechr_basic     |  265.420 µs |  159.030 µs |  288.830 µs |  171.570 µs |


- compile by rustc 1.53.0 (53cb7b09b 2021-06-17)

  1. x86_64-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memnechr            |  368.560 us |  212.620 us |  357.730 us |  214.030 us |
| memx_memnechr           |  273.090 us |  126.310 us |  264.900 us |  127.690 us |
| memx_memnechr_basic     |  305.920 us |  162.800 us |  304.140 us |  164.920 us |
| memx_memnechr_sse2      |  267.470 us |  125.320 us |  261.060 us |  127.270 us |

  2. i686-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memnechr            |  431.530 us |  235.700 us |  496.540 us |  247.550 us |
| memx_memnechr           |  484.690 us |  199.290 us |  539.420 us |  219.530 us |
| memx_memnechr_basic     |  447.280 us |  206.660 us |  555.910 us |  221.390 us |
| memx_memnechr_sse2      |  430.740 us |  180.280 us |  452.040 us |  190.260 us |

  3. i586-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memnechr            |  388.950 us |  240.800 us |  387.410 us |  237.050 us |
| memx_memnechr           |  354.960 us |  191.770 us |  370.900 us |  195.360 us |
| memx_memnechr_basic     |  357.170 us |  195.810 us |  362.970 us |  189.400 us |

  4. armv7-linux-androideabi:

|         `name`          |  `andr:en`  |  `andr:ja`  |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memchr              | 1232.100 us |  950.750 us | 1262.200 us | 1047.500 us |
| libc_memchr             | 1874.100 us | 1066.300 us | 1865.200 us |  838.210 us |
| memchr_memchr           | 1391.400 us |  642.260 us | 1441.300 us |  649.600 us |
| memx_memchr             | 1042.000 us |  555.030 us | 1129.400 us |  519.740 us |
| memx_memchr_basic       | 1005.600 us |  573.360 us | 1095.400 us |  525.020 us |

- `us` is micro seconds
- `:en` is english haystack.
- `:ja` is japanese haystack.
- `musl` is x86_64-unknown-linux-musl
- bench on intel Q6600 @ 2.40GHz or armv7 1.5GHz
