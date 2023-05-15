## Benchmark results

- compile by rustc 1.69.0 (84c898d65 2023-04-16)

  0. x86_64-unknown-linux- @i5-4570:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memchr_dbl          |   13.231 µs |   23.129 µs |   12.698 µs |   23.043 µs |
| memchr_memchr_dbl       |    6.756 µs |    7.663 µs |    6.923 µs |    7.672 µs |
| memx_memchr_dbl         |    5.800 µs |    6.676 µs |    5.721 µs |    6.625 µs |
| memx_memchr_w_basic     |    8.828 µs |   12.158 µs |    9.004 µs |   11.964 µs |
| memx_memchr_w_sse2      |    5.925 µs |    7.024 µs |    5.946 µs |    7.266 µs |
| memx_memchr_w_avx2      |    5.778 µs |    6.691 µs |    5.739 µs |    6.621 µs |

  1. x86_64-unknown-linux- @Q6600:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memchr_dbl          |   34.396 µs |   53.992 µs |   34.464 µs |   53.938 µs |
| memchr_memchr_dbl       |   21.987 µs |   26.918 µs |   21.827 µs |   28.187 µs |
| memx_memchr_dbl         |   20.219 µs |   22.363 µs |   19.761 µs |   22.715 µs |
| memx_memchr_w_basic     |   19.011 µs |   26.527 µs |   18.748 µs |   26.157 µs |
| memx_memchr_w_sse2      |   17.967 µs |   21.484 µs |   17.164 µs |   21.686 µs |


  2. i686-unknown-linux- @Q6600:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memchr_dbl          |   35.572 µs |   55.636 µs |   34.767 µs |   54.448 µs |
| memchr_memchr_dbl       |   42.535 µs |   53.605 µs |   43.199 µs |   58.502 µs |
| memx_memchr_dbl         |   27.938 µs |   30.111 µs |   27.781 µs |   30.450 µs |
| memx_memchr_w_basic     |   29.839 µs |   46.138 µs |   30.601 µs |   47.155 µs |
| memx_memchr_w_sse2      |   26.037 µs |   30.374 µs |   25.971 µs |   28.993 µs |

  3. i586-unknown-linux- @Q6600:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memchr_dbl          |   36.138 µs |   56.384 µs |   36.677 µs |   55.752 µs |
| memchr_memchr_dbl       |   41.318 µs |   52.430 µs |   43.244 µs |   58.727 µs |
| memx_memchr_dbl         |   28.890 µs |   31.068 µs |   29.227 µs |   31.480 µs |
| memx_memchr_w_basic     |   31.274 µs |   47.041 µs |   34.523 µs |   50.888 µs |

- `µs` is micro seconds
- `:en` is english haystack or short length.
- `:ja` is japanese haystack or long length.
- `gnu` is x86_64-unknown-linux-gnu
- `musl` is x86_64-unknown-linux-musl
- @i5-4570: bench on intel i5-4570 @ 3.2GHz
- @Q6600: bench on intel Q6600 @ 2.40GHz or armv7 1.5GHz
