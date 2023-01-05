## Benchmark results

- compile by rustc 1.66.0 (69f9c33d7 2022-12-12)

  1. x86_64-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memeq               |  163.570 µs |  165.340 µs |  265.090 µs |  346.240 µs |
| libc_memeq              |  162.830 µs |  168.620 µs |  260.730 µs |  346.450 µs |
| memx_memeq              |   97.587 µs |   88.667 µs |  100.320 µs |   92.333 µs |
| memx_memeq_basic        |   97.668 µs |   88.524 µs |  100.230 µs |   92.357 µs |

  2. i686-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memeq               |  175.040 µs |  197.050 µs |  342.270 µs |  403.520 µs |
| libc_memeq              |  174.900 µs |  199.760 µs |  406.010 µs |  407.670 µs |
| memx_memeq              |  128.210 µs |  118.560 µs |  129.010 µs |  120.100 µs |
| memx_memeq_basic        |  129.840 µs |  118.780 µs |  128.920 µs |  120.110 µs |

- compile by rustc 1.65.0 (897e37553 2022-11-02)

  3. i586-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memeq               |  175.060 µs |  206.250 µs |  382.770 µs |  516.170 µs |
| libc_memeq              |  175.150 µs |  202.370 µs |  380.680 µs |  525.270 µs |
| memx_memeq              |  134.680 µs |  120.110 µs |  133.940 µs |  121.000 µs |
| memx_memeq_basic        |  134.990 µs |  119.400 µs |  133.640 µs |  120.690 µs |


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
