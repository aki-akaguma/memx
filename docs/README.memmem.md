## Benchmark results

  1. x86_64-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memmem              |  612.240 us |  570.880 us |  607.150 us |  560.260 us |
| libc_memmem             |  233.250 us |  224.830 us |  320.550 us |  372.070 us |
| memchr_memmem           |  286.670 us |  317.610 us |  300.310 us |  313.860 us |
| memx_memmem             |  206.720 us |  207.170 us |  206.540 us |  204.830 us |
| memx_memmem_basic       |  202.730 us |  204.080 us |  205.690 us |  205.760 us |
| memx_memmem_sse2        |  205.070 us |  206.000 us |  209.490 us |  204.000 us |

  2. i686-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memmem              |  692.400 us |  538.660 us |  728.410 us |  585.920 us |
| libc_memmem             |  130.940 us |  121.060 us |  247.750 us |  248.880 us |
| memchr_memmem           |  468.420 us |  474.570 us |  503.080 us |  511.550 us |
| memx_memmem             |  110.800 us |  107.920 us |  129.710 us |  123.910 us |
| memx_memmem_basic       |  118.810 us |  120.630 us |  119.860 us |  118.140 us |
| memx_memmem_sse2        |  107.620 us |  103.230 us |  121.910 us |  118.210 us |

  3. i586-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memmem              |  767.280 us |  601.830 us |  719.390 us |  556.810 us |
| libc_memmem             |  130.960 us |  120.910 us |  240.250 us |  246.600 us |
| memchr_memmem           |  503.330 us |  563.840 us |  508.550 us |  566.600 us |
| memx_memmem             |  125.270 us |  128.230 us |  125.240 us |  120.300 us |
| memx_memmem_basic       |  126.870 us |  128.910 us |  125.320 us |  120.160 us |

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
