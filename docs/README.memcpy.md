## Benchmark results

- compile by rustc 1.67.1 (d5a82bbd2 2023-02-07)

  1. x86_64-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memcpy              |   54.815 µs |  191.130 µs |  162.920 µs |  350.420 µs |
| libc_memcpy             |   54.754 µs |  186.660 µs |  159.870 µs |  350.090 µs |
| memx_memcpy             |   62.470 µs |  196.110 µs |  167.690 µs |  355.040 µs |
| memx_memcpy_basic       |   59.331 µs |  185.480 µs |  162.860 µs |  352.450 µs |
| memx_memcpy_sse2        |   59.313 µs |  185.400 µs |  162.850 µs |  353.610 µs |

  2. i686-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memcpy              |   61.486 µs |  182.260 µs |  122.630 µs |  294.370 µs |
| libc_memcpy             |   67.812 µs |  186.030 µs |  136.330 µs |  299.850 µs |
| memx_memcpy             |   74.112 µs |  206.020 µs |  133.310 µs |  297.070 µs |
| memx_memcpy_basic       |   73.861 µs |  187.510 µs |  133.350 µs |  298.420 µs |
| memx_memcpy_sse2        |   73.858 µs |  187.720 µs |  133.310 µs |  297.170 µs |

  3. i586-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memcpy              |   68.692 µs |  184.290 µs |  136.020 µs |  298.750 µs |
| libc_memcpy             |   67.493 µs |  182.880 µs |  131.760 µs |  295.230 µs |
| memx_memcpy             |   71.674 µs |  184.070 µs |  136.330 µs |  297.780 µs |
| memx_memcpy_basic       |   71.298 µs |  188.710 µs |  136.330 µs |  297.810 µs |


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
