## Benchmark results

- compile by rustc 1.69.0 (84c898d65 2023-04-16)

  0. x86_64-unknown-linux- @i5-4570:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memrchr_dbl         |   11.169 µs |   22.026 µs |   11.132 µs |   22.013 µs |
| memchr_memrchr_dbl      |    6.940 µs |    7.742 µs |    6.884 µs |    7.750 µs |
| memx_memrchr_dbl        |    5.977 µs |    6.796 µs |    5.903 µs |    6.808 µs |
| memx_memrchr_w_basic    |    8.564 µs |   12.023 µs |    8.635 µs |   12.019 µs |
| memx_memrchr_w_sse2     |    5.939 µs |    7.047 µs |    5.954 µs |    7.021 µs |
| memx_memrchr_w_avx2     |    6.079 µs |    6.784 µs |    5.844 µs |    6.804 µs |

  1. x86_64-unknown-linux- @Q6600:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memrchr_dbl         |   33.685 µs |   54.123 µs |   33.662 µs |   54.197 µs |
| memchr_memrchr_dbl      |   21.596 µs |   26.078 µs |   21.568 µs |   25.938 µs |
| memx_memrchr_dbl        |   19.704 µs |   24.498 µs |   19.538 µs |   24.148 µs |
| memx_memrchr_w_basic    |   20.573 µs |   30.038 µs |   20.467 µs |   30.353 µs |
| memx_memrchr_w_sse2     |   18.304 µs |   21.646 µs |   17.821 µs |   21.541 µs |

  2. i686-unknown-linux- @Q6600:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memrchr_dbl         |   32.963 µs |   54.195 µs |   33.490 µs |   54.613 µs |
| memchr_memrchr_dbl      |   39.772 µs |   54.349 µs |   40.120 µs |   56.899 µs |
| memx_memrchr_dbl        |   26.491 µs |   31.729 µs |   26.829 µs |   32.286 µs |
| memx_memrchr_w_basic    |   32.193 µs |   52.530 µs |   35.904 µs |   55.851 µs |
| memx_memrchr_w_sse2     |   25.386 µs |   29.835 µs |   25.442 µs |   30.033 µs |

  3. i586-unknown-linux- @Q6600:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memrchr_dbl         |   33.645 µs |   54.743 µs |   33.377 µs |   54.415 µs |
| memchr_memrchr_dbl      |   41.034 µs |   54.801 µs |   40.326 µs |   56.639 µs |
| memx_memrchr_dbl        |   26.494 µs |   31.689 µs |   26.146 µs |   31.901 µs |
| memx_memrchr_w_basic    |   31.843 µs |   51.169 µs |   34.367 µs |   54.109 µs |

- `µs` is micro seconds
- `:en` is english haystack or short length.
- `:ja` is japanese haystack or long length.
- `gnu` is x86_64-unknown-linux-gnu
- `musl` is x86_64-unknown-linux-musl
- @i5-4570: bench on intel i5-4570 @ 3.2GHz
- @Q6600: bench on intel Q6600 @ 2.40GHz or armv7 1.5GHz
