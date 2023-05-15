## Benchmark results

- compile by rustc 1.69.0 (84c898d65 2023-04-16)

  0. x86_64-unknown-linux- @i5-4570:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memrchr_tpl         |   11.656 µs |   22.218 µs |   11.927 µs |   22.444 µs |
| memchr_memrchr_tpl      |    8.491 µs |    9.722 µs |    8.459 µs |    9.745 µs |
| memx_memrchr_tpl        |    6.886 µs |    8.378 µs |    6.828 µs |    8.489 µs |
| memx_memrchr_t_basic    |   12.216 µs |   17.286 µs |   11.667 µs |   16.738 µs |
| memx_memrchr_t_sse2     |    7.526 µs |    9.495 µs |    7.614 µs |    9.643 µs |
| memx_memrchr_t_avx2     |    6.907 µs |    8.350 µs |    6.812 µs |    8.476 µs |

  1. x86_64-unknown-linux- @Q6600:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memrchr_tpl         |   41.431 µs |   67.962 µs |   41.706 µs |   68.005 µs |
| memchr_memrchr_tpl      |   26.739 µs |   34.681 µs |   27.538 µs |   35.255 µs |
| memx_memrchr_tpl        |   27.587 µs |   34.713 µs |   26.905 µs |   33.325 µs |
| memx_memrchr_t_basic    |   28.681 µs |   42.614 µs |   29.037 µs |   42.511 µs |
| memx_memrchr_t_sse2     |   25.888 µs |   33.132 µs |   26.113 µs |   32.487 µs |

  2. i686-unknown-linux- @Q6600:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memrchr_tpl         |   40.031 µs |   67.423 µs |   41.564 µs |   68.606 µs |
| memchr_memrchr_tpl      |   50.744 µs |   76.247 µs |   51.515 µs |   74.419 µs |
| memx_memrchr_tpl        |   35.626 µs |   42.086 µs |   36.743 µs |   44.368 µs |
| memx_memrchr_t_basic    |   44.683 µs |   69.509 µs |   45.736 µs |   71.525 µs |
| memx_memrchr_t_sse2     |   34.553 µs |   40.578 µs |   34.710 µs |   42.250 µs |

  3. i586-unknown-linux- @Q6600:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memrchr_tpl         |   40.376 µs |   67.948 µs |   41.508 µs |   69.482 µs |
| memchr_memrchr_tpl      |   50.870 µs |   75.129 µs |   53.359 µs |   77.151 µs |
| memx_memrchr_tpl        |   34.810 µs |   41.211 µs |   36.978 µs |   43.639 µs |
| memx_memrchr_t_basic    |   43.958 µs |   69.504 µs |   47.480 µs |   72.921 µs |

- `µs` is micro seconds
- `:en` is english haystack or short length.
- `:ja` is japanese haystack or long length.
- `gnu` is x86_64-unknown-linux-gnu
- `musl` is x86_64-unknown-linux-musl
- @i5-4570: bench on intel i5-4570 @ 3.2GHz
- @Q6600: bench on intel Q6600 @ 2.40GHz or armv7 1.5GHz
