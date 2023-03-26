## Benchmark results

- compile by rustc 1.68.1 (8460ca823 2023-03-20)

  0. x86_64-unknown-linux- @Broadwell:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memchr_double       |   20.647 µs |   33.503 µs |   16.732 µs |   30.662 µs |
| memchr_memchr_double    |   10.567 µs |   11.425 µs |   10.599 µs |   11.586 µs |
| memx_memchr_double      |    8.501 µs |   10.143 µs |    8.388 µs |   10.100 µs |
| memx_memchr_w_basic     |    9.963 µs |   13.619 µs |   10.361 µs |   14.721 µs |
| memx_memchr_w_sse2      |    8.022 µs |   10.010 µs |    8.105 µs |   10.255 µs |
| memx_memchr_w_avx2      |    8.058 µs |    9.754 µs |    7.917 µs |    9.594 µs |

  1. x86_64-unknown-linux- @Q6600:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memchr_double       |   33.854 µs |   53.735 µs |   34.199 µs |   53.506 µs |
| memchr_memchr_double    |   22.179 µs |   27.060 µs |   22.383 µs |   27.470 µs |
| memx_memchr_double      |   21.566 µs |   22.234 µs |   21.118 µs |   22.793 µs |
| memx_memchr_w_basic     |   18.451 µs |   23.679 µs |   17.810 µs |   25.048 µs |
| memx_memchr_w_sse2      |   17.385 µs |   18.978 µs |   17.085 µs |   19.053 µs |

  2. i686-unknown-linux- @Q6600:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memchr_double       |   35.576 µs |   55.761 µs |   34.534 µs |   54.264 µs |
| memchr_memchr_double    |   39.506 µs |   51.746 µs |   43.004 µs |   59.927 µs |
| memx_memchr_double      |   29.097 µs |   30.539 µs |   28.447 µs |   29.230 µs |
| memx_memchr_w_basic     |   22.847 µs |   35.937 µs |   22.349 µs |   34.583 µs |
| memx_memchr_w_sse2      |   24.152 µs |   25.427 µs |   23.252 µs |   25.833 µs |

  3. i586-unknown-linux- @Q6600:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memchr_double       |   36.507 µs |   55.967 µs |   35.589 µs |   55.443 µs |
| memchr_memchr_double    |   40.858 µs |   53.461 µs |   41.895 µs |   59.468 µs |
| memx_memchr_double      |   26.589 µs |   28.262 µs |   27.938 µs |   28.976 µs |
| memx_memchr_w_basic     |   23.779 µs |   36.681 µs |   23.065 µs |   35.035 µs |

- `µs` is micro seconds
- `:en` is english haystack or short length.
- `:ja` is japanese haystack or long length.
- `gnu` is x86_64-unknown-linux-gnu
- `musl` is x86_64-unknown-linux-musl
- bench on intel Q6600 @ 2.40GHz or armv7 1.5GHz
