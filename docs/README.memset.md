## Benchmark results

- compile by rustc 1.68.1 (8460ca823 2023-03-20)

  0. x86_64-unknown-linux- @Broadwell:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memset              |    7.111 µs |   10.493 µs |    7.490 µs |   11.285 µs |
| libc_memset             |    7.159 µs |   10.254 µs |    7.398 µs |   11.529 µs |
| memx_memset             |    6.169 µs |   10.368 µs |    6.080 µs |   10.215 µs |
| memx_memset_basic       |    9.468 µs |   14.236 µs |    8.977 µs |   13.920 µs |
| memx_memset_sse2        |    6.465 µs |   10.412 µs |    6.047 µs |   10.356 µs |
| memx_memset_avx2        |    6.426 µs |   10.268 µs |    6.056 µs |   10.246 µs |

  1. x86_64-unknown-linux- @Q6600:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memset              |   11.230 µs |   16.331 µs |   12.183 µs |   18.670 µs |
| libc_memset             |   11.094 µs |   16.230 µs |   12.206 µs |   18.542 µs |
| memx_memset             |    9.887 µs |   14.741 µs |    9.922 µs |   14.890 µs |
| memx_memset_basic       |   13.212 µs |   19.617 µs |   13.077 µs |   20.115 µs |
| memx_memset_sse2        |    9.841 µs |   14.760 µs |   10.555 µs |   15.180 µs |

  2. i686-unknown-linux- @Q6600:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memset              |   10.148 µs |   15.079 µs |   13.820 µs |   19.160 µs |
| libc_memset             |   10.124 µs |   15.110 µs |   13.567 µs |   19.290 µs |
| memx_memset             |   11.132 µs |   15.733 µs |   11.287 µs |   15.720 µs |
| memx_memset_basic       |   22.107 µs |   31.950 µs |   21.683 µs |   31.854 µs |
| memx_memset_sse2        |   11.014 µs |   15.492 µs |   11.122 µs |   15.817 µs |

  3. i586-unknown-linux- @Q6600:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memset              |   10.135 µs |   15.264 µs |   13.641 µs |   19.219 µs |
| libc_memset             |   10.178 µs |   15.224 µs |   13.437 µs |   19.132 µs |
| memx_memset             |   11.440 µs |   15.683 µs |   10.858 µs |   15.670 µs |
| memx_memset_basic       |   22.061 µs |   31.649 µs |   21.465 µs |   31.500 µs |


- compile by rustc 1.53.0 (53cb7b09b 2021-06-17)

  1. x86_64-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memset              |    1.766 us |    3.818 us |    1.458 us |    3.266 us |
| libc_memset             |    1.779 us |    3.777 us |    1.436 us |    3.253 us |
| memx_memset             |    1.131 us |    2.324 us |    1.133 us |    2.501 us |
| memx_memset_basic       |    1.391 us |    2.953 us |    1.402 us |    3.125 us |
| memx_memset_sse2        |    1.165 us |    2.348 us |    1.162 us |    2.515 us |

  2. i686-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memset              |    1.083 us |    2.584 us |    2.258 us |    4.164 us |
| libc_memset             |    1.211 us |    2.553 us |    2.314 us |    4.228 us |
| memx_memset             |    1.102 us |    2.517 us |    1.217 us |    2.417 us |
| memx_memset_basic       |    2.677 us |    4.905 us |    2.778 us |    5.030 us |
| memx_memset_sse2        |    1.109 us |    2.507 us |    1.220 us |    2.387 us |

  3. i586-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memset              |    1.128 us |    2.549 us |    2.303 us |    4.251 us |
| libc_memset             |    1.193 us |    2.533 us |    2.295 us |    4.184 us |
| memx_memset             |    2.846 us |    5.331 us |    2.675 us |    5.077 us |
| memx_memset_basic       |    2.763 us |    5.267 us |    2.743 us |    5.178 us |

  4. armv7-linux-androideabi:

|         `name`          |  `andr:en`  |  `andr:ja`  |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memset              |  358.810 us |  419.420 us |    7.229 us |    8.361 us |
| libc_memset             |  199.310 us |  365.870 us |    4.615 us |    8.253 us |
| memx_memset             |  144.800 us |  265.430 us |    8.695 us |   14.089 us |
| memx_memset_basic       |  147.050 us |  261.970 us |    7.665 us |   15.335 us |

- `µs` is micro seconds
- `:en` is english haystack or short length.
- `:ja` is japanese haystack or long length.
- `gnu` is x86_64-unknown-linux-gnu
- `musl` is x86_64-unknown-linux-musl
- bench on intel Q6600 @ 2.40GHz or armv7 1.5GHz
