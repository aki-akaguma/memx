## Benchmark results

- compile by rustc 1.66.0 (69f9c33d7 2022-12-12)

  1. x86_64-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memrnechr           |  428.330 µs |  257.680 µs |  419.190 µs |  250.830 µs |
| memx_memrnechr          |  331.020 µs |  155.300 µs |  326.310 µs |  155.960 µs |
| memx_memrnechr_basic    |  323.300 µs |  173.000 µs |  320.020 µs |  174.470 µs |
| memx_memrnechr_sse2     |  318.960 µs |  148.870 µs |  316.290 µs |  149.410 µs |

  2. i686-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memrnechr           |  397.440 µs |  228.810 µs |  403.330 µs |  229.440 µs |
| memx_memrnechr          |  334.010 µs |  154.620 µs |  363.040 µs |  163.750 µs |
| memx_memrnechr_basic    |  306.350 µs |  169.230 µs |  311.060 µs |  174.180 µs |
| memx_memrnechr_sse2     |  322.170 µs |  147.510 µs |  330.710 µs |  154.040 µs |

- compile by rustc 1.65.0 (897e37553 2022-11-02)

  3. i586-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memrnechr           |  366.170 µs |  243.150 µs |  376.460 µs |  239.600 µs |
| memx_memrnechr          |  257.810 µs |  165.470 µs |  267.750 µs |  173.470 µs |
| memx_memrnechr_basic    |  256.890 µs |  166.650 µs |  268.220 µs |  173.360 µs |


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
