## Benchmark results

- compile by rustc 1.69.0 (84c898d65 2023-04-16)

  0. x86_64-unknown-linux- @i5-4570:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memrchr_qpl         |   16.597 µs |   30.526 µs |   17.118 µs |   31.291 µs |
| memx_memrchr_qpl        |    8.195 µs |    9.970 µs |    8.111 µs |    9.897 µs |
| memx_memrchr_q_basic    |   15.506 µs |   22.859 µs |   15.546 µs |   22.843 µs |
| memx_memrchr_q_sse2     |    9.476 µs |   11.915 µs |    9.626 µs |   12.003 µs |
| memx_memrchr_q_avx2     |    8.022 µs |    9.720 µs |    7.917 µs |    9.665 µs |

  1. x86_64-unknown-linux- @Q6600:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memrchr_qpl         |   46.737 µs |   82.002 µs |   47.376 µs |   82.507 µs |
| memx_memrchr_qpl        |   32.992 µs |   42.375 µs |   33.956 µs |   42.284 µs |
| memx_memrchr_q_basic    |   37.519 µs |   57.185 µs |   36.952 µs |   58.894 µs |
| memx_memrchr_q_sse2     |   32.916 µs |   41.010 µs |   33.032 µs |   40.939 µs |

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
