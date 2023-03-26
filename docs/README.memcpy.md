## Benchmark results

- compile by rustc 1.68.1 (8460ca823 2023-03-20)

  0. x86_64-unknown-linux- @Broadwell:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memcpy              |    6.872 µs |    8.269 µs |   26.543 µs |   22.304 µs |
| libc_memcpy             |    6.880 µs |    8.169 µs |   26.817 µs |   22.084 µs |
| memx_memcpy             |    9.241 µs |    9.383 µs |   29.432 µs |   22.945 µs |
| memx_memcpy_basic       |    6.694 µs |    8.244 µs |   27.044 µs |   21.569 µs |
| memx_memcpy_sse2        |    6.893 µs |    8.462 µs |   26.734 µs |   21.426 µs |
| memx_memcpy_avx2        |    7.586 µs |    9.036 µs |   27.975 µs |   22.115 µs |

  1. x86_64-unknown-linux- @Q6600:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memcpy              |   12.585 µs |   29.271 µs |   36.705 µs |   54.589 µs |
| libc_memcpy             |   12.532 µs |   29.643 µs |   36.689 µs |   54.775 µs |
| memx_memcpy             |   15.738 µs |   31.160 µs |   40.529 µs |   56.633 µs |
| memx_memcpy_basic       |   12.886 µs |   29.182 µs |   37.028 µs |   54.639 µs |
| memx_memcpy_sse2        |   12.896 µs |   29.211 µs |   36.982 µs |   54.712 µs |

  2. i686-unknown-linux- @Q6600:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memcpy              |   17.984 µs |   28.515 µs |   32.633 µs |   48.324 µs |
| libc_memcpy             |   17.008 µs |   28.270 µs |   32.259 µs |   48.993 µs |
| memx_memcpy             |   20.701 µs |   30.442 µs |   33.687 µs |   48.947 µs |
| memx_memcpy_basic       |   18.291 µs |   29.301 µs |   35.110 µs |   50.182 µs |
| memx_memcpy_sse2        |   18.176 µs |   29.624 µs |   35.185 µs |   49.874 µs |

  3. i586-unknown-linux- @Q6600:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memcpy              |   18.279 µs |   28.471 µs |   31.871 µs |   49.239 µs |
| libc_memcpy             |   17.106 µs |   28.679 µs |   32.376 µs |   48.146 µs |
| memx_memcpy             |   24.550 µs |   37.095 µs |   38.170 µs |   51.933 µs |
| memx_memcpy_basic       |   18.540 µs |   29.480 µs |   31.768 µs |   50.320 µs |


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
