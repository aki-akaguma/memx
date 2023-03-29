## Benchmark results

- compile by rustc 1.68.1 (8460ca823 2023-03-20)

  0. x86_64-unknown-linux- @Broadwell:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memeq               |    8.239 µs |   11.692 µs |   12.135 µs |   41.049 µs |
| libc_memeq              |    7.954 µs |   11.677 µs |   12.070 µs |   40.801 µs |
| memx_memeq              |    7.663 µs |   11.426 µs |    7.792 µs |   11.229 µs |
| memx_memeq_basic        |    7.772 µs |   11.212 µs |    7.821 µs |   11.034 µs |
| memx_memeq_sse2         |    7.814 µs |   11.225 µs |    7.846 µs |   11.401 µs |
| memx_memeq_avx2         |    8.319 µs |   11.787 µs |    8.235 µs |   11.690 µs |

  1. x86_64-unknown-linux- @Q6600:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memeq               |   21.714 µs |   48.965 µs |   24.999 µs |   57.882 µs |
| libc_memeq              |   20.764 µs |   49.277 µs |   25.193 µs |   58.202 µs |
| memx_memeq              |   15.790 µs |   25.952 µs |   15.849 µs |   27.237 µs |
| memx_memeq_basic        |   16.342 µs |   26.146 µs |   16.363 µs |   26.620 µs |
| memx_memeq_sse2         |   15.788 µs |   25.257 µs |   15.836 µs |   25.577 µs |

  2. i686-unknown-linux- @Q6600:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memeq               |   21.906 µs |   50.611 µs |   29.949 µs |   65.910 µs |
| libc_memeq              |   21.971 µs |   50.354 µs |   30.168 µs |   65.946 µs |
| memx_memeq              |   16.008 µs |   29.042 µs |   19.279 µs |   34.356 µs |
| memx_memeq_basic        |   15.813 µs |   28.354 µs |   18.949 µs |   32.780 µs |
| memx_memeq_sse2         |   15.891 µs |   29.010 µs |   19.509 µs |   33.430 µs |

  3. i586-unknown-linux- @Q6600:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memeq               |   22.069 µs |   51.272 µs |   29.555 µs |   66.116 µs |
| libc_memeq              |   21.957 µs |   50.619 µs |   29.652 µs |   66.075 µs |
| memx_memeq              |   16.790 µs |   28.692 µs |   19.481 µs |   36.806 µs |
| memx_memeq_basic        |   16.178 µs |   28.745 µs |   19.659 µs |   33.279 µs |


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
