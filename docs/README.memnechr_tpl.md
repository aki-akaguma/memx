## Benchmark results

- compile by rustc 1.69.0 (84c898d65 2023-04-16)

  0. x86_64-unknown-linux- @i5-4570:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memnechr_tpl        |   51.147 µs |  145.330 µs |   55.040 µs |  145.710 µs |
| memx_memnechr_tpl       |    7.252 µs |    8.606 µs |    7.215 µs |    8.541 µs |
| memx_memnechr_t_basic   |   12.481 µs |   17.893 µs |   12.475 µs |   17.887 µs |
| memx_memnechr_t_sse2    |    7.704 µs |    9.456 µs |    7.618 µs |    9.556 µs |
| memx_memnechr_t_avx2    |    7.293 µs |    8.644 µs |    7.266 µs |    8.555 µs |

  1. x86_64-unknown-linux- @Q6600:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memnechr_tpl        |   79.552 µs |  115.960 µs |   83.702 µs |  118.430 µs |
| memx_memnechr_tpl       |   25.684 µs |   29.633 µs |   25.462 µs |   30.198 µs |
| memx_memnechr_t_basic   |   31.562 µs |   45.958 µs |   31.370 µs |   46.193 µs |
| memx_memnechr_t_sse2    |   21.507 µs |   25.422 µs |   21.098 µs |   26.249 µs |

  2. i686-unknown-linux- @Q6600:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memnechr_tpl        |   36.325 µs |   56.418 µs |   36.511 µs |   56.600 µs |
| memx_memnechr_tpl       |   33.886 µs |   38.469 µs |   34.468 µs |   38.194 µs |
| memx_memnechr_t_basic   |   45.327 µs |   70.329 µs |   45.637 µs |   71.647 µs |
| memx_memnechr_t_sse2    |   28.141 µs |   34.742 µs |   30.680 µs |   34.587 µs |

  3. i586-unknown-linux- @Q6600:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memnechr_tpl        |   37.363 µs |   56.419 µs |   35.167 µs |   56.618 µs |
| memx_memnechr_tpl       |   32.413 µs |   39.494 µs |   34.646 µs |   38.685 µs |
| memx_memnechr_t_basic   |   47.209 µs |   73.086 µs |   48.376 µs |   73.219 µs |

- `µs` is micro seconds
- `:en` is english haystack or short length.
- `:ja` is japanese haystack or long length.
- `gnu` is x86_64-unknown-linux-gnu
- `musl` is x86_64-unknown-linux-musl
- @i5-4570: bench on intel i5-4570 @ 3.2GHz
- @Q6600: bench on intel Q6600 @ 2.40GHz or armv7 1.5GHz
