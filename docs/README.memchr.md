## Benchmark results

- compile by rustc 1.69.0 (84c898d65 2023-04-16)

  0. x86_64-unknown-linux- @i5-4570:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memchr              |   11.419 µs |   19.754 µs |   10.772 µs |   19.372 µs |
| libc_memchr             |    4.849 µs |    5.764 µs |    8.757 µs |   10.205 µs |
| memchr_memchr           |    4.767 µs |    5.547 µs |    4.689 µs |    5.417 µs |
| memx_memchr             |    4.451 µs |    5.402 µs |    4.426 µs |    5.322 µs |
| memx_memchr_basic       |    5.695 µs |    7.498 µs |    5.689 µs |    7.616 µs |
| memx_memchr_sse2        |    4.800 µs |    5.901 µs |    4.827 µs |    6.111 µs |
| memx_memchr_avx2        |    4.447 µs |    5.382 µs |    4.423 µs |    5.341 µs |

  1. x86_64-unknown-linux- @Q6600:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memchr              |   25.486 µs |   38.850 µs |   25.337 µs |   38.914 µs |
| libc_memchr             |   15.043 µs |   15.575 µs |   33.467 µs |   42.649 µs |
| memchr_memchr           |   16.488 µs |   20.106 µs |   16.281 µs |   20.458 µs |
| memx_memchr             |   13.521 µs |   16.070 µs |   13.410 µs |   16.272 µs |
| memx_memchr_basic       |   12.925 µs |   17.384 µs |   13.529 µs |   17.963 µs |
| memx_memchr_sse2        |   11.665 µs |   13.748 µs |   11.823 µs |   14.056 µs |

  2. i686-unknown-linux- @Q6600:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memchr              |   24.138 µs |   37.495 µs |   24.541 µs |   38.832 µs |
| libc_memchr             |   18.985 µs |   19.098 µs |   32.892 µs |   42.655 µs |
| memchr_memchr           |   27.310 µs |   34.959 µs |   28.393 µs |   35.869 µs |
| memx_memchr             |   20.387 µs |   21.250 µs |   20.596 µs |   23.687 µs |
| memx_memchr_basic       |   20.668 µs |   33.785 µs |   21.694 µs |   35.418 µs |
| memx_memchr_sse2        |   15.648 µs |   16.760 µs |   18.070 µs |   18.835 µs |

  3. i586-unknown-linux- @Q6600:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memchr              |   24.541 µs |   37.953 µs |   24.566 µs |   38.036 µs |
| libc_memchr             |   18.644 µs |   19.084 µs |   31.991 µs |   41.918 µs |
| memchr_memchr           |   28.627 µs |   35.509 µs |   29.467 µs |   37.143 µs |
| memx_memchr             |   19.927 µs |   21.454 µs |   21.084 µs |   24.202 µs |
| memx_memchr_basic       |   20.973 µs |   33.587 µs |   25.580 µs |   40.360 µs |

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
