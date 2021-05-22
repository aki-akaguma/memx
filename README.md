# memx
memory functions like a libc memcmp(), memchr(), memmem(), memcpy(), memset()

## Features

* Rewriting with rust lang.
* minimum support: rustc 1.41.1 (f3e1a954d 2020-02-24)

## Todo

- [ ] Support the zero overhead trait.

## Benchmark

- compile by rustc 1.52.1 (9bc8c42bb 2021-05-09)

  1. x86_64:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memchr              |  369.320 us |  175.200 us |  367.500 us |  174.240 us |
| memx_memchr             |  257.810 us |   82.803 us |  262.850 us |   82.090 us |
| memx_memchr_basic       |  260.680 us |   93.369 us |  263.890 us |   93.209 us |
| memx_memchr_libc        |  290.900 us |   83.691 us |  616.360 us |  203.530 us |
| memchr_memchr           |  222.060 us |   65.809 us |  225.610 us |   64.784 us |

|         `name`          | `bench:en`  | `bench:ja`  |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memcmp              |  256.250 uc |  335.030 uc |  417.780 uc |  372.920 uc |
| memx_memcmp             |  245.860 uc |  308.080 uc |  236.750 uc |  301.260 uc |
| memx_memcmp_basic       |  233.880 uc |  282.990 uc |  240.250 uc |  283.920 uc |
| memx_memcmp_libc        |  350.370 uc |  425.330 uc |  485.850 uc |  579.300 uc |

|         `name`          | `bench:en`  | `bench:ja`  |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memcpy              |  246.550 uc |  437.850 uc |  638.930 uc | 1145.800 uc |
| memx_memcpy             |  332.360 uc |  466.830 uc |  329.470 uc |  471.780 uc |
| memx_memcpy_basic       |  332.210 uc |  467.190 uc |  329.580 uc |  472.080 uc |
| memx_memcpy_libc        |  288.240 uc |  504.140 uc |  675.400 uc | 1214.300 uc |

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memeq               |  275.290 us |  325.640 us |  428.130 us |  380.860 us |
| memx_memeq              |  192.310 us |  248.040 us |  186.260 us |  249.310 us |
| memx_memeq_basic        |  192.140 us |  247.650 us |  186.200 us |  248.890 us |
| memx_memeq_libc         |  334.610 us |  398.930 us |  482.210 us |  543.460 us |

|         `name`          | `bench:en`  | `bench:ja`  |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memmem              |  514.280 uc |  456.430 uc |  510.550 uc |  472.260 uc |
| memx_memmem             |  114.860 uc |  114.660 uc |  116.720 uc |  116.660 uc |
| memx_memmem_basic       |  115.220 uc |  115.490 uc |  115.990 uc |  116.610 uc |
| memx_memmem_libc        |  132.520 uc |  122.250 uc |  220.890 uc |  276.960 uc |
| memchr_memmem           |  185.760 uc |  209.940 uc |  194.000 uc |  222.120 uc |

|         `name`          | `bench:en`  | `bench:ja`  |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memset              |    1.695 uc |    3.686 uc |    1.481 uc |    3.314 uc |
| memx_memset             |    1.959 uc |    4.107 uc |    1.743 uc |    3.670 uc |
| memx_memset_basic       |    2.889 uc |    5.146 uc |    2.896 uc |    5.245 uc |
| memx_memset_libc        |    1.857 uc |    3.901 uc |    1.659 uc |    3.544 uc |


  2. i686:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memchr              |  335.730 us |  173.060 us |  333.950 us |  172.740 us |
| memx_memchr             |  285.690 us |   89.645 us |  294.270 us |   92.383 us |
| memx_memchr_basic       |  291.360 us |  132.090 us |  316.710 us |  137.620 us |
| memx_memchr_libc        |  371.590 us |   97.953 us |  591.740 us |  164.630 us |
| memchr_memchr           |  493.090 us |  196.520 us |  518.660 us |  200.080 us |

|         `name`          | `bench:en`  | `bench:ja`  |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memcmp              |  266.600 uc |  365.520 uc |  557.240 uc |  530.380 uc |
| memx_memcmp             |  315.770 uc |  371.150 uc |  324.900 uc |  422.970 uc |
| memx_memcmp_basic       |  327.260 uc |  371.040 uc |  321.080 uc |  399.910 uc |
| memx_memcmp_libc        |  391.420 uc |  526.450 uc |  666.070 uc |  739.410 uc |

|         `name`          | `bench:en`  | `bench:ja`  |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memcpy              |  323.310 uc |  586.280 uc |  418.100 uc |  745.570 uc |
| memx_memcpy             |  435.160 uc |  830.570 uc |  363.750 uc |  810.380 uc |
| memx_memcpy_basic       |  436.470 uc |  831.460 uc |  363.070 uc |  809.770 uc |
| memx_memcpy_libc        |  389.430 uc |  741.410 uc |  494.500 uc |  890.600 uc |

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memeq               |  280.420 us |  350.530 us |  626.070 us |  651.890 us |
| memx_memeq              |  301.310 us |  358.370 us |  316.790 us |  371.520 us |
| memx_memeq_basic        |  306.400 us |  355.980 us |  313.500 us |  358.960 us |
| memx_memeq_libc         |  366.260 us |  477.360 us |  707.470 us |  838.500 us |

|         `name`          | `bench:en`  | `bench:ja`  |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memmem              |  692.010 uc |  558.620 uc |  724.800 uc |  590.770 uc |
| memx_memmem             |  138.470 uc |  135.850 uc |  136.450 uc |  136.980 uc |
| memx_memmem_basic       |  139.030 uc |  135.280 uc |  137.430 uc |  137.520 uc |
| memx_memmem_libc        |  185.490 uc |  172.060 uc |  247.830 uc |  258.750 uc |
| memchr_memmem           |  473.520 uc |  487.570 uc |  506.670 uc |  492.670 uc |

|         `name`          | `bench:en`  | `bench:ja`  |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memset              |    1.355 uc |    2.755 uc |    2.510 uc |    4.622 uc |
| memx_memset             |    1.498 uc |    3.117 uc |    2.618 uc |    4.808 uc |
| memx_memset_basic       |    2.922 uc |    5.265 uc |    2.980 uc |    5.367 uc |
| memx_memset_libc        |    1.398 uc |    2.926 uc |    2.651 uc |    4.812 uc |


  3. armv7-linux-androideabi:

|         `name`          |  `andr:en`  |  `andr:ja`  |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memchr              | 1141.800 us |  972.290 us | 1353.900 us | 1022.100 us |
| memx_memchr             | 1004.700 us |  594.170 us |  981.910 us |  581.470 us |
| memx_memchr_basic       |  971.860 us |  607.440 us |  943.010 us |  542.700 us |
| memx_memchr_libc        | 2336.200 us | 1164.400 us | 2031.400 us |  833.020 us |
| memchr_memchr           | 1538.200 us |  694.770 us | 1476.200 us |  620.350 us |

|         `name`          | `andro:en`  | `andro:ja`  |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memcmp              | 2888.200 uc | 2469.200 uc | 2108.000 uc | 2083.500 uc |
| memx_memcmp             | 1148.400 uc | 1439.500 uc | 1376.400 uc | 1521.300 uc |
| memx_memcmp_basic       | 1146.600 uc | 1429.700 uc | 1381.200 uc | 1519.500 uc |
| memx_memcmp_libc        | 2363.000 uc | 3198.000 uc | 2310.200 uc | 2390.100 uc |

|         `name`          | `andro:en`  | `andro:ja`  |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memcpy              | 2685.300 uc | 3121.200 uc | 2736.200 uc | 4416.000 uc |
| memx_memcpy             | 1171.100 uc | 1170.100 uc | 1000.800 uc | 1793.300 uc |
| memx_memcpy_basic       | 1142.800 uc | 1144.200 uc | 1068.000 uc | 1818.200 uc |
| memx_memcpy_libc        | 2198.700 uc | 2197.300 uc | 1856.500 uc | 2985.200 uc |

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memeq               | 2500.500 us | 2590.400 us | 2139.500 us | 2295.200 us |
| memx_memeq              | 1078.800 us | 1319.800 us |  879.160 us | 1134.000 us |
| memx_memeq_basic        | 1124.300 us | 1478.200 us |  915.480 us | 1159.900 us |
| memx_memeq_libc         | 2279.300 us | 3124.900 us | 2264.000 us | 2371.700 us |

|         `name`          | `andro:en`  | `andro:ja`  |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memmem              | 2781.000 uc | 2119.100 uc | 2717.800 uc | 1984.600 uc |
| memx_memmem             |  580.130 uc |  582.210 uc |  617.470 uc |  655.810 uc |
| memx_memmem_basic       |  579.280 uc |  582.330 uc |  617.390 uc |  659.470 uc |
| memx_memmem_libc        | 1982.600 uc | 1997.000 uc | 1454.600 uc | 1488.700 uc |
| memchr_memmem           | 1934.900 uc | 1960.200 uc | 1844.700 uc | 1954.500 uc |

|         `name`          | `andro:en`  | `andro:ja`  |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memset              |   84.693 uc |  153.710 uc |   75.490 uc |   15.630 uc |
| memx_memset             |   45.985 uc |   82.639 uc |   45.347 uc |   81.686 uc |
| memx_memset_basic       |   45.744 uc |   82.058 uc |   45.365 uc |   81.650 uc |
| memx_memset_libc        |   60.770 uc |  113.470 uc |    4.496 uc |    8.083 uc |

- std is std::str::find()
- `us` is micro seconds
- `:en` is english haystack.
- `:ja` is japanese haystack.
- `musl` is x86_64-unknown-linux-musl
- bench on intel Q6600 @ 2.40GHz


## Changelogs

[This crate's changelog here.](https://github.com/aki-akaguma/memx/blob/main/CHANGELOG.md)


## References

 * [making-a-char-searcher-in-c](https://pzemtsov.github.io/2019/09/26/making-a-char-searcher-in-c.html)
 * [bithacks-ZeroInWord](https://graphics.stanford.edu/~seander/bithacks.html#ZeroInWord)
