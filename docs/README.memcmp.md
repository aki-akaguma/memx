## Benchmark results

  1. x86_64-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memcmp              |  155.000 us |  171.210 us |  334.440 us |  351.020 us |
| libc_memcmp             |  178.500 us |  179.940 us |  336.950 us |  352.400 us |
| memx_memcmp             |  137.770 us |  134.500 us |  133.490 us |  133.010 us |
| memx_memcmp_basic       |  137.150 us |  134.330 us |  133.780 us |  133.040 us |
| memx_memcmp_sse2        |  134.920 us |  173.680 us |  133.110 us |  171.420 us |

  2. i686-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memcmp              |  163.170 us |  210.790 us |  330.010 us |  480.150 us |
| libc_memcmp             |  197.750 us |  221.010 us |  318.240 us |  488.850 us |
| memx_memcmp             |  201.490 us |  222.180 us |  198.040 us |  220.050 us |
| memx_memcmp_basic       |  194.600 us |  217.620 us |  182.950 us |  218.380 us |
| memx_memcmp_sse2        |  174.310 us |  209.020 us |  211.350 us |  216.760 us |

  3. i586-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memcmp              |  160.380 us |  255.080 us |  327.820 us |  476.370 us |
| libc_memcmp             |  182.740 us |  203.180 us |  304.110 us |  492.750 us |
| memx_memcmp             |  192.010 us |  210.150 us |  182.030 us |  217.630 us |
| memx_memcmp_basic       |  193.360 us |  209.560 us |  187.670 us |  211.180 us |

  4. armv7-linux-androideabi:

|         `name`          |  `andr:en`  |  `andr:ja`  |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memcmp              | 1120.900 us | 1214.700 us | 1381.700 us | 1793.800 us |
| libc_memcmp             | 1248.900 us | 1195.200 us | 1351.200 us | 1777.000 us |
| memx_memcmp             |  652.500 us |  671.780 us |  681.010 us |  707.360 us |
| memx_memcmp_basic       |  673.990 us |  723.850 us |  904.490 us |  679.730 us |


- compile by rustc 1.52.1 (9bc8c42bb 2021-05-09)
- `us` is micro seconds
- `:en` is english haystack.
- `:ja` is japanese haystack.
- `musl` is x86_64-unknown-linux-musl
- bench on intel Q6600 @ 2.40GHz or armv7 1.5GHz
