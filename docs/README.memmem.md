## Benchmark results

- compile by rustc 1.68.1 (8460ca823 2023-03-20)

  0. x86_64-unknown-linux- @Broadwell:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memmem              |   11.750 µs |   10.461 µs |   11.618 µs |   10.389 µs |
| libc_memmem             |    3.155 µs |    5.742 µs |    5.670 µs |   10.288 µs |
| memchr_memmem           |   13.596 µs |   12.863 µs |   13.203 µs |   12.828 µs |
| memx_memmem             |    0.924 µs |    1.462 µs |    0.876 µs |    1.445 µs |
| memx_memmem_basic       |    2.144 µs |    3.796 µs |    2.194 µs |    3.888 µs |
| memx_memmem_sse2        |    1.142 µs |    1.951 µs |    1.142 µs |    1.941 µs |
| memx_memmem_avx2        |    0.918 µs |    1.461 µs |    0.878 µs |    1.460 µs |

  1. x86_64-unknown-linux- @Q6600:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memmem              |   24.481 µs |   25.394 µs |   24.423 µs |   26.217 µs |
| libc_memmem             |    4.614 µs |   10.685 µs |    9.027 µs |   14.186 µs |
| memchr_memmem           |   26.218 µs |   29.658 µs |   26.220 µs |   29.958 µs |
| memx_memmem             |    1.973 µs |    3.903 µs |    1.969 µs |    3.904 µs |
| memx_memmem_basic       |    4.052 µs |    9.001 µs |    3.852 µs |    8.983 µs |
| memx_memmem_sse2        |    2.006 µs |    3.918 µs |    1.980 µs |    3.873 µs |

  2. i686-unknown-linux- @Q6600:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memmem              |   34.634 µs |   36.644 µs |   32.989 µs |   35.976 µs |
| libc_memmem             |    5.509 µs |   11.062 µs |   10.973 µs |   15.579 µs |
| memchr_memmem           |   34.451 µs |   61.212 µs |   34.375 µs |   61.814 µs |
| memx_memmem             |    2.320 µs |    3.809 µs |    2.232 µs |    3.787 µs |
| memx_memmem_basic       |    6.394 µs |   12.918 µs |    6.395 µs |   12.805 µs |
| memx_memmem_sse2        |    2.166 µs |    3.696 µs |    2.212 µs |    3.801 µs |

  3. i586-unknown-linux- @Q6600:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memmem              |   34.709 µs |   38.838 µs |   34.759 µs |   39.996 µs |
| libc_memmem             |    5.448 µs |   11.104 µs |   10.732 µs |   17.492 µs |
| memchr_memmem           |   33.306 µs |   64.011 µs |   36.366 µs |   68.740 µs |
| memx_memmem             |    2.424 µs |    3.895 µs |    2.243 µs |    3.984 µs |
| memx_memmem_basic       |    6.471 µs |   12.692 µs |    6.172 µs |   12.992 µs |


- compile by rustc 1.53.0 (53cb7b09b 2021-06-17)

  1. x86_64-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memmem              |  513.410 us |  472.390 us |  509.130 us |  447.490 us |
| libc_memmem             |  131.950 us |  121.220 us |  219.310 us |  273.500 us |
| memchr_memmem           |  196.830 us |  225.500 us |  195.990 us |  213.690 us |
| memx_memmem             |  103.520 us |  104.140 us |  102.050 us |  103.850 us |
| memx_memmem_basic       |  105.840 us |  104.520 us |  105.040 us |  104.540 us |
| memx_memmem_sse2        |  103.510 us |  103.680 us |  104.710 us |  103.340 us |

  2. i686-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memmem              |  689.260 us |  555.490 us |  714.560 us |  583.750 us |
| libc_memmem             |  131.130 us |  120.530 us |  241.670 us |  252.080 us |
| memchr_memmem           |  472.230 us |  482.810 us |  503.740 us |  491.810 us |
| memx_memmem             |  117.360 us |  112.680 us |  128.280 us |  127.470 us |
| memx_memmem_basic       |  121.310 us |  120.250 us |  133.310 us |  121.630 us |
| memx_memmem_sse2        |  112.680 us |  109.490 us |  123.990 us |  122.480 us |

  3. i586-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memmem              |  751.850 us |  601.410 us |  719.720 us |  576.260 us |
| libc_memmem             |  131.200 us |  120.900 us |  241.750 us |  245.320 us |
| memchr_memmem           |  502.970 us |  569.400 us |  496.330 us |  564.220 us |
| memx_memmem             |  127.740 us |  124.620 us |  147.610 us |  129.860 us |
| memx_memmem_basic       |  123.320 us |  126.560 us |  142.830 us |  123.180 us |

  4. armv7-linux-androideabi:

|         `name`          |  `andr:en`  |  `andr:ja`  |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memmem              | 7409.000 us | 3891.700 us | 3709.500 us | 1918.400 us |
| libc_memmem             | 3591.800 us | 3580.600 us | 1554.300 us | 1453.900 us |
| memchr_memmem           | 3747.600 us | 3391.700 us | 2182.600 us | 1897.900 us |
| memx_memmem             | 2243.900 us | 2005.800 us |  546.910 us |  582.020 us |
| memx_memmem_basic       | 2244.100 us | 2122.200 us |  570.210 us |  549.610 us |

- `µs` is micro seconds
- `:en` is english haystack or short length.
- `:ja` is japanese haystack or long length.
- `gnu` is x86_64-unknown-linux-gnu
- `musl` is x86_64-unknown-linux-musl
- bench on intel Q6600 @ 2.40GHz or armv7 1.5GHz
