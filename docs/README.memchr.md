## Benchmark results

- compile by rustc 1.68.1 (8460ca823 2023-03-20)

  0. x86_64-unknown-linux- @Broadwell:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memchr              |   12.003 µs |   21.965 µs |   12.573 µs |   24.881 µs |
| libc_memchr             |    7.011 µs |    8.648 µs |   11.991 µs |   14.102 µs |
| memchr_memchr           |    6.958 µs |    8.063 µs |    7.070 µs |    7.865 µs |
| memx_memchr             |    6.679 µs |    8.210 µs |    6.768 µs |    8.058 µs |
| memx_memchr_basic       |    7.100 µs |    9.349 µs |    7.166 µs |    9.414 µs |
| memx_memchr_sse2        |    6.731 µs |    8.405 µs |    6.798 µs |    8.495 µs |
| memx_memchr_avx2        |    6.333 µs |    7.801 µs |    6.264 µs |    7.938 µs |

  1. x86_64-unknown-linux- @Q6600:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memchr              |   25.260 µs |   38.890 µs |   25.264 µs |   38.901 µs |
| libc_memchr             |   15.025 µs |   15.685 µs |   33.034 µs |   42.657 µs |
| memchr_memchr           |   16.507 µs |   20.060 µs |   16.270 µs |   20.816 µs |
| memx_memchr             |   15.264 µs |   16.899 µs |   14.973 µs |   16.851 µs |
| memx_memchr_basic       |   13.107 µs |   16.053 µs |   13.279 µs |   16.964 µs |
| memx_memchr_sse2        |   10.515 µs |   12.421 µs |   10.289 µs |   12.576 µs |

  2. i686-unknown-linux- @Q6600:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memchr              |   24.344 µs |   38.858 µs |   24.245 µs |   37.740 µs |
| libc_memchr             |   18.862 µs |   18.994 µs |   33.576 µs |   42.055 µs |
| memchr_memchr           |   27.527 µs |   33.884 µs |   27.228 µs |   35.540 µs |
| memx_memchr             |   19.613 µs |   22.784 µs |   19.917 µs |   23.689 µs |
| memx_memchr_basic       |   12.273 µs |   18.638 µs |   13.647 µs |   20.743 µs |
| memx_memchr_sse2        |   12.660 µs |   13.436 µs |   14.615 µs |   16.036 µs |

  3. i586-unknown-linux- @Q6600:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memchr              |   24.334 µs |   37.894 µs |   24.527 µs |   38.092 µs |
| libc_memchr             |   18.629 µs |   19.033 µs |   32.220 µs |   42.448 µs |
| memchr_memchr           |   28.361 µs |   36.415 µs |   30.451 µs |   37.890 µs |
| memx_memchr             |   19.678 µs |   21.881 µs |   19.686 µs |   21.404 µs |
| memx_memchr_basic       |   11.607 µs |   17.890 µs |   13.420 µs |   20.188 µs |


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

- `µs` is micro seconds
- `:en` is english haystack or short length.
- `:ja` is japanese haystack or long length.
- `gnu` is x86_64-unknown-linux-gnu
- `musl` is x86_64-unknown-linux-musl
- bench on intel Q6600 @ 2.40GHz or armv7 1.5GHz
