## Benchmark results

- compile by rustc 1.68.1 (8460ca823 2023-03-20)

  0. x86_64-unknown-linux- @Broadwell:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memrchr_double      |   16.347 µs |   29.675 µs |   16.338 µs |   30.004 µs |
| memchr_memrchr_double   |    9.646 µs |   11.211 µs |    9.807 µs |   11.142 µs |
| memx_memrchr_double     |    7.820 µs |    8.997 µs |    7.799 µs |    9.174 µs |
| memx_memrchr_w_basic    |   10.837 µs |   14.912 µs |   10.758 µs |   15.322 µs |
| memx_memrchr_w_sse2     |    7.855 µs |    9.622 µs |    7.855 µs |    9.683 µs |
| memx_memrchr_w_avx2     |    7.726 µs |    8.977 µs |    7.672 µs |    9.033 µs |

  1. x86_64-unknown-linux- @Q6600:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memrchr_double      |   32.306 µs |   53.684 µs |   32.588 µs |   52.928 µs |
| memchr_memrchr_double   |   21.110 µs |   25.723 µs |   21.468 µs |   26.379 µs |
| memx_memrchr_double     |   19.338 µs |   22.600 µs |   19.299 µs |   21.975 µs |
| memx_memrchr_w_basic    |   21.571 µs |   28.116 µs |   21.257 µs |   28.435 µs |
| memx_memrchr_w_sse2     |   18.459 µs |   19.682 µs |   18.359 µs |   20.344 µs |

  2. i686-unknown-linux- @Q6600:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memrchr_double      |   32.434 µs |   53.547 µs |   33.135 µs |   53.164 µs |
| memchr_memrchr_double   |   38.737 µs |   53.952 µs |   39.950 µs |   55.311 µs |
| memx_memrchr_double     |   26.044 µs |   29.574 µs |   27.170 µs |   30.613 µs |
| memx_memrchr_w_basic    |   30.237 µs |   44.752 µs |   30.073 µs |   44.622 µs |
| memx_memrchr_w_sse2     |   25.523 µs |   27.700 µs |   24.942 µs |   27.348 µs |

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
- bench on intel Q6600 @ 2.40GHz or armv7 1.5GHz
