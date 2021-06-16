## Benchmark results

  1. x86_64-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memrchr             |  670.620 us |  383.100 us |  654.610 us |  371.280 us |
| libc_memrchr            |  432.310 us |  260.550 us |  676.470 us |  405.750 us |
| memchr_memrchr          |  428.700 us |  274.280 us |  426.520 us |  269.510 us |
| memx_memrchr            |  461.000 us |  287.400 us |  473.650 us |  295.010 us |
| memx_memrchr_basic      |  446.750 us |  290.320 us |  444.060 us |  305.120 us |
| memx_memrchr_sse2       |  460.920 us |  286.990 us |  467.890 us |  290.920 us |

  2. i686-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memrchr             |  276.000 us |  199.920 us |  398.520 us |  201.210 us |
| libc_memrchr            |  234.730 us |   77.082 us |  539.700 us |  216.280 us |
| memchr_memrchr          |  553.740 us |  213.880 us |  547.610 us |  211.840 us |
| memx_memrchr            |  373.390 us |  107.110 us |  401.700 us |  114.970 us |
| memx_memrchr_basic      |  300.910 us |  125.380 us |  325.980 us |  138.210 us |
| memx_memrchr_sse2       |  307.460 us |   89.040 us |  320.720 us |   92.805 us |

  3. i586-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memrchr             |  414.870 us |  197.680 us |  295.240 us |  202.240 us |
| libc_memrchr            |  234.880 us |   77.747 us |  608.850 us |  212.540 us |
| memchr_memrchr          |  520.250 us |  196.770 us |  509.230 us |  199.230 us |
| memx_memrchr            |  293.240 us |  125.740 us |  330.080 us |  143.590 us |
| memx_memrchr_basic      |  296.340 us |  127.530 us |  320.770 us |  137.420 us |

  4. armv7-linux-androideabi:

|         `name`          |  `andr:en`  |  `andr:ja`  |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memrchr             | 1739.600 us |  941.630 us | 1498.100 us |  843.520 us |
| libc_memrchr            | 1736.600 us | 1037.000 us | 1525.500 us | 1170.300 us |
| memchr_memrchr          | 1411.300 us |  659.830 us | 1475.600 us |  667.510 us |
| memx_memrchr            | 1027.100 us |  529.770 us | 1088.300 us |  496.660 us |
| memx_memrchr_basic      | 1023.500 us |  545.150 us | 1090.600 us |  490.000 us |

- compile by rustc 1.52.1 (9bc8c42bb 2021-05-09)
- `us` is micro seconds
- `:en` is english haystack.
- `:ja` is japanese haystack.
- `musl` is x86_64-unknown-linux-musl
- bench on intel Q6600 @ 2.40GHz or armv7 1.5GHz
