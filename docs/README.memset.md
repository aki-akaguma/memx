## Benchmark results

- compile by rustc 1.68.1 (8460ca823 2023-03-20)

  0. x86_64-unknown-linux- @Broadwell:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memset              |    7.120 µs |   10.169 µs |    7.418 µs |   11.322 µs |
| libc_memset             |    7.067 µs |   10.416 µs |    7.423 µs |   11.399 µs |
| memx_memset             |    5.936 µs |   10.264 µs |    5.976 µs |   10.048 µs |
| memx_memset_basic       |    7.169 µs |   10.291 µs |    7.510 µs |   11.424 µs |
| memx_memset_sse2        |    5.962 µs |   10.214 µs |    6.133 µs |   10.245 µs |
| memx_memset_avx2        |    5.944 µs |   10.088 µs |    6.197 µs |   10.098 µs |

  1. x86_64-unknown-linux- @Q6600:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memset              |   11.272 µs |   16.499 µs |   12.331 µs |   18.720 µs |
| libc_memset             |   11.175 µs |   16.461 µs |   12.192 µs |   18.671 µs |
| memx_memset             |    9.585 µs |   14.938 µs |    9.702 µs |   14.758 µs |
| memx_memset_basic       |   11.243 µs |   16.307 µs |   12.334 µs |   18.890 µs |
| memx_memset_sse2        |    9.771 µs |   14.601 µs |    9.631 µs |   14.697 µs |

  2. i686-unknown-linux- @Q6600:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memset              |   10.145 µs |   15.273 µs |   13.510 µs |   19.100 µs |
| libc_memset             |   10.366 µs |   15.083 µs |   13.640 µs |   19.179 µs |
| memx_memset             |   10.416 µs |   15.504 µs |   10.331 µs |   15.176 µs |
| memx_memset_basic       |   10.174 µs |   15.132 µs |   13.531 µs |   19.220 µs |
| memx_memset_sse2        |    9.653 µs |   14.655 µs |    9.866 µs |   14.698 µs |

  3. i586-unknown-linux- @Q6600:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memset              |   10.198 µs |   15.099 µs |   13.340 µs |   19.088 µs |
| libc_memset             |   10.239 µs |   15.130 µs |   13.435 µs |   19.140 µs |
| memx_memset             |   10.443 µs |   15.124 µs |   10.484 µs |   15.267 µs |
| memx_memset_basic       |   10.347 µs |   15.152 µs |   13.429 µs |   19.122 µs |


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
