## Benchmark results

- compile by rustc 1.68.1 (8460ca823 2023-03-20)

  0. x86_64-unknown-linux- @Broadwell:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memcpy              |    4.717 µs |    5.114 µs |   29.647 µs |    7.031 µs |
| libc_memcpy             |    4.663 µs |    4.910 µs |   29.559 µs |    7.199 µs |
| memx_memcpy             |    9.436 µs |    5.082 µs |   10.267 µs |    5.086 µs |
| memx_memcpy_basic       |   10.316 µs |    9.079 µs |   10.548 µs |    7.622 µs |
| memx_memcpy_sse2        |   10.199 µs |    4.878 µs |   10.002 µs |    5.084 µs |
| memx_memcpy_avx2        |    9.585 µs |    4.958 µs |   10.207 µs |    4.942 µs |

  1. x86_64-unknown-linux- @Q6600:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memcpy              |    7.405 µs |    7.555 µs |   38.428 µs |    9.597 µs |
| libc_memcpy             |    7.312 µs |    7.515 µs |   45.793 µs |    9.655 µs |
| memx_memcpy             |   13.836 µs |    6.893 µs |   13.645 µs |    6.850 µs |
| memx_memcpy_basic       |   13.972 µs |   10.501 µs |   13.481 µs |   13.112 µs |
| memx_memcpy_sse2        |   13.813 µs |    6.872 µs |   13.669 µs |    6.874 µs |

  2. i686-unknown-linux- @Q6600:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memcpy              |   16.389 µs |   10.308 µs |   30.689 µs |   14.848 µs |
| libc_memcpy             |   14.882 µs |    9.883 µs |   30.687 µs |   14.821 µs |
| memx_memcpy             |   21.152 µs |    9.938 µs |   21.752 µs |   10.049 µs |
| memx_memcpy_basic       |   18.042 µs |   24.151 µs |   19.232 µs |   24.625 µs |
| memx_memcpy_sse2        |   20.738 µs |    9.762 µs |   21.491 µs |   10.113 µs |

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
- bench on intel Q6600 @ 2.40GHz or armv7 1.5GHz
