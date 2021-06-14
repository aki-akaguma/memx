## Benchmark results

  1. x86_64-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memcpy              |   87.447 us |  194.190 us |  428.600 us |  408.600 us |
| libc_memcpy             |   79.030 us |  190.210 us |  426.620 us |  405.900 us |
| memx_memcpy             |  138.240 us |  187.750 us |  151.110 us |  196.720 us |
| memx_memcpy_basic       |  117.740 us |  371.440 us |  117.290 us |  368.380 us |
| memx_memcpy_sse2        |  129.610 us |  191.230 us |  126.420 us |  196.730 us |

  2. i686-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memcpy              |  116.090 us |  207.410 us |  263.800 us |  335.080 us |
| libc_memcpy             |  115.320 us |  205.510 us |  258.200 us |  329.000 us |
| memx_memcpy             |  178.810 us |  247.960 us |  246.520 us |  335.170 us |
| memx_memcpy_basic       |  144.690 us |  270.970 us |  217.620 us |  252.560 us |
| memx_memcpy_sse2        |  139.190 us |  197.000 us |  187.940 us |  273.270 us |

  3. i586-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memcpy              |  114.420 us |  204.230 us |  260.540 us |  314.100 us |
| libc_memcpy             |  112.660 us |  200.080 us |  257.900 us |  310.810 us |
| memx_memcpy             |  150.720 us |  340.450 us |  164.230 us |  349.840 us |
| memx_memcpy_basic       |  151.440 us |  341.010 us |  164.980 us |  350.120 us |

  4. armv7-linux-androideabi:

|         `name`          |  `andr:en`  |  `andr:ja`  |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memcpy              | 1217.200 us | 2530.200 us | 1265.000 us | 1319.300 us |
| libc_memcpy             |  876.010 us | 2507.700 us |  867.750 us | 1321.600 us |
| memx_memcpy             |  379.580 us | 1006.100 us |  364.270 us | 1068.500 us |
| memx_memcpy_basic       |  445.450 us | 1213.300 us |  346.710 us | 1047.600 us |


- compile by rustc 1.52.1 (9bc8c42bb 2021-05-09)
- `us` is micro seconds
- `:en` is english haystack.
- `:ja` is japanese haystack.
- `musl` is x86_64-unknown-linux-musl
- bench on intel Q6600 @ 2.40GHz or armv7 1.5GHz
