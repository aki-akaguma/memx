## Benchmark results

- compile by rustc 1.68.0 (2c8cc3432 2023-03-06)

  0. x86_64-unknown-linux- @Broadwell:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memmem              |  278.030 µs |  208.700 µs |  271.660 µs |  202.990 µs |
| libc_memmem             |  105.090 µs |   95.090 µs |  157.460 µs |  165.850 µs |
| memchr_memmem           |   91.249 µs |   88.535 µs |   94.208 µs |   95.364 µs |
| memx_memmem             |   49.344 µs |   49.042 µs |   50.024 µs |   55.130 µs |
| memx_memmem_basic       |   54.149 µs |   51.810 µs |   57.750 µs |   61.667 µs |
| memx_memmem_sse2        |   48.481 µs |   51.661 µs |   50.996 µs |   57.258 µs |

  1. x86_64-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memmem              |  471.990 µs |  437.020 µs |  468.750 µs |  432.500 µs |
| libc_memmem             |  125.490 µs |  116.130 µs |  221.430 µs |  267.870 µs |
| memchr_memmem           |  157.740 µs |  174.280 µs |  161.380 µs |  177.930 µs |
| memx_memmem             |   97.555 µs |   97.045 µs |   97.257 µs |   96.883 µs |
| memx_memmem_basic       |   98.635 µs |   99.485 µs |   96.792 µs |   99.694 µs |
| memx_memmem_sse2        |   97.512 µs |   97.141 µs |   97.275 µs |   96.935 µs |

  2. i686-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memmem              |  650.190 µs |  547.930 µs |  711.140 µs |  524.760 µs |
| libc_memmem             |  171.640 µs |  121.360 µs |  237.370 µs |  246.230 µs |
| memchr_memmem           |  436.360 µs |  434.400 µs |  431.230 µs |  430.750 µs |
| memx_memmem             |  107.950 µs |  106.680 µs |  110.800 µs |  112.130 µs |
| memx_memmem_basic       |  121.560 µs |  123.300 µs |  128.720 µs |  125.170 µs |
| memx_memmem_sse2        |  107.850 µs |  106.720 µs |  110.710 µs |  112.240 µs |

  3. i586-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memmem              |  774.790 µs |  590.860 µs |  719.420 µs |  578.450 µs |
| libc_memmem             |  131.180 µs |  120.850 µs |  229.420 µs |  248.290 µs |
| memchr_memmem           |  447.310 µs |  494.390 µs |  502.910 µs |  541.760 µs |
| memx_memmem             |  128.130 µs |  132.340 µs |  127.420 µs |  126.980 µs |
| memx_memmem_basic       |  128.110 µs |  132.060 µs |  127.370 µs |  127.020 µs |


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


- `us` is micro seconds
- `:en` is english haystack.
- `:ja` is japanese haystack.
- `musl` is x86_64-unknown-linux-musl
- bench on intel Q6600 @ 2.40GHz or armv7 1.5GHz
