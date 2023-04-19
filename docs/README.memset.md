## Benchmark results

- compile by rustc 1.68.1 (8460ca823 2023-03-20)

  0. x86_64-unknown-linux- @Broadwell:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memset              |    2.774 µs |    6.022 µs |    3.239 µs |    7.595 µs |
| libc_memset             |    2.784 µs |    5.958 µs |    3.136 µs |    7.588 µs |
| memx_memset             |    5.079 µs |    5.054 µs |    5.060 µs |    5.105 µs |
| memx_memset_basic       |    5.474 µs |    6.457 µs |    5.342 µs |    6.485 µs |
| memx_memset_sse2        |    4.690 µs |    5.104 µs |    4.642 µs |    5.176 µs |
| memx_memset_avx2        |    5.179 µs |    5.360 µs |    4.963 µs |    5.249 µs |

  1. x86_64-unknown-linux- @Q6600:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memset              |    5.125 µs |   10.806 µs |    5.705 µs |    9.830 µs |
| libc_memset             |    5.049 µs |   10.668 µs |    5.667 µs |    9.999 µs |
| memx_memset             |    6.658 µs |    7.914 µs |    6.741 µs |    7.925 µs |
| memx_memset_basic       |    8.123 µs |    7.848 µs |    8.167 µs |    7.840 µs |
| memx_memset_sse2        |    6.670 µs |    7.917 µs |    6.705 µs |    7.920 µs |

  2. i686-unknown-linux- @Q6600:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memset              |    8.816 µs |   10.073 µs |   11.519 µs |   18.208 µs |
| libc_memset             |    8.676 µs |   10.065 µs |   11.515 µs |   17.600 µs |
| memx_memset             |   10.856 µs |   11.119 µs |   12.274 µs |   12.220 µs |
| memx_memset_basic       |    8.970 µs |   14.811 µs |    9.985 µs |   15.658 µs |
| memx_memset_sse2        |   10.031 µs |   10.495 µs |   11.721 µs |   11.931 µs |

  3. i586-unknown-linux- @Q6600:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memset              |    8.321 µs |   10.131 µs |   10.499 µs |   17.495 µs |
| libc_memset             |    8.362 µs |   10.373 µs |   10.766 µs |   17.559 µs |
| memx_memset             |    9.852 µs |   11.450 µs |   11.744 µs |   12.035 µs |
| memx_memset_basic       |   10.262 µs |   14.613 µs |   11.439 µs |   16.257 µs |


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
