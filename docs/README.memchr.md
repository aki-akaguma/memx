## Benchmark results

  1. x86_64-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memchr              |  369.320 us |  175.200 us |  367.500 us |  174.240 us |
| memx_memchr             |  257.810 us |   82.803 us |  262.850 us |   82.090 us |
| memx_memchr_basic       |  260.680 us |   93.369 us |  263.890 us |   93.209 us |
| memx_memchr_libc        |  290.900 us |   83.691 us |  616.360 us |  203.530 us |
| memchr_memchr           |  222.060 us |   65.809 us |  225.610 us |   64.784 us |

  2. i686-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memchr              |  335.730 us |  173.060 us |  333.950 us |  172.740 us |
| memx_memchr             |  285.690 us |   89.645 us |  294.270 us |   92.383 us |
| memx_memchr_basic       |  291.360 us |  132.090 us |  316.710 us |  137.620 us |
| memx_memchr_libc        |  371.590 us |   97.953 us |  591.740 us |  164.630 us |
| memchr_memchr           |  493.090 us |  196.520 us |  518.660 us |  200.080 us |

  3. i586-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memchr              |  359.620 us |  175.450 us |  349.660 us |  178.190 us |
| memx_memchr             |  314.460 us |  137.320 us |  323.040 us |  140.850 us |
| memx_memchr_basic       |  308.430 us |  133.810 us |  327.580 us |  143.920 us |
| memx_memchr_libc        |  368.470 us |   98.345 us |  616.150 us |  167.310 us |
| memchr_memchr           |  500.860 us |  181.290 us |  499.920 us |  177.790 us |

  4. armv7-linux-androideabi:

|         `name`          |  `andr:en`  |  `andr:ja`  |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memchr              | 1141.800 us |  972.290 us | 1353.900 us | 1022.100 us |
| memx_memchr             | 1004.700 us |  594.170 us |  981.910 us |  581.470 us |
| memx_memchr_basic       |  971.860 us |  607.440 us |  943.010 us |  542.700 us |
| memx_memchr_libc        | 2336.200 us | 1164.400 us | 2031.400 us |  833.020 us |
| memchr_memchr           | 1538.200 us |  694.770 us | 1476.200 us |  620.350 us |

- compile by rustc 1.52.1 (9bc8c42bb 2021-05-09)
- `us` is micro seconds
- `:en` is english haystack.
- `:ja` is japanese haystack.
- `musl` is x86_64-unknown-linux-musl
- bench on intel Q6600 @ 2.40GHz or armv7 1.5GHz
