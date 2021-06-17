## Benchmark results

  1. x86_64-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memmem              |  511.400 us |  453.200 us |  512.010 us |  454.050 us |
| libc_memmem             |  134.800 us |  121.410 us |  219.500 us |  263.250 us |
| memchr_memmem           |  186.260 us |  223.420 us |  197.950 us |  223.020 us |
| memx_memmem             |  104.890 us |  102.480 us |  103.330 us |  103.150 us |
| memx_memmem_basic       |  106.580 us |  104.250 us |  105.590 us |  106.460 us |
| memx_memmem_sse2        |  102.550 us |  102.280 us |  102.990 us |  102.950 us |

  2. i686-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memmem              |  701.680 us |  559.120 us |  719.210 us |  585.210 us |
| libc_memmem             |  132.300 us |  120.890 us |  242.860 us |  249.940 us |
| memchr_memmem           |  479.350 us |  482.440 us |  505.110 us |  505.980 us |
| memx_memmem             |  110.500 us |  107.390 us |  130.500 us |  123.280 us |
| memx_memmem_basic       |  119.030 us |  120.500 us |  123.300 us |  118.030 us |
| memx_memmem_sse2        |  107.440 us |  104.240 us |  121.100 us |  120.450 us |

  3. i586-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memmem              |  753.880 us |  593.650 us |  723.660 us |  555.480 us |
| libc_memmem             |  132.000 us |  122.050 us |  234.000 us |  242.070 us |
| memchr_memmem           |  505.680 us |  561.220 us |  501.950 us |  557.370 us |
| memx_memmem             |  126.240 us |  128.140 us |  125.060 us |  119.450 us |
| memx_memmem_basic       |  125.090 us |  128.140 us |  125.580 us |  119.810 us |

  4. armv7-linux-androideabi:

|         `name`          |  `andr:en`  |  `andr:ja`  |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memmem              | 7409.000 us | 3891.700 us | 3709.500 us | 1918.400 us |
| libc_memmem             | 3591.800 us | 3580.600 us | 1554.300 us | 1453.900 us |
| memchr_memmem           | 3747.600 us | 3391.700 us | 2182.600 us | 1897.900 us |
| memx_memmem             | 2243.900 us | 2005.800 us |  546.910 us |  582.020 us |
| memx_memmem_basic       | 2244.100 us | 2122.200 us |  570.210 us |  549.610 us |


- compile by rustc 1.52.1 (9bc8c42bb 2021-05-09)
- `us` is micro seconds
- `:en` is english haystack.
- `:ja` is japanese haystack.
- `musl` is x86_64-unknown-linux-musl
- bench on intel Q6600 @ 2.40GHz or armv7 1.5GHz
