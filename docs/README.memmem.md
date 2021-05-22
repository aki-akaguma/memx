## Benchmark results

  1. x86_64-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memmem              |  508.460 us |  452.220 us |  504.780 us |  470.390 us |
| memx_memmem             |  118.020 us |  116.690 us |  118.140 us |  118.340 us |
| memx_memmem_basic       |  116.450 us |  116.630 us |  117.520 us |  117.300 us |
| memx_memmem_libc        |  133.890 us |  121.600 us |  221.370 us |  263.090 us |
| memchr_memmem           |  195.490 us |  210.630 us |  193.760 us |  218.470 us |

  2. i686-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memmem              |  689.060 us |  542.100 us |  721.660 us |  586.120 us |
| memx_memmem             |  136.160 us |  136.630 us |  141.650 us |  141.660 us |
| memx_memmem_basic       |  137.250 us |  137.040 us |  141.410 us |  141.540 us |
| memx_memmem_libc        |  185.060 us |  171.100 us |  242.380 us |  255.130 us |
| memchr_memmem           |  473.540 us |  472.860 us |  499.210 us |  500.520 us |

  3. i586-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memmem              |  767.680 us |  589.120 us |  718.480 us |  569.270 us |
| memx_memmem             |  140.450 us |  142.310 us |  142.670 us |  141.820 us |
| memx_memmem_basic       |  140.030 us |  142.640 us |  143.270 us |  142.960 us |
| memx_memmem_libc        |  172.350 us |  168.560 us |  241.490 us |  246.850 us |
| memchr_memmem           |  505.510 us |  552.910 us |  505.030 us |  545.200 us |

  4. armv7-linux-androideabi:

|         `name`          |  `andr:en`  |  `andr:ja`  |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memmem              | 3254.600 us | 2101.300 us | 2804.700 us | 1908.500 us |
| memx_memmem             |  669.990 us |  610.850 us |  598.950 us |  592.990 us |
| memx_memmem_basic       |  655.490 us |  632.170 us |  620.310 us |  598.610 us |
| memx_memmem_libc        | 2053.600 us | 2009.800 us | 1504.800 us | 1501.200 us |
| memchr_memmem           | 2050.200 us | 1973.700 us | 1848.600 us | 1895.600 us |


- compile by rustc 1.52.1 (9bc8c42bb 2021-05-09)
- `us` is micro seconds
- `:en` is english haystack.
- `:ja` is japanese haystack.
- `musl` is x86_64-unknown-linux-musl
- bench on intel Q6600 @ 2.40GHz or armv7 1.5GHz
