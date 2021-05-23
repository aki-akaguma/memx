## Benchmark results

  1. x86_64-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memset              |    1.682 us |    3.654 us |    1.478 us |    3.294 us |
| memx_memset             |    1.820 us |    3.918 us |    1.685 us |    3.506 us |
| memx_memset_basic       |    2.862 us |    5.167 us |    2.882 us |    5.213 us |
| memx_memset_libc        |    1.814 us |    3.927 us |    1.651 us |    3.490 us |

  2. i686-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memset              |    1.284 us |    2.794 us |    2.598 us |    4.598 us |
| memx_memset             |    1.352 us |    2.901 us |    2.537 us |    4.723 us |
| memx_memset_basic       |    2.922 us |    5.247 us |    2.985 us |    5.387 us |
| memx_memset_libc        |    1.353 us |    2.915 us |    2.651 us |    4.808 us |

  3. i586-unknown-linux-:

|         `name`          |  `gnu:en`   |  `gnu:ja`   |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memset              |    1.334 us |    2.713 us |    2.482 us |    4.529 us |
| memx_memset             |    1.352 us |    2.876 us |    2.510 us |    4.640 us |
| memx_memset_basic       |    8.609 us |   16.132 us |    8.533 us |   16.132 us |
| memx_memset_libc        |    1.341 us |    2.912 us |    2.483 us |    4.649 us |

  4. armv7-linux-androideabi:

|         `name`          |  `andr:en`  |  `andr:ja`  |  `musl:en`  |  `musl:ja`  |
|:------------------------|------------:|------------:|------------:|------------:|
| std_memset              |   49.439 us |   88.989 us |    4.211 us |    7.415 us |
| memx_memset             |   37.369 us |   66.996 us |   37.452 us |   66.808 us |
| memx_memset_basic       |   61.745 us |  111.160 us |   37.122 us |   66.816 us |
| memx_memset_libc        |   49.617 us |   89.291 us |    4.223 us |    7.611 us |


- compile by rustc 1.52.1 (9bc8c42bb 2021-05-09)
- `us` is micro seconds
- `:en` is english haystack.
- `:ja` is japanese haystack.
- `musl` is x86_64-unknown-linux-musl
- bench on intel Q6600 @ 2.40GHz or armv7 1.5GHz
