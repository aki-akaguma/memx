## Benchmark results

- compile by rustc 1.67.1 (d5a82bbd2 2023-02-07)

  1. x86_64-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memcmp              |  180.850 µs |  233.360 µs |  342.040 µs |  362.860 µs |
| libc_memcmp             |  155.270 µs |  171.970 µs |  338.280 µs |  353.930 µs |
| memx_memcmp             |  157.950 µs |  260.630 µs |  161.790 µs |  239.190 µs |
| memx_memcmp_basic       |  121.070 µs |  184.020 µs |  124.970 µs |  191.800 µs |
| memx_memcmp_sse2        |  154.020 µs |  221.620 µs |  157.970 µs |  232.430 µs |

  2. i686-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memcmp              |  189.520 µs |  209.090 µs |  288.250 µs |  383.360 µs |
| libc_memcmp             |  167.220 µs |  200.330 µs |  275.800 µs |  392.800 µs |
| memx_memcmp             |  202.760 µs |  279.000 µs |  216.870 µs |  330.230 µs |
| memx_memcmp_basic       |  145.800 µs |  229.710 µs |  153.420 µs |  242.610 µs |
| memx_memcmp_sse2        |  187.920 µs |  272.110 µs |  201.470 µs |  307.950 µs |

  3. i586-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memcmp              |  203.390 µs |  209.020 µs |  349.200 µs |  375.170 µs |
| libc_memcmp             |  169.770 µs |  198.480 µs |  355.150 µs |  412.700 µs |
| memx_memcmp             |  172.270 µs |  238.610 µs |  168.740 µs |  216.600 µs |
| memx_memcmp_basic       |  172.150 µs |  237.930 µs |  170.800 µs |  218.350 µs |


- compile by rustc 1.53.0 (53cb7b09b 2021-06-17)

  1. x86_64-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memcmp              |  155.810 us |  173.740 us |  253.170 us |  343.250 us |
| libc_memcmp             |  175.920 us |  187.280 us |  259.290 us |  344.760 us |
| memx_memcmp             |  140.330 us |  140.790 us |  134.440 us |  130.510 us |
| memx_memcmp_basic       |  136.670 us |  139.910 us |  133.930 us |  129.710 us |
| memx_memcmp_sse2        |  134.700 us |  179.240 us |  131.100 us |  167.600 us |

  2. i686-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memcmp              |  160.610 us |  258.220 us |  336.550 us |  475.040 us |
| libc_memcmp             |  182.480 us |  214.320 us |  331.690 us |  483.450 us |
| memx_memcmp             |  181.660 us |  210.840 us |  185.900 us |  183.890 us |
| memx_memcmp_basic       |  183.730 us |  212.440 us |  183.740 us |  179.500 us |
| memx_memcmp_sse2        |  164.930 us |  209.410 us |  178.200 us |  208.410 us |

  3. i586-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memcmp              |  164.030 us |  202.900 us |  335.210 us |  475.980 us |
| libc_memcmp             |  180.430 us |  201.320 us |  338.900 us |  483.720 us |
| memx_memcmp             |  186.900 us |  209.290 us |  177.570 us |  179.290 us |
| memx_memcmp_basic       |  189.610 us |  208.060 us |  177.470 us |  180.140 us |

  4. armv7-linux-androideabi:

|         `name`          |  `andr:en`  |  `andr:ja`  |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memcmp              | 1120.900 us | 1214.700 us | 1381.700 us | 1793.800 us |
| libc_memcmp             | 1248.900 us | 1195.200 us | 1351.200 us | 1777.000 us |
| memx_memcmp             |  652.500 us |  671.780 us |  681.010 us |  707.360 us |
| memx_memcmp_basic       |  673.990 us |  723.850 us |  904.490 us |  679.730 us |


- `us` is micro seconds
- `:en` is english haystack.
- `:ja` is japanese haystack.
- `musl` is x86_64-unknown-linux-musl
- bench on intel Q6600 @ 2.40GHz or armv7 1.5GHz
