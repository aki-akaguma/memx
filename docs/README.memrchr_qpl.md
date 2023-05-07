## Benchmark results

- compile by rustc 1.69.0 (84c898d65 2023-04-16)

  0. x86_64-unknown-linux- @i5-4570:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memrchr_qpl         |   16.382 µs |   30.645 µs |   16.919 µs |   31.162 µs |
| memx_memrchr_qpl        |    8.336 µs |    9.883 µs |    8.246 µs |    9.898 µs |
| memx_memrchr_q_basic    |   15.884 µs |   23.157 µs |   15.714 µs |   23.040 µs |
| memx_memrchr_q_sse2     |    9.478 µs |   11.898 µs |    9.570 µs |   11.917 µs |
| memx_memrchr_q_avx2     |    8.098 µs |    9.648 µs |    8.041 µs |    9.759 µs |

  1. x86_64-unknown-linux- @Q6600:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memrchr_qpl         |   46.695 µs |   82.123 µs |   47.600 µs |   82.755 µs |
| memx_memrchr_qpl        |   32.819 µs |   42.356 µs |   33.962 µs |   42.278 µs |
| memx_memrchr_q_basic    |   35.561 µs |   56.291 µs |   35.948 µs |   56.573 µs |
| memx_memrchr_q_sse2     |   33.078 µs |   41.048 µs |   33.028 µs |   40.943 µs |

  2. i686-unknown-linux- @Q6600:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memrchr_qpl         |   50.116 µs |   84.540 µs |   49.494 µs |   85.052 µs |
| memx_memrchr_qpl        |   43.180 µs |   51.017 µs |   42.808 µs |   52.443 µs |
| memx_memrchr_q_basic    |   56.434 µs |   91.723 µs |   54.103 µs |   89.245 µs |
| memx_memrchr_q_sse2     |   40.423 µs |   48.100 µs |   41.253 µs |   50.604 µs |

  3. i586-unknown-linux- @Q6600:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memrchr_qpl         |   48.714 µs |   83.659 µs |   48.625 µs |   83.660 µs |
| memx_memrchr_qpl        |   41.972 µs |   51.678 µs |   41.792 µs |   52.051 µs |
| memx_memrchr_q_basic    |   58.156 µs |   93.917 µs |   56.931 µs |   90.043 µs |

- `µs` is micro seconds
- `:en` is english haystack or short length.
- `:ja` is japanese haystack or long length.
- `gnu` is x86_64-unknown-linux-gnu
- `musl` is x86_64-unknown-linux-musl
- @i5-4570: bench on intel i5-4570 @ 3.2GHz
- @Q6600: bench on intel Q6600 @ 2.40GHz or armv7 1.5GHz
