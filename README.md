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

|         `name`          | `bench:en`  | `bench:ja`  |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memchr              |  225.480 uc |  225.440 uc |  226.240 uc |  226.010 uc |
| memx_memchr             |  258.560 uc |   81.691 uc |  262.690 uc |   81.100 uc |
| memx_memchr_basic       |  255.280 uc |   94.201 uc |  261.980 uc |   94.397 uc |
| memx_memchr_libc        |  290.440 uc |   83.619 uc |  614.890 uc |  209.530 uc |
| memchr_memchr           |  220.390 uc |   65.749 uc |  226.130 uc |   64.531 uc |

|         `name`          | `bench:en`  | `bench:ja`  |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memcmp              |  253.010 uc |  334.350 uc |  419.660 uc |  372.390 uc |
| memx_memcmp             |  231.610 uc |  282.000 uc |  229.550 uc |  282.410 uc |
| memx_memcmp_basic       |  568.900 uc |  496.000 uc |  567.050 uc |  495.880 uc |
| memx_memcmp_libc        |  350.580 uc |  425.370 uc |  537.150 uc |  476.790 uc |

|         `name`          | `bench:en`  | `bench:ja`  |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memcpy              |  246.550 uc |  437.850 uc |  638.930 uc | 1145.800 uc |
| memx_memcpy             |  332.360 uc |  466.830 uc |  329.470 uc |  471.780 uc |
| memx_memcpy_basic       |  332.210 uc |  467.190 uc |  329.580 uc |  472.080 uc |
| memx_memcpy_libc        |  288.240 uc |  504.140 uc |  675.400 uc | 1214.300 uc |

|         `name`          | `bench:en`  | `bench:ja`  |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memeq               |  277.660 uc |  330.270 uc |  570.310 uc |  523.730 uc |
| memx_memeq              |  190.760 uc |  252.530 uc |  188.910 uc |  253.260 uc |
| memx_memeq_basic        |  569.100 uc |  494.500 uc |  568.900 uc |  494.340 uc |
| memx_memeq_libc         |  338.200 uc |  404.930 uc |  604.540 uc |  658.000 uc |

|         `name`          | `bench:en`  | `bench:ja`  |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memmem              |  519.260 uc |  459.990 uc |  519.170 uc |  452.600 uc |
| memx_memmem             |  117.870 uc |  118.820 uc |  116.980 uc |  117.240 uc |
| memx_memmem_basic       |  114.770 uc |  114.570 uc |  115.440 uc |  117.770 uc |
| memx_memmem_libc        |  132.730 uc |  121.720 uc |  220.420 uc |  263.270 uc |
| memchr_memmem           |  195.140 uc |  223.020 uc |  187.820 uc |  227.010 uc |

|         `name`          | `bench:en`  | `bench:ja`  |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memset              |    1.695 uc |    3.686 uc |    1.481 uc |    3.314 uc |
| memx_memset             |    1.959 uc |    4.107 uc |    1.743 uc |    3.670 uc |
| memx_memset_basic       |    2.889 uc |    5.146 uc |    2.896 uc |    5.245 uc |
| memx_memset_libc        |    1.857 uc |    3.901 uc |    1.659 uc |    3.544 uc |


  2. i686:

|         `name`          | `bench:en`  | `bench:ja`  |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memchr              |   83.806 uc |   83.824 uc |   85.359 uc |   85.323 uc |
| memx_memchr             |  335.410 uc |  132.370 uc |  336.920 uc |  138.760 uc |
| memx_memchr_basic       |  302.390 uc |  131.960 uc |  339.340 uc |  139.240 uc |
| memx_memchr_libc        |  372.080 uc |  100.040 uc |  580.580 uc |  153.270 uc |
| memchr_memchr           |  488.860 uc |  194.320 uc |  533.760 uc |  199.280 uc |

|         `name`          | `bench:en`  | `bench:ja`  |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memcmp              |  262.970 uc |  365.830 uc |  562.350 uc |  526.390 uc |
| memx_memcmp             |  308.270 uc |  416.620 uc |  320.760 uc |  444.170 uc |
| memx_memcmp_basic       |  610.440 uc |  842.000 uc |  621.930 uc |  863.560 uc |
| memx_memcmp_libc        |  402.930 uc |  529.090 uc |  646.850 uc |  728.580 uc |

|         `name`          | `bench:en`  | `bench:ja`  |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memcpy              |  323.310 uc |  586.280 uc |  418.100 uc |  745.570 uc |
| memx_memcpy             |  435.160 uc |  830.570 uc |  363.750 uc |  810.380 uc |
| memx_memcpy_basic       |  436.470 uc |  831.460 uc |  363.070 uc |  809.770 uc |
| memx_memcpy_libc        |  389.430 uc |  741.410 uc |  494.500 uc |  890.600 uc |

|         `name`          | `bench:en`  | `bench:ja`  |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memeq               |  285.640 uc |  350.920 uc |  632.690 uc |  657.960 uc |
| memx_memeq              |  289.600 uc |  383.810 uc |  307.630 uc |  419.700 uc |
| memx_memeq_basic        |  587.200 uc |  525.210 uc |  578.270 uc |  531.900 uc |
| memx_memeq_libc         |  367.890 uc |  482.970 uc |  720.300 uc |  857.800 uc |

|         `name`          | `bench:en`  | `bench:ja`  |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memmem              |  694.170 uc |  553.590 uc |  719.230 uc |  592.090 uc |
| memx_memmem             |  138.060 uc |  141.380 uc |  139.860 uc |  140.870 uc |
| memx_memmem_basic       |  138.450 uc |  141.520 uc |  140.610 uc |  139.890 uc |
| memx_memmem_libc        |  183.220 uc |  170.680 uc |  246.280 uc |  257.000 uc |
| memchr_memmem           |  470.810 uc |  468.150 uc |  497.870 uc |  510.480 uc |

|         `name`          | `bench:en`  | `bench:ja`  |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memset              |    1.355 uc |    2.755 uc |    2.510 uc |    4.622 uc |
| memx_memset             |    1.498 uc |    3.117 uc |    2.618 uc |    4.808 uc |
| memx_memset_basic       |    2.922 uc |    5.265 uc |    2.980 uc |    5.367 uc |
| memx_memset_libc        |    1.398 uc |    2.926 uc |    2.651 uc |    4.812 uc |


  3. armv7-linux-androideabi:

|         `name`          | `andro:en`  | `andro:ja`  |
|:------------------------|------------:|------------:|
| std_memchr              |  783.860 uc |  876.260 uc |
| memx_memchr             |  923.360 uc |  550.750 uc |
| memx_memchr_basic       |  910.230 uc |  551.400 uc |
| memx_memchr_libc        | 2256.000 uc | 1103.600 uc |
| memchr_memchr           | 1379.000 uc |  611.320 uc |

|         `name`          | `andro:en`  | `andro:ja`  |
|:------------------------|------------:|------------:|
| std_memcmp              | 2533.200 uc | 3178.400 uc |
| memx_memcmp             | 2137.500 uc | 2185.600 uc |
| memx_memcmp_basic       | 2136.800 uc | 2187.200 uc |
| memx_memcmp_libc        | 2396.000 uc | 3217.200 uc |

|         `name`          | `andro:en`  | `andro:ja`  |
|:------------------------|------------:|------------:|
| std_memcpy              | 2685.300 uc | 3121.200 uc |
| memx_memcpy             | 1171.100 uc | 1170.100 uc |
| memx_memcpy_basic       | 1142.800 uc | 1144.200 uc |
| memx_memcpy_libc        | 2198.700 uc | 2197.300 uc |

|         `name`          | `andro:en`  | `andro:ja`  |
|:------------------------|------------:|------------:|
| std_memeq               | 2762.700 uc | 3116.400 uc |
| memx_memeq              | 2756.600 uc | 2638.300 uc |
| memx_memeq_basic        | 2616.600 uc | 2609.300 uc |
| memx_memeq_libc         | 2298.500 uc | 3123.100 uc |

|         `name`          | `andro:en`  | `andro:ja`  |
|:------------------------|------------:|------------:|
| std_memmem              | 3308.000 uc | 2657.800 uc |
| memx_memmem             |  590.420 uc |  586.190 uc |
| memx_memmem_basic       |  588.150 uc |  586.600 uc |
| memx_memmem_libc        | 1987.000 uc | 1985.200 uc |
| memchr_memmem           | 1918.800 uc | 1936.700 uc |

|         `name`          | `andro:en`  | `andro:ja`  |
|:------------------------|------------:|------------:|
| std_memset              |   84.693 uc |  153.710 uc |
| memx_memset             |   45.985 uc |   82.639 uc |
| memx_memset_basic       |   45.744 uc |   82.058 uc |
| memx_memset_libc        |   60.770 uc |  113.470 uc |

- std is std::str::find()
- `us` is micro seconds
- `:en` is english haystack.
- `:ja` is japanese haystack.
- `musl` is x86_64-unknown-linux-musl
- bench on intel Q6600 @ 2.40GHz


## Changelogs

[This crate's changelog here.](https://github.com/aki-akaguma/memx/blob/main/CHANGELOG.md)


## References
