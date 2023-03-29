## Benchmark results

- compile by rustc 1.68.1 (8460ca823 2023-03-20)

  0. x86_64-unknown-linux- @Broadwell:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memchr_double       |   20.761 µs |   33.469 µs |   16.824 µs |   31.097 µs |
| memchr_memchr_double    |   10.574 µs |   11.386 µs |   10.584 µs |   11.399 µs |
| memx_memchr_double      |    8.118 µs |    9.413 µs |    7.880 µs |    9.797 µs |
| memx_memchr_w_basic     |    9.977 µs |   13.688 µs |   10.423 µs |   14.520 µs |
| memx_memchr_w_sse2      |    8.055 µs |    9.668 µs |    8.232 µs |    9.901 µs |
| memx_memchr_w_avx2      |    8.092 µs |    9.592 µs |    7.907 µs |    9.633 µs |

  1. x86_64-unknown-linux- @Q6600:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memchr_double       |   33.973 µs |   53.522 µs |   33.864 µs |   53.683 µs |
| memchr_memchr_double    |   22.679 µs |   27.112 µs |   22.669 µs |   27.963 µs |
| memx_memchr_double      |   18.898 µs |   20.289 µs |   18.990 µs |   20.707 µs |
| memx_memchr_w_basic     |   19.132 µs |   24.197 µs |   17.939 µs |   24.858 µs |
| memx_memchr_w_sse2      |   16.985 µs |   18.948 µs |   17.048 µs |   19.035 µs |

  2. i686-unknown-linux- @Q6600:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memchr_double       |   36.011 µs |   55.269 µs |   34.472 µs |   54.449 µs |
| memchr_memchr_double    |   40.295 µs |   53.646 µs |   42.817 µs |   59.361 µs |
| memx_memchr_double      |   26.827 µs |   29.255 µs |   27.379 µs |   29.083 µs |
| memx_memchr_w_basic     |   23.208 µs |   36.440 µs |   22.982 µs |   35.569 µs |
| memx_memchr_w_sse2      |   25.545 µs |   27.511 µs |   25.728 µs |   26.963 µs |

  3. i586-unknown-linux- @Q6600:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memchr_double       |   36.146 µs |   55.961 µs |   35.787 µs |   55.630 µs |
| memchr_memchr_double    |   40.472 µs |   53.376 µs |   43.799 µs |   60.609 µs |
| memx_memchr_double      |   27.315 µs |   29.214 µs |   26.909 µs |   28.598 µs |
| memx_memchr_w_basic     |   23.965 µs |   37.017 µs |   22.437 µs |   35.473 µs |

- `µs` is micro seconds
- `:en` is english haystack or short length.
- `:ja` is japanese haystack or long length.
- `gnu` is x86_64-unknown-linux-gnu
- `musl` is x86_64-unknown-linux-musl
- bench on intel Q6600 @ 2.40GHz or armv7 1.5GHz
