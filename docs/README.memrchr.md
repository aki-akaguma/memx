## Benchmark results

- compile by rustc 1.69.0 (84c898d65 2023-04-16)

  0. x86_64-unknown-linux- @i5-4570:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memrchr             |   11.274 µs |   20.522 µs |   11.130 µs |   20.152 µs |
| libc_memrchr            |    5.332 µs |    6.276 µs |   10.804 µs |   19.944 µs |
| memchr_memrchr          |    5.186 µs |    6.308 µs |    5.106 µs |    6.269 µs |
| memx_memrchr            |    4.903 µs |    5.549 µs |    4.825 µs |    5.543 µs |
| memx_memrchr_basic      |    6.495 µs |    8.680 µs |    6.507 µs |    8.669 µs |
| memx_memrchr_sse2       |    5.255 µs |    6.263 µs |    5.240 µs |    6.225 µs |
| memx_memrchr_avx2       |    4.904 µs |    5.561 µs |    4.821 µs |    5.532 µs |

  1. x86_64-unknown-linux- @Q6600:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memrchr             |   29.111 µs |   47.890 µs |   28.226 µs |   47.997 µs |
| libc_memrchr            |   13.660 µs |   16.161 µs |   32.948 µs |   53.964 µs |
| memchr_memrchr          |   16.227 µs |   20.514 µs |   16.234 µs |   20.053 µs |
| memx_memrchr            |   14.435 µs |   17.287 µs |   14.507 µs |   17.637 µs |
| memx_memrchr_basic      |   15.310 µs |   22.185 µs |   15.135 µs |   21.245 µs |
| memx_memrchr_sse2       |   11.647 µs |   13.745 µs |   12.733 µs |   14.284 µs |

  2. i686-unknown-linux- @Q6600:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memrchr             |   26.110 µs |   42.332 µs |   26.344 µs |   42.584 µs |
| libc_memrchr            |   16.111 µs |   19.111 µs |   31.867 µs |   47.544 µs |
| memchr_memrchr          |   24.125 µs |   32.402 µs |   26.120 µs |   34.935 µs |
| memx_memrchr            |   19.841 µs |   23.663 µs |   21.598 µs |   24.846 µs |
| memx_memrchr_basic      |   20.308 µs |   32.906 µs |   20.891 µs |   34.362 µs |
| memx_memrchr_sse2       |   16.313 µs |   18.964 µs |   17.675 µs |   19.049 µs |

  3. i586-unknown-linux- @Q6600:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memrchr             |   26.118 µs |   42.126 µs |   26.218 µs |   42.019 µs |
| libc_memrchr            |   16.298 µs |   19.065 µs |   30.441 µs |   45.702 µs |
| memchr_memrchr          |   26.686 µs |   36.143 µs |   26.542 µs |   36.065 µs |
| memx_memrchr            |   20.654 µs |   23.185 µs |   20.945 µs |   24.220 µs |
| memx_memrchr_basic      |   20.719 µs |   33.980 µs |   20.437 µs |   34.758 µs |


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
