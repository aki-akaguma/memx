## Benchmark results

- compile by rustc 1.68.1 (8460ca823 2023-03-20)

  0. x86_64-unknown-linux- @Broadwell:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memchr_double       |   16.912 µs |   31.257 µs |   19.342 µs |   32.854 µs |
| memchr_memchr_double    |   10.573 µs |   11.746 µs |   10.805 µs |   11.452 µs |
| memx_memchr_double      |    8.293 µs |   10.081 µs |    8.445 µs |    9.888 µs |
| memx_memchr_w_basic     |   11.870 µs |   15.753 µs |   12.180 µs |   15.372 µs |
| memx_memchr_w_sse2      |    8.178 µs |   10.190 µs |    8.243 µs |   10.523 µs |
| memx_memchr_w_avx2      |    8.262 µs |   10.042 µs |    8.081 µs |    9.660 µs |

  1. x86_64-unknown-linux- @Q6600:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memchr_double       |   33.626 µs |   53.640 µs |   34.366 µs |   53.648 µs |
| memchr_memchr_double    |   22.424 µs |   27.248 µs |   22.157 µs |   27.061 µs |
| memx_memchr_double      |   19.578 µs |   22.493 µs |   19.409 µs |   21.757 µs |
| memx_memchr_w_basic     |   19.742 µs |   25.372 µs |   19.722 µs |   24.983 µs |
| memx_memchr_w_sse2      |   17.677 µs |   20.564 µs |   16.775 µs |   20.304 µs |


  2. i686-unknown-linux- @Q6600:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memchr_double       |   36.996 µs |   55.682 µs |   34.495 µs |   54.390 µs |
| memchr_memchr_double    |   38.525 µs |   52.685 µs |   42.788 µs |   59.116 µs |
| memx_memchr_double      |   27.448 µs |   29.694 µs |   26.835 µs |   29.867 µs |
| memx_memchr_w_basic     |   28.715 µs |   41.571 µs |   28.866 µs |   40.755 µs |
| memx_memchr_w_sse2      |   25.221 µs |   26.478 µs |   25.722 µs |   27.908 µs |

  3. i586-unknown-linux- @Q6600:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memchr_double       |   36.064 µs |   55.808 µs |   36.684 µs |   55.895 µs |
| memchr_memchr_double    |   41.566 µs |   54.063 µs |   42.801 µs |   59.753 µs |
| memx_memchr_double      |   26.923 µs |   29.856 µs |   27.612 µs |   30.080 µs |
| memx_memchr_w_basic     |   32.582 µs |   45.734 µs |   35.367 µs |   50.109 µs |

- `µs` is micro seconds
- `:en` is english haystack or short length.
- `:ja` is japanese haystack or long length.
- `gnu` is x86_64-unknown-linux-gnu
- `musl` is x86_64-unknown-linux-musl
- bench on intel Q6600 @ 2.40GHz or armv7 1.5GHz
