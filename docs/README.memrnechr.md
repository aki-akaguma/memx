## Benchmark results

- compile by rustc 1.68.0 (2c8cc3432 2023-03-06)

  0. x86_64-unknown-linux- @Broadwell:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memrnechr           |  230.580 µs |  139.600 µs |  237.180 µs |  146.740 µs |
| memx_memrnechr          |  190.150 µs |   97.914 µs |  203.450 µs |  115.600 µs |
| memx_memrnechr_basic    |  190.280 µs |  109.150 µs |  200.170 µs |  107.390 µs |
| memx_memrnechr_sse2     |  186.830 µs |   97.672 µs |  190.150 µs |  104.340 µs |

  1. x86_64-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memrnechr           |  424.070 µs |  255.100 µs |  427.110 µs |  264.710 µs |
| memx_memrnechr          |  331.310 µs |  156.300 µs |  328.640 µs |  156.880 µs |
| memx_memrnechr_basic    |  320.220 µs |  174.140 µs |  318.250 µs |  173.890 µs |
| memx_memrnechr_sse2     |  321.140 µs |  149.880 µs |  319.930 µs |  150.610 µs |

  2. i686-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memrnechr           |  399.950 µs |  232.820 µs |  402.920 µs |  234.990 µs |
| memx_memrnechr          |  337.730 µs |  155.790 µs |  363.950 µs |  164.780 µs |
| memx_memrnechr_basic    |  317.000 µs |  173.950 µs |  313.180 µs |  171.780 µs |
| memx_memrnechr_sse2     |  325.160 µs |  148.870 µs |  332.720 µs |  153.710 µs |

  3. i586-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memrnechr           |  386.400 µs |  253.630 µs |  376.420 µs |  239.300 µs |
| memx_memrnechr          |  259.500 µs |  167.910 µs |  275.830 µs |  173.590 µs |
| memx_memrnechr_basic    |  258.510 µs |  167.150 µs |  276.190 µs |  174.630 µs |


- compile by rustc 1.53.0 (53cb7b09b 2021-06-17)

  1. x86_64-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memrnechr           |  399.760 us |  245.040 us |  400.870 us |  249.640 us |
| memx_memrnechr          |  312.040 us |  145.270 us |  302.340 us |  145.330 us |
| memx_memrnechr_basic    |  326.360 us |  175.180 us |  322.870 us |  176.190 us |
| memx_memrnechr_sse2     |  298.880 us |  144.610 us |  294.750 us |  144.580 us |

  2. i686-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memrnechr           |  450.060 us |  250.070 us |  503.380 us |  261.390 us |
| memx_memrnechr          |  473.720 us |  215.490 us |  552.980 us |  220.340 us |
| memx_memrnechr_basic    |  445.230 us |  218.590 us |  490.650 us |  221.440 us |
| memx_memrnechr_sse2     |  431.460 us |  185.510 us |  474.330 us |  199.520 us |

  3. i586-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memrnechr           |  376.170 us |  231.200 us |  368.450 us |  237.870 us |
| memx_memrnechr          |  337.620 us |  196.370 us |  348.710 us |  196.230 us |
| memx_memrnechr_basic    |  338.860 us |  195.930 us |  347.750 us |  195.600 us |

  4. armv7-linux-androideabi:

|         `name`          |  `andr:en`  |  `andr:ja`  |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memchr              | 1232.100 us |  950.750 us | 1262.200 us | 1047.500 us |
| libc_memchr             | 1874.100 us | 1066.300 us | 1865.200 us |  838.210 us |
| memchr_memchr           | 1391.400 us |  642.260 us | 1441.300 us |  649.600 us |
| memx_memchr             | 1042.000 us |  555.030 us | 1129.400 us |  519.740 us |
| memx_memchr_basic       | 1005.600 us |  573.360 us | 1095.400 us |  525.020 us |

- `us` is micro seconds
- `:en` is english haystack.
- `:ja` is japanese haystack.
- `musl` is x86_64-unknown-linux-musl
- bench on intel Q6600 @ 2.40GHz or armv7 1.5GHz
