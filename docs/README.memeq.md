## Benchmark results

- compile by rustc 1.68.1 (8460ca823 2023-03-20)

  0. x86_64-unknown-linux- @Broadwell:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memeq               |    2.003 µs |    1.931 µs |    3.746 µs |   10.377 µs |
| libc_memeq              |    1.996 µs |    1.941 µs |    3.704 µs |   10.477 µs |
| memx_memeq              |    3.220 µs |    2.131 µs |    3.351 µs |    2.035 µs |
| memx_memeq_basic        |    3.865 µs |    1.995 µs |    3.256 µs |    2.063 µs |
| memx_memeq_sse2         |    3.767 µs |    2.282 µs |    3.090 µs |    2.259 µs |
| memx_memeq_avx2         |    3.096 µs |    2.114 µs |    3.319 µs |    2.049 µs |

  1. x86_64-unknown-linux- @Q6600:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memeq               |    6.878 µs |    6.810 µs |    9.681 µs |   17.853 µs |
| libc_memeq              |    6.959 µs |    6.806 µs |    9.675 µs |   17.829 µs |
| memx_memeq              |    7.187 µs |    5.199 µs |    7.447 µs |    5.183 µs |
| memx_memeq_basic        |    6.395 µs |    3.312 µs |    6.743 µs |    3.348 µs |
| memx_memeq_sse2         |    6.966 µs |    4.357 µs |    7.204 µs |    4.589 µs |

  2. i686-unknown-linux- @Q6600:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memeq               |    7.699 µs |    7.789 µs |   12.389 µs |   21.250 µs |
| libc_memeq              |    7.640 µs |    7.303 µs |   11.858 µs |   21.214 µs |
| memx_memeq              |    8.839 µs |    7.149 µs |    9.266 µs |    7.093 µs |
| memx_memeq_basic        |    7.753 µs |    6.610 µs |    8.524 µs |    6.948 µs |
| memx_memeq_sse2         |    8.678 µs |    6.097 µs |    9.126 µs |    6.914 µs |

  3. i586-unknown-linux- @Q6600:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memeq               |    7.732 µs |    7.342 µs |   12.024 µs |   20.842 µs |
| libc_memeq              |    8.086 µs |    7.550 µs |   12.004 µs |   21.281 µs |
| memx_memeq              |    8.656 µs |    6.802 µs |    9.019 µs |    8.131 µs |
| memx_memeq_basic        |    9.001 µs |    6.765 µs |   10.882 µs |    7.840 µs |


- compile by rustc 1.53.0 (53cb7b09b 2021-06-17)

  1. x86_64-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memeq               |  165.510 us |  168.340 us |  337.740 us |  356.760 us |
| libc_memeq              |  166.160 us |  172.420 us |  342.770 us |  360.790 us |
| memx_memeq              |  118.030 us |  118.830 us |  114.340 us |  108.960 us |
| memx_memeq_basic        |  117.170 us |  118.780 us |  114.380 us |  109.280 us |

  2. i686-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memeq               |  172.830 us |  193.700 us |  367.620 us |  517.440 us |
| libc_memeq              |  173.100 us |  198.420 us |  370.670 us |  517.830 us |
| memx_memeq              |  157.930 us |  159.710 us |  172.850 us |  153.050 us |
| memx_memeq_basic        |  161.000 us |  157.600 us |  171.320 us |  152.750 us |

  3. i586-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memeq               |  190.700 us |  246.930 us |  376.540 us |  510.110 us |
| libc_memeq              |  168.630 us |  189.100 us |  357.910 us |  507.500 us |
| memx_memeq              |  159.320 us |  153.230 us |  165.410 us |  152.640 us |
| memx_memeq_basic        |  159.720 us |  154.340 us |  168.250 us |  153.200 us |

  4. armv7-linux-androideabi:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memeq               | 1263.000 us | 1236.600 us | 1317.700 us | 1721.000 us |
| memx_memeq              |  779.600 us |  752.600 us |  658.390 us |  604.610 us |
| memx_memeq_basic        |  755.180 us |  746.410 us |  694.570 us |  598.540 us |

- `µs` is micro seconds
- `:en` is english haystack or short length.
- `:ja` is japanese haystack or long length.
- `gnu` is x86_64-unknown-linux-gnu
- `musl` is x86_64-unknown-linux-musl
- bench on intel Q6600 @ 2.40GHz or armv7 1.5GHz
