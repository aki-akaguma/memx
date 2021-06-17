## Benchmark results

  1. x86_64-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memcpy              |   97.953 us |  257.430 us |  452.130 us |  430.670 us |
| libc_memcpy             |   96.029 us |  245.490 us |  446.250 us |  424.720 us |
| memx_memcpy             |  125.950 us |  289.210 us |  173.370 us |  262.010 us |
| memx_memcpy_basic       |  114.030 us |  338.960 us |  161.020 us |  330.740 us |
| memx_memcpy_sse2        |  122.380 us |  256.230 us |  169.510 us |  244.860 us |

  2. i686-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memcpy              |  212.130 us |  272.340 us |  275.770 us |  355.950 us |
| libc_memcpy             |  165.260 us |  264.730 us |  279.140 us |  355.760 us |
| memx_memcpy             |  263.220 us |  339.170 us |  304.360 us |  355.060 us |
| memx_memcpy_basic       |  214.240 us |  351.310 us |  224.680 us |  375.030 us |
| memx_memcpy_sse2        |  226.240 us |  292.010 us |  222.450 us |  289.260 us |

  3. i586-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memcpy              |  135.420 us |  237.650 us |  270.370 us |  322.330 us |
| libc_memcpy             |  123.750 us |  221.080 us |  278.640 us |  324.410 us |
| memx_memcpy             |  163.430 us |  288.740 us |  163.400 us |  310.360 us |
| memx_memcpy_basic       |  163.770 us |  291.970 us |  163.530 us |  309.130 us |

  4. armv7-linux-androideabi:

|         `name`          |  `andr:en`  |  `andr:ja`  |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memcpy              | 1071.000 us | 2871.600 us |  985.590 us | 1400.000 us |
| libc_memcpy             | 1042.100 us | 2949.400 us | 1058.600 us | 1550.600 us |
| memx_memcpy             |  601.610 us | 1268.700 us |  402.380 us | 1148.700 us |
| memx_memcpy_basic       |  525.990 us | 1086.900 us |  336.710 us |  835.900 us |


- compile by rustc 1.52.1 (9bc8c42bb 2021-05-09)
- `us` is micro seconds
- `:en` is english haystack.
- `:ja` is japanese haystack.
- `musl` is x86_64-unknown-linux-musl
- bench on intel Q6600 @ 2.40GHz or armv7 1.5GHz
