## Benchmark results

- compile by rustc 1.69.0 (84c898d65 2023-04-16)

  0. x86_64-unknown-linux- @i5-4570:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memchr              |   11.390 µs |   19.761 µs |   11.380 µs |   19.764 µs |
| libc_memchr             |    4.851 µs |    5.806 µs |    8.210 µs |    9.561 µs |
| memchr_memchr           |    4.768 µs |    5.528 µs |    4.658 µs |    5.376 µs |
| memx_memchr             |    4.434 µs |    5.399 µs |    4.392 µs |    5.289 µs |
| memx_memchr_basic       |    5.266 µs |    6.708 µs |    5.256 µs |    6.785 µs |
| memx_memchr_sse2        |    4.759 µs |    5.816 µs |    4.779 µs |    6.012 µs |
| memx_memchr_avx2        |    4.438 µs |    5.396 µs |    4.389 µs |    5.299 µs |

  1. x86_64-unknown-linux- @Q6600:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memchr              |   25.836 µs |   39.580 µs |   25.868 µs |   39.541 µs |
| libc_memchr             |   15.282 µs |   15.858 µs |   33.682 µs |   43.082 µs |
| memchr_memchr           |   16.761 µs |   20.563 µs |   16.563 µs |   20.787 µs |
| memx_memchr             |   14.046 µs |   15.997 µs |   14.345 µs |   15.902 µs |
| memx_memchr_basic       |   12.548 µs |   16.517 µs |   12.841 µs |   15.903 µs |
| memx_memchr_sse2        |   11.315 µs |   12.640 µs |   11.032 µs |   13.548 µs |

  2. i686-unknown-linux- @Q6600:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memchr              |   24.555 µs |   37.967 µs |   24.129 µs |   37.762 µs |
| libc_memchr             |   19.008 µs |   19.170 µs |   32.406 µs |   41.869 µs |
| memchr_memchr           |   27.383 µs |   33.856 µs |   27.662 µs |   35.465 µs |
| memx_memchr             |   19.789 µs |   21.075 µs |   20.450 µs |   22.022 µs |
| memx_memchr_basic       |   17.901 µs |   24.898 µs |   19.611 µs |   27.883 µs |
| memx_memchr_sse2        |   16.757 µs |   17.133 µs |   17.107 µs |   18.664 µs |

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
- @i5-4570: bench on intel i5-4570 @ 3.2GHz
- @Q6600: bench on intel Q6600 @ 2.40GHz or armv7 1.5GHz
