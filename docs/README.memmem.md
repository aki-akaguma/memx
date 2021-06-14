## Benchmark results

  1. x86_64-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memmem              |  507.400 us |  465.280 us |  510.230 us |  457.730 us |
| libc_memmem             |  129.650 us |  121.430 us |  220.580 us |  258.550 us |
| memchr_memmem           |  190.940 us |  213.760 us |  194.090 us |  222.130 us |
| memx_memmem             |  105.390 us |  103.030 us |  103.380 us |  103.550 us |
| memx_memmem_basic       |  108.310 us |  102.480 us |  103.440 us |  104.090 us |

  2. i686-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memmem              |  695.510 us |  579.970 us |  723.300 us |  593.220 us |
| libc_memmem             |  131.750 us |  120.970 us |  279.430 us |  254.980 us |
| memchr_memmem           |  477.030 us |  480.980 us |  492.690 us |  492.220 us |
| memx_memmem             |  107.340 us |  106.840 us |  120.170 us |  120.480 us |
| memx_memmem_basic       |  120.760 us |  120.850 us |  118.120 us |  118.350 us |

  3. i586-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memmem              |  764.970 us |  652.410 us |  722.620 us |  555.600 us |
| libc_memmem             |  131.350 us |  170.720 us |  236.130 us |  279.530 us |
| memchr_memmem           |  507.120 us |  565.100 us |  511.600 us |  555.120 us |
| memx_memmem             |  122.480 us |  122.480 us |  122.800 us |  127.500 us |
| memx_memmem_basic       |  122.760 us |  122.410 us |  123.460 us |  123.090 us |

  4. armv7-linux-androideabi:

|         `name`          |  `andr:en`  |  `andr:ja`  |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memmem              | 2758.500 us | 1986.200 us | 2836.700 us | 1912.000 us |
| libc_memmem             | 1970.700 us | 1967.300 us | 1428.300 us | 1437.100 us |
| memchr_memmem           | 1927.900 us | 1938.000 us | 1794.400 us | 1871.600 us |
| memx_memmem             |  582.890 us |  583.510 us |  525.980 us |  525.160 us |
| memx_memmem_basic       |  580.670 us |  582.230 us |  503.110 us |  502.320 us |


- compile by rustc 1.52.1 (9bc8c42bb 2021-05-09)
- `us` is micro seconds
- `:en` is english haystack.
- `:ja` is japanese haystack.
- `musl` is x86_64-unknown-linux-musl
- bench on intel Q6600 @ 2.40GHz or armv7 1.5GHz
