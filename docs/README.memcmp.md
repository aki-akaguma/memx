## Benchmark results

- compile by rustc 1.68.1 (8460ca823 2023-03-20)

  0. x86_64-unknown-linux- @Broadwell:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memcmp              |    9.206 µs |   13.009 µs |   12.456 µs |   41.429 µs |
| libc_memcmp             |    8.504 µs |   12.186 µs |   12.556 µs |   42.673 µs |
| memx_memcmp             |    7.976 µs |   12.712 µs |    8.485 µs |   12.945 µs |
| memx_memcmp_basic       |    8.391 µs |   13.061 µs |    8.059 µs |   12.645 µs |
| memx_memcmp_sse2        |    7.909 µs |   12.663 µs |    8.632 µs |   12.952 µs |
| memx_memcmp_avx2        |    8.753 µs |   13.362 µs |    8.552 µs |   14.860 µs |

  1. x86_64-unknown-linux- @Q6600:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memcmp              |   21.283 µs |   50.419 µs |   26.478 µs |   61.473 µs |
| libc_memcmp             |   21.914 µs |   50.101 µs |   26.236 µs |   61.064 µs |
| memx_memcmp             |   17.002 µs |   28.701 µs |   16.734 µs |   30.325 µs |
| memx_memcmp_basic       |   15.779 µs |   27.268 µs |   16.231 µs |   27.943 µs |
| memx_memcmp_sse2        |   17.092 µs |   28.277 µs |   18.139 µs |   30.390 µs |

  2. i686-unknown-linux- @Q6600:
  
|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memcmp              |   22.985 µs |   52.392 µs |   28.305 µs |   63.462 µs |
| libc_memcmp             |   22.283 µs |   50.046 µs |   28.242 µs |   64.040 µs |
| memx_memcmp             |   16.612 µs |   34.187 µs |   19.503 µs |   37.988 µs |
| memx_memcmp_basic       |   16.729 µs |   32.967 µs |   19.580 µs |   38.129 µs |
| memx_memcmp_sse2        |   17.602 µs |   32.624 µs |   19.210 µs |   40.641 µs |

  3. i586-unknown-linux- @Q6600:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memcmp              |   25.117 µs |   55.438 µs |   29.250 µs |   65.438 µs |
| libc_memcmp             |   23.589 µs |   52.018 µs |   28.100 µs |   63.683 µs |
| memx_memcmp             |   18.484 µs |   37.211 µs |   20.130 µs |   37.754 µs |
| memx_memcmp_basic       |   17.538 µs |   35.694 µs |   20.169 µs |   38.847 µs |


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
