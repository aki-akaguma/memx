## Benchmark results

- compile by rustc 1.69.0 (84c898d65 2023-04-16)

  0. x86_64-unknown-linux- @i5-4570:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memeq               |    1.368 µs |    1.298 µs |    2.948 µs |    7.649 µs |
| libc_memeq              |    1.360 µs |    1.300 µs |    3.004 µs |    7.666 µs |
| memx_memeq              |    2.249 µs |    1.658 µs |    2.249 µs |    1.662 µs |
| memx_memeq_basic        |    2.184 µs |    1.461 µs |    2.160 µs |    1.459 µs |
| memx_memeq_sse2         |    2.060 µs |    1.669 µs |    2.061 µs |    1.671 µs |
| memx_memeq_avx2         |    2.157 µs |    1.589 µs |    2.146 µs |    1.589 µs |

  1. x86_64-unknown-linux- @Q6600:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memeq               |    6.998 µs |    6.925 µs |    8.882 µs |   17.492 µs |
| libc_memeq              |    6.983 µs |    6.948 µs |    8.879 µs |   17.459 µs |
| memx_memeq              |    6.603 µs |    5.129 µs |    6.456 µs |    5.393 µs |
| memx_memeq_basic        |    6.042 µs |    3.523 µs |    6.006 µs |    3.562 µs |
| memx_memeq_sse2         |    6.332 µs |    5.443 µs |    6.240 µs |    5.381 µs |

  2. i686-unknown-linux- @Q6600:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memeq               |    7.706 µs |    7.445 µs |   11.967 µs |   20.954 µs |
| libc_memeq              |    7.685 µs |    7.289 µs |   12.364 µs |   20.447 µs |
| memx_memeq              |    7.898 µs |    6.958 µs |    8.989 µs |    7.378 µs |
| memx_memeq_basic        |    7.428 µs |    6.545 µs |    7.664 µs |    6.700 µs |
| memx_memeq_sse2         |    7.884 µs |    6.855 µs |    8.511 µs |    7.060 µs |

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
- @i5-4570: bench on intel i5-4570 @ 3.2GHz
- @Q6600: bench on intel Q6600 @ 2.40GHz or armv7 1.5GHz
