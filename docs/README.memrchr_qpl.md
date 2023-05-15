## Benchmark results

- compile by rustc 1.69.0 (84c898d65 2023-04-16)

  0. x86_64-unknown-linux- @i5-4570:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memrchr_qpl         |   16.332 µs |   30.775 µs |   16.445 µs |   31.005 µs |
| memx_memrchr_qpl        |    8.138 µs |    9.998 µs |    8.009 µs |    9.917 µs |
| memx_memrchr_q_basic    |   15.374 µs |   22.754 µs |   15.494 µs |   22.813 µs |
| memx_memrchr_q_sse2     |    9.488 µs |   12.023 µs |    9.541 µs |   12.075 µs |
| memx_memrchr_q_avx2     |    7.932 µs |    9.758 µs |    7.773 µs |    9.729 µs |

  1. x86_64-unknown-linux- @Q6600:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memrchr_qpl         |   46.436 µs |   81.384 µs |   47.044 µs |   81.458 µs |
| memx_memrchr_qpl        |   32.799 µs |   40.383 µs |   33.540 µs |   40.763 µs |
| memx_memrchr_q_basic    |   34.997 µs |   54.103 µs |   35.024 µs |   53.817 µs |
| memx_memrchr_q_sse2     |   31.364 µs |   39.226 µs |   31.508 µs |   39.520 µs |

  2. i686-unknown-linux- @Q6600:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memrchr_qpl         |   49.251 µs |   83.259 µs |   49.160 µs |   84.333 µs |
| memx_memrchr_qpl        |   41.740 µs |   50.188 µs |   41.134 µs |   50.225 µs |
| memx_memrchr_q_basic    |   54.216 µs |   88.662 µs |   53.699 µs |   85.931 µs |
| memx_memrchr_q_sse2     |   39.658 µs |   47.196 µs |   39.276 µs |   48.056 µs |

  3. i586-unknown-linux- @Q6600:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memrchr_qpl         |   47.991 µs |   82.831 µs |   47.712 µs |   83.137 µs |
| memx_memrchr_qpl        |   41.392 µs |   50.641 µs |   40.171 µs |   49.576 µs |
| memx_memrchr_q_basic    |   52.212 µs |   81.203 µs |   56.673 µs |   89.222 µs |

- `µs` is micro seconds
- `:en` is english haystack or short length.
- `:ja` is japanese haystack or long length.
- `gnu` is x86_64-unknown-linux-gnu
- `musl` is x86_64-unknown-linux-musl
- @i5-4570: bench on intel i5-4570 @ 3.2GHz
- @Q6600: bench on intel Q6600 @ 2.40GHz or armv7 1.5GHz
