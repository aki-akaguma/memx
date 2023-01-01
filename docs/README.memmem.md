## Benchmark results

- compile by rustc 1.65.0 (897e37553 2022-11-02)

  1. x86_64-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memmem              |  470.150 µs |  454.060 µs |  465.820 µs |  440.740 µs |
| libc_memmem             |  125.300 µs |  115.360 µs |  218.410 µs |  278.530 µs |
| memchr_memmem           |  173.630 µs |  195.140 µs |  172.610 µs |  195.530 µs |
| memx_memmem             |  101.220 µs |   99.295 µs |   97.919 µs |   99.120 µs |
| memx_memmem_basic       |  106.150 µs |  105.100 µs |  104.910 µs |  105.420 µs |
| memx_memmem_sse2        |   98.234 µs |   98.341 µs |   97.353 µs |   98.120 µs |

  2. i686-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memmem              |  665.860 µs |  531.470 µs |  649.540 µs |  538.610 µs |
| libc_memmem             |  133.470 µs |  120.560 µs |  244.460 µs |  262.320 µs |
| memchr_memmem           |  445.670 µs |  444.670 µs |  455.130 µs |  461.570 µs |
| memx_memmem             |  120.180 µs |  121.190 µs |  127.640 µs |  127.430 µs |
| memx_memmem_basic       |  120.580 µs |  123.010 µs |  119.230 µs |  119.630 µs |
| memx_memmem_sse2        |  115.520 µs |  116.740 µs |  123.480 µs |  121.880 µs |

  3. i586-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memmem              |  658.910 µs |  617.960 µs |  706.850 µs |  593.510 µs |
| libc_memmem             |  130.870 µs |  121.310 µs |  241.470 µs |  245.340 µs |
| memchr_memmem           |  472.730 µs |  522.840 µs |  505.310 µs |  561.450 µs |
| memx_memmem             |  130.850 µs |  131.500 µs |  123.380 µs |  117.590 µs |
| memx_memmem_basic       |  130.550 µs |  131.100 µs |  118.560 µs |  117.660 µs |


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
