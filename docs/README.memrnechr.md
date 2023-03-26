## Benchmark results

- compile by rustc 1.68.1 (8460ca823 2023-03-20)

  0. x86_64-unknown-linux- @Broadwell:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memrnechr           |   19.722 µs |   31.909 µs |   17.388 µs |   30.811 µs |
| memx_memrnechr          |    7.955 µs |    9.034 µs |    7.814 µs |    8.814 µs |
| memx_memrnechr_basic    |    8.880 µs |   11.323 µs |    8.832 µs |   11.203 µs |
| memx_memrnechr_sse2     |    6.986 µs |    8.461 µs |    6.970 µs |    8.277 µs |
| memx_memnechr_avx2      |    7.377 µs |    8.251 µs |    7.329 µs |    8.134 µs |

  1. x86_64-unknown-linux- @Q6600:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memrnechr           |   29.321 µs |   48.838 µs |   29.398 µs |   48.940 µs |
| memx_memrnechr          |   17.029 µs |   19.032 µs |   16.739 µs |   18.545 µs |
| memx_memrnechr_basic    |   14.354 µs |   18.536 µs |   14.231 µs |   18.462 µs |
| memx_memrnechr_sse2     |   13.630 µs |   15.530 µs |   13.248 µs |   15.304 µs |

  2. i686-unknown-linux- @Q6600:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memrnechr           |   28.877 µs |   45.556 µs |   29.262 µs |   45.900 µs |
| memx_memrnechr          |   21.063 µs |   24.443 µs |   21.016 µs |   25.968 µs |
| memx_memrnechr_basic    |   14.901 µs |   21.645 µs |   14.822 µs |   22.526 µs |
| memx_memrnechr_sse2     |   16.765 µs |   17.847 µs |   17.431 µs |   18.380 µs |

  3. i586-unknown-linux- @Q6600:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memrnechr           |   28.164 µs |   44.918 µs |   28.689 µs |   45.156 µs |
| memx_memrnechr          |   20.351 µs |   22.878 µs |   20.755 µs |   23.106 µs |
| memx_memrnechr_basic    |   14.883 µs |   21.784 µs |   14.809 µs |   22.143 µs |


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

- `µs` is micro seconds
- `:en` is english haystack or short length.
- `:ja` is japanese haystack or long length.
- `gnu` is x86_64-unknown-linux-gnu
- `musl` is x86_64-unknown-linux-musl
- bench on intel Q6600 @ 2.40GHz or armv7 1.5GHz
