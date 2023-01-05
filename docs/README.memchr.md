## Benchmark results

- compile by rustc 1.66.0 (69f9c33d7 2022-12-12)

  1. x86_64-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memchr              |  374.200 µs |  177.860 µs |  374.310 µs |  177.000 µs |
| libc_memchr             |  252.250 µs |   72.838 µs |  567.380 µs |  203.350 µs |
| memchr_memchr           |  225.890 µs |   66.878 µs |  234.180 µs |   66.049 µs |
| memx_memchr             |  242.280 µs |   67.633 µs |  247.270 µs |   67.499 µs |
| memx_memchr_basic       |  188.800 µs |   69.875 µs |  193.900 µs |   70.755 µs |
| memx_memchr_sse2        |  222.670 µs |   61.851 µs |  220.530 µs |   61.600 µs |

  2. i686-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memchr              |  353.570 µs |  174.890 µs |  346.600 µs |  174.720 µs |
| libc_memchr             |  268.770 µs |   77.227 µs |  511.490 µs |  132.290 µs |
| memchr_memchr           |  347.750 µs |  157.010 µs |  360.430 µs |  140.390 µs |
| memx_memchr             |  251.270 µs |   67.976 µs |  286.730 µs |   77.159 µs |
| memx_memchr_basic       |  214.100 µs |  102.220 µs |  190.370 µs |  101.310 µs |
| memx_memchr_sse2        |  234.020 µs |   67.812 µs |  254.910 µs |   70.738 µs |

- compile by rustc 1.65.0 (897e37553 2022-11-02)

  3. i586-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memchr              |  363.050 µs |  176.950 µs |  362.750 µs |  178.870 µs |
| libc_memchr             |  270.080 µs |   77.518 µs |  540.450 µs |  137.140 µs |
| memchr_memchr           |  357.530 µs |  159.880 µs |  382.640 µs |  158.240 µs |
| memx_memchr             |  215.590 µs |  105.880 µs |  189.560 µs |   99.674 µs |
| memx_memchr_basic       |  215.590 µs |  105.840 µs |  189.580 µs |   99.554 µs |


- compile by rustc 1.53.0 (53cb7b09b 2021-06-17)

  1. x86_64-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memchr              |  369.140 us |  175.220 us |  367.450 us |  174.240 us |
| libc_memchr             |  249.860 us |   72.885 us |  556.310 us |  193.510 us |
| memchr_memchr           |  221.400 us |   65.674 us |  225.190 us |   64.664 us |
| memx_memchr             |  224.860 us |   62.641 us |  229.790 us |   62.381 us |
| memx_memchr_basic       |  226.690 us |   84.038 us |  228.820 us |   83.574 us |
| memx_memchr_sse2        |  218.150 us |   61.512 us |  225.590 us |   60.974 us |

  2. i686-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memchr              |  354.390 us |  172.580 us |  341.740 us |  172.530 us |
| libc_memchr             |  264.970 us |   77.258 us |  504.580 us |  134.980 us |
| memchr_memchr           |  495.090 us |  196.640 us |  531.300 us |  199.020 us |
| memx_memchr             |  341.450 us |   91.119 us |  389.200 us |  101.150 us |
| memx_memchr_basic       |  270.310 us |  117.170 us |  286.350 us |  123.790 us |
| memx_memchr_sse2        |  266.010 us |   73.213 us |  276.880 us |   76.584 us |

  3. i586-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memchr              |  373.800 us |  175.510 us |  348.800 us |  178.230 us |
| libc_memchr             |  263.730 us |   76.091 us |  486.350 us |  132.040 us |
| memchr_memchr           |  498.680 us |  177.700 us |  497.360 us |  171.890 us |
| memx_memchr             |  269.420 us |  116.490 us |  283.440 us |  121.410 us |
| memx_memchr_basic       |  271.500 us |  116.470 us |  284.360 us |  121.250 us |

  4. armv7-linux-androideabi:

|         `name`          |  `andr:en`  |  `andr:ja`  |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memchr              | 1232.100 us |  950.750 us | 1262.200 us | 1047.500 us |
| libc_memchr             | 1874.100 us | 1066.300 us | 1865.200 us |  838.210 us |
| memchr_memchr           | 1391.400 us |  642.260 us | 1441.300 us |  649.600 us |
| memx_memchr             | 1042.000 us |  555.030 us | 1129.400 us |  519.740 us |
| memx_memchr_basic       | 1005.600 us |  573.360 us | 1095.400 us |  525.020 us |

- `us` is micro seconds
- `:en` is english haystack.
- `:ja` is japanese haystack.
- `musl` is x86_64-unknown-linux-musl
- bench on intel Q6600 @ 2.40GHz or armv7 1.5GHz
