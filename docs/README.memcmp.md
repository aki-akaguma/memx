## Benchmark results

- compile by rustc 1.68.0 (2c8cc3432 2023-03-06)

  0. x86_64-unknown-linux- @Broadwell:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memcmp              |   51.116 µs |   75.029 µs |  115.650 µs |  324.560 µs |
| libc_memcmp             |   58.160 µs |   75.467 µs |  118.320 µs |  326.130 µs |
| memx_memcmp             |   49.451 µs |   73.042 µs |   49.989 µs |   74.839 µs |
| memx_memcmp_basic       |   49.219 µs |   73.485 µs |   50.039 µs |   73.462 µs |
| memx_memcmp_sse2        |   49.254 µs |   73.860 µs |   50.058 µs |   72.620 µs |

  1. x86_64-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memcmp              |  148.390 µs |  261.870 µs |  247.550 µs |  432.000 µs |
| libc_memcmp             |  155.350 µs |  268.120 µs |  249.140 µs |  429.910 µs |
| memx_memcmp             |   92.252 µs |  143.800 µs |   93.866 µs |  139.320 µs |
| memx_memcmp_basic       |   92.243 µs |  143.570 µs |   93.853 µs |  139.290 µs |
| memx_memcmp_sse2        |   92.256 µs |  143.600 µs |   93.877 µs |  139.300 µs |

  2. i686-unknown-linux-:
  
|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memcmp              |  161.650 µs |  281.780 µs |  222.800 µs |  447.690 µs |
| libc_memcmp             |  167.270 µs |  292.110 µs |  211.960 µs |  456.110 µs |
| memx_memcmp             |  113.280 µs |  218.720 µs |  117.340 µs |  275.910 µs |
| memx_memcmp_basic       |  112.510 µs |  219.870 µs |  116.340 µs |  275.990 µs |
| memx_memcmp_sse2        |  113.390 µs |  219.080 µs |  116.390 µs |  276.030 µs |

  3. i586-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memcmp              |  167.380 µs |  285.700 µs |  255.440 µs |  450.610 µs |
| libc_memcmp             |  164.010 µs |  288.090 µs |  261.260 µs |  467.470 µs |
| memx_memcmp             |  117.730 µs |  226.440 µs |  121.390 µs |  239.380 µs |
| memx_memcmp_basic       |  117.760 µs |  226.590 µs |  121.020 µs |  253.740 µs |


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
