## Benchmark results

  1. x86_64-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memchr              |  369.580 us |  175.850 us |  368.400 us |  174.760 us |
| libc_memchr             |  248.400 us |   72.346 us |  554.470 us |  193.060 us |
| memchr_memchr           |  221.120 us |   66.201 us |  226.170 us |   64.654 us |
| memx_memchr             |  224.950 us |   66.688 us |  232.070 us |   65.137 us |
| memx_memchr_basic       |  232.010 us |   83.149 us |  235.130 us |   81.555 us |
| memx_memchr_sse2        |  219.020 us |   65.638 us |  226.610 us |   63.985 us |

  2. i686-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memchr              |  336.490 us |  173.990 us |  342.830 us |  173.070 us |
| libc_memchr             |  276.020 us |   72.742 us |  536.880 us |  146.840 us |
| memchr_memchr           |  516.130 us |  197.810 us |  537.790 us |  198.860 us |
| memx_memchr             |  346.660 us |   92.432 us |  388.880 us |  101.640 us |
| memx_memchr_basic       |  303.110 us |  121.470 us |  311.470 us |  124.280 us |
| memx_memchr_sse2        |  277.540 us |   77.050 us |  300.430 us |   82.391 us |

  3. i586-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memchr              |  374.740 us |  175.420 us |  374.900 us |  178.750 us |
| libc_memchr             |  266.070 us |   75.992 us |  525.240 us |  134.310 us |
| memchr_memchr           |  522.830 us |  181.980 us |  496.070 us |  176.800 us |
| memx_memchr             |  290.240 us |  121.120 us |  310.510 us |  126.290 us |
| memx_memchr_basic       |  291.420 us |  122.270 us |  313.640 us |  126.070 us |

  4. armv7-linux-androideabi:

|         `name`          |  `andr:en`  |  `andr:ja`  |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memchr              | 1351.500 us |  951.720 us | 1520.400 us |  936.690 us |
| libc_memchr             | 1824.500 us | 1002.100 us | 1799.800 us |  789.140 us |
| memchr_memchr           | 1412.200 us |  629.850 us | 1394.900 us |  594.710 us |
| memx_memchr             |  992.480 us |  532.630 us | 1055.800 us |  510.510 us |
| memx_memchr_basic       | 1069.400 us |  523.320 us | 1055.400 us |  485.650 us |

- compile by rustc 1.52.1 (9bc8c42bb 2021-05-09)
- `us` is micro seconds
- `:en` is english haystack.
- `:ja` is japanese haystack.
- `musl` is x86_64-unknown-linux-musl
- bench on intel Q6600 @ 2.40GHz or armv7 1.5GHz
