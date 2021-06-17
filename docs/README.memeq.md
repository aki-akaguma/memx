## Benchmark results

  1. x86_64-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memeq               |  167.020 us |  164.290 us |  337.640 us |  356.560 us |
| libc_memeq              |  166.310 us |  164.670 us |  342.080 us |  360.590 us |
| memx_memeq              |  118.680 us |  112.200 us |  114.830 us |  111.070 us |
| memx_memeq_basic        |  118.630 us |  112.100 us |  114.910 us |  111.080 us |

  2. i686-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memeq               |  170.950 us |  266.460 us |  362.170 us |  518.300 us |
| libc_memeq              |  168.650 us |  198.740 us |  362.050 us |  518.210 us |
| memx_memeq              |  165.430 us |  156.860 us |  170.620 us |  175.800 us |
| memx_memeq_basic        |  157.570 us |  154.860 us |  170.940 us |  159.230 us |

  3. i586-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memeq               |  168.060 us |  250.150 us |  336.020 us |  507.350 us |
| libc_memeq              |  168.020 us |  193.740 us |  342.610 us |  511.750 us |
| memx_memeq              |  164.620 us |  153.500 us |  170.880 us |  156.500 us |
| memx_memeq_basic        |  168.640 us |  162.830 us |  167.900 us |  157.630 us |

  4. armv7-linux-androideabi:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memeq               | 1263.000 us | 1236.600 us | 1317.700 us | 1721.000 us |
| memx_memeq              |  779.600 us |  752.600 us |  658.390 us |  604.610 us |
| memx_memeq_basic        |  755.180 us |  746.410 us |  694.570 us |  598.540 us |


- compile by rustc 1.52.1 (9bc8c42bb 2021-05-09)
- `us` is micro seconds
- `:en` is english haystack.
- `:ja` is japanese haystack.
- `musl` is x86_64-unknown-linux-musl
- bench on intel Q6600 @ 2.40GHz or armv7 1.5GHz
