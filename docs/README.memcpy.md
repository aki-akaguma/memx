## Benchmark results

- compile by rustc 1.66.0 (69f9c33d7 2022-12-12)

  1. x86_64-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memcpy              |  109.720 µs |  238.990 µs |  443.520 µs |  423.050 µs |
| libc_memcpy             |  109.540 µs |  223.570 µs |  455.920 µs |  434.100 µs |
| memx_memcpy             |  129.940 µs |  381.330 µs |  134.690 µs |  350.630 µs |
| memx_memcpy_basic       |  109.100 µs |  361.090 µs |  113.010 µs |  332.600 µs |
| memx_memcpy_sse2        |  109.130 µs |  360.710 µs |  113.110 µs |  332.530 µs |

  2. i686-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memcpy              |  173.070 µs |  206.870 µs |  308.890 µs |  381.070 µs |
| libc_memcpy             |  178.950 µs |  218.630 µs |  313.560 µs |  404.980 µs |
| memx_memcpy             |  246.770 µs |  413.810 µs |  217.100 µs |  440.320 µs |
| memx_memcpy_basic       |  198.970 µs |  358.340 µs |  142.770 µs |  407.140 µs |
| memx_memcpy_sse2        |  255.970 µs |  394.430 µs |  193.530 µs |  431.510 µs |

- compile by rustc 1.65.0 (897e37553 2022-11-02)

  3. i586-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memcpy              |  123.410 µs |  232.970 µs |  274.290 µs |  340.060 µs |
| libc_memcpy             |  135.010 µs |  245.300 µs |  317.440 µs |  383.630 µs |
| memx_memcpy             |  141.390 µs |  429.070 µs |  137.300 µs |  370.430 µs |
| memx_memcpy_basic       |  140.950 µs |  428.540 µs |  137.300 µs |  370.960 µs |


- compile by rustc 1.53.0 (53cb7b09b 2021-06-17)

  1. x86_64-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memcpy              |  138.630 us |  207.800 us |  451.910 us |  433.550 us |
| libc_memcpy             |  134.360 us |  208.740 us |  452.620 us |  431.460 us |
| memx_memcpy             |  176.000 us |  235.350 us |  123.370 us |  274.940 us |
| memx_memcpy_basic       |  162.600 us |  256.200 us |  111.340 us |  225.810 us |
| memx_memcpy_sse2        |  170.640 us |  208.160 us |  119.570 us |  245.090 us |

  2. i686-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memcpy              |  137.790 us |  218.910 us |  274.320 us |  357.000 us |
| libc_memcpy             |  123.630 us |  206.300 us |  278.470 us |  353.410 us |
| memx_memcpy             |  213.070 us |  263.990 us |  228.690 us |  359.110 us |
| memx_memcpy_basic       |  164.670 us |  247.550 us |  163.570 us |  348.740 us |
| memx_memcpy_sse2        |  165.470 us |  221.280 us |  154.790 us |  292.220 us |

  3. i586-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memcpy              |  136.440 us |  248.370 us |  276.420 us |  355.800 us |
| libc_memcpy             |  122.100 us |  260.420 us |  278.000 us |  358.020 us |
| memx_memcpy             |  162.350 us |  300.770 us |  166.830 us |  290.480 us |
| memx_memcpy_basic       |  161.060 us |  301.330 us |  165.600 us |  289.560 us |

  4. armv7-linux-androideabi:

|         `name`          |  `andr:en`  |  `andr:ja`  |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memcpy              | 1071.000 us | 2871.600 us |  985.590 us | 1400.000 us |
| libc_memcpy             | 1042.100 us | 2949.400 us | 1058.600 us | 1550.600 us |
| memx_memcpy             |  601.610 us | 1268.700 us |  402.380 us | 1148.700 us |
| memx_memcpy_basic       |  525.990 us | 1086.900 us |  336.710 us |  835.900 us |


- compile by rustc 1.53.0 (53cb7b09b 2021-06-17)
- `us` is micro seconds
- `:en` is english haystack.
- `:ja` is japanese haystack.
- `musl` is x86_64-unknown-linux-musl
- bench on intel Q6600 @ 2.40GHz or armv7 1.5GHz
