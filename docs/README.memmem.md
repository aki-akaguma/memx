## Benchmark results

- compile by rustc 1.69.0 (84c898d65 2023-04-16)

  0. x86_64-unknown-linux- @i5-4570:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memmem              |    8.445 µs |    7.395 µs |    8.519 µs |    7.522 µs |
| libc_memmem             |    2.173 µs |    3.828 µs |    3.948 µs |    7.010 µs |
| memchr_memmem           |    9.127 µs |    8.645 µs |    9.178 µs |    8.726 µs |
| memx_memmem             |    0.798 µs |    1.230 µs |    0.794 µs |    1.197 µs |
| memx_memmem_basic       |    1.900 µs |    3.362 µs |    1.901 µs |    3.359 µs |
| memx_memmem_sse2        |    0.975 µs |    1.585 µs |    0.997 µs |    1.600 µs |
| memx_memmem_avx2        |    0.803 µs |    1.232 µs |    0.777 µs |    1.187 µs |

  1. x86_64-unknown-linux- @Q6600:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memmem              |   25.139 µs |   27.011 µs |   25.313 µs |   27.119 µs |
| libc_memmem             |    4.576 µs |   10.765 µs |    9.002 µs |   15.514 µs |
| memchr_memmem           |   26.446 µs |   30.280 µs |   26.539 µs |   30.365 µs |
| memx_memmem             |    2.464 µs |    4.056 µs |    2.437 µs |    3.908 µs |
| memx_memmem_basic       |    4.665 µs |    8.825 µs |    4.715 µs |    8.799 µs |
| memx_memmem_sse2        |    2.455 µs |    4.029 µs |    2.426 µs |    3.885 µs |

  2. i686-unknown-linux- @Q6600:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memmem              |   33.883 µs |   36.170 µs |   31.517 µs |   32.983 µs |
| libc_memmem             |    5.463 µs |   10.972 µs |   11.085 µs |   15.675 µs |
| memchr_memmem           |   34.811 µs |   61.324 µs |   33.721 µs |   61.819 µs |
| memx_memmem             |    2.674 µs |    3.890 µs |    2.664 µs |    4.156 µs |
| memx_memmem_basic       |    9.123 µs |   16.687 µs |    9.413 µs |   17.235 µs |
| memx_memmem_sse2        |    2.656 µs |    3.868 µs |    2.583 µs |    4.143 µs |

  3. i586-unknown-linux- @Q6600:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memmem              |   34.484 µs |   47.667 µs |   33.514 µs |   38.781 µs |
| libc_memmem             |    8.836 µs |   11.033 µs |   10.882 µs |   17.589 µs |
| memchr_memmem           |   33.475 µs |   63.777 µs |   36.294 µs |   70.701 µs |
| memx_memmem             |    2.856 µs |    3.955 µs |    2.762 µs |    4.188 µs |
| memx_memmem_basic       |    9.485 µs |   17.256 µs |    9.687 µs |   18.002 µs |


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
