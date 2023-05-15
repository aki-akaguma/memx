## Benchmark results

- compile by rustc 1.69.0 (84c898d65 2023-04-16)

  0. x86_64-unknown-linux- @i5-4570:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memrnechr           |   11.176 µs |   21.297 µs |   11.475 µs |   22.406 µs |
| memx_memrnechr          |    5.391 µs |    6.047 µs |    5.324 µs |    6.017 µs |
| memx_memrnechr_basic    |    7.300 µs |    9.291 µs |    7.298 µs |    9.207 µs |
| memx_memrnechr_sse2     |    5.452 µs |    6.551 µs |    5.487 µs |    6.415 µs |
| memx_memnechr_avx2      |    5.392 µs |    6.035 µs |    5.332 µs |    6.035 µs |

  1. x86_64-unknown-linux- @Q6600:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memrnechr           |   30.069 µs |   49.447 µs |   30.156 µs |   49.418 µs |
| memx_memrnechr          |   14.395 µs |   17.757 µs |   15.087 µs |   17.696 µs |
| memx_memrnechr_basic    |   15.724 µs |   21.179 µs |   15.703 µs |   21.155 µs |
| memx_memrnechr_sse2     |   12.121 µs |   14.517 µs |   12.350 µs |   13.881 µs |

  2. i686-unknown-linux- @Q6600:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memrnechr           |   28.474 µs |   45.121 µs |   28.309 µs |   45.172 µs |
| memx_memrnechr          |   20.734 µs |   24.071 µs |   21.683 µs |   23.816 µs |
| memx_memrnechr_basic    |   21.401 µs |   33.238 µs |   19.872 µs |   31.356 µs |
| memx_memrnechr_sse2     |   17.130 µs |   18.543 µs |   18.597 µs |   20.931 µs |

  3. i586-unknown-linux- @Q6600:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memrnechr           |   28.098 µs |   44.993 µs |   28.675 µs |   45.074 µs |
| memx_memrnechr          |   21.418 µs |   25.527 µs |   22.279 µs |   24.495 µs |
| memx_memrnechr_basic    |   19.648 µs |   31.352 µs |   20.093 µs |   31.326 µs |


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
- @i5-4570: bench on intel i5-4570 @ 3.2GHz
- @Q6600: bench on intel Q6600 @ 2.40GHz or armv7 1.5GHz
