## Benchmark results

- compile by rustc 1.69.0 (84c898d65 2023-04-16)

  0. x86_64-unknown-linux- @i5-4570:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memchr_tpl          |   11.660 µs |   22.404 µs |   11.628 µs |   22.398 µs |
| memchr_memchr_tpl       |    8.719 µs |    9.836 µs |    8.605 µs |    9.769 µs |
| memx_memchr_tpl         |    6.625 µs |    8.149 µs |    6.478 µs |    8.045 µs |
| memx_memchr_t_basic     |   12.383 µs |   17.493 µs |   12.396 µs |   17.482 µs |
| memx_memchr_t_sse2      |    7.212 µs |    9.121 µs |    7.164 µs |    9.100 µs |
| memx_memchr_t_avx2      |    6.632 µs |    8.110 µs |    6.493 µs |    8.022 µs |

  1. x86_64-unknown-linux- @Q6600:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memchr_tpl          |   41.708 µs |   70.396 µs |   42.542 µs |   69.312 µs |
| memchr_memchr_tpl       |   28.579 µs |   35.061 µs |   27.442 µs |   35.027 µs |
| memx_memchr_tpl         |   27.189 µs |   32.574 µs |   25.863 µs |   32.525 µs |
| memx_memchr_t_basic     |   27.801 µs |   39.283 µs |   27.563 µs |   39.117 µs |
| memx_memchr_t_sse2      |   24.920 µs |   30.542 µs |   24.647 µs |   31.121 µs |

  2. i686-unknown-linux- @Q6600:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memchr_tpl          |   41.860 µs |   68.831 µs |   42.057 µs |   68.960 µs |
| memchr_memchr_tpl       |   49.360 µs |   69.968 µs |   48.250 µs |   68.074 µs |
| memx_memchr_tpl         |   35.559 µs |   42.003 µs |   35.110 µs |   42.300 µs |
| memx_memchr_t_basic     |   39.126 µs |   62.087 µs |   39.553 µs |   62.502 µs |
| memx_memchr_t_sse2      |   33.580 µs |   40.145 µs |   33.642 µs |   39.875 µs |

  3. i586-unknown-linux- @Q6600:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memchr_tpl          |   43.201 µs |   70.256 µs |   43.200 µs |   70.053 µs |
| memchr_memchr_tpl       |   49.754 µs |   69.610 µs |   49.587 µs |   69.870 µs |
| memx_memchr_tpl         |   36.160 µs |   42.256 µs |   34.523 µs |   40.763 µs |
| memx_memchr_t_basic     |   40.333 µs |   62.447 µs |   43.785 µs |   67.529 µs |

- `µs` is micro seconds
- `:en` is english haystack or short length.
- `:ja` is japanese haystack or long length.
- `gnu` is x86_64-unknown-linux-gnu
- `musl` is x86_64-unknown-linux-musl
- @i5-4570: bench on intel i5-4570 @ 3.2GHz
- @Q6600: bench on intel Q6600 @ 2.40GHz or armv7 1.5GHz
