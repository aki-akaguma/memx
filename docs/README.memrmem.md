## Benchmark results

  1. x86_64-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memrmem             |  625.120 us |  876.830 us |  619.010 us |  905.570 us |
| memx_memrmem            |  141.450 us |  142.830 us |  146.570 us |  146.870 us |
| memx_memrmem_basic      |  127.710 us |  127.740 us |  125.970 us |  126.270 us |
| memchr_memrmem          |  758.220 us |  752.340 us |  753.670 us |  757.700 us |

  2. i686-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memrmem             |  879.010 us | 1046.100 us |  931.450 us | 1070.100 us |
| memx_memrmem            |  150.330 us |  151.250 us |  159.280 us |  159.400 us |
| memx_memrmem_basic      |  131.530 us |  131.430 us |  132.860 us |  133.040 us |
| memchr_memrmem          |  939.210 us |  957.470 us |  961.980 us |  940.830 us |

  3. i586-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memrmem             |  890.920 us |  955.720 us |  930.810 us | 1000.700 us |
| memx_memrmem            |  131.650 us |  135.390 us |  132.100 us |  131.950 us |
| memx_memrmem_basic      |  131.410 us |  133.520 us |  133.850 us |  133.230 us |
| memchr_memrmem          | 1042.000 us | 1596.000 us | 1041.400 us | 1562.800 us |

  4. armv7-linux-androideabi:

|         `name`          |  `andr:en`  |  `andr:ja`  |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|


- compile by rustc 1.52.1 (9bc8c42bb 2021-05-09)
- `us` is micro seconds
- `:en` is english haystack.
- `:ja` is japanese haystack.
- `musl` is x86_64-unknown-linux-musl
- bench on intel Q6600 @ 2.40GHz or armv7 1.5GHz
