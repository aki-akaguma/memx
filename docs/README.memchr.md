## Benchmark results

  1. x86_64-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memchr              |  374.630 us |  175.210 us |  367.420 us |  174.460 us |
| libc_memchr             |  248.940 us |   71.921 us |  563.870 us |  197.420 us |
| memchr_memchr           |  221.170 us |   66.064 us |  226.870 us |   64.568 us |
| memx_memchr             |  224.130 us |   67.111 us |  232.640 us |   64.936 us |
| memx_memchr_basic       |  234.440 us |   88.423 us |  236.980 us |   81.329 us |
| memx_memchr_sse2        |  222.650 us |   66.096 us |  226.730 us |   63.800 us |

  2. i686-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memchr              |  335.560 us |  173.440 us |  334.750 us |  174.060 us |
| libc_memchr             |  267.280 us |   77.779 us |  487.730 us |  130.860 us |
| memchr_memchr           |  511.230 us |  194.470 us |  584.700 us |  200.420 us |
| memx_memchr             |  340.780 us |   91.062 us |  389.120 us |  102.260 us |
| memx_memchr_basic       |  290.440 us |  118.960 us |  298.900 us |  120.070 us |
| memx_memchr_sse2        |  270.030 us |   74.967 us |  279.270 us |   79.698 us |

  3. i586-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memchr              |  350.730 us |  177.160 us |  348.830 us |  178.990 us |
| libc_memchr             |  267.020 us |   77.061 us |  526.450 us |  132.050 us |
| memchr_memchr           |  530.550 us |  196.080 us |  499.750 us |  174.170 us |
| memx_memchr             |  302.490 us |  123.280 us |  300.970 us |  119.720 us |
| memx_memchr_basic       |  301.360 us |  119.520 us |  295.880 us |  127.110 us |

  4. armv7-linux-androideabi:

|         `name`          |  `andr:en`  |  `andr:ja`  |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memchr              | 1232.100 us |  950.750 us | 1262.200 us | 1047.500 us |
| libc_memchr             | 1874.100 us | 1066.300 us | 1865.200 us |  838.210 us |
| memchr_memchr           | 1391.400 us |  642.260 us | 1441.300 us |  649.600 us |
| memx_memchr             | 1042.000 us |  555.030 us | 1129.400 us |  519.740 us |
| memx_memchr_basic       | 1005.600 us |  573.360 us | 1095.400 us |  525.020 us |

- compile by rustc 1.52.1 (9bc8c42bb 2021-05-09)
- `us` is micro seconds
- `:en` is english haystack.
- `:ja` is japanese haystack.
- `musl` is x86_64-unknown-linux-musl
- bench on intel Q6600 @ 2.40GHz or armv7 1.5GHz
