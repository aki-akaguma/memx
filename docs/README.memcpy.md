## Benchmark results

- compile by rustc 1.68.0 (2c8cc3432 2023-03-06)

  0. x86_64-unknown-linux- @Broadwell:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memcpy              |   27.301 µs |   56.153 µs |  115.370 µs |  120.810 µs |
| libc_memcpy             |   28.078 µs |   56.701 µs |  115.530 µs |  121.540 µs |
| memx_memcpy             |   29.573 µs |   57.180 µs |  125.360 µs |  128.940 µs |
| memx_memcpy_basic       |   29.474 µs |   57.452 µs |  116.530 µs |  122.340 µs |
| memx_memcpy_sse2        |   30.091 µs |   57.347 µs |  118.680 µs |  123.310 µs |

  1. x86_64-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memcpy              |   54.965 µs |  189.450 µs |  163.040 µs |  350.810 µs |
| libc_memcpy             |   54.832 µs |  186.460 µs |  160.010 µs |  351.300 µs |
| memx_memcpy             |   62.526 µs |  196.580 µs |  167.750 µs |  355.660 µs |
| memx_memcpy_basic       |   59.354 µs |  185.310 µs |  162.980 µs |  355.480 µs |
| memx_memcpy_sse2        |   59.380 µs |  185.270 µs |  162.940 µs |  358.150 µs |

  2. i686-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memcpy              |   61.940 µs |  184.260 µs |  123.260 µs |  302.870 µs |
| libc_memcpy             |   68.255 µs |  186.960 µs |  136.690 µs |  307.890 µs |
| memx_memcpy             |   74.712 µs |  207.060 µs |  134.070 µs |  304.300 µs |
| memx_memcpy_basic       |   74.236 µs |  188.870 µs |  133.990 µs |  304.820 µs |
| memx_memcpy_sse2        |   74.304 µs |  189.360 µs |  134.080 µs |  304.300 µs |

  3. i586-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memcpy              |   65.862 µs |  183.290 µs |  137.400 µs |  306.680 µs |
| libc_memcpy             |   68.358 µs |  184.470 µs |  132.390 µs |  303.090 µs |
| memx_memcpy             |   71.714 µs |  188.380 µs |  136.710 µs |  303.870 µs |
| memx_memcpy_basic       |   71.676 µs |  189.000 µs |  137.130 µs |  304.490 µs |


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
