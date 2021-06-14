## Benchmark results

  1. x86_64-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memset              |    1.738 us |    3.800 us |    1.468 us |    3.334 us |
| libc_memset             |    1.726 us |    3.739 us |    1.442 us |    3.186 us |
| memx_memset             |    1.139 us |    2.333 us |    1.181 us |    2.567 us |
| memx_memset_basic       |    1.395 us |    2.933 us |    1.432 us |    3.098 us |
| memx_memset_sse2        |    1.171 us |    2.379 us |    1.189 us |    2.625 us |

  2. i686-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memset              |    1.210 us |    2.518 us |    2.420 us |    4.412 us |
| libc_memset             |    1.199 us |    2.527 us |    2.309 us |    4.210 us |
| memx_memset             |    1.162 us |    2.492 us |    1.200 us |    2.495 us |
| memx_memset_basic       |    2.820 us |    4.990 us |    2.755 us |    4.966 us |
| memx_memset_sse2        |    1.503 us |    2.437 us |    1.188 us |    2.483 us |

  3. i586-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memset              |    1.205 us |    2.523 us |    2.339 us |    4.371 us |
| libc_memset             |    1.190 us |    2.501 us |    2.289 us |    4.216 us |
| memx_memset             |    2.744 us |    5.261 us |    2.727 us |    5.164 us |
| memx_memset_basic       |    2.852 us |    5.375 us |    2.720 us |    5.165 us |

  4. armv7-linux-androideabi:

|         `name`          |  `andr:en`  |  `andr:ja`  |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memset              |   84.083 us |   89.588 us |    6.830 us |    7.529 us |
| libc_memset             |   60.145 us |  108.200 us |    4.194 us |    7.544 us |
| memx_memset             |    7.192 us |   13.226 us |    7.198 us |   13.223 us |
| memx_memset_basic       |    6.403 us |   11.415 us |    6.906 us |   12.360 us |


- compile by rustc 1.52.1 (9bc8c42bb 2021-05-09)
- `us` is micro seconds
- `:en` is english haystack.
- `:ja` is japanese haystack.
- `musl` is x86_64-unknown-linux-musl
- bench on intel Q6600 @ 2.40GHz or armv7 1.5GHz
