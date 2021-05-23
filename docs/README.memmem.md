## Benchmark results

  1. x86_64-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memmem              |  512.090 us |  473.260 us |  510.610 us |  449.720 us |
| memx_memmem             |  105.060 us |  105.050 us |  105.980 us |  106.300 us |
| memx_memmem_basic       |  105.200 us |  104.960 us |  106.040 us |  106.120 us |
| memx_memmem_libc        |  133.650 us |  121.720 us |  220.080 us |  268.070 us |
| memchr_memmem           |  198.580 us |  223.970 us |  199.220 us |  226.730 us |

  2. i686-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memmem              |  690.210 us |  551.820 us |  723.960 us |  587.460 us |
| memx_memmem             |  117.710 us |  117.890 us |  118.990 us |  119.010 us |
| memx_memmem_basic       |  117.030 us |  117.120 us |  119.000 us |  118.880 us |
| memx_memmem_libc        |  183.400 us |  172.070 us |  248.440 us |  257.450 us |
| memchr_memmem           |  468.380 us |  472.790 us |  500.590 us |  487.280 us |

  3. i586-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memmem              |  761.200 us |  589.770 us |  714.420 us |  560.150 us |
| memx_memmem             |  118.830 us |  121.060 us |  121.680 us |  121.520 us |
| memx_memmem_basic       |  118.930 us |  118.930 us |  121.690 us |  121.590 us |
| memx_memmem_libc        |  147.730 us |  133.870 us |  244.560 us |  253.370 us |
| memchr_memmem           |  495.620 us |  565.100 us |  501.940 us |  549.430 us |

  4. armv7-linux-androideabi:

|         `name`          |  `andr:en`  |  `andr:ja`  |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memmem              | 2486.500 us | 1898.400 us | 3042.100 us | 1751.400 us |
| memx_memmem             |  574.560 us |  581.120 us |  509.650 us |  493.640 us |
| memx_memmem_basic       |  579.510 us |  586.530 us |  517.480 us |  501.220 us |
| memx_memmem_libc        | 1779.700 us | 1780.400 us | 1289.900 us | 1316.800 us |
| memchr_memmem           | 1675.400 us | 1675.200 us | 1647.300 us | 1632.100 us |


- compile by rustc 1.52.1 (9bc8c42bb 2021-05-09)
- `us` is micro seconds
- `:en` is english haystack.
- `:ja` is japanese haystack.
- `musl` is x86_64-unknown-linux-musl
- bench on intel Q6600 @ 2.40GHz or armv7 1.5GHz
