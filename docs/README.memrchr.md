## Benchmark results

- compile by rustc 1.67.1 (d5a82bbd2 2023-02-07)

  1. x86_64-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memrchr             |  507.770 µs |  244.730 µs |  507.430 µs |  244.880 µs |
| libc_memrchr            |  219.720 µs |   74.163 µs |  551.420 µs |  270.120 µs |
| memchr_memrchr          |  260.040 µs |   79.001 µs |  257.660 µs |   79.365 µs |
| memx_memrchr            |  264.660 µs |   76.173 µs |  265.300 µs |   76.671 µs |
| memx_memrchr_basic      |  198.580 µs |   86.721 µs |  196.660 µs |   89.773 µs |
| memx_memrchr_sse2       |  248.530 µs |   71.961 µs |  247.920 µs |   72.782 µs |

  2. i686-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memrchr             |  278.610 µs |  202.130 µs |  277.060 µs |  197.070 µs |
| libc_memrchr            |  237.800 µs |   78.849 µs |  558.570 µs |  216.860 µs |
| memchr_memrchr          |  376.750 µs |  151.660 µs |  355.960 µs |  148.360 µs |
| memx_memrchr            |  265.420 µs |   75.032 µs |  304.510 µs |   85.352 µs |
| memx_memrchr_basic      |  197.840 µs |   99.773 µs |  202.510 µs |  102.440 µs |
| memx_memrchr_sse2       |  257.540 µs |   72.214 µs |  265.530 µs |   76.436 µs |

  3. i586-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memrchr             |  409.440 µs |  202.600 µs |  306.790 µs |  204.190 µs |
| libc_memrchr            |  237.290 µs |   78.064 µs |  593.070 µs |  218.930 µs |
| memchr_memrchr          |  411.980 µs |  175.630 µs |  399.070 µs |  176.650 µs |
| memx_memrchr            |  194.050 µs |   98.782 µs |  199.700 µs |   97.236 µs |
| memx_memrchr_basic      |  195.140 µs |   98.761 µs |  194.100 µs |   97.232 µs |


- compile by rustc 1.53.0 (53cb7b09b 2021-06-17)

  1. x86_64-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memrchr             |  510.110 us |  231.010 us |  500.410 us |  241.550 us |
| libc_memrchr            |  216.100 us |   73.073 us |  543.330 us |  266.120 us |
| memchr_memrchr          |  222.390 us |   69.625 us |  221.260 us |   69.196 us |
| memx_memrchr            |  257.000 us |   73.247 us |  257.010 us |   73.865 us |
| memx_memrchr_basic      |  260.940 us |   95.468 us |  260.350 us |   96.719 us |
| memx_memrchr_sse2       |  251.610 us |   72.529 us |  252.000 us |   73.265 us |

  2. i686-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memrchr             |  389.890 us |  200.600 us |  285.170 us |  200.080 us |
| libc_memrchr            |  273.230 us |   77.156 us |  606.080 us |  210.860 us |
| memchr_memrchr          |  559.270 us |  219.320 us |  525.350 us |  205.200 us |
| memx_memrchr            |  353.940 us |   97.693 us |  410.320 us |  112.270 us |
| memx_memrchr_basic      |  274.110 us |  121.560 us |  272.420 us |  119.450 us |
| memx_memrchr_sse2       |  276.410 us |   79.334 us |  314.740 us |   86.983 us |

  3. i586-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memrchr             |  300.750 us |  203.110 us |  388.990 us |  193.850 us |
| libc_memrchr            |  233.760 us |   77.464 us |  546.530 us |  208.460 us |
| memchr_memrchr          |  523.880 us |  199.920 us |  474.810 us |  185.200 us |
| memx_memrchr            |  272.020 us |  125.010 us |  281.410 us |  120.190 us |
| memx_memrchr_basic      |  276.620 us |  122.500 us |  280.800 us |  119.710 us |

  4. armv7-linux-androideabi:

|         `name`          |  `andr:en`  |  `andr:ja`  |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memrchr             | 3066.700 us | 2229.900 us | 1380.100 us |  905.320 us |
| libc_memrchr            | 3308.500 us | 2453.900 us | 1543.800 us | 1285.000 us |
| memchr_memrchr          | 2925.400 us | 2269.500 us | 1655.700 us |  687.610 us |
| memx_memrchr            | 2507.200 us | 1932.400 us | 1095.700 us |  516.470 us |
| memx_memrchr_basic      | 2503.900 us | 1978.000 us | 1136.900 us |  496.970 us |

- `us` is micro seconds
- `:en` is english haystack.
- `:ja` is japanese haystack.
- `musl` is x86_64-unknown-linux-musl
- bench on intel Q6600 @ 2.40GHz or armv7 1.5GHz
