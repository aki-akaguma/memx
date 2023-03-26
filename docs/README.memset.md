## Benchmark results

- compile by rustc 1.68.1 (8460ca823 2023-03-20)

  0. x86_64-unknown-linux- @Broadwell:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memset              |    7.333 µs |    9.954 µs |    7.421 µs |   11.226 µs |
| libc_memset             |    7.300 µs |   10.001 µs |    7.365 µs |   11.162 µs |
| memx_memset             |    6.180 µs |   10.345 µs |    6.145 µs |   11.047 µs |
| memx_memset_basic       |    7.208 µs |   10.149 µs |    7.361 µs |   11.331 µs |
| memx_memset_sse2        |    5.990 µs |   10.213 µs |    6.177 µs |   11.081 µs |
| memx_memset_avx2        |    5.932 µs |   10.251 µs |    6.090 µs |   10.996 µs |

  1. x86_64-unknown-linux- @Q6600:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memset              |   11.383 µs |   16.673 µs |   12.276 µs |   19.165 µs |
| libc_memset             |   11.131 µs |   16.852 µs |   12.281 µs |   19.129 µs |
| memx_memset             |   10.110 µs |   15.096 µs |   10.014 µs |   15.349 µs |
| memx_memset_basic       |   11.620 µs |   16.434 µs |   12.521 µs |   18.942 µs |
| memx_memset_sse2        |    9.984 µs |   15.253 µs |    9.838 µs |   14.868 µs |

  2. i686-unknown-linux- @Q6600:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memset              |   11.056 µs |   15.295 µs |   13.581 µs |   19.189 µs |
| libc_memset             |   10.544 µs |   15.629 µs |   13.826 µs |   19.149 µs |
| memx_memset             |   10.132 µs |   15.240 µs |   10.578 µs |   15.148 µs |
| memx_memset_basic       |   10.443 µs |   15.592 µs |   13.667 µs |   19.174 µs |
| memx_memset_sse2        |    9.966 µs |   14.932 µs |   10.021 µs |   14.771 µs |

  3. i586-unknown-linux- @Q6600:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memset              |   10.442 µs |   15.196 µs |   13.466 µs |   19.051 µs |
| libc_memset             |   10.260 µs |   15.214 µs |   13.494 µs |   19.152 µs |
| memx_memset             |   10.107 µs |   15.306 µs |   10.339 µs |   15.159 µs |
| memx_memset_basic       |   10.391 µs |   15.344 µs |   13.352 µs |   19.183 µs |


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
