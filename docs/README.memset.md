## Benchmark results

- compile by rustc 1.68.0 (2c8cc3432 2023-03-06)

  0. x86_64-unknown-linux- @Broadwell:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memset              |    5.795 µs |   10.241 µs |    5.750 µs |   10.387 µs |
| libc_memset             |    5.714 µs |   10.243 µs |    5.745 µs |   10.330 µs |
| memx_memset             |    4.574 µs |    9.198 µs |    4.623 µs |    9.158 µs |
| memx_memset_basic       |    5.774 µs |   10.691 µs |    5.743 µs |   10.208 µs |
| memx_memset_sse2        |    4.590 µs |    9.375 µs |    4.631 µs |    9.037 µs |

  1. x86_64-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memset              |    9.029 µs |   15.844 µs |    9.789 µs |   17.650 µs |
| libc_memset             |    8.947 µs |   15.921 µs |    9.756 µs |   17.826 µs |
| memx_memset             |    7.742 µs |   14.093 µs |    7.823 µs |   14.270 µs |
| memx_memset_basic       |    8.730 µs |   16.393 µs |    9.739 µs |   17.646 µs |
| memx_memset_sse2        |    7.845 µs |   14.246 µs |    7.827 µs |   14.250 µs |

  2. i686-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memset              |    8.098 µs |   14.942 µs |   10.223 µs |   18.264 µs |
| libc_memset             |    8.153 µs |   15.142 µs |   10.273 µs |   18.157 µs |
| memx_memset             |    7.845 µs |   14.213 µs |    7.937 µs |   14.165 µs |
| memx_memset_basic       |    8.060 µs |   15.001 µs |   10.298 µs |   18.068 µs |
| memx_memset_sse2        |    7.758 µs |   14.421 µs |    7.960 µs |   13.989 µs |

  3. i586-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memset              |    8.028 µs |   14.696 µs |   10.130 µs |   18.290 µs |
| libc_memset             |    8.129 µs |   14.495 µs |    9.983 µs |   18.110 µs |
| memx_memset             |    8.085 µs |   14.597 µs |   10.060 µs |   18.236 µs |
| memx_memset_basic       |    8.093 µs |   14.735 µs |   10.081 µs |   18.130 µs |


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


- `us` is micro seconds
- `:en` is english haystack.
- `:ja` is japanese haystack.
- `musl` is x86_64-unknown-linux-musl
- bench on intel Q6600 @ 2.40GHz or armv7 1.5GHz
