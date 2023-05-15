## Benchmark results

- compile by rustc 1.69.0 (84c898d65 2023-04-16)

  0. x86_64-unknown-linux- @i5-4570:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memrnechr_tpl       |   10.126 µs |   22.048 µs |   10.863 µs |   21.362 µs |
| memx_memrnechr_tpl      |    8.014 µs |    9.205 µs |    8.030 µs |    9.165 µs |
| memx_memrnechr_t_basic  |   12.402 µs |   17.391 µs |   12.077 µs |   17.170 µs |
| memx_memrnechr_t_sse2   |    8.149 µs |   10.064 µs |    8.205 µs |   10.010 µs |
| memx_memnechr_t_avx2    |    8.052 µs |    9.197 µs |    8.005 µs |    9.127 µs |

  1. x86_64-unknown-linux- @Q6600:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memrnechr_tpl       |   34.285 µs |   55.547 µs |   34.318 µs |   55.736 µs |
| memx_memrnechr_tpl      |   24.528 µs |   30.680 µs |   26.466 µs |   32.140 µs |
| memx_memrnechr_t_basic  |   33.306 µs |   45.913 µs |   32.525 µs |   46.717 µs |
| memx_memrnechr_t_sse2   |   23.249 µs |   27.180 µs |   23.528 µs |   27.012 µs |

  2. i686-unknown-linux- @Q6600:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memrnechr_tpl       |   37.942 µs |   64.591 µs |   39.988 µs |   61.067 µs |
| memx_memrnechr_tpl      |   32.993 µs |   40.057 µs |   34.629 µs |   41.341 µs |
| memx_memrnechr_t_basic  |   44.229 µs |   67.952 µs |   46.197 µs |   72.974 µs |
| memx_memrnechr_t_sse2   |   29.541 µs |   34.876 µs |   30.165 µs |   35.826 µs |

  3. i586-unknown-linux- @Q6600:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memrnechr_tpl       |   36.979 µs |   58.434 µs |   41.699 µs |   63.738 µs |
| memx_memrnechr_tpl      |   32.737 µs |   40.616 µs |   33.149 µs |   40.084 µs |
| memx_memrnechr_t_basic  |   42.986 µs |   66.475 µs |   46.847 µs |   71.898 µs |

- `µs` is micro seconds
- `:en` is english haystack or short length.
- `:ja` is japanese haystack or long length.
- `gnu` is x86_64-unknown-linux-gnu
- `musl` is x86_64-unknown-linux-musl
- @i5-4570: bench on intel i5-4570 @ 3.2GHz
- @Q6600: bench on intel Q6600 @ 2.40GHz or armv7 1.5GHz
