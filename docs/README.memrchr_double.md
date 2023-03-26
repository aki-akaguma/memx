## Benchmark results

- compile by rustc 1.68.1 (8460ca823 2023-03-20)

  0. x86_64-unknown-linux- @Broadwell:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memrchr_double      |   16.407 µs |   30.070 µs |   16.433 µs |   29.680 µs |
| memchr_memrchr_double   |    9.712 µs |   11.080 µs |    9.712 µs |   11.339 µs |
| memx_memrchr_double     |    8.377 µs |    9.678 µs |    8.279 µs |    9.972 µs |
| memx_memrchr_w_basic    |   10.609 µs |   14.760 µs |   10.634 µs |   15.005 µs |
| memx_memrchr_w_sse2     |    7.752 µs |    9.699 µs |    7.835 µs |    9.700 µs |
| memx_memrchr_w_avx2     |    7.724 µs |    9.011 µs |    7.690 µs |    9.376 µs |

  1. x86_64-unknown-linux- @Q6600:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memrchr_double      |   32.264 µs |   52.811 µs |   32.290 µs |   53.627 µs |
| memchr_memrchr_double   |   21.199 µs |   25.968 µs |   21.282 µs |   25.442 µs |
| memx_memrchr_double     |   22.959 µs |   26.569 µs |   22.514 µs |   25.398 µs |
| memx_memrchr_w_basic    |   20.451 µs |   27.177 µs |   20.396 µs |   27.608 µs |
| memx_memrchr_w_sse2     |   18.192 µs |   21.379 µs |   18.082 µs |   20.629 µs |

  2. i686-unknown-linux- @Q6600:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memrchr_double      |   32.228 µs |   53.236 µs |   32.243 µs |   53.278 µs |
| memchr_memrchr_double   |   38.347 µs |   53.671 µs |   39.741 µs |   56.870 µs |
| memx_memrchr_double     |   28.089 µs |   29.476 µs |   28.725 µs |   30.788 µs |
| memx_memrchr_w_basic    |   26.396 µs |   41.177 µs |   28.003 µs |   45.047 µs |
| memx_memrchr_w_sse2     |   24.048 µs |   25.688 µs |   23.778 µs |   27.246 µs |

  3. i586-unknown-linux- @Q6600:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memrchr_double      |   33.191 µs |   53.324 µs |   32.742 µs |   53.430 µs |
| memchr_memrchr_double   |   39.102 µs |   53.703 µs |   39.477 µs |   55.236 µs |
| memx_memrchr_double     |   26.688 µs |   28.662 µs |   27.421 µs |   29.871 µs |
| memx_memrchr_w_basic    |   25.671 µs |   39.614 µs |   27.966 µs |   44.508 µs |


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
- bench on intel Q6600 @ 2.40GHz or armv7 1.5GHz
