## Benchmark results

- compile by rustc 1.69.0 (84c898d65 2023-04-16)

  0. x86_64-unknown-linux- @i5-4570:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memchr_qpl          |   15.833 µs |   31.529 µs |   16.479 µs |   30.140 µs |
| memx_memchr_qpl         |    7.368 µs |    9.302 µs |    7.176 µs |    9.055 µs |
| memx_memchr_q_basic     |   15.655 µs |   22.270 µs |   15.347 µs |   22.016 µs |
| memx_memchr_q_sse2      |    8.481 µs |   11.225 µs |    8.739 µs |   11.362 µs |
| memx_memchr_q_avx2      |    7.202 µs |    9.196 µs |    7.347 µs |    8.912 µs |

  1. x86_64-unknown-linux- @Q6600:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memchr_qpl          |   48.756 µs |   83.279 µs |   49.499 µs |   83.325 µs |
| memx_memchr_qpl         |   30.562 µs |   40.171 µs |   30.467 µs |   40.155 µs |
| memx_memchr_q_basic     |   35.825 µs |   52.808 µs |   35.493 µs |   52.768 µs |
| memx_memchr_q_sse2      |   29.564 µs |   36.789 µs |   29.091 µs |   37.242 µs |

  2. i686-unknown-linux- @Q6600:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memchr_qpl          |   50.804 µs |   85.372 µs |   49.569 µs |   86.194 µs |
| memx_memchr_qpl         |   39.645 µs |   48.074 µs |   38.267 µs |   46.799 µs |
| memx_memchr_q_basic     |   50.765 µs |   81.422 µs |   50.282 µs |   81.096 µs |
| memx_memchr_q_sse2      |   38.946 µs |   45.156 µs |   37.174 µs |   44.152 µs |

  3. i586-unknown-linux- @Q6600:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memchr_qpl          |   50.627 µs |   84.791 µs |   51.030 µs |   85.685 µs |
| memx_memchr_qpl         |   40.232 µs |   48.393 µs |   38.826 µs |   47.389 µs |
| memx_memchr_q_basic     |   52.681 µs |   80.242 µs |   54.969 µs |   84.450 µs |

- `µs` is micro seconds
- `:en` is english haystack or short length.
- `:ja` is japanese haystack or long length.
- `gnu` is x86_64-unknown-linux-gnu
- `musl` is x86_64-unknown-linux-musl
- @i5-4570: bench on intel i5-4570 @ 3.2GHz
- @Q6600: bench on intel Q6600 @ 2.40GHz or armv7 1.5GHz
