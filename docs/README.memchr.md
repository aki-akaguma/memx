## Benchmark results

- compile by rustc 1.68.1 (8460ca823 2023-03-20)

  0. x86_64-unknown-linux- @Broadwell:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memchr              |   11.817 µs |   22.599 µs |   15.456 µs |   24.139 µs |
| libc_memchr             |    7.138 µs |    8.446 µs |   12.582 µs |   14.606 µs |
| memchr_memchr           |    6.934 µs |    8.111 µs |    6.869 µs |    7.843 µs |
| memx_memchr             |    6.468 µs |    8.102 µs |    6.365 µs |    8.133 µs |
| memx_memchr_basic       |    8.094 µs |   10.143 µs |    8.415 µs |   10.504 µs |
| memx_memchr_sse2        |    6.748 µs |    8.204 µs |    6.803 µs |    8.582 µs |
| memx_memchr_avx2        |    6.578 µs |    8.020 µs |    6.364 µs |    8.231 µs |

  1. x86_64-unknown-linux- @Q6600:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memchr              |   25.334 µs |   39.082 µs |   25.846 µs |   39.546 µs |
| libc_memchr             |   15.529 µs |   15.780 µs |   33.334 µs |   43.023 µs |
| memchr_memchr           |   16.549 µs |   20.328 µs |   16.343 µs |   20.300 µs |
| memx_memchr             |   13.166 µs |   14.881 µs |   13.193 µs |   15.606 µs |
| memx_memchr_basic       |   12.525 µs |   15.881 µs |   12.811 µs |   16.180 µs |
| memx_memchr_sse2        |   10.901 µs |   12.364 µs |   10.429 µs |   12.553 µs |

  2. i686-unknown-linux- @Q6600:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memchr              |   24.490 µs |   37.918 µs |   24.361 µs |   38.090 µs |
| libc_memchr             |   19.227 µs |   19.033 µs |   32.721 µs |   41.757 µs |
| memchr_memchr           |   28.597 µs |   35.477 µs |   28.217 µs |   35.871 µs |
| memx_memchr             |   18.762 µs |   19.293 µs |   19.617 µs |   21.612 µs |
| memx_memchr_basic       |   16.718 µs |   24.241 µs |   17.572 µs |   25.529 µs |
| memx_memchr_sse2        |   15.787 µs |   16.405 µs |   15.611 µs |   17.403 µs |

  3. i586-unknown-linux- @Q6600:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memchr              |   25.010 µs |   37.954 µs |   25.642 µs |   38.799 µs |
| libc_memchr             |   18.739 µs |   18.871 µs |   33.255 µs |   42.756 µs |
| memchr_memchr           |   29.565 µs |   36.036 µs |   30.031 µs |   38.261 µs |
| memx_memchr             |   18.994 µs |   20.473 µs |   19.767 µs |   22.437 µs |
| memx_memchr_basic       |   19.651 µs |   27.052 µs |   24.656 µs |   32.751 µs |

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
