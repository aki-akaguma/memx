## Benchmark results

  1. x86_64-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memrchr             |  504.300 us |  241.790 us |  500.630 us |  242.570 us |
| libc_memrchr            |  217.120 us |   73.578 us |  560.240 us |  271.240 us |
| memchr_memrchr          |  220.510 us |   70.399 us |  222.230 us |   70.051 us |
| memx_memrchr            |  266.530 us |   83.726 us |  263.970 us |   83.645 us |
| memx_memrchr_basic      |  266.940 us |  105.140 us |  263.160 us |  103.760 us |
| memx_memrchr_sse2       |  265.270 us |   83.303 us |  262.220 us |   83.231 us |

  2. i686-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memrchr             |  272.600 us |  199.490 us |  285.640 us |  202.050 us |
| libc_memrchr            |  235.810 us |   76.800 us |  606.330 us |  221.380 us |
| memchr_memrchr          |  540.350 us |  208.750 us |  575.750 us |  215.730 us |
| memx_memrchr            |  367.620 us |  105.290 us |  400.490 us |  114.640 us |
| memx_memrchr_basic      |  295.540 us |  122.690 us |  322.890 us |  136.940 us |
| memx_memrchr_sse2       |  294.630 us |   87.397 us |  322.800 us |   90.917 us |

  3. i586-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memrchr             |  302.200 us |  203.670 us |  385.810 us |  193.600 us |
| libc_memrchr            |  233.120 us |   77.362 us |  580.160 us |  212.720 us |
| memchr_memrchr          |  517.990 us |  203.410 us |  534.020 us |  202.090 us |
| memx_memrchr            |  300.880 us |  126.610 us |  316.690 us |  136.530 us |
| memx_memrchr_basic      |  304.930 us |  123.670 us |  314.820 us |  136.410 us |

  4. armv7-linux-androideabi:

|         `name`          |  `andr:en`  |  `andr:ja`  |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memrchr             | 3066.700 us | 2229.900 us | 1380.100 us |  905.320 us |
| libc_memrchr            | 3308.500 us | 2453.900 us | 1543.800 us | 1285.000 us |
| memchr_memrchr          | 2925.400 us | 2269.500 us | 1655.700 us |  687.610 us |
| memx_memrchr            | 2507.200 us | 1932.400 us | 1095.700 us |  516.470 us |
| memx_memrchr_basic      | 2503.900 us | 1978.000 us | 1136.900 us |  496.970 us |

- compile by rustc 1.52.1 (9bc8c42bb 2021-05-09)
- `us` is micro seconds
- `:en` is english haystack.
- `:ja` is japanese haystack.
- `musl` is x86_64-unknown-linux-musl
- bench on intel Q6600 @ 2.40GHz or armv7 1.5GHz
