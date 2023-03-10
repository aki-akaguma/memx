## Benchmark results

- compile by rustc 1.68.0 (2c8cc3432 2023-03-06)

  0. x86_64-unknown-linux- @Broadwell:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memchr              |  133.030 µs |  107.910 µs |  130.800 µs |  102.750 µs |
| libc_memchr             |  176.800 µs |   43.116 µs |  212.750 µs |   83.152 µs |
| memchr_memchr           |  187.770 µs |   50.553 µs |  180.380 µs |   49.706 µs |
| memx_memchr             |  141.910 µs |   41.480 µs |  163.340 µs |   49.395 µs |
| memx_memchr_basic       |  147.700 µs |   49.448 µs |  148.700 µs |   55.718 µs |
| memx_memchr_sse2        |  133.750 µs |   42.972 µs |  157.570 µs |   42.267 µs |

  1. x86_64-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memchr              |  351.210 µs |  178.040 µs |  350.930 µs |  176.730 µs |
| libc_memchr             |  253.670 µs |   72.690 µs |  569.630 µs |  195.940 µs |
| memchr_memchr           |  224.710 µs |   67.114 µs |  232.510 µs |   65.791 µs |
| memx_memchr             |  243.000 µs |   68.027 µs |  246.770 µs |   66.479 µs |
| memx_memchr_basic       |  188.640 µs |   69.961 µs |  193.530 µs |   69.365 µs |
| memx_memchr_sse2        |  214.570 µs |   61.997 µs |  219.520 µs |   60.671 µs |

  2. i686-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memchr              |  353.550 µs |  174.720 µs |  338.080 µs |  174.730 µs |
| libc_memchr             |  267.230 µs |   74.932 µs |  549.530 µs |  132.710 µs |
| memchr_memchr           |  385.240 µs |  166.480 µs |  366.920 µs |  145.460 µs |
| memx_memchr             |  253.760 µs |   68.418 µs |  281.780 µs |   76.685 µs |
| memx_memchr_basic       |  194.120 µs |   96.815 µs |  189.980 µs |  112.540 µs |
| memx_memchr_sse2        |  232.700 µs |   67.587 µs |  248.630 µs |   70.880 µs |

  3. i586-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memchr              |  345.710 µs |  176.930 µs |  350.310 µs |  179.450 µs |
| libc_memchr             |  268.490 µs |   76.812 µs |  504.380 µs |  135.950 µs |
| memchr_memchr           |  420.550 µs |  178.830 µs |  396.730 µs |  154.570 µs |
| memx_memchr             |  194.890 µs |  100.400 µs |  192.150 µs |  106.370 µs |
| memx_memchr_basic       |  195.370 µs |  100.870 µs |  192.190 µs |  106.560 µs |


- compile by rustc 1.53.0 (53cb7b09b 2021-06-17)

  1. x86_64-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memchr              |  369.140 us |  175.220 us |  367.450 us |  174.240 us |
| libc_memchr             |  249.860 us |   72.885 us |  556.310 us |  193.510 us |
| memchr_memchr           |  221.400 us |   65.674 us |  225.190 us |   64.664 us |
| memx_memchr             |  224.860 us |   62.641 us |  229.790 us |   62.381 us |
| memx_memchr_basic       |  226.690 us |   84.038 us |  228.820 us |   83.574 us |
| memx_memchr_sse2        |  218.150 us |   61.512 us |  225.590 us |   60.974 us |

  2. i686-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memchr              |  354.390 us |  172.580 us |  341.740 us |  172.530 us |
| libc_memchr             |  264.970 us |   77.258 us |  504.580 us |  134.980 us |
| memchr_memchr           |  495.090 us |  196.640 us |  531.300 us |  199.020 us |
| memx_memchr             |  341.450 us |   91.119 us |  389.200 us |  101.150 us |
| memx_memchr_basic       |  270.310 us |  117.170 us |  286.350 us |  123.790 us |
| memx_memchr_sse2        |  266.010 us |   73.213 us |  276.880 us |   76.584 us |

  3. i586-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memchr              |  373.800 us |  175.510 us |  348.800 us |  178.230 us |
| libc_memchr             |  263.730 us |   76.091 us |  486.350 us |  132.040 us |
| memchr_memchr           |  498.680 us |  177.700 us |  497.360 us |  171.890 us |
| memx_memchr             |  269.420 us |  116.490 us |  283.440 us |  121.410 us |
| memx_memchr_basic       |  271.500 us |  116.470 us |  284.360 us |  121.250 us |

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
