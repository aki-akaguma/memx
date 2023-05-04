## Benchmark results

- compile by rustc 1.69.0 (84c898d65 2023-04-16)

  0. x86_64-unknown-linux- @i5-4570:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memcmp              |    1.732 µs |    1.616 µs |    3.575 µs |    8.799 µs |
| libc_memcmp             |    1.703 µs |    1.574 µs |    3.537 µs |    9.931 µs |
| memx_memcmp             |    2.932 µs |    2.069 µs |    2.987 µs |    2.355 µs |
| memx_memcmp_basic       |    2.775 µs |    2.541 µs |    2.798 µs |    2.287 µs |
| memx_memcmp_sse2        |    2.828 µs |    2.201 µs |    2.885 µs |    2.343 µs |
| memx_memcmp_avx2        |    2.900 µs |    2.032 µs |    2.902 µs |    2.310 µs |

  1. x86_64-unknown-linux- @Q6600:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memcmp              |    7.216 µs |    7.612 µs |   10.167 µs |   19.071 µs |
| libc_memcmp             |    7.439 µs |    7.263 µs |   10.049 µs |   18.999 µs |
| memx_memcmp             |    7.753 µs |    6.994 µs |    7.770 µs |    7.618 µs |
| memx_memcmp_basic       |    7.383 µs |    6.946 µs |    7.125 µs |    6.835 µs |
| memx_memcmp_sse2        |    7.441 µs |    6.372 µs |    7.321 µs |    6.734 µs |

  2. i686-unknown-linux- @Q6600:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memcmp              |    8.093 µs |    7.672 µs |   11.664 µs |   20.733 µs |
| libc_memcmp             |    7.968 µs |    7.462 µs |   11.296 µs |   20.118 µs |
| memx_memcmp             |    9.502 µs |    9.395 µs |   10.534 µs |    9.730 µs |
| memx_memcmp_basic       |    7.828 µs |    9.194 µs |    8.899 µs |    9.351 µs |
| memx_memcmp_sse2        |    9.473 µs |    8.720 µs |   10.432 µs |    8.943 µs |

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
- @i5-4570: bench on intel i5-4570 @ 3.2GHz
- @Q6600: bench on intel Q6600 @ 2.40GHz or armv7 1.5GHz
