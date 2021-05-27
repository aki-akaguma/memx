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
| std_memset              |    1.333 us |    2.812 us |    2.644 us |    4.844 us |
| memx_memset             |    2.974 us |    5.596 us |    2.782 us |    5.486 us |
| memx_memset_basic       |    2.955 us |    5.506 us |    2.788 us |    5.532 us |
| memx_memset_libc        |    1.379 us |    2.999 us |    2.740 us |    5.048 us |

  4. armv7-linux-androideabi:

|         `name`          |  `andr:en`  |  `andr:ja`  |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memset              |   92.803 us |  108.650 us |    4.823 us |    8.665 us |
| memx_memset             |    7.644 us |   14.233 us |    7.311 us |   13.658 us |
| memx_memset_basic       |    7.818 us |   15.946 us |    7.104 us |   15.799 us |
| memx_memset_libc        |   60.526 us |  115.040 us |    4.469 us |   10.768 us |


- compile by rustc 1.52.1 (9bc8c42bb 2021-05-09)
- `us` is micro seconds
- `:en` is english haystack.
- `:ja` is japanese haystack.
- `musl` is x86_64-unknown-linux-musl
- bench on intel Q6600 @ 2.40GHz or armv7 1.5GHz
