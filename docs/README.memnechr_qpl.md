## Benchmark results

- compile by rustc 1.69.0 (84c898d65 2023-04-16)

  0. x86_64-unknown-linux- @i5-4570:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memnechr_qpl        |   11.740 µs |   24.161 µs |   13.517 µs |   25.210 µs |
| memx_memnechr_qpl       |    8.000 µs |    9.641 µs |    7.875 µs |    9.560 µs |
| memx_memnechr_q_basic   |   14.520 µs |   21.289 µs |   14.465 µs |   21.265 µs |
| memx_memnechr_q_sse2    |    8.649 µs |   10.825 µs |    8.797 µs |   11.218 µs |
| memx_memnechr_q_avx2    |    8.127 µs |    9.764 µs |    7.818 µs |    9.516 µs |

  1. x86_64-unknown-linux- @Q6600:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memnechr_qpl        |   45.555 µs |   68.901 µs |   51.659 µs |   75.694 µs |
| memx_memnechr_qpl       |   28.645 µs |   33.688 µs |   27.462 µs |   33.499 µs |
| memx_memnechr_q_basic   |   37.043 µs |   54.641 µs |   36.161 µs |   53.466 µs |
| memx_memnechr_q_sse2    |   24.425 µs |   29.044 µs |   23.489 µs |   29.778 µs |

  2. i686-unknown-linux- @Q6600:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memnechr_qpl        |   38.448 µs |   63.688 µs |   39.609 µs |   64.439 µs |
| memx_memnechr_qpl       |   36.842 µs |   43.150 µs |   36.896 µs |   44.356 µs |
| memx_memnechr_q_basic   |   56.264 µs |   88.528 µs |   60.622 µs |   94.584 µs |
| memx_memnechr_q_sse2    |   32.614 µs |   39.231 µs |   32.558 µs |   38.232 µs |

  3. i586-unknown-linux- @Q6600:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memnechr_qpl        |   42.754 µs |   65.919 µs |   44.337 µs |   68.423 µs |
| memx_memnechr_qpl       |   35.133 µs |   42.226 µs |   36.313 µs |   41.917 µs |
| memx_memnechr_q_basic   |   55.842 µs |   89.469 µs |   60.674 µs |   95.512 µs |

- `µs` is micro seconds
- `:en` is english haystack or short length.
- `:ja` is japanese haystack or long length.
- `gnu` is x86_64-unknown-linux-gnu
- `musl` is x86_64-unknown-linux-musl
- @i5-4570: bench on intel i5-4570 @ 3.2GHz
- @Q6600: bench on intel Q6600 @ 2.40GHz or armv7 1.5GHz
