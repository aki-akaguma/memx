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
//#![no_std]
#![cfg_attr(not(test), no_std)]
use core::cmp::Ordering;

pub mod arch;
pub mod iter;
pub mod mem;
pub mod utils;

pub use utils::{B1Dbl, B1Qpl, B1Sgl, B1Tpl};

/// used by memcpy()
#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
pub struct RangeError;

/// This mimics `libc::memchr()`, same as `buf.iter().position(|&x| x == c1)`.
pub fn memchr(buf: &[u8], c1: u8) -> Option<usize> {
    let needle = B1Sgl::new(c1);
    #[rustfmt::skip]
    #[cfg(not(feature = "test_pointer_width"))]
    let r = {
        #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
        { arch::x86::_memchr_sgl_impl(buf, needle) }
        #[cfg(not(any(target_arch = "x86_64", target_arch = "x86")))]
        { mem::_memchr_sgl_impl(buf, needle) }
    };
    #[cfg(feature = "test_pointer_width")]
    let r = mem::_memchr_sgl_impl(buf, needle);
    //
    r
}

/// This mimics `libc::memrchr()`, same as `buf.iter().rposition(|&x| x == c1)`.
pub fn memrchr(buf: &[u8], c1: u8) -> Option<usize> {
    let needle = B1Sgl::new(c1);
    #[rustfmt::skip]
    #[cfg(not(feature = "test_pointer_width"))]
    let r = {
        #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
        { arch::x86::_memrchr_sgl_impl(buf, needle) }
        #[cfg(not(any(target_arch = "x86_64", target_arch = "x86")))]
        { mem::_memrchr_sgl_impl(buf, needle) }
    };
    #[cfg(feature = "test_pointer_width")]
    let r = mem::_memrchr_sgl_impl(buf, needle);
    //
    r
}

/// This is same as `buf.iter().position(|&x| x != c)`, not included libc.
pub fn memnechr(buf: &[u8], c1: u8) -> Option<usize> {
    let needle = B1Sgl::new(c1);
    #[rustfmt::skip]
    #[cfg(not(feature = "test_pointer_width"))]
    let r = {
        #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
        { arch::x86::_memnechr_sgl_impl(buf, needle) }
        #[cfg(not(any(target_arch = "x86_64", target_arch = "x86")))]
        { mem::_memnechr_sgl_impl(buf, needle) }
    };
    #[cfg(feature = "test_pointer_width")]
    let r = mem::_memnechr_sgl_impl(buf, needle);
    //
    r
}

/// This is same as `buf.iter().rposition(|&x| x != c)`, not included libc.
pub fn memrnechr(buf: &[u8], c1: u8) -> Option<usize> {
    let needle = B1Sgl::new(c1);
    #[rustfmt::skip]
    #[cfg(not(feature = "test_pointer_width"))]
    let r = {
        #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
        { arch::x86::_memrnechr_sgl_impl(buf, needle) }
        #[cfg(not(any(target_arch = "x86_64", target_arch = "x86")))]
        { mem::_memrnechr_sgl_impl(buf, needle) }
    };
    #[cfg(feature = "test_pointer_width")]
    let r = mem::_memrnechr_sgl_impl(buf, needle);
    //
    r
}

/// This is same as `buf.iter().position(|&x| x == c1 || x == c2)`.
pub fn memchr_dbl(buf: &[u8], c1: u8, c2: u8) -> Option<usize> {
    let needle = B1Dbl::new(c1, c2);
    #[rustfmt::skip]
    #[cfg(not(feature = "test_pointer_width"))]
    let r = {
        #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
        { arch::x86::_memchr_dbl_impl(buf, needle) }
        #[cfg(not(any(target_arch = "x86_64", target_arch = "x86")))]
        { mem::_memchr_dbl_impl(buf, needle) }
    };
    #[cfg(feature = "test_pointer_width")]
    let r = mem::_memchr_dbl_impl(buf, needle);
    //
    r
}

/// This is same as `buf.iter().rposition(|&x| x == c1 || x == c2)`.
pub fn memrchr_dbl(buf: &[u8], c1: u8, c2: u8) -> Option<usize> {
    let needle = B1Dbl::new(c1, c2);
    #[rustfmt::skip]
    #[cfg(not(feature = "test_pointer_width"))]
    let r = {
        #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
        { arch::x86::_memrchr_dbl_impl(buf, needle) }
        #[cfg(not(any(target_arch = "x86_64", target_arch = "x86")))]
        { mem::_memrchr_dbl_impl(buf, needle) }
    };
    #[cfg(feature = "test_pointer_width")]
    let r = mem::_memrchr_dbl_impl(buf, needle);
    //
    r
}

/// This is same as `buf.iter().position(|&x| x == c1 || x == c2 || x == c3)`.
pub fn memchr_tpl(buf: &[u8], c1: u8, c2: u8, c3: u8) -> Option<usize> {
    let needle = B1Tpl::new(c1, c2, c3);
    #[rustfmt::skip]
    #[cfg(not(feature = "test_pointer_width"))]
    let r = {
        #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
        { arch::x86::_memchr_tpl_impl(buf, needle) }
        #[cfg(not(any(target_arch = "x86_64", target_arch = "x86")))]
        { mem::_memchr_tpl_impl(buf, needle) }
    };
    #[cfg(feature = "test_pointer_width")]
    let r = mem::_memchr_tpl_impl(buf, needle);
    //
    r
}

/// This is same as `buf.iter().rposition(|&x| x == c1 || x == c2 || x == c3)`.
pub fn memrchr_tpl(buf: &[u8], c1: u8, c2: u8, c3: u8) -> Option<usize> {
    let needle = B1Tpl::new(c1, c2, c3);
    #[rustfmt::skip]
    #[cfg(not(feature = "test_pointer_width"))]
    let r = {
        #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
        { arch::x86::_memrchr_tpl_impl(buf, needle) }
        #[cfg(not(any(target_arch = "x86_64", target_arch = "x86")))]
        { mem::_memrchr_tpl_impl(buf, needle) }
    };
    #[cfg(feature = "test_pointer_width")]
    let r = mem::_memrchr_tpl_impl(buf, needle);
    //
    r
}

/// This is same as `buf.iter().position(|&x| x == c1 || x == c2 || x == c3 || x == c4)`.
pub fn memchr_qpl(buf: &[u8], c1: u8, c2: u8, c3: u8, c4: u8) -> Option<usize> {
    let needle = B1Qpl::new(c1, c2, c3, c4);
    #[rustfmt::skip]
    #[cfg(not(feature = "test_pointer_width"))]
    let r = {
        #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
        { arch::x86::_memchr_qpl_impl(buf, needle) }
        #[cfg(not(any(target_arch = "x86_64", target_arch = "x86")))]
        { mem::_memchr_qpl_impl(buf, needle) }
    };
    #[cfg(feature = "test_pointer_width")]
    let r = mem::_memchr_qpl_impl(buf, needle);
    //
    r
}

/// This is same as `buf.iter().rposition(|&x| x == c1 || x == c2 || x == c3 || x == c4)`.
pub fn memrchr_qpl(buf: &[u8], c1: u8, c2: u8, c3: u8, c4: u8) -> Option<usize> {
    let needle = B1Qpl::new(c1, c2, c3, c4);
    #[rustfmt::skip]
    #[cfg(not(feature = "test_pointer_width"))]
    let r = {
        #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
        { arch::x86::_memrchr_qpl_impl(buf, needle) }
        #[cfg(not(any(target_arch = "x86_64", target_arch = "x86")))]
        { mem::_memrchr_qpl_impl(buf, needle) }
    };
    #[cfg(feature = "test_pointer_width")]
    let r = mem::_memrchr_qpl_impl(buf, needle);
    //
    r
}

/// This mimics `libc::memcmp()`, same as `a.cmp(&b)`.
pub fn memcmp(a: &[u8], b: &[u8]) -> Ordering {
    #[rustfmt::skip]
    #[cfg(not(feature = "test_pointer_width"))]
    let r = {
        #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
        { arch::x86::_memcmp_impl(a, b) }
        #[cfg(not(any(target_arch = "x86_64", target_arch = "x86")))]
        { mem::_memcmp_impl(a, b) }
    };
    #[cfg(feature = "test_pointer_width")]
    let r = mem::_memcmp_impl(a, b);
    //
    r
}

/// This mimics `libc::bcmp()`, same as `a == b`.
pub fn memeq(a: &[u8], b: &[u8]) -> bool {
    #[rustfmt::skip]
    #[cfg(not(feature = "test_pointer_width"))]
    let r = {
        #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
        { arch::x86::_memeq_impl(a, b) }
        #[cfg(not(any(target_arch = "x86_64", target_arch = "x86")))]
        { mem::_memeq_impl(a, b) }
    };
    #[cfg(feature = "test_pointer_width")]
    let r = mem::_memeq_impl(a, b);
    //
    r
}

/// This mimics `libc::memmem()`, same as `(haystack as &str).find(needle as &str)` or `haystack.windows(needle.len()).position(|window| window == needle)`.
///
/// This `memmem()` function is implemented using stochastic naive algorithm.
/// ref.) [The optimized naive string-search algorithm](https://crates.io/crates/naive_opt)
///
pub fn memmem(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    #[rustfmt::skip]
    #[cfg(not(feature = "test_pointer_width"))]
    let r = {
        #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
        { arch::x86::_memmem_impl(haystack, needle) }
        #[cfg(not(any(target_arch = "x86_64", target_arch = "x86")))]
        { mem::_memmem_impl(haystack, needle) }
    };
    #[cfg(feature = "test_pointer_width")]
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
    #[rustfmt::skip]
    #[cfg(not(feature = "test_pointer_width"))]
    let r = {
        #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
        { arch::x86::_memrmem_impl(haystack, needle) }
        #[cfg(not(any(target_arch = "x86_64", target_arch = "x86")))]
        { mem::_memrmem_impl(haystack, needle) }
    };
    #[cfg(feature = "test_pointer_width")]
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
    #[rustfmt::skip]
    #[cfg(not(feature = "test_pointer_width"))]
    let r = {
        #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
        { arch::x86::_memcpy_impl(dst, src) }
        #[cfg(not(any(target_arch = "x86_64", target_arch = "x86")))]
        { mem::_memcpy_impl(dst, src) }
    };
    #[cfg(feature = "test_pointer_width")]
    let r = mem::_memcpy_impl(dst, src);
    //
    r
}

/// This mimics `libc::memset()`, same as `buf.fill(c)`.
///
/// The `memset()` function fills `buf` with the `c`.
pub fn memset(buf: &mut [u8], c: u8) {
    #[rustfmt::skip]
    #[cfg(not(feature = "test_pointer_width"))]
    {
        #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
        arch::x86::_memset_impl(buf, c);
        #[cfg(not(any(target_arch = "x86_64", target_arch = "x86")))]
        mem::_memset_impl(buf, c);
    };
    #[cfg(feature = "test_pointer_width")]
    mem::_memset_impl(buf, c);
}
