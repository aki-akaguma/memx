## Benchmark results

- compile by rustc 1.69.0 (84c898d65 2023-04-16)

  0. x86_64-unknown-linux- @i5-4570:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memnechr_dbl        |   40.175 µs |  109.590 µs |   39.441 µs |  108.760 µs |
| memx_memnechr_dbl       |    6.247 µs |    7.294 µs |    6.251 µs |    7.133 µs |
| memx_memnechr_w_basic   |    9.820 µs |   13.310 µs |    9.822 µs |   13.337 µs |
| memx_memnechr_w_sse2    |    6.551 µs |    7.709 µs |    6.558 µs |    7.754 µs |
| memx_memnechr_w_avx2    |    6.253 µs |    7.258 µs |    6.267 µs |    7.180 µs |

  1. x86_64-unknown-linux- @Q6600:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memnechr_dbl        |   60.703 µs |   82.125 µs |   60.509 µs |   82.598 µs |
| memx_memnechr_dbl       |   20.098 µs |   22.308 µs |   19.791 µs |   22.486 µs |
| memx_memnechr_w_basic   |   23.796 µs |   32.600 µs |   23.190 µs |   33.716 µs |
| memx_memnechr_w_sse2    |   16.633 µs |   18.686 µs |   15.840 µs |   19.449 µs |

  2. i686-unknown-linux- @Q6600:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memnechr_dbl        |   30.770 µs |   47.215 µs |   30.860 µs |   46.580 µs |
| memx_memnechr_dbl       |   28.356 µs |   30.724 µs |   29.382 µs |   31.590 µs |
| memx_memnechr_w_basic   |   35.187 µs |   54.514 µs |   33.795 µs |   51.752 µs |
| memx_memnechr_w_sse2    |   24.041 µs |   26.225 µs |   24.691 µs |   26.394 µs |

  3. i586-unknown-linux- @Q6600:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memnechr_dbl        |   30.637 µs |   47.764 µs |   31.474 µs |   48.000 µs |
| memx_memnechr_dbl       |   28.813 µs |   31.402 µs |   29.116 µs |   33.228 µs |
| memx_memnechr_w_basic   |   34.773 µs |   53.274 µs |   39.779 µs |   59.040 µs |

- `µs` is micro seconds
- `:en` is english haystack or short length.
- `:ja` is japanese haystack or long length.
- `gnu` is x86_64-unknown-linux-gnu
- `musl` is x86_64-unknown-linux-musl
- @i5-4570: bench on intel i5-4570 @ 3.2GHz
- @Q6600: bench on intel Q6600 @ 2.40GHz or armv7 1.5GHz
