## Benchmark results

  1. x86_64-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memeq               |  167.540 us |  165.970 us |  258.030 us |  343.230 us |
| memx_memeq              |  111.280 us |  105.880 us |  111.990 us |  104.760 us |
| memx_memeq_basic        |  114.120 us |  105.890 us |  110.430 us |  104.710 us |
| memx_memeq_libc         |  202.400 us |  188.950 us |  290.630 us |  394.840 us |

  2. i686-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memeq               |  168.140 us |  195.390 us |  364.370 us |  520.140 us |
| memx_memeq              |  160.750 us |  152.510 us |  168.050 us |  156.820 us |
| memx_memeq_basic        |  159.900 us |  152.200 us |  167.060 us |  156.670 us |
| memx_memeq_libc         |  219.240 us |  243.490 us |  423.940 us |  583.040 us |

  3. i586-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memeq               |  168.800 us |  194.180 us |  367.740 us |  513.910 us |
| memx_memeq              |  155.680 us |  150.390 us |  158.010 us |  155.440 us |
| memx_memeq_basic        |  155.670 us |  150.380 us |  159.310 us |  155.560 us |
| memx_memeq_libc         |  209.360 us |  241.050 us |  417.680 us |  576.340 us |

  4. armv7-linux-androideabi:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memeq               | 1119.700 us | 1148.200 us | 1392.300 us | 1799.400 us |
| memx_memeq              |  591.360 us |  624.110 us |  509.610 us |  557.040 us |
| memx_memeq_basic        |  614.770 us |  663.520 us |  510.900 us |  537.800 us |
| memx_memeq_libc         | 1404.700 us | 1338.200 us | 1414.900 us | 1863.300 us |


- compile by rustc 1.52.1 (9bc8c42bb 2021-05-09)
- `us` is micro seconds
- `:en` is english haystack.
- `:ja` is japanese haystack.
- `musl` is x86_64-unknown-linux-musl
- bench on intel Q6600 @ 2.40GHz or armv7 1.5GHz
