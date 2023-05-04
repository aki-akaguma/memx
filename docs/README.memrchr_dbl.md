## Benchmark results

- compile by rustc 1.69.0 (84c898d65 2023-04-16)

  0. x86_64-unknown-linux- @i5-4570:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memrchr_dbl         |   12.235 µs |   21.802 µs |   11.121 µs |   20.681 µs |
| memchr_memrchr_dbl      |    6.839 µs |    7.751 µs |    6.797 µs |    7.811 µs |
| memx_memrchr_dbl        |    5.933 µs |    6.795 µs |    5.852 µs |    6.969 µs |
| memx_memrchr_w_basic    |    8.761 µs |   12.011 µs |    8.342 µs |   11.628 µs |
| memx_memrchr_w_sse2     |    5.835 µs |    6.849 µs |    5.852 µs |    6.838 µs |
| memx_memrchr_w_avx2     |    5.977 µs |    6.814 µs |    5.870 µs |    6.973 µs |

  1. x86_64-unknown-linux- @Q6600:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memrchr_dbl         |   33.601 µs |   53.750 µs |   33.067 µs |   53.803 µs |
| memchr_memrchr_dbl      |   21.458 µs |   25.744 µs |   22.105 µs |   26.392 µs |
| memx_memrchr_dbl        |   20.621 µs |   24.656 µs |   20.752 µs |   24.059 µs |
| memx_memrchr_w_basic    |   21.814 µs |   30.338 µs |   21.158 µs |   30.836 µs |
| memx_memrchr_w_sse2     |   18.978 µs |   21.930 µs |   19.296 µs |   22.296 µs |

  2. i686-unknown-linux- @Q6600:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memrchr_dbl         |   33.137 µs |   53.089 µs |   33.147 µs |   53.273 µs |
| memchr_memrchr_dbl      |   38.929 µs |   53.550 µs |   39.695 µs |   55.342 µs |
| memx_memrchr_dbl        |   27.640 µs |   31.709 µs |   27.090 µs |   32.048 µs |
| memx_memrchr_w_basic    |   31.747 µs |   47.265 µs |   34.754 µs |   51.597 µs |
| memx_memrchr_w_sse2     |   26.858 µs |   29.387 µs |   26.953 µs |   31.395 µs |

  3. i586-unknown-linux- @Q6600:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memrchr_double      |   32.927 µs |   53.516 µs |   33.282 µs |   53.319 µs |
| memchr_memrchr_double   |   38.743 µs |   53.234 µs |   40.087 µs |   55.135 µs |
| memx_memrchr_double     |   26.705 µs |   29.733 µs |   27.134 µs |   30.484 µs |
| memx_memrchr_w_basic    |   35.702 µs |   50.514 µs |   41.044 µs |   58.467 µs |


- compile by rustc 1.53.0 (53cb7b09b 2021-06-17)

  1. x86_64-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memrchr             |  510.110 us |  231.010 us |  500.410 us |  241.550 us |
| libc_memrchr            |  216.100 us |   73.073 us |  543.330 us |  266.120 us |
| memchr_memrchr          |  222.390 us |   69.625 us |  221.260 us |   69.196 us |
| memx_memrchr            |  257.000 us |   73.247 us |  257.010 us |   73.865 us |
| memx_memrchr_basic      |  260.940 us |   95.468 us |  260.350 us |   96.719 us |
| memx_memrchr_sse2       |  251.610 us |   72.529 us |  252.000 us |   73.265 us |

  2. i686-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memrchr             |  389.890 us |  200.600 us |  285.170 us |  200.080 us |
| libc_memrchr            |  273.230 us |   77.156 us |  606.080 us |  210.860 us |
| memchr_memrchr          |  559.270 us |  219.320 us |  525.350 us |  205.200 us |
| memx_memrchr            |  353.940 us |   97.693 us |  410.320 us |  112.270 us |
| memx_memrchr_basic      |  274.110 us |  121.560 us |  272.420 us |  119.450 us |
| memx_memrchr_sse2       |  276.410 us |   79.334 us |  314.740 us |   86.983 us |

  3. i586-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memrchr             |  300.750 us |  203.110 us |  388.990 us |  193.850 us |
| libc_memrchr            |  233.760 us |   77.464 us |  546.530 us |  208.460 us |
| memchr_memrchr          |  523.880 us |  199.920 us |  474.810 us |  185.200 us |
| memx_memrchr            |  272.020 us |  125.010 us |  281.410 us |  120.190 us |
| memx_memrchr_basic      |  276.620 us |  122.500 us |  280.800 us |  119.710 us |

  4. armv7-linux-androideabi:

|         `name`          |  `andr:en`  |  `andr:ja`  |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memrchr             | 3066.700 us | 2229.900 us | 1380.100 us |  905.320 us |
| libc_memrchr            | 3308.500 us | 2453.900 us | 1543.800 us | 1285.000 us |
| memchr_memrchr          | 2925.400 us | 2269.500 us | 1655.700 us |  687.610 us |
| memx_memrchr            | 2507.200 us | 1932.400 us | 1095.700 us |  516.470 us |
| memx_memrchr_basic      | 2503.900 us | 1978.000 us | 1136.900 us |  496.970 us |

- `µs` is micro seconds
- `:en` is english haystack or short length.
- `:ja` is japanese haystack or long length.
- `gnu` is x86_64-unknown-linux-gnu
- `musl` is x86_64-unknown-linux-musl
- @i5-4570: bench on intel i5-4570 @ 3.2GHz
- @Q6600: bench on intel Q6600 @ 2.40GHz or armv7 1.5GHz
