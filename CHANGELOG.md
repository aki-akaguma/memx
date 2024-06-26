# Changelog: memx

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]


## [0.1.32] (2024-06-19)
### Changed
* `test_pointer_width_128` was removed

### Fixed
* clippy: unexpected_cfgs

## [0.1.31] (2024-06-09)
### Changed
* rename: `config` to `config.toml`
* update crates: criterion(0.5), regex(1.10)
* test support 1.60.0 on github workflows
* build support 1.60.0 on github workflows

### Fixed
* clippy: clippy::unnecessary_cast
* clippy: clippy::useless_vec
* `#[allow(dead_code)]`: pub trait PtrOps, pub(crate) trait BitOrt

## [0.1.30] (2023-05-15)
### Added
* `memchr_tpl()`, `memrchr_tpl()`, `memchr_qpl()`, `memrchr_qpl()`
* `memnechr_tpl()`, `memrnechr_tpl()`, `memnechr_qpl()`, `memrnechr_qpl()`

### Changed
* update depends: `regex(=1.7)`
* rename: `memchr_double()` to `memchr_dbl()`
* refactoring source codes
* many fine tunes

### Removed
* remove depends: `rustc_version`

## [0.1.29] (2023-04-06)
### Added
* support `prefetch` and `alignment check` for good performance.
* `memchr_double()`, `memrchr_double()` into `bench`
* `grcov` target into `Makefile`

### Changed
* The `avx2` switch was rewritten using a function pointer.
* update benchmark results
* refactor function: `_read_a_native_endian_from_ptr_u64()`
* refactor bit operation : `PackedU64`
* renewal benchmark

### Fixed
* some bug: cargo test --features test_pointer_width_128
* sometimes returns incorrect values: `mem::_memrchr_impl()`
* sometimes returns incorrect values: `mem::_memnechr_impl()`

## [0.1.28] (2023-03-15)
### Added
* `rustfmt::skip` into `x86/x86_set.rs`

### Changed
* updated bench results
* reenabled `#![no_std]`
* fine tuned `mem/mem_eq.rs`
* fine tuned `mem/mem_cmp.rs`
* refactor bench dependency: `Makefile`, `makefile.build`

### Fixed
* invalid dependency: `target/stamp.bench-build-gnu` of `makefile.build`

## [0.1.27] (2023-03-10)
### Added
* `#![doc(hidden)]` into `mem/mod.rs` and `arch/mod.rs`
* `clippy::uninlined_format_args` to `Makefile`
* `tarpaulin` supports into `Makefile`

### Changed
* updated `README.tpl`
* refined code coverage
* refactored test sources
* updated benchmark results
* tuned `x86_64`, `x86`, `aarch64`: `mem_cmp()`, `mem_eq()`, `mem_cpy()`
* refactor: `mem_set()`
* bits: `x86_chr()`, `x86_chr_double()`, `x86_rchr()`, `x86_rchr_double()`
* bits: `x86_nechr()`, `x86_rnechr()`

### Fixed
* `$(MAKE)`
* clippy: `uninlined_format_args`

## [0.1.26] (2023-02-12)
### Added
* `.github/workflows/test-ubuntu.yml`
* `.github/workflows/test-macos.yml`
* `.github/workflows/test-windows.yml`
* test status badges into `README.tpl`
* miri supports on tests

### Changed
* refactored `Makefile`

### Removed
* `COPYING`

### Fixed
* `LICENSE-APACHE`, `LICENSE-MIT`
* clippy: `redundant_field_names`, `unnecessary_unwrap`, `needless_bool`

## [0.1.25] (2023-01-28)
### Added
* `.github/workflows/test.yml`
* test status badges into `README.tpl`

### Fixed
* Makefile: rustc version `1.66.0` to `1.66.1`
* clippy: `redundant_clone`, `bool_assert_comparison`, `manual_find`
* clippy: `needless_borrow`
* bug: `signal: 4, SIGILL: illegal instruction`, `_mm256_cmpeq_epi8()` requires `AVX2` instead of `AVX`

## [0.1.24] (2023-01-10)
### Added
* version difference link into `CHANGELOG.md`
* rust-version = "1.56.0" into Cargo.toml
* `all-test-version` target into Makefile
* rust-version = "1.57.0" into xbench/Cargo.toml
* badges into README.tpl

### Changed
* update README.md
* change clean target of Makefile

## [0.1.23] (2023-01-05)
### Added
* lto = true into profile.release of Carg.toml

### Changed
* update benchmark results
* tune up memcmp()
* revert memcpy() to simpe code because of poor performance

### Fixed
* clippy friendly: x86_rnechr.rs, mem_rchr.rs, mem_rnechr.rs, mem_set.rs

## [0.1.22] (2023-01-04)
### Added
* runtime cpu features supports with the `cpufeatures` crate.

### Changed
* update benchmark results
* update crates: criterion(0.4.0)
* change criterion unit 'uc' to 'μc'

### Fixed
* out-of-bounds pointer arithmetic at x86 sse2: x86_chr, x86_chr_double.
* compile error on `no_std`

## [0.1.21] (2022-06-13)
### Changed
* changes to edition 2021

## [0.1.20] (2022-02-11)
### Added
* add `memchr_double()`, `memrchr_double()`, `memchr_double_iter()`, and `memrchr_double_iter()`.

## [0.1.19] (2022-02-07)
### Fixed
* some bugs on `avx`

## [0.1.18] (2021-11-14)
### Changed
* update crates: clf(0.1.3)
* update crates: anyhow(1.0.45), cc(1.0.72)

## [0.1.17] (2021-09-10)
### Changed
* update crates: clf(0.1.2)

## [0.1.16] (2021-09-10)
### Changed
* update crates: anyhow(1.0.43), libc(0.2.101)

## [0.1.15] (2021-08-06)
### Fixed
* errors on `cargo miri test`

## [0.1.14] (2021-07-06)
### Added
* add rustc version check to `build.rs`
* add rustversion to `dev-dependencies` for test_std_memset::test_memset()

### Changed
* rewrite doc
* update licenses

## [0.1.13] (2021-06-23)
### Added
* add fn memnechr() and fn memrnechr()

### Changed
* clippy and fmt the source codes

## [0.1.12] (2021-06-19)
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

## [0.1.11] (2021-06-17)
### Added
* add clf::cache_line_flush_with_slice() into benches

### Changed
* fine tune memcpy() ... etc

## [0.1.10] (2021-06-14)
### Changed
* rename memx_mem_memXXX to memx_memXXX_basic
* update bench results

## [0.1.9] (2021-06-03)
### Added
* add iter::memchr_iter(), iter::memrchr_iter()
* add memrmem(), iter::memmem_iter(), iter::memrmem_iter()

## [0.1.8] (2021-06-02)
### Added
* add support #!\[no_std\]
* add memrchr()

## [0.1.7] (2021-05-31)
### Added
* add test to memchr(), memcmp(), memeq(), memmem()

### Changed
* tune memcmp()

### Fixed
* attempt to subtract with overflow on x86_chr::_memchr_sse2_impl()

## [0.1.6] (2021-05-29)
### Fixed
* compilable rust 1.41.1

## [0.1.5] (2021-05-28)
### Changed
* tune memcpy()
* tune memset(), using alignment address
* tune x86_cpy::_memcpy_impl()

### Fixed
* "fix memcpy() on armv7
* bug fix: mem::mem_set::_start_set_64(), mem::mem_set::_start_set_32()

## [0.1.4] (2021-05-22)
### Changed
* more refine tunings: memcmp(), memeq(), memmem()

## [0.1.3] (2021-05-18)
### Added
* add the benchmarking of armv7 android

### Changed
* refactering source code: x86

## [0.1.2] (2021-05-16)
### Added
* add extern crate memchr into memchr bench
* add arm support to memset()
* add memmem() and memcpy()

## [0.1.1] (2021-05-14)
### Added
* add memcmp(), memeq(), memset(), memchr()

### Changed
* refactering source code

## [0.1.0] (2021-05-12)
* first commit

[Unreleased]: https://github.com/aki-akaguma/memx/compare/v0.1.32..HEAD
[0.1.32]: https://github.com/aki-akaguma/memx/compare/v0.1.31..v0.1.32
[0.1.31]: https://github.com/aki-akaguma/memx/compare/v0.1.30..v0.1.31
[0.1.30]: https://github.com/aki-akaguma/memx/compare/v0.1.29..v0.1.30
[0.1.29]: https://github.com/aki-akaguma/memx/compare/v0.1.28..v0.1.29
[0.1.28]: https://github.com/aki-akaguma/memx/compare/v0.1.27..v0.1.28
[0.1.27]: https://github.com/aki-akaguma/memx/compare/v0.1.26..v0.1.27
[0.1.26]: https://github.com/aki-akaguma/memx/compare/v0.1.25..v0.1.26
[0.1.25]: https://github.com/aki-akaguma/memx/compare/v0.1.24..v0.1.25
[0.1.24]: https://github.com/aki-akaguma/memx/compare/v0.1.23..v0.1.24
[0.1.23]: https://github.com/aki-akaguma/memx/compare/v0.1.22..v0.1.23
[0.1.22]: https://github.com/aki-akaguma/memx/compare/v0.1.21..v0.1.22
[0.1.21]: https://github.com/aki-akaguma/memx/compare/v0.1.20..v0.1.21
[0.1.20]: https://github.com/aki-akaguma/memx/compare/v0.1.19..v0.1.20
[0.1.19]: https://github.com/aki-akaguma/memx/compare/v0.1.18..v0.1.19
[0.1.18]: https://github.com/aki-akaguma/memx/compare/v0.1.17..v0.1.18
[0.1.17]: https://github.com/aki-akaguma/memx/compare/v0.1.16..v0.1.17
[0.1.16]: https://github.com/aki-akaguma/memx/compare/v0.1.15..v0.1.16
[0.1.15]: https://github.com/aki-akaguma/memx/compare/v0.1.14..v0.1.15
[0.1.14]: https://github.com/aki-akaguma/memx/compare/v0.1.13..v0.1.14
[0.1.13]: https://github.com/aki-akaguma/memx/compare/v0.1.12..v0.1.13
[0.1.12]: https://github.com/aki-akaguma/memx/compare/v0.1.11..v0.1.12
[0.1.11]: https://github.com/aki-akaguma/memx/compare/v0.1.10..v0.1.11
[0.1.10]: https://github.com/aki-akaguma/memx/compare/v0.1.9..v0.1.10
[0.1.9]: https://github.com/aki-akaguma/memx/compare/v0.1.8..v0.1.9
[0.1.8]: https://github.com/aki-akaguma/memx/compare/v0.1.7..v0.1.8
[0.1.7]: https://github.com/aki-akaguma/memx/compare/v0.1.6..v0.1.7
[0.1.6]: https://github.com/aki-akaguma/memx/compare/v0.1.5..v0.1.6
[0.1.5]: https://github.com/aki-akaguma/memx/compare/v0.1.4..v0.1.5
[0.1.4]: https://github.com/aki-akaguma/memx/compare/v0.1.3..v0.1.4
[0.1.3]: https://github.com/aki-akaguma/memx/compare/v0.1.2..v0.1.3
[0.1.2]: https://github.com/aki-akaguma/memx/compare/v0.1.1..v0.1.2
[0.1.1]: https://github.com/aki-akaguma/memx/compare/v0.1.0..v0.1.1
[0.1.0]: https://github.com/aki-akaguma/memx/releases/tag/v0.1.0
