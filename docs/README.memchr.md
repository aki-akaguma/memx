## Benchmark results

- compile by rustc 1.67.1 (d5a82bbd2 2023-02-07)

  1. x86_64-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memchr              |  374.450 µs |  177.610 µs |  350.420 µs |  176.480 µs |
| libc_memchr             |  253.510 µs |   72.648 µs |  581.550 µs |  202.900 µs |
| memchr_memchr           |  225.560 µs |   67.144 µs |  231.720 µs |   65.656 µs |
| memx_memchr             |  239.800 µs |   67.618 µs |  246.680 µs |   66.453 µs |
| memx_memchr_basic       |  188.120 µs |   69.965 µs |  193.420 µs |   70.606 µs |
| memx_memchr_sse2        |  214.290 µs |   61.788 µs |  219.510 µs |   60.615 µs |

  2. i686-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memchr              |  336.770 µs |  173.410 µs |  337.860 µs |  174.520 µs |
| libc_memchr             |  268.220 µs |   74.718 µs |  521.080 µs |  132.260 µs |
| memchr_memchr           |  337.060 µs |  154.720 µs |  376.740 µs |  152.920 µs |
| memx_memchr             |  246.800 µs |   68.233 µs |  279.400 µs |   75.708 µs |
| memx_memchr_basic       |  216.860 µs |  102.170 µs |  188.250 µs |  105.700 µs |
| memx_memchr_sse2        |  233.380 µs |   67.435 µs |  248.940 µs |   70.668 µs |

  3. i586-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memchr              |  360.190 µs |  176.780 µs |  363.920 µs |  179.240 µs |
| libc_memchr             |  271.960 µs |   77.004 µs |  531.690 µs |  135.550 µs |
| memchr_memchr           |  357.390 µs |  158.850 µs |  384.300 µs |  154.630 µs |
| memx_memchr             |  219.390 µs |  105.990 µs |  197.540 µs |  103.370 µs |
| memx_memchr_basic       |  219.180 µs |  106.120 µs |  197.720 µs |   99.820 µs |


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
