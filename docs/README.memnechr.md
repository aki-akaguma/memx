## Benchmark results

- compile by rustc 1.69.0 (84c898d65 2023-04-16)

  0. x86_64-unknown-linux- @i5-4570:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memnechr            |   11.404 µs |   19.792 µs |    8.684 µs |   17.267 µs |
| memx_memnechr           |    4.781 µs |    5.619 µs |    4.642 µs |    5.577 µs |
| memx_memnechr_basic     |    7.010 µs |    8.789 µs |    6.997 µs |    8.766 µs |
| memx_memnechr_sse2      |    5.083 µs |    6.126 µs |    5.073 µs |    6.164 µs |
| memx_memnechr_avx2      |    4.731 µs |    5.648 µs |    4.646 µs |    5.577 µs |

  1. x86_64-unknown-linux- @Q6600:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memnechr            |   25.417 µs |   38.904 µs |   26.222 µs |   39.318 µs |
| memx_memnechr           |   15.788 µs |   18.433 µs |   14.177 µs |   16.772 µs |
| memx_memnechr_basic     |   15.847 µs |   20.714 µs |   15.760 µs |   21.862 µs |
| memx_memnechr_sse2      |   12.266 µs |   13.267 µs |   11.790 µs |   13.323 µs |

  2. i686-unknown-linux- @Q6600:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memnechr            |   24.153 µs |   37.500 µs |   24.501 µs |   37.934 µs |
| memx_memnechr           |   20.985 µs |   23.885 µs |   22.518 µs |   24.953 µs |
| memx_memnechr_basic     |   20.639 µs |   32.940 µs |   22.729 µs |   35.810 µs |
| memx_memnechr_sse2      |   17.385 µs |   20.159 µs |   18.270 µs |   19.269 µs |

  3. i586-unknown-linux- @Q6600:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memnechr            |   24.336 µs |   37.821 µs |   25.624 µs |   38.722 µs |
| memx_memnechr           |   21.190 µs |   23.495 µs |   21.982 µs |   23.446 µs |
| memx_memnechr_basic     |   22.899 µs |   34.513 µs |   25.723 µs |   38.287 µs |


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
