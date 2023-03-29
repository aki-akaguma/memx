## Benchmark results

- compile by rustc 1.68.1 (8460ca823 2023-03-20)

  0. x86_64-unknown-linux- @Broadwell:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memchr              |   12.337 µs |   22.095 µs |   12.782 µs |   25.429 µs |
| libc_memchr             |    7.096 µs |    8.465 µs |   11.930 µs |   14.007 µs |
| memchr_memchr           |    6.981 µs |    8.254 µs |    7.043 µs |    7.974 µs |
| memx_memchr             |    6.345 µs |    8.120 µs |    6.325 µs |    7.796 µs |
| memx_memchr_basic       |    7.137 µs |    9.187 µs |    7.069 µs |    9.427 µs |
| memx_memchr_sse2        |    6.784 µs |    8.181 µs |    6.756 µs |    8.286 µs |
| memx_memchr_avx2        |    6.359 µs |    8.099 µs |    6.267 µs |    8.172 µs |

  1. x86_64-unknown-linux- @Q6600:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memchr              |   25.507 µs |   38.860 µs |   25.875 µs |   39.364 µs |
| libc_memchr             |   15.131 µs |   15.689 µs |   32.727 µs |   42.266 µs |
| memchr_memchr           |   16.528 µs |   20.773 µs |   16.367 µs |   20.172 µs |
| memx_memchr             |   13.594 µs |   16.368 µs |   13.114 µs |   15.785 µs |
| memx_memchr_basic       |   13.912 µs |   15.889 µs |   13.691 µs |   16.949 µs |
| memx_memchr_sse2        |   10.499 µs |   12.426 µs |   10.790 µs |   13.257 µs |

  2. i686-unknown-linux- @Q6600:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memchr              |   24.551 µs |   37.949 µs |   24.120 µs |   37.815 µs |
| libc_memchr             |   18.996 µs |   19.270 µs |   32.017 µs |   40.587 µs |
| memchr_memchr           |   27.499 µs |   33.931 µs |   26.578 µs |   34.575 µs |
| memx_memchr             |   18.661 µs |   21.547 µs |   19.495 µs |   21.222 µs |
| memx_memchr_basic       |   12.305 µs |   18.935 µs |   13.228 µs |   20.131 µs |
| memx_memchr_sse2        |   15.113 µs |   16.713 µs |   16.271 µs |   17.331 µs |

  3. i586-unknown-linux- @Q6600:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memchr              |   24.615 µs |   37.954 µs |   25.671 µs |   38.570 µs |
| libc_memchr             |   18.750 µs |   19.160 µs |   33.436 µs |   43.205 µs |
| memchr_memchr           |   28.684 µs |   35.843 µs |   30.040 µs |   38.082 µs |
| memx_memchr             |   18.837 µs |   21.052 µs |   19.807 µs |   21.585 µs |
| memx_memchr_basic       |   12.276 µs |   18.715 µs |   13.105 µs |   20.003 µs |

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
