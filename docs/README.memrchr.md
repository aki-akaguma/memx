## Benchmark results

- compile by rustc 1.68.0 (2c8cc3432 2023-03-06)

  0. x86_64-unknown-linux- @Broadwell:

|:------------------------|------------:|------------:|------------:|------------:|
| std_memrchr             |   19.387 µs |   31.556 µs |   15.799 µs |   30.052 µs |
| libc_memrchr            |    7.657 µs |    9.059 µs |   14.247 µs |   27.313 µs |
| memchr_memrchr          |    7.323 µs |    9.022 µs |    7.221 µs |    9.104 µs |
| memx_memrchr            |    7.166 µs |    8.386 µs |    7.486 µs |    8.767 µs |
| memx_memrchr_basic      |    7.499 µs |   10.643 µs |    7.514 µs |   10.804 µs |
| memx_memrchr_sse2       |    6.461 µs |    7.801 µs |    6.537 µs |    8.072 µs |
| memx_memrchr_avx2       |    6.564 µs |    7.638 µs |    6.473 µs |    7.833 µs |

  1. x86_64-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memrchr             |   28.192 µs |   48.011 µs |   29.169 µs |   47.712 µs |
| libc_memrchr            |   13.480 µs |   15.799 µs |   32.845 µs |   53.965 µs |
| memchr_memrchr          |   16.224 µs |   20.203 µs |   16.279 µs |   19.938 µs |
| memx_memrchr            |   15.877 µs |   18.987 µs |   15.673 µs |   17.950 µs |
| memx_memrchr_basic      |   16.760 µs |   22.554 µs |   15.976 µs |   23.319 µs |
| memx_memrchr_sse2       |   12.811 µs |   14.259 µs |   13.574 µs |   14.732 µs |

  2. i686-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memrchr             |   25.619 µs |   42.320 µs |   25.555 µs |   41.498 µs |
| libc_memrchr            |   15.995 µs |   19.069 µs |   29.833 µs |   45.260 µs |
| memchr_memrchr          |   24.593 µs |   33.092 µs |   26.757 µs |   35.659 µs |
| memx_memrchr            |   19.257 µs |   23.506 µs |   20.343 µs |   24.462 µs |
| memx_memrchr_basic      |   18.407 µs |   28.889 µs |   17.392 µs |   25.537 µs |
| memx_memrchr_sse2       |   16.786 µs |   18.273 µs |   17.602 µs |   18.335 µs |

  3. i586-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memrchr             |   26.704 µs |   42.400 µs |   26.024 µs |   42.794 µs |
| libc_memrchr            |   16.274 µs |   18.612 µs |   30.091 µs |   45.258 µs |
| memchr_memrchr          |   26.494 µs |   36.218 µs |   26.456 µs |   36.144 µs |
| memx_memrchr            |   20.533 µs |   23.624 µs |   21.336 µs |   24.730 µs |
| memx_memrchr_basic      |   18.114 µs |   28.348 µs |   17.864 µs |   26.227 µs |


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

- `us` is micro seconds
- `:en` is english haystack.
- `:ja` is japanese haystack.
- `musl` is x86_64-unknown-linux-musl
- bench on intel Q6600 @ 2.40GHz or armv7 1.5GHz
