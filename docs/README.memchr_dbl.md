## Benchmark results

- compile by rustc 1.69.0 (84c898d65 2023-04-16)

  0. x86_64-unknown-linux- @i5-4570:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memchr_dbl          |   14.953 µs |   23.831 µs |   12.353 µs |   21.933 µs |
| memchr_memchr_dbl       |    7.164 µs |    7.726 µs |    7.069 µs |    7.638 µs |
| memx_memchr_dbl         |    5.931 µs |    6.626 µs |    5.692 µs |    6.546 µs |
| memx_memchr_w_basic     |    8.969 µs |   11.638 µs |    8.703 µs |   11.371 µs |
| memx_memchr_w_sse2      |    5.840 µs |    6.927 µs |    5.829 µs |    7.166 µs |
| memx_memchr_w_avx2      |    5.965 µs |    6.658 µs |    5.712 µs |    6.604 µs |

  1. x86_64-unknown-linux- @Q6600:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memchr_dbl          |   34.409 µs |   54.482 µs |   34.745 µs |   54.408 µs |
| memchr_memchr_dbl       |   23.178 µs |   27.314 µs |   23.239 µs |   28.868 µs |
| memx_memchr_dbl         |   20.278 µs |   22.184 µs |   20.240 µs |   22.553 µs |
| memx_memchr_w_basic     |   21.655 µs |   27.728 µs |   20.755 µs |   27.152 µs |
| memx_memchr_w_sse2      |   19.062 µs |   20.899 µs |   18.815 µs |   21.298 µs |


  2. i686-unknown-linux- @Q6600:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memchr_dbl          |   35.732 µs |   55.339 µs |   34.646 µs |   54.306 µs |
| memchr_memchr_dbl       |   40.797 µs |   54.585 µs |   41.978 µs |   59.012 µs |
| memx_memchr_dbl         |   27.747 µs |   30.255 µs |   27.353 µs |   30.381 µs |
| memx_memchr_w_basic     |   29.607 µs |   42.060 µs |   34.127 µs |   47.317 µs |
| memx_memchr_w_sse2      |   26.684 µs |   29.809 µs |   27.312 µs |   29.762 µs |

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
- @i5-4570: bench on intel i5-4570 @ 3.2GHz
- @Q6600: bench on intel Q6600 @ 2.40GHz or armv7 1.5GHz
