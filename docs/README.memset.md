## Benchmark results

  1. x86_64-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memset              |    1.723 us |    3.705 us |    1.530 us |    3.276 us |
| memx_memset             |    1.129 us |    2.389 us |    1.191 us |    2.517 us |
| memx_memset_basic       |    1.375 us |    2.922 us |    1.386 us |    3.102 us |
| memx_memset_libc        |    1.869 us |    3.926 us |    1.691 us |    3.597 us |

  2. i686-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memset              |    1.347 us |    2.955 us |    2.557 us |    4.673 us |
| memx_memset             |    1.325 us |    2.651 us |    1.363 us |    2.747 us |
| memx_memset_basic       |    2.690 us |    5.229 us |    2.724 us |    5.279 us |
| memx_memset_libc        |    1.410 us |    3.002 us |    2.664 us |    4.829 us |

  3. i586-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memset              |    1.345 us |    2.750 us |    2.607 us |    4.785 us |
| memx_memset             |    2.788 us |    5.236 us |    2.788 us |    5.278 us |
| memx_memset_basic       |    2.962 us |    5.372 us |    2.892 us |    5.447 us |
| memx_memset_libc        |    1.367 us |    2.911 us |    2.712 us |    4.902 us |

  4. armv7-linux-androideabi:

|         `name`          |  `andr:en`  |  `andr:ja`  |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memset              |   89.362 us |  107.800 us |    4.556 us |    8.427 us |
| memx_memset             |    5.995 us |   10.970 us |    7.772 us |   12.914 us |
| memx_memset_basic       |    5.958 us |   10.729 us |    7.369 us |   12.187 us |
| memx_memset_libc        |   61.210 us |  111.990 us |    4.411 us |    7.947 us |


- compile by rustc 1.52.1 (9bc8c42bb 2021-05-09)
- `us` is micro seconds
- `:en` is english haystack.
- `:ja` is japanese haystack.
- `musl` is x86_64-unknown-linux-musl
- bench on intel Q6600 @ 2.40GHz or armv7 1.5GHz
