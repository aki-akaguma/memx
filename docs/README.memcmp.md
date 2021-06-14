## Benchmark results

  1. x86_64-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memcmp              |  157.960 us |  167.430 us |  335.020 us |  350.780 us |
| libc_memcmp             |  176.670 us |  179.070 us |  338.900 us |  356.950 us |
| memx_memcmp             |  134.170 us |  133.800 us |  132.810 us |  132.290 us |
| memx_memcmp_basic       |  135.010 us |  133.710 us |  132.460 us |  132.340 us |
| memx_memcmp_sse2        |  133.990 us |  172.160 us |  132.040 us |  170.570 us |

  2. i686-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memcmp              |  192.510 us |  201.060 us |  327.400 us |  479.060 us |
| libc_memcmp             |  182.590 us |  213.710 us |  302.210 us |  485.930 us |
| memx_memcmp             |  192.280 us |  219.670 us |  178.640 us |  212.710 us |
| memx_memcmp_basic       |  187.780 us |  212.980 us |  180.980 us |  215.940 us |
| memx_memcmp_sse2        |  169.920 us |  206.950 us |  210.300 us |  221.380 us |

  3. i586-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memcmp              |  164.420 us |  204.190 us |  329.840 us |  476.200 us |
| libc_memcmp             |  182.860 us |  206.390 us |  330.710 us |  485.520 us |
| memx_memcmp             |  191.730 us |  213.620 us |  180.950 us |  208.590 us |
| memx_memcmp_basic       |  190.560 us |  214.720 us |  178.870 us |  209.070 us |

  4. armv7-linux-androideabi:

|         `name`          |  `andr:en`  |  `andr:ja`  |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memcmp              | 1401.200 us | 1113.700 us | 1275.200 us | 1516.500 us |
| libc_memcmp             | 1206.400 us | 1172.900 us | 1171.000 us | 1543.700 us |
| memx_memcmp             |  625.920 us |  676.380 us |  785.140 us |  683.550 us |
| memx_memcmp_basic       |  611.750 us |  706.160 us |  653.590 us |  666.110 us |


- compile by rustc 1.52.1 (9bc8c42bb 2021-05-09)
- `us` is micro seconds
- `:en` is english haystack.
- `:ja` is japanese haystack.
- `musl` is x86_64-unknown-linux-musl
- bench on intel Q6600 @ 2.40GHz or armv7 1.5GHz
