## Benchmark results

  1. x86_64-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memcpy              |  247.010 us |  312.060 us |  628.810 us | 1128.100 us |
| memx_memcpy             |  334.780 us |  469.310 us |  329.230 us |  472.580 us |
| memx_memcpy_basic       |  334.620 us |  469.520 us |  328.920 us |  472.950 us |
| memx_memcpy_libc        |  278.520 us |  391.430 us |  693.380 us | 1243.900 us |

  2. i686-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memcpy              |  352.700 us |  570.680 us |  415.600 us |  743.220 us |
| memx_memcpy             |  509.690 us |  858.110 us |  374.570 us |  810.130 us |
| memx_memcpy_basic       |  510.450 us |  856.320 us |  374.760 us |  810.960 us |
| memx_memcpy_libc        |  389.880 us |  743.440 us |  497.350 us |  898.140 us |

  3. i586-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memcpy              |  230.530 us |  574.980 us |  416.280 us |  741.060 us |
| memx_memcpy             |  359.440 us |  801.210 us |  373.280 us |  659.100 us |
| memx_memcpy_basic       |  359.510 us |  802.900 us |  373.340 us |  661.020 us |
| memx_memcpy_libc        |  325.180 us |  730.420 us |  490.890 us |  887.450 us |

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
