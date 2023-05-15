## Benchmark results

- compile by rustc 1.69.0 (84c898d65 2023-04-16)

  0. x86_64-unknown-linux- @i5-4570:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memeq               |    1.669 µs |    1.331 µs |    2.725 µs |    7.462 µs |
| libc_memeq              |    1.660 µs |    1.315 µs |    2.725 µs |    7.507 µs |
| memx_memeq              |    2.069 µs |    1.518 µs |    2.153 µs |    1.587 µs |
| memx_memeq_basic        |    1.977 µs |    1.335 µs |    2.104 µs |    1.374 µs |
| memx_memeq_sse2         |    2.097 µs |    1.680 µs |    2.053 µs |    1.672 µs |
| memx_memeq_avx2         |    1.981 µs |    1.463 µs |    2.067 µs |    1.504 µs |

  1. x86_64-unknown-linux- @Q6600:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memeq               |    6.881 µs |    7.346 µs |    8.493 µs |   17.076 µs |
| libc_memeq              |    6.838 µs |    7.417 µs |    8.588 µs |   17.082 µs |
| memx_memeq              |    6.318 µs |    5.443 µs |    6.280 µs |    5.215 µs |
| memx_memeq_basic        |    5.877 µs |    3.744 µs |    5.811 µs |    3.485 µs |
| memx_memeq_sse2         |    6.022 µs |    5.401 µs |    6.196 µs |    5.229 µs |

  2. i686-unknown-linux- @Q6600:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memeq               |    8.022 µs |    7.569 µs |   12.436 µs |   20.412 µs |
| libc_memeq              |    7.979 µs |    7.351 µs |   11.977 µs |   20.912 µs |
| memx_memeq              |    8.643 µs |    7.236 µs |    9.035 µs |    7.644 µs |
| memx_memeq_basic        |    7.177 µs |    6.478 µs |    7.541 µs |    6.736 µs |
| memx_memeq_sse2         |    7.813 µs |    6.880 µs |    8.636 µs |    7.474 µs |

  3. i586-unknown-linux- @Q6600:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memeq               |    7.753 µs |    7.696 µs |   12.372 µs |   20.998 µs |
| libc_memeq              |    7.827 µs |    7.449 µs |   12.197 µs |   21.062 µs |
| memx_memeq              |    8.990 µs |    7.192 µs |    8.658 µs |    7.541 µs |
| memx_memeq_basic        |    8.759 µs |    6.960 µs |    9.664 µs |    8.118 µs |


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
