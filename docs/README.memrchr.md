## Benchmark results

- compile by rustc 1.65.0 (897e37553 2022-11-02)

  1. x86_64-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memrchr             |  507.340 µs |  244.890 µs |  516.440 µs |  233.890 µs |
| libc_memrchr            |  220.150 µs |   74.205 µs |  560.610 µs |  272.420 µs |
| memchr_memrchr          |  262.760 µs |   79.733 µs |  259.720 µs |   78.523 µs |
| memx_memrchr            |  305.910 µs |   85.519 µs |  306.880 µs |   85.552 µs |
| memx_memrchr_basic      |  258.260 µs |   99.509 µs |  266.980 µs |   98.424 µs |
| memx_memrchr_sse2       |  256.570 µs |   72.875 µs |  254.370 µs |   72.784 µs |

  2. i686-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memrchr             |  279.880 µs |  202.110 µs |  402.610 µs |  203.400 µs |
| libc_memrchr            |  233.570 µs |   78.106 µs |  553.570 µs |  220.370 µs |
| memchr_memrchr          |  498.240 µs |  197.180 µs |  506.510 µs |  195.750 µs |
| memx_memrchr            |  417.080 µs |  114.500 µs |  420.180 µs |  116.340 µs |
| memx_memrchr_basic      |  301.930 µs |  128.340 µs |  322.470 µs |  137.480 µs |
| memx_memrchr_sse2       |  291.410 µs |   81.245 µs |  302.440 µs |   84.498 µs |

  3. i586-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memrchr             |  416.940 µs |  199.670 µs |  294.750 µs |  204.270 µs |
| libc_memrchr            |  237.840 µs |   77.558 µs |  571.430 µs |  220.370 µs |
| memchr_memrchr          |  503.520 µs |  184.430 µs |  503.810 µs |  189.680 µs |
| memx_memrchr            |  303.460 µs |  130.900 µs |  316.340 µs |  138.870 µs |
| memx_memrchr_basic      |  299.930 µs |  129.920 µs |  323.540 µs |  139.270 µs |


- compile by rustc 1.53.0 (53cb7b09b 2021-06-17)

  1. x86_64-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memrchr             |  510.110 us |  231.010 us |  500.410 us |  241.550 us |
| libc_memrchr            |  216.100 us |   73.073 us |  543.330 us |  266.120 us |
| memchr_memrchr          |  222.390 us |   69.625 us |  221.260 us |   69.196 us |
| memx_memrchr            |  257.000 us |   73.247 us |  257.010 us |   73.865 us |
| memx_memrchr_basic      |  260.940 us |   95.468 us |  260.350 us |   96.719 us |
| memx_memrchr_sse2       |  251.610 us |   72.529 us |  252.000 us |   73.265 us |

  2. i686-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memrchr             |  389.890 us |  200.600 us |  285.170 us |  200.080 us |
| libc_memrchr            |  273.230 us |   77.156 us |  606.080 us |  210.860 us |
| memchr_memrchr          |  559.270 us |  219.320 us |  525.350 us |  205.200 us |
| memx_memrchr            |  353.940 us |   97.693 us |  410.320 us |  112.270 us |
| memx_memrchr_basic      |  274.110 us |  121.560 us |  272.420 us |  119.450 us |
| memx_memrchr_sse2       |  276.410 us |   79.334 us |  314.740 us |   86.983 us |

  3. i586-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memrchr             |  300.750 us |  203.110 us |  388.990 us |  193.850 us |
| libc_memrchr            |  233.760 us |   77.464 us |  546.530 us |  208.460 us |
| memchr_memrchr          |  523.880 us |  199.920 us |  474.810 us |  185.200 us |
| memx_memrchr            |  272.020 us |  125.010 us |  281.410 us |  120.190 us |
| memx_memrchr_basic      |  276.620 us |  122.500 us |  280.800 us |  119.710 us |

  4. armv7-linux-androideabi:

|         `name`          |  `andr:en`  |  `andr:ja`  |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memrchr             | 3066.700 us | 2229.900 us | 1380.100 us |  905.320 us |
| libc_memrchr            | 3308.500 us | 2453.900 us | 1543.800 us | 1285.000 us |
| memchr_memrchr          | 2925.400 us | 2269.500 us | 1655.700 us |  687.610 us |
| memx_memrchr            | 2507.200 us | 1932.400 us | 1095.700 us |  516.470 us |
| memx_memrchr_basic      | 2503.900 us | 1978.000 us | 1136.900 us |  496.970 us |

- `us` is micro seconds
- `:en` is english haystack.
- `:ja` is japanese haystack.
- `musl` is x86_64-unknown-linux-musl
- bench on intel Q6600 @ 2.40GHz or armv7 1.5GHz
