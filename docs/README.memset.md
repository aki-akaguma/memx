## Benchmark results

  1. x86_64-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memset              |    1.750 us |    3.798 us |    1.459 us |    3.276 us |
| libc_memset             |    1.791 us |    3.704 us |    1.437 us |    3.204 us |
| memx_memset             |    1.179 us |    2.442 us |    1.188 us |    2.554 us |
| memx_memset_basic       |    1.389 us |    2.914 us |    1.393 us |    3.132 us |
| memx_memset_sse2        |    1.189 us |    2.414 us |    1.151 us |    2.540 us |

  2. i686-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memset              |    1.117 us |    2.552 us |    2.393 us |    4.429 us |
| libc_memset             |    1.219 us |    2.571 us |    2.289 us |    4.260 us |
| memx_memset             |    1.157 us |    2.474 us |    1.135 us |    2.482 us |
| memx_memset_basic       |    2.751 us |    4.989 us |    2.756 us |    5.040 us |
| memx_memset_sse2        |    1.128 us |    2.502 us |    1.212 us |    2.421 us |

  3. i586-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memset              |    1.121 us |    2.478 us |    2.339 us |    4.188 us |
| libc_memset             |    1.220 us |    2.470 us |    2.283 us |    4.188 us |
| memx_memset             |    2.788 us |    4.933 us |    2.727 us |    5.209 us |
| memx_memset_basic       |    2.873 us |    5.269 us |    2.842 us |    5.361 us |

  4. armv7-linux-androideabi:

|         `name`          |  `andr:en`  |  `andr:ja`  |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memset              |  358.810 us |  419.420 us |    7.229 us |    8.361 us |
| libc_memset             |  199.310 us |  365.870 us |    4.615 us |    8.253 us |
| memx_memset             |  144.800 us |  265.430 us |    8.695 us |   14.089 us |
| memx_memset_basic       |  147.050 us |  261.970 us |    7.665 us |   15.335 us |


- compile by rustc 1.52.1 (9bc8c42bb 2021-05-09)
- `us` is micro seconds
- `:en` is english haystack.
- `:ja` is japanese haystack.
- `musl` is x86_64-unknown-linux-musl
- bench on intel Q6600 @ 2.40GHz or armv7 1.5GHz
