## Benchmark results

  1. x86_64-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memeq               |  167.850 us |  168.090 us |  257.350 us |  343.470 us |
| memx_memeq              |  110.980 us |  105.270 us |  109.690 us |  104.320 us |
| memx_memeq_basic        |  112.690 us |  105.210 us |  111.390 us |  104.260 us |
| memx_memeq_libc         |  201.790 us |  188.950 us |  289.160 us |  394.210 us |

  2. i686-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memeq               |  169.070 us |  197.240 us |  359.320 us |  519.610 us |
| memx_memeq              |  163.060 us |  151.060 us |  167.030 us |  153.910 us |
| memx_memeq_basic        |  161.680 us |  151.100 us |  166.060 us |  153.910 us |
| memx_memeq_libc         |  220.290 us |  236.890 us |  424.980 us |  580.920 us |

  3. i586-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memeq               |  168.670 us |  193.420 us |  336.970 us |  511.170 us |
| memx_memeq              |  193.980 us |  149.850 us |  163.520 us |  152.600 us |
| memx_memeq_basic        |  193.970 us |  149.930 us |  160.700 us |  152.800 us |
| memx_memeq_libc         |  206.490 us |  240.370 us |  416.780 us |  579.240 us |

  4. armv7-linux-androideabi:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memeq               |  944.360 us |  908.180 us | 1307.900 us | 1257.000 us |
| memx_memeq              |  504.520 us |  492.920 us |  620.440 us |  444.600 us |
| memx_memeq_basic        |  476.000 us |  515.120 us |  616.620 us |  444.550 us |
| memx_memeq_libc         | 1122.700 us | 1066.900 us |  966.330 us | 1307.600 us |


- compile by rustc 1.52.1 (9bc8c42bb 2021-05-09)
- `us` is micro seconds
- `:en` is english haystack.
- `:ja` is japanese haystack.
- `musl` is x86_64-unknown-linux-musl
- bench on intel Q6600 @ 2.40GHz or armv7 1.5GHz
