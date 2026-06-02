# Changelog: memx

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.2.2] - 2026-05-27
### Changed
- Update regex crate to 1.12 in xtask.
- Update cpufeatures (0.3) and criterion (0.8) crates in xbench.

### Fixed
- Resolve `clippy::byte_char_slices` lint.
- Resolve `clippy::question_mark` lint.

## [0.2.1] - 2026-05-23
### Added
- Include code review report `docs/review.1.md`.
- Provide documentation for stochastic search weights in `src/utils/mod.rs`.
- Support AArch64 for `prefetch_read_data` using inline assembly.
- Implement generic loop unrolling infrastructure in `src/utils/mod.rs` (`_unroll_loop`, `_unroll_loop_with_prefetch`, `_unroll_loop_dual`, `_unroll_loop_dual_action`, etc.).

### Changed
- Refactor all core memory manipulation modules in `src/mem/` to use generic structured loops and unrolling infrastructure for improved maintainability.
- Refactor all x86 architecture-specific modules in `src/arch/x86/` to use generic unrolling utilities, ensuring consistency across the codebase.
- Optimize alignment logic in both core and architecture-specific modules.
- Move generic loop unrolling infrastructure to a dedicated module `src/utils/unroll.rs` and add detailed developer documentation.

### Fixed
- Restore and verify "rewindable" (non-consuming) iterator semantics to match project requirements.

### Removed
- Remove `#![cfg(not(tarpaulin_include))]` from xbench.

## [0.2.0] - 2025-09-24
### Added
- Include specification documents in `specs/`.
- Provide additional test cases.

### Fixed
- Resolve lifetime issues.
- Address `clippy::useless_concat` lint.
- Address `clippy::needless_range_loop` lint.

## [0.1.32] - 2024-06-19
### Changed
- Remove `test_pointer_width_128`.

### Fixed
- Resolve `clippy::unexpected_cfgs` lint.

## [0.1.31] - 2024-06-09
### Changed
- Rename `config` to `config.toml`.
- Update criterion (0.5) and regex (1.10) crates.
- Support Rust 1.60.0 on GitHub Workflows for testing and building.

### Fixed
- Resolve `clippy::unnecessary_cast` lint.
- Resolve `clippy::useless_vec` lint.
- Address `#[allow(dead_code)]` for `PtrOps` and `BitOrt` traits.

## [0.1.30] - 2023-05-15
### Added
- Implement `memchr_tpl()`, `memrchr_tpl()`, `memchr_qpl()`, and `memrchr_qpl()` functions.
- Implement `memnechr_tpl()`, `memrnechr_tpl()`, `memnechr_qpl()`, and `memrnechr_qpl()` functions.

### Changed
- Update regex dependency to version 1.7.
- Rename `memchr_double()` to `memchr_dbl()`.
- Refactor source code and apply various fine-tunings.

### Removed
- Remove `rustc_version` dependency.

## [0.1.29] - 2023-04-06
### Added
- Support prefetch and alignment checks to improve performance.
- Include `memchr_double()` and `memrchr_double()` benchmarks.
- Provide `grcov` target in `Makefile`.

### Changed
- Rewrite `avx2` switch using a function pointer.
- Update benchmark results and refresh overall benchmarking.
- Refactor `_read_a_native_endian_from_ptr_u64()` function.
- Refactor `PackedU64` bit operations.

### Fixed
- Resolve bug in `cargo test --features test_pointer_width_128`.
- Correct incorrect return values in `mem::_memrchr_impl()`.
- Correct incorrect return values in `mem::_memnechr_impl()`.

## [0.1.28] - 2023-03-15
### Added
- Include `rustfmt::skip` attribute in `x86/x86_set.rs`.

### Changed
- Update benchmark results.
- Re-enable `#![no_std]` support.
- Fine-tune `mem/mem_eq.rs` and `mem/mem_cmp.rs`.
- Refactor benchmark dependencies in `Makefile` and `makefile.build`.

### Fixed
- Resolve invalid dependency `target/stamp.bench-build-gnu` in `makefile.build`.

## [0.1.27] - 2023-03-10
### Added
- Include `#![doc(hidden)]` attribute to `mem/mod.rs` and `arch/mod.rs`.
- Provide `clippy::uninlined_format_args` lint to `Makefile`.
- Support Tarpaulin in `Makefile`.

### Changed
- Update `README.tpl`.
- Refine code coverage and refactor test sources.
- Update benchmark results.
- Tune `mem_cmp()`, `mem_eq()`, and `mem_cpy()` for `x86_64`, `x86`, and `aarch64`.
- Refactor `mem_set()`.
- Update bit operations for `x86_chr()`, `x86_chr_double()`, `x86_rchr()`, `x86_rchr_double()`, `x86_nechr()`, and `x86_rnechr()`.

### Fixed
- Correct use of `$(MAKE)` in build scripts.
- Resolve `clippy::uninlined_format_args` lint issues.

## [0.1.26] - 2023-02-12
### Added
- Provide GitHub Workflows for Ubuntu, macOS, and Windows.
- Include test status badges in `README.tpl`.
- Support Miri for tests.

### Changed
- Refactor `Makefile`.

### Removed
- Remove `COPYING` file.

### Fixed
- Correct `LICENSE-APACHE` and `LICENSE-MIT` files.
- Resolve `clippy::redundant_field_names` lint.
- Resolve `clippy::unnecessary_unwrap` and `clippy::needless_bool` lints.

## [0.1.25] - 2023-01-28
### Added
- Provide `.github/workflows/test.yml`.
- Include test status badges in `README.tpl`.

### Fixed
- Correct Rust version in `Makefile` from `1.66.0` to `1.66.1`.
- Resolve `clippy::redundant_clone` and `clippy::bool_assert_comparison` lints.
- Resolve `clippy::manual_find` and `clippy::needless_borrow` lints.
- Resolve illegal instruction error (SIGILL) caused by `_mm256_cmpeq_epi8()` requiring AVX2.

## [0.1.24] - 2023-01-10
### Added
- Include version difference links in `CHANGELOG.md`.
- Specify `rust-version = "1.56.0"` in `Cargo.toml`.
- Provide `all-test-version` target in `Makefile`.
- Specify `rust-version = "1.57.0"` in `xbench/Cargo.toml`.
- Include project badges in `README.tpl`.

### Changed
- Update `README.md`.
- Modify the `clean` target in `Makefile`.

## [0.1.23] - 2023-01-05
### Added
- Enable `lto = true` in the release profile of `Cargo.toml`.

### Changed
- Update benchmark results.
- Tune up `memcmp()`.
- Revert `memcpy()` to simpler code due to performance issues.

### Fixed
- Resolve clippy lints in `x86_rnechr.rs`, `mem_rchr.rs`, `mem_rnechr.rs`, and `mem_set.rs`.

## [0.1.22] - 2023-01-04
### Added
- Support runtime CPU features using the `cpufeatures` crate.

### Changed
- Update benchmark results.
- Update criterion crate to 0.4.0.
- Change criterion unit from 'uc' to 'μc'.

### Fixed
- Resolve out-of-bounds pointer arithmetic in x86 SSE2 implementations of `x86_chr` and `x86_chr_double`.
- Resolve compilation errors when using `no_std`.

## [0.1.21] - 2022-06-13
### Changed
- Migrate to Rust 2021 edition.

## [0.1.20] - 2022-02-11
### Added
- Implement `memchr_double()`, `memrchr_double()`, `memchr_double_iter()`, and `memrchr_double_iter()` functions.

## [0.1.19] - 2022-02-07
### Fixed
- Resolve bug fixes for AVX implementations.

## [0.1.18] - 2021-11-14
### Changed
- Update `clf` crate to 0.1.3.
- Update `anyhow` (1.0.45) and `cc` (1.0.72) crates.

## [0.1.17] - 2021-09-10
### Changed
- Update `clf` crate to 0.1.2.

## [0.1.16] - 2021-09-10
### Changed
- Update `anyhow` (1.0.43) and `libc` (0.2.101) crates.

## [0.1.15] - 2021-08-06
### Fixed
- Resolve errors identified during `cargo miri test`.

## [0.1.14] - 2021-07-06
### Added
- Implement rustc version check in `build.rs`.
- Provide `rustversion` in `dev-dependencies` for `test_std_memset::test_memset()`.

### Changed
- Rewrite documentation.
- Update licenses.

## [0.1.13] - 2021-06-23
### Added
- Implement `memnechr()` and `memrnechr()` functions.

### Changed
- Apply clippy and fmt to source code.

## [0.1.12] - 2021-06-19
### Added
- Include `rustflags = "-C llvm-args=--disable-memop-opt"` in `.cargo/config`.
- Provide documentation comments.
- Include `xbench = "bench --package xbench --offline"` in `.cargo/config`.

### Changed
- Refresh benchmark results.
- Move benchmarks into the `xbench` workspace.

### Removed
- Remove unnecessary `mod libc`.
- Remove redundant `n: usize` parameter from `memset`.

## [0.1.11] - 2021-06-17
### Added
- Support `clf::cache_line_flush_with_slice()` in benchmarks.

### Changed
- Fine-tune `memcpy()` and other functions.

## [0.1.10] - 2021-06-14
### Changed
- Rename `memx_mem_memXXX` to `memx_memXXX_basic`.
- Update benchmark results.

## [0.1.9] - 2021-06-03
### Added
- Implement `iter::memchr_iter()` and `iter::memrchr_iter()` iterators.
- Implement `memrmem()`, `iter::memmem_iter()`, and `iter::memrmem_iter()` functions.

## [0.1.8] - 2021-06-02
### Added
- Support `#![no_std]`.
- Implement `memrchr()` function.

## [0.1.7] - 2021-05-31
### Added
- Provide tests for `memchr()`, `memcmp()`, `memeq()`, and `memmem()`.

### Changed
- Tune `memcmp()`.

### Fixed
- Resolve overflow error in `x86_chr::_memchr_sse2_impl()`.

## [0.1.6] - 2021-05-29
### Fixed
- Ensure compatibility with Rust 1.41.1.

## [0.1.5] - 2021-05-28
### Changed
- Tune `memcpy()`.
- Tune `memset()` using alignment addressing.
- Tune `x86_cpy::_memcpy_impl()`.

### Fixed
- Correct `memcpy()` implementation on armv7.
- Resolve bug fixes in `mem::mem_set::_start_set_64()` and `mem::mem_set::_start_set_32()`.

## [0.1.4] - 2021-05-22
### Changed
- Refine tunings for `memcmp()`, `memeq()`, and `memmem()`.

## [0.1.3] - 2021-05-18
### Added
- Support benchmarking for armv7 Android.

### Changed
- Refactor x86 source code.

## [0.1.2] - 2021-05-16
### Added
- Include `memchr` crate in memchr benchmarks.
- Support Arm for `memset()`.
- Implement `memmem()` and `memcpy()` functions.

## [0.1.1] - 2021-05-14
### Added
- Implement `memcmp()`, `memeq()`, `memset()`, and `memchr()` functions.

### Changed
- Refactor source code.

## [0.1.0] - 2021-05-12
- Initial release.

[Unreleased]: https://github.com/aki-akaguma/memx/compare/v0.2.2..HEAD
[0.2.2]: https://github.com/aki-akaguma/memx/compare/v0.2.1..v0.2.2
[0.2.1]: https://github.com/aki-akaguma/memx/compare/v0.2.0..v0.2.1
[0.2.0]: https://github.com/aki-akaguma/memx/compare/v0.1.32..v0.2.0
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
