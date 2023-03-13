## Benchmark results

- compile by rustc 1.68.0 (2c8cc3432 2023-03-06)

  0. x86_64-unknown-linux- @Broadwell:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memcmp              |   50.911 µs |   75.389 µs |  115.900 µs |  318.450 µs |
| libc_memcmp             |   57.942 µs |   77.056 µs |  118.290 µs |  316.640 µs |
| memx_memcmp             |   49.382 µs |   75.064 µs |   49.938 µs |   73.346 µs |
| memx_memcmp_basic       |   49.229 µs |   73.719 µs |   49.909 µs |   72.778 µs |
| memx_memcmp_sse2        |   49.135 µs |   73.359 µs |   49.860 µs |   73.803 µs |

  1. x86_64-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memcmp              |  148.600 µs |  263.870 µs |  247.560 µs |  431.610 µs |
| libc_memcmp             |  155.280 µs |  267.730 µs |  249.240 µs |  430.010 µs |
| memx_memcmp             |   92.076 µs |  143.620 µs |   93.754 µs |  139.620 µs |
| memx_memcmp_basic       |   92.367 µs |  143.480 µs |   93.653 µs |  139.290 µs |
| memx_memcmp_sse2        |   92.329 µs |  143.550 µs |   94.390 µs |  139.390 µs |

  2. i686-unknown-linux-:
  
|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memcmp              |  164.580 µs |  290.150 µs |  228.840 µs |  470.960 µs |
| libc_memcmp             |  170.340 µs |  299.090 µs |  216.930 µs |  463.060 µs |
| memx_memcmp             |  113.430 µs |  246.430 µs |  114.920 µs |  260.110 µs |
| memx_memcmp_basic       |  100.390 µs |  229.860 µs |  101.030 µs |  240.180 µs |
| memx_memcmp_sse2        |   99.179 µs |  229.730 µs |  101.200 µs |  240.350 µs |

  3. i586-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memcmp              |  170.450 µs |  294.110 µs |  259.630 µs |  459.860 µs |
| libc_memcmp             |  167.520 µs |  300.590 µs |  267.650 µs |  478.660 µs |
| memx_memcmp             |   99.509 µs |  250.480 µs |  101.820 µs |  209.500 µs |
| memx_memcmp_basic       |   98.178 µs |  245.190 µs |  101.120 µs |  209.660 µs |


- compile by rustc 1.53.0 (53cb7b09b 2021-06-17)

  1. x86_64-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memcmp              |  155.810 us |  173.740 us |  253.170 us |  343.250 us |
| libc_memcmp             |  175.920 us |  187.280 us |  259.290 us |  344.760 us |
| memx_memcmp             |  140.330 us |  140.790 us |  134.440 us |  130.510 us |
| memx_memcmp_basic       |  136.670 us |  139.910 us |  133.930 us |  129.710 us |
| memx_memcmp_sse2        |  134.700 us |  179.240 us |  131.100 us |  167.600 us |

  2. i686-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memcmp              |  160.610 us |  258.220 us |  336.550 us |  475.040 us |
| libc_memcmp             |  182.480 us |  214.320 us |  331.690 us |  483.450 us |
| memx_memcmp             |  181.660 us |  210.840 us |  185.900 us |  183.890 us |
| memx_memcmp_basic       |  183.730 us |  212.440 us |  183.740 us |  179.500 us |
| memx_memcmp_sse2        |  164.930 us |  209.410 us |  178.200 us |  208.410 us |

  3. i586-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memcmp              |  164.030 us |  202.900 us |  335.210 us |  475.980 us |
| libc_memcmp             |  180.430 us |  201.320 us |  338.900 us |  483.720 us |
| memx_memcmp             |  186.900 us |  209.290 us |  177.570 us |  179.290 us |
| memx_memcmp_basic       |  189.610 us |  208.060 us |  177.470 us |  180.140 us |

  4. armv7-linux-androideabi:

|         `name`          |  `andr:en`  |  `andr:ja`  |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memcmp              | 1120.900 us | 1214.700 us | 1381.700 us | 1793.800 us |
| libc_memcmp             | 1248.900 us | 1195.200 us | 1351.200 us | 1777.000 us |
| memx_memcmp             |  652.500 us |  671.780 us |  681.010 us |  707.360 us |
| memx_memcmp_basic       |  673.990 us |  723.850 us |  904.490 us |  679.730 us |


- `us` is micro seconds
- `:en` is english haystack.
- `:ja` is japanese haystack.
- `musl` is x86_64-unknown-linux-musl
- bench on intel Q6600 @ 2.40GHz or armv7 1.5GHz
