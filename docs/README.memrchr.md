## Benchmark results

- compile by rustc 1.69.0 (84c898d65 2023-04-16)

  0. x86_64-unknown-linux- @i5-4570:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memrchr             |   11.762 µs |   20.752 µs |   10.808 µs |   20.020 µs |
| libc_memrchr            |    5.322 µs |    6.240 µs |   10.844 µs |   20.322 µs |
| memchr_memrchr          |    5.187 µs |    6.307 µs |    5.100 µs |    6.277 µs |
| memx_memrchr            |    4.920 µs |    5.659 µs |    4.824 µs |    5.700 µs |
| memx_memrchr_basic      |    5.521 µs |    7.271 µs |    5.509 µs |    7.374 µs |
| memx_memrchr_sse2       |    4.861 µs |    5.800 µs |    4.917 µs |    5.873 µs |
| memx_memrchr_avx2       |    4.904 µs |    5.659 µs |    4.813 µs |    5.680 µs |

  1. x86_64-unknown-linux- @Q6600:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memrchr             |   29.294 µs |   48.673 µs |   28.776 µs |   48.873 µs |
| libc_memrchr            |   13.740 µs |   16.472 µs |   33.519 µs |   54.810 µs |
| memchr_memrchr          |   16.606 µs |   20.771 µs |   16.574 µs |   20.639 µs |
| memx_memrchr            |   14.229 µs |   16.966 µs |   14.193 µs |   16.868 µs |
| memx_memrchr_basic      |   14.466 µs |   19.349 µs |   15.023 µs |   19.872 µs |
| memx_memrchr_sse2       |   12.356 µs |   13.795 µs |   12.209 µs |   13.597 µs |

  2. i686-unknown-linux- @Q6600:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memrchr             |   25.667 µs |   42.307 µs |   26.619 µs |   42.018 µs |
| libc_memrchr            |   15.993 µs |   18.780 µs |   31.562 µs |   46.676 µs |
| memchr_memrchr          |   24.886 µs |   33.885 µs |   26.656 µs |   35.722 µs |
| memx_memrchr            |   20.495 µs |   22.631 µs |   19.691 µs |   22.687 µs |
| memx_memrchr_basic      |   16.306 µs |   24.632 µs |   19.850 µs |   28.380 µs |
| memx_memrchr_sse2       |   16.818 µs |   17.510 µs |   15.582 µs |   17.984 µs |

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
- @i5-4570: bench on intel i5-4570 @ 3.2GHz
- @Q6600: bench on intel Q6600 @ 2.40GHz or armv7 1.5GHz
