## Benchmark results

- compile by rustc 1.68.1 (8460ca823 2023-03-20)

  0. x86_64-unknown-linux- @Broadwell:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memrchr             |   17.511 µs |   31.124 µs |   19.387 µs |   32.443 µs |
| libc_memrchr            |    7.666 µs |    9.059 µs |   18.522 µs |   29.011 µs |
| memchr_memrchr          |    7.442 µs |    8.974 µs |    7.236 µs |    9.001 µs |
| memx_memrchr            |    6.564 µs |    7.729 µs |    6.681 µs |    7.992 µs |
| memx_memrchr_basic      |    7.514 µs |   10.459 µs |    7.509 µs |   10.821 µs |
| memx_memrchr_sse2       |    6.545 µs |    7.859 µs |    6.554 µs |    8.089 µs |
| memx_memrchr_avx2       |    6.565 µs |    7.814 µs |    6.595 µs |    7.677 µs |

  1. x86_64-unknown-linux- @Q6600:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memrchr             |   28.862 µs |   47.915 µs |   28.698 µs |   47.736 µs |
| libc_memrchr            |   13.587 µs |   16.338 µs |   33.185 µs |   54.615 µs |
| memchr_memrchr          |   16.307 µs |   20.206 µs |   16.238 µs |   19.984 µs |
| memx_memrchr            |   13.933 µs |   16.558 µs |   14.578 µs |   16.984 µs |
| memx_memrchr_basic      |   16.574 µs |   22.652 µs |   16.044 µs |   22.112 µs |
| memx_memrchr_sse2       |   12.823 µs |   14.357 µs |   13.184 µs |   14.571 µs |

  2. i686-unknown-linux- @Q6600:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memrchr             |   25.531 µs |   41.600 µs |   26.363 µs |   42.625 µs |
| libc_memrchr            |   15.956 µs |   18.954 µs |   31.792 µs |   47.523 µs |
| memchr_memrchr          |   24.420 µs |   33.167 µs |   26.763 µs |   35.473 µs |
| memx_memrchr            |   20.113 µs |   22.248 µs |   20.481 µs |   23.242 µs |
| memx_memrchr_basic      |   18.558 µs |   28.879 µs |   17.435 µs |   26.171 µs |
| memx_memrchr_sse2       |   18.448 µs |   20.386 µs |   18.224 µs |   19.592 µs |

  3. i586-unknown-linux- @Q6600:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memrchr             |   26.285 µs |   42.461 µs |   26.545 µs |   42.786 µs |
| libc_memrchr            |   16.354 µs |   18.632 µs |   31.821 µs |   47.440 µs |
| memchr_memrchr          |   26.503 µs |   36.312 µs |   26.891 µs |   35.810 µs |
| memx_memrchr            |   19.783 µs |   22.720 µs |   20.466 µs |   22.708 µs |
| memx_memrchr_basic      |   18.636 µs |   28.837 µs |   17.855 µs |   26.123 µs |


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
