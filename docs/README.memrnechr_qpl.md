## Benchmark results

- compile by rustc 1.69.0 (84c898d65 2023-04-16)

  0. x86_64-unknown-linux- @i5-4570:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memrnechr_qpl       |   13.732 µs |   25.659 µs |   12.044 µs |   23.210 µs |
| memx_memrnechr_qpl      |    8.892 µs |   10.559 µs |    8.852 µs |   10.499 µs |
| memx_memrnechr_q_basic  |   14.772 µs |   21.369 µs |   14.995 µs |   21.513 µs |
| memx_memrnechr_q_sse2   |    9.546 µs |   12.062 µs |    9.518 µs |   12.028 µs |
| memx_memnechr_q_avx2    |    8.974 µs |   10.557 µs |    8.919 µs |   10.571 µs |

  1. x86_64-unknown-linux- @Q6600:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memrnechr_qpl       |   38.044 µs |   62.271 µs |   38.184 µs |   62.574 µs |
| memx_memrnechr_qpl      |   29.394 µs |   37.938 µs |   29.360 µs |   36.948 µs |
| memx_memrnechr_q_basic  |   37.894 µs |   55.476 µs |   36.748 µs |   55.308 µs |
| memx_memrnechr_q_sse2   |   27.075 µs |   32.388 µs |   27.816 µs |   32.944 µs |

  2. i686-unknown-linux- @Q6600:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memrnechr_qpl       |   43.512 µs |   69.744 µs |   45.353 µs |   70.682 µs |
| memx_memrnechr_qpl      |   35.668 µs |   44.549 µs |   35.690 µs |   45.007 µs |
| memx_memrnechr_q_basic  |   56.269 µs |   86.318 µs |   54.941 µs |   84.924 µs |
| memx_memrnechr_q_sse2   |   34.219 µs |   40.119 µs |   33.478 µs |   40.104 µs |

  3. i586-unknown-linux- @Q6600:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memrnechr_qpl       |   43.532 µs |   68.436 µs |   44.532 µs |   69.537 µs |
| memx_memrnechr_qpl      |   36.930 µs |   45.273 µs |   37.004 µs |   45.910 µs |
| memx_memrnechr_q_basic  |   54.586 µs |   84.045 µs |   53.985 µs |   84.921 µs |

- `µs` is micro seconds
- `:en` is english haystack or short length.
- `:ja` is japanese haystack or long length.
- `gnu` is x86_64-unknown-linux-gnu
- `musl` is x86_64-unknown-linux-musl
- @i5-4570: bench on intel i5-4570 @ 3.2GHz
- @Q6600: bench on intel Q6600 @ 2.40GHz or armv7 1.5GHz
