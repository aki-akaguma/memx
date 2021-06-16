## Benchmark results

  1. x86_64-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memrmem             |  671.070 us |  885.280 us |  680.500 us |  889.690 us |
| libc_memrmem            | 1755.700 us | 1832.300 us |  788.300 us |  868.110 us |
| memchr_memrmem          |  791.940 us |  805.810 us |  802.150 us |  802.870 us |
| memx_memrmem            |  287.300 us |  287.340 us |  289.780 us |  289.980 us |
| memx_memrmem_basic      |  277.600 us |  278.590 us |  282.610 us |  286.810 us |
| memx_memrmem_sse2       |  291.240 us |  286.860 us |  296.580 us |  288.740 us |

  2. i686-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memrmem             |  907.570 us | 1049.200 us |  804.720 us | 1038.300 us |
| memchr_memrmem          |  972.120 us |  975.760 us |  995.980 us |  970.510 us |
| memx_memrmem            |  164.970 us |  166.970 us |  177.630 us |  176.850 us |
| memx_memrmem_basic      |  135.250 us |  127.360 us |  139.570 us |  136.090 us |
| memx_memrmem_sse2       |  160.110 us |  163.300 us |  168.300 us |  166.920 us |

  3. i586-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memrmem             |  907.680 us |  999.080 us |  956.720 us |  968.020 us |
| memchr_memrmem          | 1076.300 us | 1627.100 us | 1065.700 us | 1610.100 us |
| memx_memrmem            |  146.340 us |  132.110 us |  144.960 us |  141.750 us |
| memx_memrmem_basic      |  146.830 us |  130.710 us |  144.430 us |  140.740 us |

  4. armv7-linux-androideabi:

|         `name`          |  `andr:en`  |  `andr:ja`  |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memrmem             | 4581.200 us | 2551.100 us | 4267.100 us | 2270.600 us |
| memchr_memrmem          | 3847.200 us | 3633.100 us | 3950.200 us | 3783.300 us |
| memx_memrmem            |  542.720 us |  558.190 us |  544.900 us |  512.220 us |
| memx_memrmem_basic      |  543.160 us |  541.400 us |  548.790 us |  514.280 us |


- compile by rustc 1.52.1 (9bc8c42bb 2021-05-09)
- `us` is micro seconds
- `:en` is english haystack.
- `:ja` is japanese haystack.
- `musl` is x86_64-unknown-linux-musl
- bench on intel Q6600 @ 2.40GHz or armv7 1.5GHz
