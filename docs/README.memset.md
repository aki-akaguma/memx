## Benchmark results

- compile by rustc 1.65.0 (897e37553 2022-11-02)

  1. x86_64-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memset              |    1.512 µs |    3.460 µs |    1.476 µs |    3.341 µs |
| libc_memset             |    1.454 µs |    3.413 µs |    1.453 µs |    3.292 µs |
| memx_memset             |    1.323 µs |    2.611 µs |    1.320 µs |    2.717 µs |
| memx_memset_basic       |    1.479 µs |    3.056 µs |    1.491 µs |    3.193 µs |
| memx_memset_sse2        |    1.147 µs |    2.431 µs |    1.147 µs |    2.559 µs |

  2. i686-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memset              |    1.202 µs |    2.648 µs |    2.446 µs |    4.453 µs |
| libc_memset             |    1.211 µs |    2.624 µs |    2.309 µs |    4.262 µs |
| memx_memset             |    1.499 µs |    2.854 µs |    1.472 µs |    2.768 µs |
| memx_memset_basic       |    3.212 µs |    5.804 µs |    3.210 µs |    5.892 µs |
| memx_memset_sse2        |    1.189 µs |    2.525 µs |    1.143 µs |    2.381 µs |

  3. i586-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memset              |    1.187 µs |    2.576 µs |    2.447 µs |    4.461 µs |
| libc_memset             |    1.203 µs |    2.540 µs |    2.301 µs |    4.237 µs |
| memx_memset             |    2.729 µs |    5.222 µs |    2.822 µs |    5.334 µs |
| memx_memset_basic       |    2.788 µs |    5.281 µs |    2.826 µs |    5.290 µs |

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
