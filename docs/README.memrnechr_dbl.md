## Benchmark results

- compile by rustc 1.69.0 (84c898d65 2023-04-16)

  0. x86_64-unknown-linux- @i5-4570:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memrnechr_dbl       |   35.006 µs |  106.330 µs |   31.145 µs |  102.330 µs |
| memx_memrnechr_dbl      |    7.073 µs |    7.575 µs |    7.095 µs |    7.606 µs |
| memx_memrnechr_w_basic  |    9.796 µs |   13.367 µs |    9.806 µs |   13.293 µs |
| memx_memrnechr_w_sse2   |    6.897 µs |    8.292 µs |    6.920 µs |    8.021 µs |
| memx_memnechr_w_avx2    |    7.176 µs |    7.705 µs |    7.055 µs |    7.519 µs |

  1. x86_64-unknown-linux- @Q6600:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memrnechr_dbl       |   57.536 µs |   81.803 µs |   57.270 µs |   80.968 µs |
| memx_memrnechr_dbl      |   18.839 µs |   24.006 µs |   18.677 µs |   22.570 µs |
| memx_memrnechr_w_basic  |   25.454 µs |   35.193 µs |   24.298 µs |   33.745 µs |
| memx_memrnechr_w_sse2   |   16.417 µs |   20.070 µs |   16.678 µs |   19.126 µs |

  2. i686-unknown-linux- @Q6600:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memrnechr_dbl       |   33.228 µs |   50.263 µs |   33.423 µs |   50.916 µs |
| memx_memrnechr_dbl      |   24.671 µs |   28.924 µs |   26.692 µs |   30.746 µs |
| memx_memrnechr_w_basic  |   36.748 µs |   53.368 µs |   33.623 µs |   50.308 µs |
| memx_memrnechr_w_sse2   |   22.258 µs |   25.050 µs |   22.220 µs |   25.233 µs |

  3. i586-unknown-linux- @Q6600:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memrnechr_dbl       |   33.179 µs |   50.127 µs |   33.709 µs |   51.091 µs |
| memx_memrnechr_dbl      |   25.809 µs |   30.472 µs |   26.868 µs |   30.655 µs |
| memx_memrnechr_w_basic  |   33.971 µs |   51.787 µs |   32.883 µs |   50.416 µs |

- `µs` is micro seconds
- `:en` is english haystack or short length.
- `:ja` is japanese haystack or long length.
- `gnu` is x86_64-unknown-linux-gnu
- `musl` is x86_64-unknown-linux-musl
- @i5-4570: bench on intel i5-4570 @ 3.2GHz
- @Q6600: bench on intel Q6600 @ 2.40GHz or armv7 1.5GHz
