## Benchmark results

- compile by rustc 1.68.1 (8460ca823 2023-03-20)

  0. x86_64-unknown-linux- @Broadwell:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memeq               |    7.953 µs |   12.023 µs |   12.151 µs |   40.967 µs |
| libc_memeq              |    7.922 µs |   11.477 µs |   12.167 µs |   40.524 µs |
| memx_memeq              |    7.193 µs |   11.003 µs |    7.331 µs |   11.097 µs |
| memx_memeq_basic        |    7.285 µs |   11.348 µs |    7.284 µs |   10.979 µs |
| memx_memeq_sse2         |    7.321 µs |   11.372 µs |    7.362 µs |   11.128 µs |
| memx_memeq_avx2         |    7.688 µs |   11.734 µs |    7.671 µs |   11.676 µs |

  1. x86_64-unknown-linux- @Q6600:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memeq               |   20.795 µs |   48.671 µs |   24.993 µs |   58.006 µs |
| libc_memeq              |   20.408 µs |   48.553 µs |   25.004 µs |   57.906 µs |
| memx_memeq              |   14.779 µs |   27.416 µs |   15.319 µs |   29.094 µs |
| memx_memeq_basic        |   15.442 µs |   29.647 µs |   15.160 µs |   25.475 µs |
| memx_memeq_sse2         |   15.265 µs |   28.259 µs |   15.866 µs |   27.689 µs |

  2. i686-unknown-linux- @Q6600:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memeq               |   22.056 µs |   50.687 µs |   30.159 µs |   66.083 µs |
| libc_memeq              |   21.808 µs |   50.752 µs |   30.004 µs |   66.052 µs |
| memx_memeq              |   15.248 µs |   29.836 µs |   18.752 µs |   32.193 µs |
| memx_memeq_basic        |   15.287 µs |   28.998 µs |   20.366 µs |   37.350 µs |
| memx_memeq_sse2         |   15.020 µs |   28.898 µs |   19.020 µs |   34.519 µs |

  3. i586-unknown-linux- @Q6600:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memeq               |   21.442 µs |   51.172 µs |   29.614 µs |   66.038 µs |
| libc_memeq              |   21.559 µs |   50.548 µs |   29.867 µs |   66.352 µs |
| memx_memeq              |   16.043 µs |   29.191 µs |   18.981 µs |   33.554 µs |
| memx_memeq_basic        |   15.981 µs |   29.222 µs |   18.984 µs |   34.431 µs |


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
