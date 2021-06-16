## Benchmark results

  1. x86_64-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memcpy              |  145.820 us |  273.960 us |  453.490 us |  443.580 us |
| libc_memcpy             |  140.720 us |  268.470 us |  453.900 us |  433.390 us |
| memx_memcpy             |  186.760 us |  282.520 us |  185.570 us |  287.860 us |
| memx_memcpy_basic       |  174.260 us |  338.830 us |  173.130 us |  337.150 us |
| memx_memcpy_sse2        |  181.280 us |  269.290 us |  181.720 us |  264.560 us |

  2. i686-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memcpy              |  202.610 us |  282.070 us |  271.270 us |  362.360 us |
| libc_memcpy             |  165.260 us |  275.070 us |  275.760 us |  352.220 us |
| memx_memcpy             |  264.650 us |  319.310 us |  241.200 us |  355.510 us |
| memx_memcpy_basic       |  213.510 us |  334.820 us |  169.520 us |  361.720 us |
| memx_memcpy_sse2        |  227.370 us |  274.080 us |  160.830 us |  289.830 us |

  3. i586-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memcpy              |  135.370 us |  244.600 us |  278.310 us |  325.310 us |
| libc_memcpy             |  123.640 us |  239.190 us |  278.190 us |  329.890 us |
| memx_memcpy             |  169.680 us |  358.780 us |  166.260 us |  369.920 us |
| memx_memcpy_basic       |  171.780 us |  358.330 us |  165.460 us |  364.710 us |

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
