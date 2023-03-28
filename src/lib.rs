/*!
`memx` minics libc.

This crate is implemented memory functions like libc memcmp(), memchr(),
memmem(), memcpy(), memset().

# Features

- Rewriting with rust lang.
- minimum support rustc 1.56.1 (59eed8a2a 2021-11-01)

# Todo

- [ ] Support the zero overhead trait.
- [x] Support more fast routine on armv7-android
- [x] Support more fast routine on x86_64
- [x] Support #!\[no_std\]

# Support status of miri :: rustc 1.68.0-nightly (77429957a 2023-01-01)

Ok lists:

- cargo +nightly miri test --target=x86_64-unknown-linux-gnu
- cargo +nightly miri test --target=i686-unknown-linux-gnu
- cargo +nightly miri test --target=i586-unknown-linux-gnu
- cargo +nightly miri test --target=aarch64-unknown-linux-gnu
- cargo +nightly miri test --target=armv7-unknown-linux-gnueabihf

Failed lists:

- nothing

*/
#![no_std]
use core::cmp::Ordering;

pub mod arch;
pub mod iter;
pub mod mem;
mod utils;

/// used by memcpy()
#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
pub struct RangeError;

/// This mimics `libc::memchr()`, same as `buf.iter().position(|&x| x == c)`.
pub fn memchr(buf: &[u8], c: u8) -> Option<usize> {
    #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
    let r = arch::x86::_memchr_impl(buf, c);
    //
    #[cfg(not(any(target_arch = "x86_64", target_arch = "x86")))]
    let r = mem::_memchr_impl(buf, c);
    //
    r
}

/// This mimics `libc::memrchr()`, same as `buf.iter().rposition(|&x| x == c)`.
pub fn memrchr(buf: &[u8], c: u8) -> Option<usize> {
    #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
    let r = arch::x86::_memrchr_impl(buf, c);
    //
    #[cfg(not(any(target_arch = "x86_64", target_arch = "x86")))]
    let r = mem::_memrchr_impl(buf, c);
    //
    r
}

/// This is same as `buf.iter().position(|&x| x != c)`, not included libc.
pub fn memnechr(buf: &[u8], c: u8) -> Option<usize> {
    #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
    let r = arch::x86::_memnechr_impl(buf, c);
    //
    #[cfg(not(any(target_arch = "x86_64", target_arch = "x86")))]
    let r = mem::_memnechr_impl(buf, c);
    //
    r
}

/// This is same as `buf.iter().rposition(|&x| x != c)`, not included libc.
pub fn memrnechr(buf: &[u8], c: u8) -> Option<usize> {
    #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
    let r = arch::x86::_memrnechr_impl(buf, c);
    //
    #[cfg(not(any(target_arch = "x86_64", target_arch = "x86")))]
    let r = mem::_memrnechr_impl(buf, c);
    //
    r
}

/// This is same as `buf.iter().position(|&x| x == c1 || x == c2)`.
pub fn memchr_double(buf: &[u8], c1: u8, c2: u8) -> Option<usize> {
    #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
    let r = arch::x86::_memchr_double_impl(buf, c1, c2);
    //
    #[cfg(not(any(target_arch = "x86_64", target_arch = "x86")))]
    let r = mem::_memchr_double_impl(buf, c1, c2);
    //
    r
}

/// This is same as `buf.iter().rposition(|&x| x == c1 || x == c2)`.
pub fn memrchr_double(buf: &[u8], c1: u8, c2: u8) -> Option<usize> {
    #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
    let r = arch::x86::_memrchr_double_impl(buf, c1, c2);
    //
    #[cfg(not(any(target_arch = "x86_64", target_arch = "x86")))]
    let r = mem::_memrchr_double_impl(buf, c1, c2);
    //
    r
}

/// This mimics `libc::memcmp()`, same as `a.cmp(&b)`.
pub fn memcmp(a: &[u8], b: &[u8]) -> Ordering {
    mem::_memcmp_impl(a, b)
    /*
    sse2 or avx2 are slower, but I don't know why.
    #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
    let r = arch::x86::_memcmp_impl(a, b);
    //
    #[cfg(not(any(target_arch = "x86_64", target_arch = "x86")))]
    let r = mem::_memcmp_impl(a, b);
    //
    r
    */
}

/// This mimics `libc::bcmp()`, same as `a == b`.
pub fn memeq(a: &[u8], b: &[u8]) -> bool {
    mem::_memeq_impl(a, b)
    /*
    sse2 or avx2 are slower, but I don't know why.
    #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
    let r = arch::x86::_memeq_impl(a, b);
    //
    #[cfg(not(any(target_arch = "x86_64", target_arch = "x86")))]
    let r = mem::_memeq_impl(a, b);
    //
    r
    */
}

/// This mimics `libc::memmem()`, same as `(haystack as &str).find(needle as &str)` or `haystack.windows(needle.len()).position(|window| window == needle)`.
///
/// This `memmem()` function is implemented using stochastic naive algorithm.
/// ref.) [The optimized naive string-search algorithm](https://crates.io/crates/naive_opt)
///
pub fn memmem(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
    let r = arch::x86::_memmem_impl(haystack, needle);
    //
    #[cfg(not(any(target_arch = "x86_64", target_arch = "x86")))]
    let r = mem::_memmem_impl(haystack, needle);
    //
    r
}

/// This mimics `libc::memrmem()`, same as `(haystack as &str).rfind(needle as &str)` or `haystack.windows(needle.len()).rposition(|window| window == needle)`.
///
/// This `memrmem()` function is implemented using stochastic naive algorithm.
/// ref.) [The optimized naive string-search algorithm](https://crates.io/crates/naive_opt)
///
pub fn memrmem(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
    let r = arch::x86::_memrmem_impl(haystack, needle);
    //
    #[cfg(not(any(target_arch = "x86_64", target_arch = "x86")))]
    let r = mem::_memrmem_impl(haystack, needle);
    //
    r
}

/// This mimics `libc::memcpy()`, same as `dst = src`.
///
/// The `memcpy()` function copies `src.len()` bytes from the `src` to the `dst`.
/// The `src` and the `dst` must not overlape.
/// This function return `Err(RangeError)` if `dst.len() < src.len()`, otherwise return `Ok(())`
pub fn memcpy(dst: &mut [u8], src: &[u8]) -> Result<(), RangeError> {
    mem::_memcpy_impl(dst, src)
    /*
    sse2 or avx2 are slower, but I don't know why.
    #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
    let r = arch::x86::_memcpy_impl(dst, src);
    //
    #[cfg(not(any(target_arch = "x86_64", target_arch = "x86")))]
    let r = mem::_memcpy_impl(dst, src);
    //
    r
    */
}

/// This mimics `libc::memset()`, same as `buf.fill(c)`.
///
/// The `memset()` function fills `buf` with the `c`.
pub fn memset(buf: &mut [u8], c: u8) {
    #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
    arch::x86::_memset_impl(buf, c);
    //
    #[cfg(not(any(target_arch = "x86_64", target_arch = "x86")))]
    mem::_memset_impl(buf, c);
}

// ascii stochastics
#[rustfmt::skip]
pub(crate) const _ASCII_STOCHAS: [u8; 128] = [
    // 0x00: NUL,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    // 0x10: DEL,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    // 0x20: SP, 0x21: b'!'
    255, 0, 2, 0, 0, 0, 0, 0, 1, 1, 0, 3, 6, 14, 19, 1,
    // 0x30: b'0'
    3, 4, 3, 2, 2, 1, 1, 1, 1, 1, 2, 0, 0, 1, 0, 0,
    // 0x40: b'@', 0x41: b'A', 0x4f: 'O',
    0, 4, 1, 5, 2, 4, 3, 0, 1, 5, 0, 0, 2, 3, 3, 2,
    // 0x50: b'P', 0x5a: b'Z'
    5, 0, 4, 6, 6, 1, 0, 0, 0, 0, 0, 1, 0, 1, 0, 1,
    // 0x60: b'`', 0x61: b'a', 0x4f: 'o',
    0, 39, 7, 20, 19, 69, 11, 9, 18, 39, 0, 2, 18, 12, 38, 38,
    // 0x70: b'p', 0x7a: b'z'
    12, 1, 34, 35, 50, 13, 5, 5, 2, 7, 0, 0, 2, 0, 0, 0,
];
