## Benchmark results

- compile by rustc 1.68.0 (2c8cc3432 2023-03-06)

  0. x86_64-unknown-linux- @Broadwell:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memeq               |   55.449 µs |   44.136 µs |  221.260 µs |  273.970 µs |
| libc_memeq              |   58.797 µs |   47.878 µs |  225.210 µs |  277.160 µs |
| memx_memeq              |   45.717 µs |   57.122 µs |   62.027 µs |   69.546 µs |
| memx_memeq_basic        |   45.861 µs |   52.864 µs |   46.218 µs |   59.928 µs |

  1. x86_64-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memeq               |  164.290 µs |  164.100 µs |  343.910 µs |  363.040 µs |
| libc_memeq              |  165.660 µs |  169.080 µs |  348.670 µs |  367.550 µs |
| memx_memeq              |  106.460 µs |  139.610 µs |  109.570 µs |  176.360 µs |
| memx_memeq_basic        |   97.880 µs |  130.270 µs |  101.340 µs |  137.070 µs |

  2. i686-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memeq               |  184.010 µs |  203.890 µs |  384.430 µs |  435.960 µs |
| libc_memeq              |  184.070 µs |  198.920 µs |  389.140 µs |  402.570 µs |
| memx_memeq              |  119.990 µs |  174.370 µs |  113.190 µs |  159.660 µs |
| memx_memeq_basic        |  102.080 µs |  132.530 µs |  100.670 µs |  127.840 µs |

  3. i586-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memeq               |  175.820 µs |  261.820 µs |  341.320 µs |  415.810 µs |
| libc_memeq              |  175.540 µs |  198.700 µs |  335.430 µs |  404.350 µs |
| memx_memeq              |  123.380 µs |  162.940 µs |  126.170 µs |  183.000 µs |
| memx_memeq_basic        |  123.550 µs |  164.280 µs |  126.300 µs |  178.640 µs |


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


- `us` is micro seconds
- `:en` is english haystack.
- `:ja` is japanese haystack.
- `musl` is x86_64-unknown-linux-musl
- bench on intel Q6600 @ 2.40GHz or armv7 1.5GHz
