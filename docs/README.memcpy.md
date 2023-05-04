## Benchmark results

- compile by rustc 1.69.0 (84c898d65 2023-04-16)

  0. x86_64-unknown-linux- @i5-4570:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memcpy              |    3.154 µs |    3.358 µs |   20.511 µs |    4.923 µs |
| libc_memcpy             |    3.464 µs |    3.236 µs |   20.548 µs |    5.024 µs |
| memx_memcpy             |    6.536 µs |    3.258 µs |    7.064 µs |    3.342 µs |
| memx_memcpy_basic       |    7.342 µs |    6.288 µs |    7.023 µs |    5.179 µs |
| memx_memcpy_sse2        |    7.061 µs |    3.424 µs |    6.924 µs |    3.128 µs |
| memx_memcpy_avx2        |    6.548 µs |    3.224 µs |    7.072 µs |    3.354 µs |

  1. x86_64-unknown-linux- @Q6600:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memcpy              |    7.529 µs |    9.469 µs |   40.096 µs |    9.819 µs |
| libc_memcpy             |    7.724 µs |    9.301 µs |   46.730 µs |    9.836 µs |
| memx_memcpy             |   14.133 µs |    7.013 µs |   13.976 µs |    6.987 µs |
| memx_memcpy_basic       |   15.395 µs |   13.385 µs |   13.992 µs |   10.722 µs |
| memx_memcpy_sse2        |   14.023 µs |    6.999 µs |   13.969 µs |    7.031 µs |

  2. i686-unknown-linux- @Q6600:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memcpy              |   16.600 µs |   10.256 µs |   31.446 µs |   15.342 µs |
| libc_memcpy             |   15.340 µs |    9.834 µs |   30.786 µs |   14.831 µs |
| memx_memcpy             |   21.488 µs |   10.016 µs |   21.364 µs |   10.354 µs |
| memx_memcpy_basic       |   17.950 µs |   24.144 µs |   19.343 µs |   24.634 µs |
| memx_memcpy_sse2        |   21.220 µs |    9.854 µs |   21.223 µs |   10.253 µs |

  3. i586-unknown-linux- @Q6600:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memcpy              |   16.006 µs |   10.087 µs |   31.185 µs |   15.001 µs |
| libc_memcpy             |   15.170 µs |    9.991 µs |   30.470 µs |   14.866 µs |
| memx_memcpy             |   20.551 µs |    9.775 µs |   22.117 µs |   10.285 µs |
| memx_memcpy_basic       |   20.785 µs |   25.466 µs |   22.206 µs |   25.838 µs |


- compile by rustc 1.53.0 (53cb7b09b 2021-06-17)

  1. x86_64-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memcpy              |  138.630 us |  207.800 us |  451.910 us |  433.550 us |
| libc_memcpy             |  134.360 us |  208.740 us |  452.620 us |  431.460 us |
| memx_memcpy             |  176.000 us |  235.350 us |  123.370 us |  274.940 us |
| memx_memcpy_basic       |  162.600 us |  256.200 us |  111.340 us |  225.810 us |
| memx_memcpy_sse2        |  170.640 us |  208.160 us |  119.570 us |  245.090 us |

  2. i686-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memcpy              |  137.790 us |  218.910 us |  274.320 us |  357.000 us |
| libc_memcpy             |  123.630 us |  206.300 us |  278.470 us |  353.410 us |
| memx_memcpy             |  213.070 us |  263.990 us |  228.690 us |  359.110 us |
| memx_memcpy_basic       |  164.670 us |  247.550 us |  163.570 us |  348.740 us |
| memx_memcpy_sse2        |  165.470 us |  221.280 us |  154.790 us |  292.220 us |

  3. i586-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memcpy              |  136.440 us |  248.370 us |  276.420 us |  355.800 us |
| libc_memcpy             |  122.100 us |  260.420 us |  278.000 us |  358.020 us |
| memx_memcpy             |  162.350 us |  300.770 us |  166.830 us |  290.480 us |
| memx_memcpy_basic       |  161.060 us |  301.330 us |  165.600 us |  289.560 us |

  4. armv7-linux-androideabi:

|         `name`          |  `andr:en`  |  `andr:ja`  |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memcpy              | 1071.000 us | 2871.600 us |  985.590 us | 1400.000 us |
| libc_memcpy             | 1042.100 us | 2949.400 us | 1058.600 us | 1550.600 us |
| memx_memcpy             |  601.610 us | 1268.700 us |  402.380 us | 1148.700 us |
| memx_memcpy_basic       |  525.990 us | 1086.900 us |  336.710 us |  835.900 us |

- `µs` is micro seconds
- `:en` is english haystack or short length.
- `:ja` is japanese haystack or long length.
- `gnu` is x86_64-unknown-linux-gnu
- `musl` is x86_64-unknown-linux-musl
- @i5-4570: bench on intel i5-4570 @ 3.2GHz
- @Q6600: bench on intel Q6600 @ 2.40GHz or armv7 1.5GHz
