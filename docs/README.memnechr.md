## Benchmark results

- compile by rustc 1.67.1 (d5a82bbd2 2023-02-07)

  1. x86_64-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memnechr            |  378.440 µs |  211.210 µs |  365.980 µs |  210.190 µs |
| memx_memnechr           |  314.280 µs |  142.280 µs |  318.370 µs |  144.450 µs |
| memx_memnechr_basic     |  310.920 µs |  158.340 µs |  309.140 µs |  160.340 µs |
| memx_memnechr_sse2      |  293.470 µs |  132.410 µs |  290.600 µs |  134.570 µs |

  2. i686-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memnechr            |  391.570 µs |  226.720 µs |  395.090 µs |  227.630 µs |
| memx_memnechr           |  345.170 µs |  151.110 µs |  346.990 µs |  151.250 µs |
| memx_memnechr_basic     |  306.280 µs |  159.940 µs |  312.620 µs |  161.460 µs |
| memx_memnechr_sse2      |  329.020 µs |  148.660 µs |  347.200 µs |  149.970 µs |

  3. i586-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memnechr            |  346.090 µs |  214.800 µs |  353.270 µs |  223.260 µs |
| memx_memnechr           |  264.710 µs |  156.940 µs |  286.920 µs |  171.480 µs |
| memx_memnechr_basic     |  264.550 µs |  156.930 µs |  286.720 µs |  172.290 µs |


- compile by rustc 1.53.0 (53cb7b09b 2021-06-17)

  1. x86_64-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memnechr            |  368.560 us |  212.620 us |  357.730 us |  214.030 us |
| memx_memnechr           |  273.090 us |  126.310 us |  264.900 us |  127.690 us |
| memx_memnechr_basic     |  305.920 us |  162.800 us |  304.140 us |  164.920 us |
| memx_memnechr_sse2      |  267.470 us |  125.320 us |  261.060 us |  127.270 us |

  2. i686-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memnechr            |  431.530 us |  235.700 us |  496.540 us |  247.550 us |
| memx_memnechr           |  484.690 us |  199.290 us |  539.420 us |  219.530 us |
| memx_memnechr_basic     |  447.280 us |  206.660 us |  555.910 us |  221.390 us |
| memx_memnechr_sse2      |  430.740 us |  180.280 us |  452.040 us |  190.260 us |

  3. i586-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memnechr            |  388.950 us |  240.800 us |  387.410 us |  237.050 us |
| memx_memnechr           |  354.960 us |  191.770 us |  370.900 us |  195.360 us |
| memx_memnechr_basic     |  357.170 us |  195.810 us |  362.970 us |  189.400 us |

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
