## Benchmark results

- compile by rustc 1.69.0 (84c898d65 2023-04-16)

  0. x86_64-unknown-linux- @i5-4570:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memchr_tpl          |   12.055 µs |   22.601 µs |   12.393 µs |   23.305 µs |
| memchr_memchr_tpl       |    8.839 µs |    9.016 µs |    8.876 µs |    9.118 µs |
| memx_memchr_tpl         |    6.683 µs |    7.232 µs |    6.605 µs |    7.251 µs |
| memx_memchr_t_basic     |   11.396 µs |   15.416 µs |   11.445 µs |   15.226 µs |
| memx_memchr_t_sse2      |    6.805 µs |    8.256 µs |    6.829 µs |    8.441 µs |
| memx_memchr_t_avx2      |    6.597 µs |    7.222 µs |    6.528 µs |    7.241 µs |

  1. x86_64-unknown-linux- @Q6600:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memchr_tpl          |   42.353 µs |   68.171 µs |   42.831 µs |   68.234 µs |
| memchr_memchr_tpl       |   26.382 µs |   31.638 µs |   26.302 µs |   31.719 µs |
| memx_memchr_tpl         |   25.594 µs |   28.058 µs |   25.328 µs |   28.296 µs |
| memx_memchr_t_basic     |   26.081 µs |   35.567 µs |   25.971 µs |   35.843 µs |
| memx_memchr_t_sse2      |   23.586 µs |   26.928 µs |   23.337 µs |   27.491 µs |

  2. i686-unknown-linux- @Q6600:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memchr_tpl          |   42.221 µs |   67.800 µs |   42.658 µs |   67.870 µs |
| memchr_memchr_tpl       |   50.316 µs |   62.670 µs |   49.020 µs |   63.579 µs |
| memx_memchr_tpl         |   36.022 µs |   38.273 µs |   36.401 µs |   38.558 µs |
| memx_memchr_t_basic     |   42.794 µs |   62.282 µs |   43.881 µs |   62.474 µs |
| memx_memchr_t_sse2      |   33.373 µs |   36.397 µs |   34.665 µs |   37.226 µs |

  3. i586-unknown-linux- @Q6600:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memchr_tpl          |   43.774 µs |   69.121 µs |   44.910 µs |   69.049 µs |
| memchr_memchr_tpl       |   50.500 µs |   63.551 µs |   50.676 µs |   81.689 µs |
| memx_memchr_tpl         |   36.484 µs |   38.684 µs |   36.865 µs |   67.387 µs |
| memx_memchr_t_basic     |   43.023 µs |   61.667 µs |   47.892 µs |   84.960 µs |

- `µs` is micro seconds
- `:en` is english haystack or short length.
- `:ja` is japanese haystack or long length.
- `gnu` is x86_64-unknown-linux-gnu
- `musl` is x86_64-unknown-linux-musl
- @i5-4570: bench on intel i5-4570 @ 3.2GHz
- @Q6600: bench on intel Q6600 @ 2.40GHz or armv7 1.5GHz
