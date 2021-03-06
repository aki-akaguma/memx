TBD
===
Unreleased changes. Release notes have not yet been written.

0.1.21 (2022-06-13)
=====

* changes to edition 2021

0.1.20 (2022-02-11)
=====

* add `memchr_double()`, `memrchr_double()`, `memchr_double_iter()`, and `memrchr_double_iter()`.

0.1.19 (2022-02-07)
=====

* bug fix: some bugs on `avx`

0.1.18 (2021-11-14)
=====

* update crates: clf(0.1.3)
* update crates: anyhow(1.0.45), cc(1.0.72)

0.1.17 (2021-09-10)
=====

* update crates: clf(0.1.2)

0.1.16 (2021-09-10)
=====

* update crates: anyhow(1.0.43), libc(0.2.101)

0.1.15 (2021-08-06)
=====

* fix bug: errors on `cargo miri test`

0.1.14 (2021-07-06)
=====

* add rustc version check to `build.rs`
* add rustversion to `dev-dependencies` for test_std_memset::test_memset()
* rewrite doc
* update licenses

0.1.13 (2021-06-23)
=====

* add fn memnechr() and fn memrnechr()
* clippy and fmt the source codes

0.1.12 (2021-06-19)
=====

* add `rustflags = "-C llvm-args=--disable-memop-opt"` into the `.cargo/config`
* remove unnesesary `mod libc`
* refresh benchmark results
* add doc comments
* remove the redundancy param: `n: usize`
  - old: `pub fn memset(buf: &mut [u8], c: u8, n: usize) -> Result<(), RangeError>;`
  - new: `pub fn memset(buf: &mut [u8], c: u8);`
* move `benches` into `xbench` workspace
* add `xbench = "bench --package xbench --offline"` into the `.cargo/config`

0.1.11 (2021-06-17)
=====

* add clf::cache_line_flush_with_slice() into benches
* fine tune memcpy() ... etc

0.1.10 (2021-06-14)
=====

* rename memx_mem_memXXX to memx_memXXX_basic
* update bench results

0.1.9 (2021-06-03)
=====

* add iter::memchr_iter(), iter::memrchr_iter()
* add memrmem(), iter::memmem_iter(), iter::memrmem_iter()

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
