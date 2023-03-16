## Benchmark results

- compile by rustc 1.68.0 (2c8cc3432 2023-03-06)

  0. x86_64-unknown-linux- @Broadwell:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memchr              |   11.803 µs |   23.314 µs |   11.586 µs |   22.924 µs |
| libc_memchr             |    7.207 µs |    8.403 µs |   12.291 µs |   14.185 µs |
| memchr_memchr           |    6.963 µs |    8.016 µs |    7.115 µs |    7.814 µs |
| memx_memchr             |    6.724 µs |    8.100 µs |    6.532 µs |    7.990 µs |
| memx_memchr_basic       |    7.074 µs |    9.336 µs |    7.080 µs |    9.473 µs |
| memx_memchr_sse2        |    6.695 µs |    8.149 µs |    6.781 µs |    8.258 µs |
| memx_memchr_avx2        |    6.371 µs |    7.780 µs |    6.276 µs |    7.779 µs |

  1. x86_64-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memchr              |   25.207 µs |   38.911 µs |   25.369 µs |   38.955 µs |
| libc_memchr             |   15.013 µs |   15.652 µs |   33.555 µs |   43.030 µs |
| memchr_memchr           |   16.546 µs |   20.369 µs |   16.274 µs |   20.309 µs |
| memx_memchr             |   14.744 µs |   17.451 µs |   14.733 µs |   17.466 µs |
| memx_memchr_basic       |   13.223 µs |   15.794 µs |   13.717 µs |   16.924 µs |
| memx_memchr_sse2        |   10.870 µs |   13.259 µs |   10.634 µs |   13.088 µs |

  2. i686-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memchr              |   24.333 µs |   37.686 µs |   24.308 µs |   37.762 µs |
| libc_memchr             |   18.856 µs |   18.876 µs |   31.736 µs |   41.463 µs |
| memchr_memchr           |   28.470 µs |   35.956 µs |   28.940 µs |   37.101 µs |
| memx_memchr             |   19.461 µs |   21.979 µs |   20.453 µs |   24.432 µs |
| memx_memchr_basic       |   12.507 µs |   19.411 µs |   13.880 µs |   21.067 µs |
| memx_memchr_sse2        |   13.271 µs |   13.681 µs |   15.160 µs |   15.566 µs |

  3. i586-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memchr              |   24.518 µs |   37.947 µs |   25.622 µs |   37.906 µs |
| libc_memchr             |   18.695 µs |   19.155 µs |   32.929 µs |   42.462 µs |
| memchr_memchr           |   29.438 µs |   36.601 µs |   30.242 µs |   38.295 µs |
| memx_memchr             |   19.405 µs |   21.108 µs |   20.295 µs |   22.124 µs |
| memx_memchr_basic       |   12.213 µs |   18.455 µs |   13.803 µs |   21.051 µs |


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
