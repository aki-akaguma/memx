## Benchmark results

- compile by rustc 1.69.0 (84c898d65 2023-04-16)

  0. x86_64-unknown-linux- @i5-4570:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memnechr            |   11.653 µs |   20.134 µs |    8.964 µs |   17.302 µs |
| memx_memnechr           |    4.914 µs |    6.059 µs |    4.873 µs |    5.934 µs |
| memx_memnechr_basic     |    6.609 µs |    8.184 µs |    6.656 µs |    8.200 µs |
| memx_memnechr_sse2      |    4.900 µs |    6.006 µs |    4.963 µs |    6.067 µs |
| memx_memnechr_avx2      |    4.908 µs |    6.055 µs |    4.868 µs |    5.905 µs |

  1. x86_64-unknown-linux- @Q6600:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memnechr            |   25.659 µs |   39.551 µs |   26.365 µs |   40.104 µs |
| memx_memnechr           |   14.087 µs |   16.128 µs |   14.054 µs |   16.443 µs |
| memx_memnechr_basic     |   15.784 µs |   19.281 µs |   16.034 µs |   19.834 µs |
| memx_memnechr_sse2      |   10.837 µs |   12.901 µs |   11.136 µs |   13.216 µs |

  2. i686-unknown-linux- @Q6600:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memnechr            |   24.257 µs |   37.766 µs |   24.314 µs |   37.733 µs |
| memx_memnechr           |   20.904 µs |   23.082 µs |   20.144 µs |   21.911 µs |
| memx_memnechr_basic     |   19.402 µs |   25.956 µs |   20.812 µs |   28.217 µs |
| memx_memnechr_sse2      |   17.077 µs |   18.496 µs |   17.285 µs |   18.493 µs |

  3. i586-unknown-linux- @Q6600:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memnechr            |   24.637 µs |   37.970 µs |   27.415 µs |   38.768 µs |
| memx_memnechr           |   20.468 µs |   21.395 µs |   19.967 µs |   21.201 µs |
| memx_memnechr_basic     |   20.860 µs |   27.731 µs |   27.275 µs |   36.620 µs |


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

- `µs` is micro seconds
- `:en` is english haystack or short length.
- `:ja` is japanese haystack or long length.
- `gnu` is x86_64-unknown-linux-gnu
- `musl` is x86_64-unknown-linux-musl
- @i5-4570: bench on intel i5-4570 @ 3.2GHz
- @Q6600: bench on intel Q6600 @ 2.40GHz or armv7 1.5GHz
