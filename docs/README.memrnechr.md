## Benchmark results

- compile by rustc 1.68.0 (2c8cc3432 2023-03-06)

  0. x86_64-unknown-linux- @Broadwell:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memrnechr           |   16.172 µs |   33.397 µs |   16.637 µs |   30.543 µs |
| memx_memrnechr          |    7.997 µs |    9.255 µs |    7.829 µs |    8.846 µs |
| memx_memrnechr_basic    |    8.840 µs |   11.472 µs |    8.828 µs |   11.264 µs |
| memx_memrnechr_sse2     |    7.011 µs |    8.537 µs |    7.010 µs |    8.444 µs |
| memx_memnechr_avx2      |    7.262 µs |    8.299 µs |    7.223 µs |    8.179 µs |

  1. x86_64-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memrnechr           |   29.922 µs |   49.263 µs |   30.040 µs |   49.266 µs |
| memx_memrnechr          |   16.922 µs |   19.331 µs |   16.996 µs |   19.410 µs |
| memx_memrnechr_basic    |   14.440 µs |   18.559 µs |   14.492 µs |   18.403 µs |
| memx_memrnechr_sse2     |   13.927 µs |   15.123 µs |   13.764 µs |   15.030 µs |

  2. i686-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memrnechr           |   29.130 µs |   44.974 µs |   30.507 µs |   45.754 µs |
| memx_memrnechr          |   19.884 µs |   24.617 µs |   20.525 µs |   25.520 µs |
| memx_memrnechr_basic    |   15.047 µs |   21.764 µs |   14.562 µs |   21.703 µs |
| memx_memrnechr_sse2     |   17.229 µs |   18.638 µs |   17.175 µs |   18.423 µs |

  3. i586-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memrnechr           |   28.612 µs |   44.719 µs |   28.319 µs |   44.679 µs |
| memx_memrnechr          |   14.655 µs |   21.409 µs |   14.806 µs |   22.122 µs |
| memx_memrnechr_basic    |   14.690 µs |   21.991 µs |   14.820 µs |   22.226 µs |


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
