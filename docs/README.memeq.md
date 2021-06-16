## Benchmark results

  1. x86_64-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memeq               |  174.300 us |  173.050 us |  349.450 us |  368.890 us |
| libc_memeq              |  174.330 us |  172.600 us |  345.290 us |  365.230 us |
| memx_memeq              |  125.530 us |  119.870 us |  124.480 us |  119.100 us |
| memx_memeq_basic        |  125.800 us |  120.070 us |  124.150 us |  118.880 us |

  2. i686-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memeq               |  176.280 us |  197.800 us |  374.720 us |  517.150 us |
| libc_memeq              |  175.160 us |  196.360 us |  365.530 us |  517.270 us |
| memx_memeq              |  166.310 us |  159.390 us |  172.200 us |  160.770 us |
| memx_memeq_basic        |  157.430 us |  163.590 us |  168.580 us |  158.330 us |

  3. i586-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memeq               |  167.610 us |  195.070 us |  375.680 us |  517.930 us |
| libc_memeq              |  169.310 us |  194.880 us |  376.310 us |  517.700 us |
| memx_memeq              |  157.170 us |  164.520 us |  207.090 us |  160.710 us |
| memx_memeq_basic        |  165.150 us |  155.170 us |  174.560 us |  164.410 us |

  4. armv7-linux-androideabi:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memeq               | 1699.700 us | 1183.700 us | 1550.800 us | 1538.500 us |
| memx_memeq              |  621.950 us |  634.840 us |  660.690 us |  562.120 us |
| memx_memeq_basic        |  649.480 us |  626.200 us |  675.230 us |  567.540 us |


- compile by rustc 1.52.1 (9bc8c42bb 2021-05-09)
- `us` is micro seconds
- `:en` is english haystack.
- `:ja` is japanese haystack.
- `musl` is x86_64-unknown-linux-musl
- bench on intel Q6600 @ 2.40GHz or armv7 1.5GHz
