## Benchmark results

- compile by rustc 1.69.0 (84c898d65 2023-04-16)

  0. x86_64-unknown-linux- @i5-4570:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memmem              |    8.516 µs |    7.501 µs |    8.529 µs |    7.438 µs |
| libc_memmem             |    2.165 µs |    3.813 µs |    3.977 µs |    6.931 µs |
| memchr_memmem           |    9.057 µs |    8.725 µs |    8.972 µs |    8.848 µs |
| memx_memmem             |    0.794 µs |    1.236 µs |    0.801 µs |    1.186 µs |
| memx_memmem_basic       |    1.599 µs |    2.784 µs |    1.664 µs |    2.915 µs |
| memx_memmem_sse2        |    0.933 µs |    1.439 µs |    0.893 µs |    1.448 µs |
| memx_memmem_avx2        |    0.785 µs |    1.226 µs |    0.781 µs |    1.187 µs |

  1. x86_64-unknown-linux- @Q6600:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memmem              |   25.963 µs |   27.103 µs |   25.581 µs |   27.355 µs |
| libc_memmem             |    4.709 µs |   11.205 µs |   10.518 µs |   18.516 µs |
| memchr_memmem           |   27.020 µs |   30.781 µs |   26.977 µs |   31.388 µs |
| memx_memmem             |    2.402 µs |    3.825 µs |    2.397 µs |    3.676 µs |
| memx_memmem_basic       |    4.018 µs |    7.399 µs |    4.046 µs |    7.406 µs |
| memx_memmem_sse2        |    2.412 µs |    3.787 µs |    2.363 µs |    3.637 µs |

  2. i686-unknown-linux- @Q6600:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memmem              |   33.696 µs |   35.169 µs |   31.054 µs |   32.622 µs |
| libc_memmem             |    8.730 µs |   11.004 µs |   10.886 µs |   15.119 µs |
| memchr_memmem           |   34.490 µs |   60.941 µs |   34.193 µs |   61.422 µs |
| memx_memmem             |    2.595 µs |    3.854 µs |    2.601 µs |    4.010 µs |
| memx_memmem_basic       |    6.655 µs |   12.380 µs |    6.911 µs |   12.710 µs |
| memx_memmem_sse2        |    2.550 µs |    3.792 µs |    2.630 µs |    3.999 µs |

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
- @i5-4570: bench on intel i5-4570 @ 3.2GHz
- @Q6600: bench on intel Q6600 @ 2.40GHz or armv7 1.5GHz
