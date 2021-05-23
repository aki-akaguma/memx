## Benchmark results

  1. x86_64-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memcmp              |  157.450 us |  166.340 us |  250.680 us |  342.930 us |
| memx_memcmp             |  128.740 us |  127.730 us |  127.670 us |  126.380 us |
| memx_memcmp_basic       |  128.390 us |  127.770 us |  126.970 us |  126.390 us |
| memx_memcmp_libc        |  213.270 us |  205.330 us |  315.380 us |  376.260 us |

  2. i686-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memcmp              |  180.930 us |  261.080 us |  337.890 us |  476.190 us |
| memx_memcmp             |  184.660 us |  212.390 us |  182.910 us |  211.960 us |
| memx_memcmp_basic       |  183.920 us |  211.370 us |  182.000 us |  211.570 us |
| memx_memcmp_libc        |  240.580 us |  254.490 us |  384.130 us |  552.220 us |

  3. i586-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memcmp              |  162.920 us |  201.330 us |  336.400 us |  477.480 us |
| memx_memcmp             |  177.560 us |  205.840 us |  182.490 us |  208.060 us |
| memx_memcmp_basic       |  176.580 us |  205.850 us |  179.620 us |  202.480 us |
| memx_memcmp_libc        |  236.070 us |  258.070 us |  396.420 us |  545.100 us |

  4. armv7-linux-androideabi:

|         `name`          |  `andr:en`  |  `andr:ja`  |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memcmp              |  966.760 us |  914.480 us | 1039.400 us | 1451.700 us |
| memx_memcmp             |  498.820 us |  548.790 us |  512.230 us |  537.870 us |
| memx_memcmp_basic       |  492.300 us |  543.620 us |  505.660 us |  525.700 us |
| memx_memcmp_libc        | 1199.700 us | 1139.700 us | 1115.500 us | 1526.500 us |


- compile by rustc 1.52.1 (9bc8c42bb 2021-05-09)
- `us` is micro seconds
- `:en` is english haystack.
- `:ja` is japanese haystack.
- `musl` is x86_64-unknown-linux-musl
- bench on intel Q6600 @ 2.40GHz or armv7 1.5GHz
