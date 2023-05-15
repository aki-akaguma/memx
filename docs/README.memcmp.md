## Benchmark results

- compile by rustc 1.69.0 (84c898d65 2023-04-16)

  0. x86_64-unknown-linux- @i5-4570:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memcmp              |    1.766 µs |    1.620 µs |    3.583 µs |    8.770 µs |
| libc_memcmp             |    1.584 µs |    1.582 µs |    3.389 µs |    8.653 µs |
| memx_memcmp             |    2.828 µs |    2.022 µs |    2.960 µs |    2.293 µs |
| memx_memcmp_basic       |    2.703 µs |    2.615 µs |    2.718 µs |    2.180 µs |
| memx_memcmp_sse2        |    2.745 µs |    2.104 µs |    2.790 µs |    2.123 µs |
| memx_memcmp_avx2        |    2.785 µs |    1.966 µs |    2.859 µs |    2.300 µs |

  1. x86_64-unknown-linux- @Q6600:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memcmp              |    7.324 µs |    7.385 µs |    9.870 µs |   18.688 µs |
| libc_memcmp             |    7.199 µs |    7.080 µs |    9.801 µs |   18.680 µs |
| memx_memcmp             |    7.359 µs |    7.129 µs |    7.552 µs |    6.799 µs |
| memx_memcmp_basic       |    7.047 µs |    6.732 µs |    9.855 µs |    6.635 µs |
| memx_memcmp_sse2        |    7.175 µs |    6.193 µs |    7.220 µs |    6.606 µs |

  2. i686-unknown-linux- @Q6600:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memcmp              |    8.277 µs |    8.098 µs |   11.645 µs |   20.766 µs |
| libc_memcmp             |    8.000 µs |    7.429 µs |   11.278 µs |   20.164 µs |
| memx_memcmp             |    9.997 µs |    9.430 µs |   10.530 µs |   10.288 µs |
| memx_memcmp_basic       |    7.808 µs |    9.021 µs |    8.665 µs |    9.585 µs |
| memx_memcmp_sse2        |    9.200 µs |    9.000 µs |   10.138 µs |    8.936 µs |

  3. i586-unknown-linux- @Q6600:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memcmp              |    9.050 µs |    9.113 µs |   11.855 µs |   20.717 µs |
| libc_memcmp             |    8.357 µs |    7.712 µs |   11.168 µs |   19.937 µs |
| memx_memcmp             |    9.523 µs |    9.171 µs |   10.366 µs |    9.902 µs |
| memx_memcmp_basic       |    9.803 µs |   10.505 µs |   10.119 µs |   11.222 µs |

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
- @i5-4570: bench on intel i5-4570 @ 3.2GHz
- @Q6600: bench on intel Q6600 @ 2.40GHz or armv7 1.5GHz
