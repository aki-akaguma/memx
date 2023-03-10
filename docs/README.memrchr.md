## Benchmark results

- compile by rustc 1.68.0 (2c8cc3432 2023-03-06)

  0. x86_64-unknown-linux- @Broadwell:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memrchr             |  156.420 µs |  139.370 µs |  156.610 µs |  139.870 µs |
| libc_memrchr            |  201.940 µs |   54.852 µs |  218.960 µs |  135.260 µs |
| memchr_memrchr          |  198.370 µs |   54.853 µs |  201.500 µs |   56.032 µs |
| memx_memrchr            |  154.240 µs |   45.106 µs |  174.510 µs |   50.074 µs |
| memx_memrchr_basic      |  139.910 µs |   52.376 µs |  152.730 µs |   58.947 µs |
| memx_memrchr_sse2       |  156.270 µs |   47.643 µs |  163.530 µs |   51.558 µs |

  1. x86_64-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memrchr             |  519.200 µs |  235.110 µs |  519.830 µs |  235.290 µs |
| libc_memrchr            |  220.740 µs |   74.655 µs |  563.340 µs |  274.370 µs |
| memchr_memrchr          |  262.280 µs |   79.149 µs |  258.640 µs |   79.585 µs |
| memx_memrchr            |  265.150 µs |   74.861 µs |  264.260 µs |   76.278 µs |
| memx_memrchr_basic      |  198.950 µs |   86.823 µs |  196.310 µs |   89.556 µs |
| memx_memrchr_sse2       |  253.990 µs |   72.176 µs |  254.460 µs |   73.112 µs |

  2. i686-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memrchr             |  397.170 µs |  204.900 µs |  284.670 µs |  198.580 µs |
| libc_memrchr            |  235.250 µs |   78.795 µs |  598.210 µs |  214.020 µs |
| memchr_memrchr          |  399.930 µs |  166.650 µs |  352.930 µs |  149.280 µs |
| memx_memrchr            |  270.430 µs |   75.235 µs |  305.980 µs |   88.732 µs |
| memx_memrchr_basic      |  199.470 µs |  102.120 µs |  202.940 µs |   98.615 µs |
| memx_memrchr_sse2       |  256.700 µs |   72.113 µs |  268.140 µs |   76.833 µs |

  3. i586-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memrchr             |  409.250 µs |  203.680 µs |  308.280 µs |  205.160 µs |
| libc_memrchr            |  238.370 µs |   78.495 µs |  604.430 µs |  212.480 µs |
| memchr_memrchr          |  416.440 µs |  176.640 µs |  407.660 µs |  177.460 µs |
| memx_memrchr            |  195.440 µs |   99.296 µs |  195.040 µs |   98.040 µs |
| memx_memrchr_basic      |  195.610 µs |  101.490 µs |  196.950 µs |   97.583 µs |


- compile by rustc 1.53.0 (53cb7b09b 2021-06-17)

  1. x86_64-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memrchr             |  510.110 us |  231.010 us |  500.410 us |  241.550 us |
| libc_memrchr            |  216.100 us |   73.073 us |  543.330 us |  266.120 us |
| memchr_memrchr          |  222.390 us |   69.625 us |  221.260 us |   69.196 us |
| memx_memrchr            |  257.000 us |   73.247 us |  257.010 us |   73.865 us |
| memx_memrchr_basic      |  260.940 us |   95.468 us |  260.350 us |   96.719 us |
| memx_memrchr_sse2       |  251.610 us |   72.529 us |  252.000 us |   73.265 us |

  2. i686-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memrchr             |  389.890 us |  200.600 us |  285.170 us |  200.080 us |
| libc_memrchr            |  273.230 us |   77.156 us |  606.080 us |  210.860 us |
| memchr_memrchr          |  559.270 us |  219.320 us |  525.350 us |  205.200 us |
| memx_memrchr            |  353.940 us |   97.693 us |  410.320 us |  112.270 us |
| memx_memrchr_basic      |  274.110 us |  121.560 us |  272.420 us |  119.450 us |
| memx_memrchr_sse2       |  276.410 us |   79.334 us |  314.740 us |   86.983 us |

  3. i586-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memrchr             |  300.750 us |  203.110 us |  388.990 us |  193.850 us |
| libc_memrchr            |  233.760 us |   77.464 us |  546.530 us |  208.460 us |
| memchr_memrchr          |  523.880 us |  199.920 us |  474.810 us |  185.200 us |
| memx_memrchr            |  272.020 us |  125.010 us |  281.410 us |  120.190 us |
| memx_memrchr_basic      |  276.620 us |  122.500 us |  280.800 us |  119.710 us |

  4. armv7-linux-androideabi:

|         `name`          |  `andr:en`  |  `andr:ja`  |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memrchr             | 3066.700 us | 2229.900 us | 1380.100 us |  905.320 us |
| libc_memrchr            | 3308.500 us | 2453.900 us | 1543.800 us | 1285.000 us |
| memchr_memrchr          | 2925.400 us | 2269.500 us | 1655.700 us |  687.610 us |
| memx_memrchr            | 2507.200 us | 1932.400 us | 1095.700 us |  516.470 us |
| memx_memrchr_basic      | 2503.900 us | 1978.000 us | 1136.900 us |  496.970 us |

- `us` is micro seconds
- `:en` is english haystack.
- `:ja` is japanese haystack.
- `musl` is x86_64-unknown-linux-musl
- bench on intel Q6600 @ 2.40GHz or armv7 1.5GHz
