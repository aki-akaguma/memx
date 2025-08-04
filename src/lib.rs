/*!
`memx` is a Rust library that provides a set of functions for memory manipulation, similar to the standard C library's `<string.h>`, but implemented in pure Rust and optimized for performance.


# Features

- A rich set of memory search, comparison, and manipulation functions.
- Optimized implementations for x86 and x86-64 architectures using SSE2 and AVX2.
- `no_std` compatibility for use in embedded systems and other resource-constrained environments.
- A full suite of iterators for all search functions, providing an idiomatic Rust interface.
*/
//#![no_std]
#![cfg_attr(not(test), no_std)]
use core::cmp::Ordering;

pub mod arch;
pub mod iter;
pub mod mem;
pub(crate) mod utils;

pub(crate) use utils::{B1Dbl, B1Qpl, B1Sgl, B1Tpl};

/// used by memcpy()
#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
pub struct RangeError;

/// This mimics `libc::memchr()`, same as `buf.iter().position(|&x| x == by1)`.
pub fn memchr(buf: &[u8], by1: u8) -> Option<usize> {
    let needle = B1Sgl::new(by1);
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

/// This mimics `libc::memrchr()`, same as `buf.iter().rposition(|&x| x == by1)`.
pub fn memrchr(buf: &[u8], by1: u8) -> Option<usize> {
    let needle = B1Sgl::new(by1);
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

/// This is same as `buf.iter().position(|&x| x == by1 || x == by2)`.
pub fn memchr_dbl(buf: &[u8], by1: u8, by2: u8) -> Option<usize> {
    let needle = B1Dbl::new(by1, by2);
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

/// This is same as `buf.iter().rposition(|&x| x == by1 || x == by2)`.
pub fn memrchr_dbl(buf: &[u8], by1: u8, by2: u8) -> Option<usize> {
    let needle = B1Dbl::new(by1, by2);
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

/// This is same as `buf.iter().position(|&x| x == by1 || x == by2 || x == by3)`.
pub fn memchr_tpl(buf: &[u8], by1: u8, by2: u8, by3: u8) -> Option<usize> {
    let needle = B1Tpl::new(by1, by2, by3);
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

/// This is same as `buf.iter().rposition(|&x| x == by1 || x == by2 || x == by3)`.
pub fn memrchr_tpl(buf: &[u8], by1: u8, by2: u8, by3: u8) -> Option<usize> {
    let needle = B1Tpl::new(by1, by2, by3);
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

/// This is same as `buf.iter().position(|&x| x == by1 || x == by2 || x == by3 || x == by4)`.
pub fn memchr_qpl(buf: &[u8], by1: u8, by2: u8, by3: u8, by4: u8) -> Option<usize> {
    let needle = B1Qpl::new(by1, by2, by3, by4);
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

/// This is same as `buf.iter().rposition(|&x| x == by1 || x == by2 || x == by3 || x == by4)`.
pub fn memrchr_qpl(buf: &[u8], by1: u8, by2: u8, by3: u8, by4: u8) -> Option<usize> {
    let needle = B1Qpl::new(by1, by2, by3, by4);
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

/// This is same as `buf.iter().position(|&x| x != by1)`, not included libc.
pub fn memnechr(buf: &[u8], by1: u8) -> Option<usize> {
    let needle = B1Sgl::new(by1);
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

/// This is same as `buf.iter().rposition(|&x| x != by1)`, not included libc.
pub fn memrnechr(buf: &[u8], by1: u8) -> Option<usize> {
    let needle = B1Sgl::new(by1);
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

/// This is same as `buf.iter().position(|&x| x != by1 && x != by2)`, not included libc.
pub fn memnechr_dbl(buf: &[u8], by1: u8, by2: u8) -> Option<usize> {
    let needle = B1Dbl::new(by1, by2);
    #[rustfmt::skip]
    #[cfg(not(feature = "test_pointer_width"))]
    let r = {
        #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
        { arch::x86::_memnechr_dbl_impl(buf, needle) }
        #[cfg(not(any(target_arch = "x86_64", target_arch = "x86")))]
        { mem::_memnechr_dbl_impl(buf, needle) }
    };
    #[cfg(feature = "test_pointer_width")]
    let r = mem::_memnechr_dbl_impl(buf, needle);
    //
    r
}

/// This is same as `buf.iter().rposition(|&x| x != by1 && x != by2)`, not included libc.
pub fn memrnechr_dbl(buf: &[u8], by1: u8, by2: u8) -> Option<usize> {
    let needle = B1Dbl::new(by1, by2);
    #[rustfmt::skip]
    #[cfg(not(feature = "test_pointer_width"))]
    let r = {
        #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
        { arch::x86::_memrnechr_dbl_impl(buf, needle) }
        #[cfg(not(any(target_arch = "x86_64", target_arch = "x86")))]
        { mem::_memrnechr_dbl_impl(buf, needle) }
    };
    #[cfg(feature = "test_pointer_width")]
    let r = mem::_memrnechr_dbl_impl(buf, needle);
    //
    r
}

/// This is same as `buf.iter().position(|&x| x != by1 && x != by2 && x != by3)`, not included libc.
pub fn memnechr_tpl(buf: &[u8], by1: u8, by2: u8, by3: u8) -> Option<usize> {
    let needle = B1Tpl::new(by1, by2, by3);
    #[rustfmt::skip]
    #[cfg(not(feature = "test_pointer_width"))]
    let r = {
        #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
        { arch::x86::_memnechr_tpl_impl(buf, needle) }
        #[cfg(not(any(target_arch = "x86_64", target_arch = "x86")))]
        { mem::_memnechr_tpl_impl(buf, needle) }
    };
    #[cfg(feature = "test_pointer_width")]
    let r = mem::_memnechr_tpl_impl(buf, needle);
    //
    r
}

/// This is same as `buf.iter().rposition(|&x| x != by1 && x != by2 && x != by3)`, not included libc.
pub fn memrnechr_tpl(buf: &[u8], by1: u8, by2: u8, by3: u8) -> Option<usize> {
    let needle = B1Tpl::new(by1, by2, by3);
    #[rustfmt::skip]
    #[cfg(not(feature = "test_pointer_width"))]
    let r = {
        #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
        { arch::x86::_memrnechr_tpl_impl(buf, needle) }
        #[cfg(not(any(target_arch = "x86_64", target_arch = "x86")))]
        { mem::_memrnechr_tpl_impl(buf, needle) }
    };
    #[cfg(feature = "test_pointer_width")]
    let r = mem::_memrnechr_tpl_impl(buf, needle);
    //
    r
}

/// This is same as `buf.iter().position(|&x| x != by1 && x != by2 && x != by3 && x != by4)`, not included libc.
pub fn memnechr_qpl(buf: &[u8], by1: u8, by2: u8, by3: u8, by4: u8) -> Option<usize> {
    let needle = B1Qpl::new(by1, by2, by3, by4);
    #[rustfmt::skip]
    #[cfg(not(feature = "test_pointer_width"))]
    let r = {
        #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
        { arch::x86::_memnechr_qpl_impl(buf, needle) }
        #[cfg(not(any(target_arch = "x86_64", target_arch = "x86")))]
        { mem::_memnechr_qpl_impl(buf, needle) }
    };
    #[cfg(feature = "test_pointer_width")]
    let r = mem::_memnechr_qpl_impl(buf, needle);
    //
    r
}

/// This is same as `buf.iter().rposition(|&x| x != by1 && x != by2 && x != by3 && x != by4)`, not included libc.
pub fn memrnechr_qpl(buf: &[u8], by1: u8, by2: u8, by3: u8, by4: u8) -> Option<usize> {
    let needle = B1Qpl::new(by1, by2, by3, by4);
    #[rustfmt::skip]
    #[cfg(not(feature = "test_pointer_width"))]
    let r = {
        #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
        { arch::x86::_memrnechr_qpl_impl(buf, needle) }
        #[cfg(not(any(target_arch = "x86_64", target_arch = "x86")))]
        { mem::_memrnechr_qpl_impl(buf, needle) }
    };
    #[cfg(feature = "test_pointer_width")]
    let r = mem::_memrnechr_qpl_impl(buf, needle);
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
