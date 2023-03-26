## Benchmark results

- compile by rustc 1.68.1 (8460ca823 2023-03-20)

  0. x86_64-unknown-linux- @Broadwell:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memmem              |   11.715 µs |   10.271 µs |   11.748 µs |   10.183 µs |
| libc_memmem             |    3.112 µs |    5.701 µs |    5.835 µs |    9.899 µs |
| memchr_memmem           |   13.182 µs |   12.585 µs |   13.095 µs |   12.733 µs |
| memx_memmem             |    0.989 µs |    1.545 µs |    0.937 µs |    1.539 µs |
| memx_memmem_basic       |    2.109 µs |    3.787 µs |    2.105 µs |    3.796 µs |
| memx_memmem_sse2        |    1.185 µs |    1.970 µs |    1.186 µs |    2.002 µs |
| memx_memmem_avx2        |    0.933 µs |    1.481 µs |    0.881 µs |    1.489 µs |

  1. x86_64-unknown-linux- @Q6600:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memmem              |   24.819 µs |   26.406 µs |   24.182 µs |   25.942 µs |
| libc_memmem             |    4.580 µs |   10.722 µs |    8.950 µs |   14.891 µs |
| memchr_memmem           |   26.248 µs |   29.273 µs |   26.216 µs |   30.181 µs |
| memx_memmem             |    2.141 µs |    4.075 µs |    2.100 µs |    3.955 µs |
| memx_memmem_basic       |    4.019 µs |    9.095 µs |    3.816 µs |    9.093 µs |
| memx_memmem_sse2        |    1.996 µs |    3.961 µs |    1.973 µs |    3.832 µs |

  2. i686-unknown-linux- @Q6600:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memmem              |   34.311 µs |   35.968 µs |   32.926 µs |   35.390 µs |
| libc_memmem             |    6.063 µs |   11.411 µs |   10.979 µs |   15.209 µs |
| memchr_memmem           |   34.732 µs |   60.941 µs |   33.805 µs |   63.770 µs |
| memx_memmem             |    2.338 µs |    3.910 µs |    2.308 µs |    3.908 µs |
| memx_memmem_basic       |    6.311 µs |   12.821 µs |    6.382 µs |   12.908 µs |
| memx_memmem_sse2        |    2.139 µs |    3.762 µs |    2.234 µs |    3.845 µs |

  3. i586-unknown-linux- @Q6600:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memmem              |   33.955 µs |   39.174 µs |   34.093 µs |   44.297 µs |
| libc_memmem             |    5.463 µs |   11.189 µs |   10.689 µs |   17.141 µs |
| memchr_memmem           |   33.321 µs |   63.579 µs |   36.500 µs |   69.005 µs |
| memx_memmem             |    2.402 µs |    4.026 µs |    2.334 µs |    4.049 µs |
| memx_memmem_basic       |    6.363 µs |   12.722 µs |    6.287 µs |   13.018 µs |


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
