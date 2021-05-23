## Benchmark results

  1. x86_64-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memchr              |  367.940 us |  175.130 us |  367.300 us |  174.140 us |
| memx_memchr             |  215.850 us |   64.370 us |  223.060 us |   62.937 us |
| memx_memchr_basic       |  225.830 us |   84.399 us |  228.900 us |   84.685 us |
| memx_memchr_libc        |  290.760 us |   83.630 us |  617.350 us |  209.290 us |
| memchr_memchr           |  222.200 us |   65.922 us |  226.770 us |   64.382 us |

  2. i686-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memchr              |  335.520 us |  172.600 us |  333.450 us |  172.390 us |
| memx_memchr             |  331.000 us |   90.219 us |  379.600 us |  100.380 us |
| memx_memchr_basic       |  279.320 us |  114.660 us |  296.190 us |  122.810 us |
| memx_memchr_libc        |  363.040 us |   96.982 us |  584.130 us |  154.070 us |
| memchr_memchr           |  487.460 us |  194.830 us |  523.040 us |  199.350 us |

  3. i586-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memchr              |  373.570 us |  175.300 us |  374.490 us |  178.070 us |
| memx_memchr             |  283.500 us |  119.040 us |  284.730 us |  119.020 us |
| memx_memchr_basic       |  281.260 us |  118.330 us |  284.740 us |  118.330 us |
| memx_memchr_libc        |  356.980 us |   97.649 us |  605.810 us |  154.960 us |
| memchr_memchr           |  479.940 us |  175.450 us |  494.180 us |  171.320 us |

  4. armv7-linux-androideabi:

|         `name`          |  `andr:en`  |  `andr:ja`  |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memchr              | 1367.400 us |  884.600 us | 1139.500 us |  872.730 us |
| memx_memchr             |  921.300 us |  499.560 us |  861.670 us |  457.810 us |
| memx_memchr_basic       |  919.000 us |  508.130 us |  861.610 us |  459.260 us |
| memx_memchr_libc        | 2023.400 us | 1017.700 us | 1727.400 us |  783.800 us |
| memchr_memchr           | 1253.500 us |  591.180 us | 1265.400 us |  569.050 us |

- compile by rustc 1.52.1 (9bc8c42bb 2021-05-09)
- `us` is micro seconds
- `:en` is english haystack.
- `:ja` is japanese haystack.
- `musl` is x86_64-unknown-linux-musl
- bench on intel Q6600 @ 2.40GHz or armv7 1.5GHz
