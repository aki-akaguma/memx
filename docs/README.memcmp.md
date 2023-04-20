## Benchmark results

- compile by rustc 1.68.1 (8460ca823 2023-03-20)

  0. x86_64-unknown-linux- @Broadwell:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memcmp              |    2.518 µs |    2.241 µs |    4.172 µs |   10.150 µs |
| libc_memcmp             |    2.574 µs |    2.152 µs |    3.981 µs |   10.317 µs |
| memx_memcmp             |    4.497 µs |    3.141 µs |    4.402 µs |    3.243 µs |
| memx_memcmp_basic       |    4.304 µs |    3.441 µs |    4.509 µs |    4.005 µs |
| memx_memcmp_sse2        |    4.405 µs |    3.090 µs |    4.435 µs |    3.048 µs |
| memx_memcmp_avx2        |    4.348 µs |    3.123 µs |    4.253 µs |    3.135 µs |

  1. x86_64-unknown-linux- @Q6600:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memcmp              |    7.541 µs |    7.721 µs |   10.180 µs |   18.842 µs |
| libc_memcmp             |    7.212 µs |    7.543 µs |    9.909 µs |   18.823 µs |
| memx_memcmp             |    8.871 µs |    6.420 µs |    8.786 µs |    6.643 µs |
| memx_memcmp_basic       |    8.115 µs |    6.010 µs |    7.340 µs |    6.421 µs |
| memx_memcmp_sse2        |    8.664 µs |    6.072 µs |    8.654 µs |    6.389 µs |

  2. i686-unknown-linux- @Q6600:
  
|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memcmp              |    8.203 µs |    8.065 µs |   11.682 µs |   20.772 µs |
| libc_memcmp             |    8.050 µs |    7.564 µs |   11.430 µs |   19.999 µs |
| memx_memcmp             |   10.794 µs |    8.967 µs |   11.311 µs |    9.214 µs |
| memx_memcmp_basic       |    9.656 µs |    9.266 µs |    9.841 µs |    9.435 µs |
| memx_memcmp_sse2        |   10.563 µs |    7.962 µs |   10.893 µs |    8.181 µs |

  3. i586-unknown-linux- @Q6600:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memcmp              |    8.985 µs |    9.194 µs |   11.808 µs |   20.947 µs |
| libc_memcmp             |    8.287 µs |    7.189 µs |   11.156 µs |   20.008 µs |
| memx_memcmp             |   10.712 µs |    9.759 µs |   10.663 µs |    8.611 µs |
| memx_memcmp_basic       |   10.177 µs |    9.740 µs |   11.049 µs |   10.633 µs |

- compile by rustc 1.53.0 (53cb7b09b 2021-06-17)

  1. x86_64-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memcmp              |  155.810 us |  173.740 us |  253.170 us |  343.250 us |
| libc_memcmp             |  175.920 us |  187.280 us |  259.290 us |  344.760 us |
| memx_memcmp             |  140.330 us |  140.790 us |  134.440 us |  130.510 us |
| memx_memcmp_basic       |  136.670 us |  139.910 us |  133.930 us |  129.710 us |
| memx_memcmp_sse2        |  134.700 us |  179.240 us |  131.100 us |  167.600 us |

  2. i686-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memcmp              |  160.610 us |  258.220 us |  336.550 us |  475.040 us |
| libc_memcmp             |  182.480 us |  214.320 us |  331.690 us |  483.450 us |
| memx_memcmp             |  181.660 us |  210.840 us |  185.900 us |  183.890 us |
| memx_memcmp_basic       |  183.730 us |  212.440 us |  183.740 us |  179.500 us |
| memx_memcmp_sse2        |  164.930 us |  209.410 us |  178.200 us |  208.410 us |

  3. i586-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memcmp              |  164.030 us |  202.900 us |  335.210 us |  475.980 us |
| libc_memcmp             |  180.430 us |  201.320 us |  338.900 us |  483.720 us |
| memx_memcmp             |  186.900 us |  209.290 us |  177.570 us |  179.290 us |
| memx_memcmp_basic       |  189.610 us |  208.060 us |  177.470 us |  180.140 us |

  4. armv7-linux-androideabi:

|         `name`          |  `andr:en`  |  `andr:ja`  |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memcmp              | 1120.900 us | 1214.700 us | 1381.700 us | 1793.800 us |
| libc_memcmp             | 1248.900 us | 1195.200 us | 1351.200 us | 1777.000 us |
| memx_memcmp             |  652.500 us |  671.780 us |  681.010 us |  707.360 us |
| memx_memcmp_basic       |  673.990 us |  723.850 us |  904.490 us |  679.730 us |

- `µs` is micro seconds
- `:en` is english haystack or short length.
- `:ja` is japanese haystack or long length.
- `gnu` is x86_64-unknown-linux-gnu
- `musl` is x86_64-unknown-linux-musl
- bench on intel Q6600 @ 2.40GHz or armv7 1.5GHz
