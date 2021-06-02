## Benchmark results

  1. x86_64-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memrchr             |  514.700 us |  277.620 us |  525.830 us |  237.260 us |
| memx_memrchr            |  263.110 us |  107.010 us |  259.810 us |   83.782 us |
| memx_memrchr_basic      |  330.900 us |  102.660 us |  263.330 us |  103.200 us |
| memx_memrchr_libc       |  266.860 us |   84.854 us |  608.250 us |  288.650 us |
| memchr_memrchr          |  331.550 us |   70.906 us |  224.240 us |   71.196 us |

  2. i686-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memrchr             |  394.770 us |  201.620 us |  400.980 us |  201.450 us |
| memx_memrchr            |  297.450 us |   87.943 us |  299.870 us |   86.482 us |
| memx_memrchr_basic      |  290.270 us |  126.260 us |  308.190 us |  130.590 us |
| memx_memrchr_libc       |  334.040 us |  102.950 us |  654.960 us |  235.820 us |
| memchr_memrchr          |  575.510 us |  220.570 us |  558.810 us |  213.730 us |

  3. i586-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memrchr             |  416.480 us |  198.590 us |  387.710 us |  194.460 us |
| memx_memrchr            |  286.790 us |  126.610 us |  316.880 us |  136.680 us |
| memx_memrchr_basic      |  297.630 us |  127.590 us |  321.880 us |  138.910 us |
| memx_memrchr_libc       |  326.570 us |  101.990 us |  684.830 us |  243.660 us |
| memchr_memrchr          |  514.320 us |  203.430 us |  531.580 us |  200.630 us |

  4. armv7-linux-androideabi:

|         `name`          |  `andr:en`  |  `andr:ja`  |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|

- compile by rustc 1.52.1 (9bc8c42bb 2021-05-09)
- `us` is micro seconds
- `:en` is english haystack.
- `:ja` is japanese haystack.
- `musl` is x86_64-unknown-linux-musl
- bench on intel Q6600 @ 2.40GHz or armv7 1.5GHz
