## Benchmark results

- compile by rustc 1.69.0 (84c898d65 2023-04-16)

  0. x86_64-unknown-linux- @i5-4570:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memchr_qpl          |   15.988 µs |   30.633 µs |   15.899 µs |   30.648 µs |
| memx_memchr_qpl         |    7.428 µs |    9.443 µs |    7.271 µs |    9.279 µs |
| memx_memchr_q_basic     |   15.061 µs |   21.853 µs |   14.838 µs |   21.567 µs |
| memx_memchr_q_sse2      |    8.543 µs |   11.027 µs |    8.755 µs |   11.290 µs |
| memx_memchr_q_avx2      |    7.353 µs |    9.227 µs |    7.234 µs |    9.123 µs |

  1. x86_64-unknown-linux- @Q6600:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memchr_qpl          |   48.194 µs |   82.463 µs |   48.653 µs |   82.656 µs |
| memx_memchr_qpl         |   30.626 µs |   37.733 µs |   30.228 µs |   39.564 µs |
| memx_memchr_q_basic     |   33.740 µs |   49.458 µs |   33.404 µs |   48.803 µs |
| memx_memchr_q_sse2      |   29.169 µs |   36.408 µs |   28.915 µs |   36.477 µs |

  2. i686-unknown-linux- @Q6600:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memchr_qpl          |   50.510 µs |   84.396 µs |   50.364 µs |   84.050 µs |
| memx_memchr_qpl         |   39.733 µs |   46.940 µs |   38.860 µs |   47.266 µs |
| memx_memchr_q_basic     |   50.304 µs |   79.800 µs |   51.717 µs |   81.275 µs |
| memx_memchr_q_sse2      |   38.752 µs |   46.465 µs |   38.082 µs |   45.462 µs |

  3. i586-unknown-linux- @Q6600:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memchr_qpl          |   50.343 µs |   83.896 µs |   50.659 µs |   84.759 µs |
| memx_memchr_qpl         |   38.813 µs |   46.186 µs |   40.278 µs |   48.547 µs |
| memx_memchr_q_basic     |   50.283 µs |   79.170 µs |   51.242 µs |   79.662 µs |

- `µs` is micro seconds
- `:en` is english haystack or short length.
- `:ja` is japanese haystack or long length.
- `gnu` is x86_64-unknown-linux-gnu
- `musl` is x86_64-unknown-linux-musl
- @i5-4570: bench on intel i5-4570 @ 3.2GHz
- @Q6600: bench on intel Q6600 @ 2.40GHz or armv7 1.5GHz
