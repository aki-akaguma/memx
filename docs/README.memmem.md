## Benchmark results

  1. x86_64-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memmem              |  513.760 us |  458.930 us |  508.560 us |  464.610 us |
| memx_memmem             |   99.803 us |  100.030 us |   98.755 us |   99.827 us |
| memx_memmem_basic       |  105.370 us |  105.370 us |  106.900 us |  106.820 us |
| memx_memmem_libc        |  131.360 us |  121.760 us |  221.930 us |  270.970 us |
| memchr_memmem           |  188.010 us |  215.750 us |  201.190 us |  225.180 us |

  2. i686-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memmem              |  693.300 us |  550.590 us |  720.670 us |  583.730 us |
| memx_memmem             |  108.520 us |  109.180 us |  119.920 us |  122.560 us |
| memx_memmem_basic       |  121.650 us |  117.310 us |  119.040 us |  119.030 us |
| memx_memmem_libc        |  183.870 us |  137.300 us |  296.680 us |  254.710 us |
| memchr_memmem           |  468.330 us |  480.130 us |  486.570 us |  495.170 us |

  3. i586-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memmem              |  758.220 us |  589.250 us |  712.350 us |  551.120 us |
| memx_memmem             |  118.990 us |  118.880 us |  122.560 us |  123.150 us |
| memx_memmem_basic       |  118.660 us |  123.170 us |  121.710 us |  122.840 us |
| memx_memmem_libc        |  183.310 us |  169.360 us |  240.450 us |  247.970 us |
| memchr_memmem           |  503.040 us |  568.980 us |  493.270 us |  548.830 us |

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
