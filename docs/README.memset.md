## Benchmark results

- compile by rustc 1.69.0 (84c898d65 2023-04-16)

  0. x86_64-unknown-linux- @i5-4570:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memset              |    2.006 µs |    3.914 µs |    2.295 µs |    5.377 µs |
| libc_memset             |    2.003 µs |    4.015 µs |    2.139 µs |    5.236 µs |
| memx_memset             |    3.388 µs |    3.537 µs |    3.515 µs |    3.499 µs |
| memx_memset_basic       |    3.733 µs |    4.086 µs |    3.933 µs |    4.057 µs |
| memx_memset_sse2        |    3.322 µs |    3.446 µs |    3.324 µs |    3.446 µs |
| memx_memset_avx2        |    3.445 µs |    3.441 µs |    3.515 µs |    3.436 µs |

  1. x86_64-unknown-linux- @Q6600:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memset              |    5.027 µs |   10.801 µs |    5.675 µs |    9.825 µs |
| libc_memset             |    4.952 µs |   10.616 µs |    5.671 µs |    9.992 µs |
| memx_memset             |    6.656 µs |    7.924 µs |    6.657 µs |    7.908 µs |
| memx_memset_basic       |    8.125 µs |    7.829 µs |    8.153 µs |    7.843 µs |
| memx_memset_sse2        |    6.668 µs |    7.918 µs |    6.662 µs |    7.904 µs |

  2. i686-unknown-linux- @Q6600:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memset              |    8.886 µs |    9.921 µs |   11.520 µs |   17.585 µs |
| libc_memset             |    8.696 µs |    9.868 µs |   11.554 µs |   17.662 µs |
| memx_memset             |   10.508 µs |   11.029 µs |   12.145 µs |   12.334 µs |
| memx_memset_basic       |    8.796 µs |   14.588 µs |    9.960 µs |   15.654 µs |
| memx_memset_sse2        |   10.430 µs |   10.559 µs |   11.902 µs |   11.907 µs |

  3. i586-unknown-linux- @Q6600:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memset              |    8.339 µs |   10.199 µs |   11.097 µs |   17.336 µs |
| libc_memset             |    8.351 µs |   10.230 µs |   11.183 µs |   17.384 µs |
| memx_memset             |    9.896 µs |   11.374 µs |   11.736 µs |   12.586 µs |
| memx_memset_basic       |   10.200 µs |   14.680 µs |   11.637 µs |   16.207 µs |


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
- @i5-4570: bench on intel i5-4570 @ 3.2GHz
- @Q6600: bench on intel Q6600 @ 2.40GHz or armv7 1.5GHz
