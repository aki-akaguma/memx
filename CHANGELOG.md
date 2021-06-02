TBD
===
Unreleased changes. Release notes have not yet been written.

0.1.8 (2021-06-02)
=====

* add support #!\[no_std\]
* add memrchr()

0.1.7 (2021-05-31)
=====

* add test to memchr(), memcmp(), memeq(), memmem()
* bug fix: attempt to subtract with overflow on x86_chr::_memchr_sse2_impl()
* tune memcmp()

0.1.6 (2021-05-29)
=====

* bug fix: compilable rust 1.41.1

0.1.5 (2021-05-28)
=====

* tune memcpy()
* "fix memcpy() on armv7
* tune memset(), using alignment address
* bug fix: mem::mem_set::_start_set_64(), mem::mem_set::_start_set_32()
* tune x86_cpy::_memcpy_impl()

0.1.4 (2021-05-22)
=====

* more refine tunings: memcmp(), memeq(), memmem()

0.1.3 (2021-05-18)
=====

* refactering source code: x86
* add the benchmarking of armv7 android

0.1.2 (2021-05-16)
=====

* add extern crate memchr into memchr bench
* add arm support to memset()
* add memmem() and memcpy()

0.1.1 (2021-05-14)
=====

* refactering source code
* add memcmp(), memeq(), memset(), memchr()

0.1.0 (2021-05-12)
=====

first commit
