## Benchmark results

- compile by rustc 1.68.1 (8460ca823 2023-03-20)

  0. x86_64-unknown-linux- @Broadwell:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memcpy              |    6.905 µs |    8.252 µs |   26.396 µs |   21.470 µs |
| libc_memcpy             |    6.933 µs |    8.169 µs |   26.452 µs |   21.544 µs |
| memx_memcpy             |    6.659 µs |    8.272 µs |   26.963 µs |   21.481 µs |
| memx_memcpy_basic       |    7.006 µs |    8.336 µs |   27.005 µs |   21.474 µs |
| memx_memcpy_sse2        |    6.719 µs |    8.352 µs |   27.722 µs |   21.675 µs |
| memx_memcpy_avx2        |    7.307 µs |    8.532 µs |   27.442 µs |   21.733 µs |

  1. x86_64-unknown-linux- @Q6600:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memcpy              |   12.519 µs |   28.749 µs |   36.668 µs |   55.076 µs |
| libc_memcpy             |   12.527 µs |   29.658 µs |   36.701 µs |   55.203 µs |
| memx_memcpy             |   12.888 µs |   29.175 µs |   37.002 µs |   54.996 µs |
| memx_memcpy_basic       |   12.893 µs |   29.253 µs |   37.002 µs |   54.760 µs |
| memx_memcpy_sse2        |   12.857 µs |   29.174 µs |   36.987 µs |   54.691 µs |

  2. i686-unknown-linux- @Q6600:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memcpy              |   17.881 µs |   28.076 µs |   32.421 µs |   48.516 µs |
| libc_memcpy             |   17.481 µs |   28.591 µs |   32.177 µs |   48.314 µs |
| memx_memcpy             |   18.324 µs |   29.248 µs |   35.419 µs |   49.145 µs |
| memx_memcpy_basic       |   18.319 µs |   29.176 µs |   35.455 µs |   49.189 µs |
| memx_memcpy_sse2        |   18.324 µs |   29.243 µs |   35.439 µs |   49.236 µs |

  3. i586-unknown-linux- @Q6600:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memcpy              |   18.207 µs |   28.353 µs |   31.892 µs |   48.031 µs |
| libc_memcpy             |   17.241 µs |   28.473 µs |   31.910 µs |   47.717 µs |
| memx_memcpy             |   18.074 µs |   29.117 µs |   33.305 µs |   48.948 µs |
| memx_memcpy_basic       |   18.058 µs |   29.117 µs |   32.956 µs |   48.917 µs |


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
- bench on intel Q6600 @ 2.40GHz or armv7 1.5GHz
