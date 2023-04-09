## Benchmark results

- compile by rustc 1.68.1 (8460ca823 2023-03-20)

  0. x86_64-unknown-linux- @Broadwell:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memrchr             |   17.047 µs |   30.992 µs |   18.716 µs |   32.441 µs |
| libc_memrchr            |    7.616 µs |    9.168 µs |   14.835 µs |   27.913 µs |
| memchr_memrchr          |    7.426 µs |    9.719 µs |    7.224 µs |    9.145 µs |
| memx_memrchr            |    7.266 µs |    8.285 µs |    7.147 µs |    8.109 µs |
| memx_memrchr_basic      |    8.320 µs |   11.216 µs |    8.406 µs |   10.935 µs |
| memx_memrchr_sse2       |    6.838 µs |    8.333 µs |    6.897 µs |    8.170 µs |
| memx_memrchr_avx2       |    7.227 µs |    8.367 µs |    7.108 µs |    8.161 µs |

  1. x86_64-unknown-linux- @Q6600:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memrchr             |   28.378 µs |   48.133 µs |   28.944 µs |   48.238 µs |
| libc_memrchr            |   13.461 µs |   15.743 µs |   32.975 µs |   54.093 µs |
| memchr_memrchr          |   16.312 µs |   20.259 µs |   16.290 µs |   19.954 µs |
| memx_memrchr            |   13.866 µs |   16.073 µs |   13.843 µs |   16.431 µs |
| memx_memrchr_basic      |   14.508 µs |   19.705 µs |   14.546 µs |   19.458 µs |
| memx_memrchr_sse2       |   11.575 µs |   13.920 µs |   11.470 µs |   13.887 µs |

  2. i686-unknown-linux- @Q6600:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memrchr             |   26.124 µs |   43.044 µs |   26.368 µs |   42.551 µs |
| libc_memrchr            |   16.043 µs |   18.896 µs |   31.855 µs |   47.001 µs |
| memchr_memrchr          |   24.329 µs |   32.378 µs |   26.148 µs |   35.087 µs |
| memx_memrchr            |   19.503 µs |   21.987 µs |   19.487 µs |   23.263 µs |
| memx_memrchr_basic      |   17.122 µs |   25.704 µs |   20.152 µs |   31.542 µs |
| memx_memrchr_sse2       |   16.281 µs |   18.805 µs |   18.254 µs |   19.090 µs |

  3. i586-unknown-linux- @Q6600:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memrchr             |   26.129 µs |   42.214 µs |   26.333 µs |   42.881 µs |
| libc_memrchr            |   16.025 µs |   18.447 µs |   31.258 µs |   46.567 µs |
| memchr_memrchr          |   26.816 µs |   36.011 µs |   26.764 µs |   35.891 µs |
| memx_memrchr            |   19.601 µs |   21.783 µs |   19.910 µs |   22.346 µs |
| memx_memrchr_basic      |   21.053 µs |   31.155 µs |   26.745 µs |   37.206 µs |


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
