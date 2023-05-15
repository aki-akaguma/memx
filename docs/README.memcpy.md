## Benchmark results

- compile by rustc 1.69.0 (84c898d65 2023-04-16)

  0. x86_64-unknown-linux- @i5-4570:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memcpy              |    2.965 µs |    3.379 µs |   20.278 µs |    4.901 µs |
| libc_memcpy             |    3.483 µs |    3.205 µs |   20.289 µs |    5.002 µs |
| memx_memcpy             |    7.016 µs |    3.332 µs |    6.527 µs |    3.232 µs |
| memx_memcpy_basic       |    7.041 µs |    5.133 µs |    7.320 µs |    6.248 µs |
| memx_memcpy_sse2        |    6.864 µs |    3.125 µs |    7.006 µs |    3.331 µs |
| memx_memcpy_avx2        |    7.009 µs |    3.331 µs |    6.504 µs |    3.212 µs |

  1. x86_64-unknown-linux- @Q6600:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memcpy              |    7.372 µs |    7.531 µs |   39.319 µs |    9.589 µs |
| libc_memcpy             |    7.318 µs |    7.483 µs |   46.665 µs |    9.595 µs |
| memx_memcpy             |   13.868 µs |    6.883 µs |   13.674 µs |    6.842 µs |
| memx_memcpy_basic       |   13.964 µs |   10.575 µs |   13.441 µs |   13.105 µs |
| memx_memcpy_sse2        |   13.850 µs |    6.899 µs |   13.653 µs |    6.837 µs |

  2. i686-unknown-linux- @Q6600:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memcpy              |   16.381 µs |   10.334 µs |   31.080 µs |   14.925 µs |
| libc_memcpy             |   15.144 µs |    9.852 µs |   30.693 µs |   14.834 µs |
| memx_memcpy             |   23.592 µs |    9.747 µs |   21.920 µs |    9.981 µs |
| memx_memcpy_basic       |   18.417 µs |   24.154 µs |   20.142 µs |   24.628 µs |
| memx_memcpy_sse2        |   20.956 µs |    9.762 µs |   21.510 µs |   10.059 µs |

  3. i586-unknown-linux- @Q6600:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memcpy              |   15.972 µs |   10.041 µs |   31.421 µs |   15.296 µs |
| libc_memcpy             |   15.077 µs |    9.891 µs |   31.069 µs |   15.181 µs |
| memx_memcpy             |   19.989 µs |    9.425 µs |   21.517 µs |   10.410 µs |
| memx_memcpy_basic       |   20.780 µs |   25.508 µs |   22.087 µs |   25.826 µs |


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

- `µs` is micro seconds
- `:en` is english haystack or short length.
- `:ja` is japanese haystack or long length.
- `gnu` is x86_64-unknown-linux-gnu
- `musl` is x86_64-unknown-linux-musl
- @i5-4570: bench on intel i5-4570 @ 3.2GHz
- @Q6600: bench on intel Q6600 @ 2.40GHz or armv7 1.5GHz
