## Benchmark results

  1. x86_64-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memrmem             |  585.130 us |  903.110 us |  617.810 us |  909.770 us |
| libc_memrmem            | 1504.300 us | 1558.400 us |  604.980 us |  648.610 us |
| memchr_memrmem          |  769.770 us |  750.020 us |  758.980 us |  757.300 us |
| memx_memrmem            |  143.060 us |  143.690 us |  141.740 us |  141.660 us |
| memx_memrmem_basic      |  128.340 us |  128.350 us |  126.140 us |  126.020 us |

  2. i686-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memrmem             |  888.060 us | 1035.100 us |  931.730 us | 1048.100 us |
| memchr_memrmem          |  953.280 us |  961.110 us |  963.390 us |  943.960 us |
| memx_memrmem            |  150.780 us |  150.700 us |  168.700 us |  167.490 us |
| memx_memrmem_basic      |  129.820 us |  129.940 us |  131.440 us |  132.910 us |

  3. i586-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memrmem             |  895.380 us |  968.770 us |  935.150 us |  963.800 us |
| memchr_memrmem          | 1051.000 us | 1600.800 us | 1045.800 us | 1564.600 us |
| memx_memrmem            |  134.450 us |  134.530 us |  132.990 us |  134.490 us |
| memx_memrmem_basic      |  133.340 us |  133.430 us |  135.110 us |  135.710 us |

  4. armv7-linux-androideabi:

|         `name`          |  `andr:en`  |  `andr:ja`  |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memrmem             | 4581.200 us | 2551.100 us | 4267.100 us | 2270.600 us |
| memchr_memrmem          | 3847.200 us | 3633.100 us | 3950.200 us | 3783.300 us |
| memx_memrmem            |  542.720 us |  558.190 us |  544.900 us |  512.220 us |
| memx_memrmem_basic      |  543.160 us |  541.400 us |  548.790 us |  514.280 us |


- compile by rustc 1.52.1 (9bc8c42bb 2021-05-09)
- `us` is micro seconds
- `:en` is english haystack.
- `:ja` is japanese haystack.
- `musl` is x86_64-unknown-linux-musl
- bench on intel Q6600 @ 2.40GHz or armv7 1.5GHz
