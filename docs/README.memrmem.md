## Benchmark results

  1. x86_64-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memrmem             |  541.760 us |  763.640 us |  537.580 us |  747.780 us |
| libc_memrmem            | 1692.700 us | 1762.000 us |  741.910 us |  789.600 us |
| memchr_memrmem          |  658.270 us |  733.670 us |  653.340 us |  669.100 us |
| memx_memrmem            |  120.790 us |  119.090 us |  120.330 us |  124.460 us |
| memx_memrmem_basic      |  112.740 us |  113.380 us |  111.980 us |  112.850 us |
| memx_memrmem_sse2       |  118.770 us |  116.950 us |  119.010 us |  123.590 us |

  2. i686-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memrmem             |  727.800 us |  871.760 us |  777.920 us |  864.800 us |
| memchr_memrmem          |  819.390 us |  808.960 us |  826.100 us |  804.240 us |
| memx_memrmem            |  128.100 us |  129.370 us |  138.170 us |  138.530 us |
| memx_memrmem_basic      |  126.750 us |  119.320 us |  132.470 us |  129.270 us |
| memx_memrmem_sse2       |  124.140 us |  125.910 us |  131.990 us |  132.920 us |

  3. i586-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memrmem             |  733.460 us |  864.620 us |  764.410 us |  803.260 us |
| memchr_memrmem          |  868.500 us | 1279.200 us |  866.660 us | 1270.500 us |
| memx_memrmem            |  133.180 us |  123.310 us |  133.380 us |  127.490 us |
| memx_memrmem_basic      |  136.670 us |  122.110 us |  135.490 us |  127.900 us |

  4. armv7-linux-androideabi:

|         `name`          |  `andr:en`  |  `andr:ja`  |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memrmem             | 5735.900 us | 3668.800 us | 3054.100 us | 2006.900 us |
| memchr_memrmem          | 4526.400 us | 4486.400 us | 3291.500 us | 3245.300 us |
| memx_memrmem            | 2013.100 us | 2139.300 us |  544.170 us |  534.090 us |
| memx_memrmem_basic      | 2005.800 us | 2027.800 us |  556.750 us |  542.600 us |


- compile by rustc 1.52.1 (9bc8c42bb 2021-05-09)
- `us` is micro seconds
- `:en` is english haystack.
- `:ja` is japanese haystack.
- `musl` is x86_64-unknown-linux-musl
- bench on intel Q6600 @ 2.40GHz or armv7 1.5GHz
