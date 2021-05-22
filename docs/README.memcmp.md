## Benchmark results

  1. x86_64-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memcmp              |  154.990 us |  199.260 us |  251.470 us |  344.360 us |
| memx_memcmp             |  129.400 us |  127.890 us |  127.700 us |  126.590 us |
| memx_memcmp_basic       |  128.620 us |  127.630 us |  127.630 us |  126.590 us |
| memx_memcmp_libc        |  211.900 us |  205.080 us |  306.940 us |  377.120 us |

  2. i686-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memcmp              |  161.060 us |  198.180 us |  328.080 us |  476.870 us |
| memx_memcmp             |  180.540 us |  206.110 us |  183.560 us |  210.210 us |
| memx_memcmp_basic       |  187.390 us |  211.130 us |  185.250 us |  212.000 us |
| memx_memcmp_libc        |  250.760 us |  254.710 us |  383.420 us |  551.120 us |

  3. i586-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memcmp              |  159.830 us |  201.990 us |  326.520 us |  477.070 us |
| memx_memcmp             |  188.600 us |  208.140 us |  181.900 us |  215.080 us |
| memx_memcmp_basic       |  187.830 us |  209.470 us |  181.080 us |  214.560 us |
| memx_memcmp_libc        |  237.820 us |  248.260 us |  392.710 us |  547.680 us |

  4. armv7-linux-androideabi:

|         `name`          |  `andr:en`  |  `andr:ja`  |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memcmp              | 1221.500 us | 1131.200 us | 1347.600 us | 1791.800 us |
| memx_memcmp             |  602.640 us |  682.080 us |  615.540 us |  652.830 us |
| memx_memcmp_basic       |  639.620 us |  709.870 us |  635.270 us |  694.450 us |
| memx_memcmp_libc        | 1491.900 us | 1436.800 us | 1416.500 us | 1886.400 us |


- compile by rustc 1.52.1 (9bc8c42bb 2021-05-09)
- `us` is micro seconds
- `:en` is english haystack.
- `:ja` is japanese haystack.
- `musl` is x86_64-unknown-linux-musl
- bench on intel Q6600 @ 2.40GHz or armv7 1.5GHz
