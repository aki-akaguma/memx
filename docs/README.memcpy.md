## Benchmark results

  1. x86_64-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memcpy              |  130.080 us |  236.550 us |  429.860 us |  409.530 us |
| memx_memcpy             |  159.490 us |  230.870 us |  168.480 us |  182.720 us |
| memx_memcpy_basic       |  162.870 us |  419.290 us |  157.150 us |  372.260 us |
| memx_memcpy_libc        |  152.200 us |  299.160 us |  465.240 us |  442.500 us |

  2. i686-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memcpy              |  116.130 us |  223.410 us |  262.330 us |  313.730 us |
| memx_memcpy             |  228.910 us |  330.930 us |  229.020 us |  270.320 us |
| memx_memcpy_basic       |  159.610 us |  350.880 us |  171.390 us |  313.410 us |
| memx_memcpy_libc        |  165.780 us |  268.730 us |  333.670 us |  384.030 us |

  3. i586-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memcpy              |  116.080 us |  204.530 us |  260.790 us |  315.600 us |
| memx_memcpy             |  159.940 us |  362.310 us |  168.230 us |  365.090 us |
| memx_memcpy_basic       |  159.210 us |  362.640 us |  168.210 us |  360.810 us |
| memx_memcpy_libc        |  172.230 us |  240.580 us |  320.460 us |  367.400 us |

  4. armv7-linux-androideabi:

|         `name`          |  `andr:en`  |  `andr:ja`  |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memcpy              | 2248.000 us | 3332.300 us | 1176.000 us | 1340.500 us |
| memx_memcpy             |  481.950 us | 1110.100 us |  306.910 us |  802.230 us |
| memx_memcpy_basic       |  339.900 us |  833.760 us |  356.480 us | 1065.700 us |
| memx_memcpy_libc        | 1132.100 us | 3350.100 us | 1039.600 us | 1472.300 us |


- compile by rustc 1.52.1 (9bc8c42bb 2021-05-09)
- `us` is micro seconds
- `:en` is english haystack.
- `:ja` is japanese haystack.
- `musl` is x86_64-unknown-linux-musl
- bench on intel Q6600 @ 2.40GHz or armv7 1.5GHz
