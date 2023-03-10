## Benchmark results

- compile by rustc 1.68.0 (2c8cc3432 2023-03-06)

  0. x86_64-unknown-linux- @Broadwell:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memcmp              |   64.127 µs |   52.760 µs |  156.900 µs |  258.640 µs |
| libc_memcmp             |   57.305 µs |   45.962 µs |  161.870 µs |  259.870 µs |
| memx_memcmp             |   53.461 µs |   68.993 µs |   84.945 µs |  101.820 µs |
| memx_memcmp_basic       |   53.705 µs |   69.030 µs |   52.636 µs |   69.434 µs |
| memx_memcmp_sse2        |   54.833 µs |   68.791 µs |   76.008 µs |   87.927 µs |

  1. x86_64-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memcmp              |  170.920 µs |  229.390 µs |  342.440 µs |  362.540 µs |
| libc_memcmp             |  151.560 µs |  173.160 µs |  334.050 µs |  353.730 µs |
| memx_memcmp             |  155.490 µs |  224.680 µs |  158.840 µs |  232.400 µs |
| memx_memcmp_basic       |  120.810 µs |  183.910 µs |  124.470 µs |  205.590 µs |
| memx_memcmp_sse2        |  150.820 µs |  221.100 µs |  154.310 µs |  227.390 µs |

  2. i686-unknown-linux-:
  
|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memcmp              |  188.230 µs |  233.630 µs |  288.600 µs |  384.900 µs |
| libc_memcmp             |  168.300 µs |  221.990 µs |  276.270 µs |  394.620 µs |
| memx_memcmp             |  202.610 µs |  277.150 µs |  221.280 µs |  305.010 µs |
| memx_memcmp_basic       |  146.720 µs |  230.100 µs |  154.620 µs |  244.300 µs |
| memx_memcmp_sse2        |  187.080 µs |  269.040 µs |  208.400 µs |  287.530 µs |

  3. i586-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memcmp              |  205.330 µs |  217.610 µs |  305.410 µs |  374.750 µs |
| libc_memcmp             |  167.940 µs |  198.560 µs |  292.880 µs |  402.570 µs |
| memx_memcmp             |  181.920 µs |  238.850 µs |  168.080 µs |  218.130 µs |
| memx_memcmp_basic       |  182.230 µs |  238.760 µs |  170.380 µs |  218.480 µs |


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
