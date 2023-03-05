## Benchmark results

- compile by rustc 1.67.1 (d5a82bbd2 2023-02-07)

  1. x86_64-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memmem              |  472.620 µs |  445.250 µs |  470.860 µs |  442.220 µs |
| libc_memmem             |  127.540 µs |  117.850 µs |  222.690 µs |  274.860 µs |
| memchr_memmem           |  171.750 µs |  183.030 µs |  169.780 µs |  188.060 µs |
| memx_memmem             |  111.530 µs |  109.760 µs |  109.470 µs |  109.080 µs |
| memx_memmem_basic       |  110.580 µs |  108.100 µs |  110.550 µs |  110.410 µs |
| memx_memmem_sse2        |  110.070 µs |  109.800 µs |  109.700 µs |  109.130 µs |

  2. i686-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memmem              |  660.590 µs |  558.750 µs |  726.870 µs |  550.030 µs |
| libc_memmem             |  132.190 µs |  122.550 µs |  236.710 µs |  250.290 µs |
| memchr_memmem           |  448.440 µs |  440.940 µs |  441.750 µs |  449.240 µs |
| memx_memmem             |  113.810 µs |  116.390 µs |  124.320 µs |  123.260 µs |
| memx_memmem_basic       |  134.440 µs |  132.780 µs |  132.120 µs |  135.900 µs |
| memx_memmem_sse2        |  113.640 µs |  116.200 µs |  124.300 µs |  123.480 µs |

  3. i586-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memmem              |  778.110 µs |  609.280 µs |  723.090 µs |  593.360 µs |
| libc_memmem             |  132.810 µs |  122.290 µs |  235.300 µs |  256.030 µs |
| memchr_memmem           |  456.790 µs |  498.950 µs |  504.930 µs |  549.480 µs |
| memx_memmem             |  147.610 µs |  144.750 µs |  149.380 µs |  147.560 µs |
| memx_memmem_basic       |  147.700 µs |  144.630 µs |  149.680 µs |  147.680 µs |


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
