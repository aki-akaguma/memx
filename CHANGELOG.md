# Changelog: memx

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased


## 0.1.23 (2023-01-05)
### Added
* lto = true into profile.release of Carg.toml

### Changed
* update benchmark results
* tune up memcmp()
* revert memcpy() to simpe code because of poor performance

### Fixed
* clippy friendly: x86_rnechr.rs, mem_rchr.rs, mem_rnechr.rs, mem_set.rs

## 0.1.22 (2023-01-04)
### Added
* runtime cpu features supports with the `cpufeatures` crate.

### Changed
* update benchmark results
* update crates: criterion(0.4.0)
* change criterion unit 'uc' to 'μc'

### Fixed
* out-of-bounds pointer arithmetic at x86 sse2: x86_chr, x86_chr_double.
* compile error on `no_std`

## 0.1.21 (2022-06-13)
### Changed
* changes to edition 2021

## 0.1.20 (2022-02-11)
### Added
* add `memchr_double()`, `memrchr_double()`, `memchr_double_iter()`, and `memrchr_double_iter()`.

## 0.1.19 (2022-02-07)
### Fixed
* some bugs on `avx`

## 0.1.18 (2021-11-14)
### Changed
* update crates: clf(0.1.3)
* update crates: anyhow(1.0.45), cc(1.0.72)

## 0.1.17 (2021-09-10)
### Changed
* update crates: clf(0.1.2)

## 0.1.16 (2021-09-10)
### Changed
* update crates: anyhow(1.0.43), libc(0.2.101)

## 0.1.15 (2021-08-06)
### Fixed
* errors on `cargo miri test`

## 0.1.14 (2021-07-06)
### Added
* add rustc version check to `build.rs`
* add rustversion to `dev-dependencies` for test_std_memset::test_memset()

### Changed
* rewrite doc
* update licenses

## 0.1.13 (2021-06-23)
### Added
* add fn memnechr() and fn memrnechr()

### Changed
* clippy and fmt the source codes

## 0.1.12 (2021-06-19)
### Added
* add `rustflags = "-C llvm-args=--disable-memop-opt"` into the `.cargo/config`
* add doc comments
* add `xbench = "bench --package xbench --offline"` into the `.cargo/config`

### Changed
* refresh benchmark results
* move `benches` into `xbench` workspace

### Removed
* remove unnesesary `mod libc`
* remove the redundancy param: `n: usize`
  - old: `pub fn memset(buf: &mut [u8], c: u8, n: usize) -> Result<(), RangeError>;`
  - new: `pub fn memset(buf: &mut [u8], c: u8);`

## 0.1.11 (2021-06-17)
### Added
* add clf::cache_line_flush_with_slice() into benches

### Changed
* fine tune memcpy() ... etc

## 0.1.10 (2021-06-14)
### Changed
* rename memx_mem_memXXX to memx_memXXX_basic
* update bench results

## 0.1.9 (2021-06-03)
### Added
* add iter::memchr_iter(), iter::memrchr_iter()
* add memrmem(), iter::memmem_iter(), iter::memrmem_iter()

## 0.1.8 (2021-06-02)
### Added
* add support #!\[no_std\]
* add memrchr()

## 0.1.7 (2021-05-31)
### Added
* add test to memchr(), memcmp(), memeq(), memmem()

### Changed
* tune memcmp()

### Fixed
* attempt to subtract with overflow on x86_chr::_memchr_sse2_impl()

## 0.1.6 (2021-05-29)
### Fixed
* compilable rust 1.41.1

## 0.1.5 (2021-05-28)
### Changed
* tune memcpy()
* tune memset(), using alignment address
* tune x86_cpy::_memcpy_impl()

### Fixed
* "fix memcpy() on armv7
* bug fix: mem::mem_set::_start_set_64(), mem::mem_set::_start_set_32()

## 0.1.4 (2021-05-22)
### Changed
* more refine tunings: memcmp(), memeq(), memmem()

## 0.1.3 (2021-05-18)
### Added
* add the benchmarking of armv7 android

### Changed
* refactering source code: x86

## 0.1.2 (2021-05-16)
### Added
* add extern crate memchr into memchr bench
* add arm support to memset()
* add memmem() and memcpy()

## 0.1.1 (2021-05-14)
### Added
* add memcmp(), memeq(), memset(), memchr()

### Changed
* refactering source code

## 0.1.0 (2021-05-12)
* first commit
