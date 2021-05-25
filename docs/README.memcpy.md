## Benchmark results

  1. x86_64-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memcpy              |  129.890 us |  236.980 us |  430.260 us |  408.790 us |
| memx_memcpy             |  155.740 us |  376.160 us |  159.840 us |  327.690 us |
| memx_memcpy_basic       |  163.000 us |  416.210 us |  156.710 us |  370.890 us |
| memx_memcpy_libc        |  151.950 us |  300.930 us |  464.110 us |  441.460 us |

  2. i686-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memcpy              |  119.070 us |  260.440 us |  263.530 us |  314.640 us |
| memx_memcpy             |  148.850 us |  422.030 us |  162.200 us |  376.220 us |
| memx_memcpy_basic       |  163.250 us |  345.860 us |  165.430 us |  299.340 us |
| memx_memcpy_libc        |  170.890 us |  272.880 us |  312.720 us |  362.280 us |

  3. i586-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memcpy              |  116.450 us |  202.510 us |  260.080 us |  315.710 us |
| memx_memcpy             |  162.100 us |  351.880 us |  162.860 us |  361.650 us |
| memx_memcpy_basic       |  161.740 us |  352.140 us |  162.820 us |  363.630 us |
| memx_memcpy_libc        |  166.510 us |  234.560 us |  321.070 us |  367.840 us |

  4. armv7-linux-androideabi:

|         `name`          |  `andr:en`  |  `andr:ja`  |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memcpy              | 3101.600 us | 3957.600 us | 1427.000 us | 2278.100 us |
| memx_memcpy             | 1058.100 us | 1634.200 us |  854.870 us | 1491.200 us |
| memx_memcpy_basic       |  829.260 us | 1523.000 us |  961.300 us | 1630.600 us |
| memx_memcpy_libc        | 2298.400 us | 4107.700 us | 1561.700 us | 2536.300 us |


- compile by rustc 1.52.1 (9bc8c42bb 2021-05-09)
- `us` is micro seconds
- `:en` is english haystack.
- `:ja` is japanese haystack.
- `musl` is x86_64-unknown-linux-musl
- bench on intel Q6600 @ 2.40GHz or armv7 1.5GHz
