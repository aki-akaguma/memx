## Benchmark results

- compile by rustc 1.68.0 (2c8cc3432 2023-03-06)

  0. x86_64-unknown-linux- @Broadwell:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memnechr            |   11.687 µs |   22.876 µs |   11.709 µs |   22.744 µs |
| memx_memnechr           |    7.379 µs |    8.729 µs |    7.261 µs |    8.653 µs |
| memx_memnechr_basic     |    8.991 µs |   11.467 µs |    8.972 µs |   11.314 µs |
| memx_memnechr_sse2      |    6.920 µs |    8.411 µs |    6.893 µs |    8.536 µs |
| memx_memnechr_avx2      |    7.064 µs |    8.516 µs |    7.020 µs |    8.274 µs |

  1. x86_64-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memnechr            |   25.934 µs |   39.434 µs |   25.971 µs |   39.421 µs |
| memx_memnechr           |   15.403 µs |   17.277 µs |   15.024 µs |   17.731 µs |
| memx_memnechr_basic     |   13.745 µs |   17.050 µs |   13.908 µs |   17.178 µs |
| memx_memnechr_sse2      |   10.901 µs |   13.402 µs |   10.522 µs |   12.987 µs |

  2. i686-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memnechr            |   24.157 µs |   37.753 µs |   24.286 µs |   37.790 µs |
| memx_memnechr           |   19.690 µs |   22.497 µs |   20.501 µs |   25.767 µs |
| memx_memnechr_basic     |   13.967 µs |   21.106 µs |   14.571 µs |   20.944 µs |
| memx_memnechr_sse2      |   13.705 µs |   15.522 µs |   14.776 µs |   15.987 µs |

  3. i586-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memnechr            |   24.518 µs |   38.062 µs |   25.661 µs |   38.271 µs |
| memx_memnechr           |   20.409 µs |   23.143 µs |   20.196 µs |   22.747 µs |
| memx_memnechr_basic     |   14.127 µs |   20.698 µs |   14.371 µs |   20.730 µs |


- compile by rustc 1.53.0 (53cb7b09b 2021-06-17)

  1. x86_64-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memnechr            |  368.560 us |  212.620 us |  357.730 us |  214.030 us |
| memx_memnechr           |  273.090 us |  126.310 us |  264.900 us |  127.690 us |
| memx_memnechr_basic     |  305.920 us |  162.800 us |  304.140 us |  164.920 us |
| memx_memnechr_sse2      |  267.470 us |  125.320 us |  261.060 us |  127.270 us |

  2. i686-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memnechr            |  431.530 us |  235.700 us |  496.540 us |  247.550 us |
| memx_memnechr           |  484.690 us |  199.290 us |  539.420 us |  219.530 us |
| memx_memnechr_basic     |  447.280 us |  206.660 us |  555.910 us |  221.390 us |
| memx_memnechr_sse2      |  430.740 us |  180.280 us |  452.040 us |  190.260 us |

  3. i586-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memnechr            |  388.950 us |  240.800 us |  387.410 us |  237.050 us |
| memx_memnechr           |  354.960 us |  191.770 us |  370.900 us |  195.360 us |
| memx_memnechr_basic     |  357.170 us |  195.810 us |  362.970 us |  189.400 us |

  4. armv7-linux-androideabi:

|         `name`          |  `andr:en`  |  `andr:ja`  |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memchr              | 1232.100 us |  950.750 us | 1262.200 us | 1047.500 us |
| libc_memchr             | 1874.100 us | 1066.300 us | 1865.200 us |  838.210 us |
| memchr_memchr           | 1391.400 us |  642.260 us | 1441.300 us |  649.600 us |
| memx_memchr             | 1042.000 us |  555.030 us | 1129.400 us |  519.740 us |
| memx_memchr_basic       | 1005.600 us |  573.360 us | 1095.400 us |  525.020 us |

- `us` is micro seconds
- `:en` is english haystack.
- `:ja` is japanese haystack.
- `musl` is x86_64-unknown-linux-musl
- bench on intel Q6600 @ 2.40GHz or armv7 1.5GHz
