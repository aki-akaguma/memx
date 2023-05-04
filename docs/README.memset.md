## Benchmark results

- compile by rustc 1.69.0 (84c898d65 2023-04-16)

  0. x86_64-unknown-linux- @i5-4570:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memset              |    2.007 µs |    3.921 µs |    2.294 µs |    5.380 µs |
| libc_memset             |    1.854 µs |    4.015 µs |    2.143 µs |    5.242 µs |
| memx_memset             |    3.444 µs |    3.456 µs |    3.656 µs |    3.504 µs |
| memx_memset_basic       |    3.771 µs |    4.095 µs |    3.721 µs |    4.055 µs |
| memx_memset_sse2        |    3.183 µs |    3.330 µs |    3.296 µs |    3.194 µs |
| memx_memset_avx2        |    3.448 µs |    3.528 µs |    3.710 µs |    3.517 µs |

  1. x86_64-unknown-linux- @Q6600:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memset              |    5.151 µs |   11.052 µs |    5.809 µs |   10.052 µs |
| libc_memset             |    5.095 µs |   10.836 µs |    5.781 µs |   10.219 µs |
| memx_memset             |    6.792 µs |    8.093 µs |    6.806 µs |    8.106 µs |
| memx_memset_basic       |    8.271 µs |    8.016 µs |    8.274 µs |    8.076 µs |
| memx_memset_sse2        |    6.803 µs |    8.069 µs |    6.780 µs |    8.074 µs |

  2. i686-unknown-linux- @Q6600:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memset              |    8.873 µs |    9.943 µs |   11.553 µs |   17.650 µs |
| libc_memset             |    8.871 µs |    9.914 µs |   11.499 µs |   17.487 µs |
| memx_memset             |   10.249 µs |   10.596 µs |   12.441 µs |   12.083 µs |
| memx_memset_basic       |    8.807 µs |   14.551 µs |   10.010 µs |   15.732 µs |
| memx_memset_sse2        |   10.566 µs |   10.279 µs |   11.737 µs |   11.531 µs |

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
- @i5-4570: bench on intel i5-4570 @ 3.2GHz
- @Q6600: bench on intel Q6600 @ 2.40GHz or armv7 1.5GHz
