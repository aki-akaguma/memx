## Benchmark results

- compile by rustc 1.69.0 (84c898d65 2023-04-16)

  0. x86_64-unknown-linux- @i5-4570:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memrchr_tpl         |   11.972 µs |   23.075 µs |   11.653 µs |   23.481 µs |
| memchr_memrchr_tpl      |    8.551 µs |    8.990 µs |    8.534 µs |    9.022 µs |
| memx_memrchr_tpl        |    7.052 µs |    7.321 µs |    7.117 µs |    7.576 µs |
| memx_memrchr_t_basic    |   11.413 µs |   15.821 µs |   11.561 µs |   15.897 µs |
| memx_memrchr_t_sse2     |    6.761 µs |    8.281 µs |    7.010 µs |    8.439 µs |
| memx_memrchr_t_avx2     |    7.035 µs |    7.314 µs |    6.962 µs |    7.469 µs |

  1. x86_64-unknown-linux- @Q6600:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memrchr_tpl         |   42.203 µs |   67.399 µs |   42.311 µs |   67.429 µs |
| memchr_memrchr_tpl      |   26.792 µs |   32.419 µs |   26.591 µs |   30.677 µs |
| memx_memrchr_tpl        |   26.283 µs |   31.734 µs |   26.666 µs |   30.816 µs |
| memx_memrchr_t_basic    |   28.226 µs |   40.844 µs |   35.650 µs |   40.232 µs |
| memx_memrchr_t_sse2     |   24.298 µs |   29.765 µs |   24.587 µs |   29.789 µs |

  2. i686-unknown-linux- @Q6600:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memrchr_tpl         |   40.421 µs |   66.698 µs |   42.104 µs |   67.732 µs |
| memchr_memrchr_tpl      |   51.711 µs |   70.402 µs |   52.003 µs |   70.266 µs |
| memx_memrchr_tpl        |   37.523 µs |   41.335 µs |   37.804 µs |   42.529 µs |
| memx_memrchr_t_basic    |   46.291 µs |   69.886 µs |   50.409 µs |   72.268 µs |
| memx_memrchr_t_sse2     |   35.211 µs |   39.202 µs |   36.363 µs |   41.349 µs |

  3. i586-unknown-linux- @Q6600:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memrchr_tpl         |   41.092 µs |   67.198 µs |   42.372 µs |   69.058 µs |
| memchr_memrchr_tpl      |   51.314 µs |   71.437 µs |   53.968 µs |   72.180 µs |
| memx_memrchr_tpl        |   37.088 µs |   41.042 µs |   38.215 µs |   43.133 µs |
| memx_memrchr_t_basic    |   46.319 µs |   69.546 µs |   52.746 µs |   78.055 µs |

- `µs` is micro seconds
- `:en` is english haystack or short length.
- `:ja` is japanese haystack or long length.
- `gnu` is x86_64-unknown-linux-gnu
- `musl` is x86_64-unknown-linux-musl
- @i5-4570: bench on intel i5-4570 @ 3.2GHz
- @Q6600: bench on intel Q6600 @ 2.40GHz or armv7 1.5GHz
