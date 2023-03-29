## Benchmark results

- compile by rustc 1.68.1 (8460ca823 2023-03-20)

  0. x86_64-unknown-linux- @Broadwell:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memcmp              |    9.311 µs |   12.886 µs |   13.574 µs |   42.170 µs |
| libc_memcmp             |    8.545 µs |   12.263 µs |   13.972 µs |   41.290 µs |
| memx_memcmp             |    8.571 µs |   13.029 µs |    9.246 µs |   13.427 µs |
| memx_memcmp_basic       |    9.015 µs |   13.352 µs |    8.800 µs |   13.237 µs |
| memx_memcmp_sse2        |    8.577 µs |   13.006 µs |    9.299 µs |   13.392 µs |
| memx_memcmp_avx2        |    9.461 µs |   13.731 µs |    9.188 µs |   13.651 µs |

  1. x86_64-unknown-linux- @Q6600:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memcmp              |   21.797 µs |   49.223 µs |   26.260 µs |   61.137 µs |
| libc_memcmp             |   20.931 µs |   49.693 µs |   26.390 µs |   61.512 µs |
| memx_memcmp             |   18.154 µs |   28.131 µs |   18.387 µs |   28.229 µs |
| memx_memcmp_basic       |   17.654 µs |   29.893 µs |   17.768 µs |   28.555 µs |
| memx_memcmp_sse2        |   17.756 µs |   28.939 µs |   18.129 µs |   28.917 µs |

  2. i686-unknown-linux- @Q6600:
  
|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memcmp              |   23.618 µs |   51.517 µs |   28.378 µs |   63.564 µs |
| libc_memcmp             |   22.104 µs |   50.144 µs |   28.302 µs |   63.646 µs |
| memx_memcmp             |   18.586 µs |   32.974 µs |   19.677 µs |   34.808 µs |
| memx_memcmp_basic       |   18.333 µs |   32.842 µs |   18.856 µs |   34.541 µs |
| memx_memcmp_sse2        |   18.708 µs |   33.198 µs |   18.914 µs |   34.034 µs |

  3. i586-unknown-linux- @Q6600:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memcmp              |   24.872 µs |   54.680 µs |   29.187 µs |   65.543 µs |
| libc_memcmp             |   22.892 µs |   51.637 µs |   27.964 µs |   63.729 µs |
| memx_memcmp             |   18.565 µs |   32.786 µs |   23.122 µs |   35.438 µs |
| memx_memcmp_basic       |   19.344 µs |   33.176 µs |   21.972 µs |   38.609 µs |

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
