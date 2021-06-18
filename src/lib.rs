//!
//! `memx` minics libc.
//!

use core::cmp::Ordering;

pub mod arch;
pub mod iter;
pub mod mem;

/// used by memcpy()
#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
pub struct RangeError;

/// This mimics `libc::memchr()`, same as `buf.iter().position(|&x| x == c)`.
pub fn memchr(buf: &[u8], c: u8) -> Option<usize> {
    #[cfg(all(
        any(target_arch = "x86_64", target_arch = "x86"),
        any(target_feature = "sse2", target_feature = "avx")
    ))]
    let r = arch::x86::_memchr_impl(buf, c);
    //
    #[cfg(any(
        not(any(target_arch = "x86_64", target_arch = "x86",)),
        not(any(target_feature = "sse2", target_feature = "avx"))
    ))]
    let r = mem::_memchr_impl(buf, c);
    //
    r
}

/// This mimics `libc::memrchr()`, same as `buf.iter().rposition(|&x| x == c)`.
pub fn memrchr(buf: &[u8], c: u8) -> Option<usize> {
    #[cfg(all(
        any(target_arch = "x86_64", target_arch = "x86"),
        any(target_feature = "sse2", target_feature = "avx")
    ))]
    let r = arch::x86::_memrchr_impl(buf, c);
    //
    #[cfg(any(
        not(any(target_arch = "x86_64", target_arch = "x86",)),
        not(any(target_feature = "sse2", target_feature = "avx"))
    ))]
    let r = mem::_memrchr_impl(buf, c);
    //
    r
}

/// This mimics `libc::memcmp()`, same as `a.cmp(&b)`.
pub fn memcmp(a: &[u8], b: &[u8]) -> Ordering {
    /*
     * why is sse2 slower ?
     *
    #[cfg(all(
        any(target_arch = "x86_64", target_arch = "x86"),
        any(target_feature = "sse2", target_feature = "avx")
    ))]
    let r = arch::x86::_memcmp_impl(a, b);
    //
    #[cfg(any(
        not(any(target_arch = "x86_64", target_arch = "x86",)),
        not(any(target_feature = "sse2", target_feature = "avx"))
    ))]
    let r = mem::_memcmp_impl(a, b);
    */
    let r = mem::_memcmp_impl(a, b);
    //
    r
}

/// This mimics `libc::bcmp()`, same as `a == b`.
pub fn memeq(a: &[u8], b: &[u8]) -> bool {
    let r = mem::_memeq_impl(a, b);
    //
    r
}

/// This mimics `libc::memmem()`, same as `(haystack as &str).find(needle as &str)`.
pub fn memmem(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    #[cfg(all(
        any(target_arch = "x86_64", target_arch = "x86"),
        any(target_feature = "sse2", target_feature = "avx")
    ))]
    let r = arch::x86::_memmem_impl(haystack, needle);
    #[cfg(any(
        not(any(target_arch = "x86_64", target_arch = "x86",)),
        not(any(target_feature = "sse2", target_feature = "avx"))
    ))]
    let r = mem::_memmem_impl(haystack, needle);
    //
    r
}

/// This mimics `libc::memrmem()`, same as `(haystack as &str).rfind(needle as &str)`.
pub fn memrmem(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    #[cfg(all(
        any(target_arch = "x86_64", target_arch = "x86"),
        any(target_feature = "sse2", target_feature = "avx")
    ))]
    let r = arch::x86::_memrmem_impl(haystack, needle);
    #[cfg(any(
        not(any(target_arch = "x86_64", target_arch = "x86",)),
        not(any(target_feature = "sse2", target_feature = "avx"))
    ))]
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
    #[cfg(all(
        any(target_arch = "x86_64", target_arch = "x86"),
        any(target_feature = "sse2", target_feature = "avx")
    ))]
    let r = arch::x86::_memcpy_impl(dst, src);
    #[cfg(any(
        not(any(target_arch = "x86_64", target_arch = "x86",)),
        not(any(target_feature = "sse2", target_feature = "avx"))
    ))]
    let r = mem::_memcpy_impl(dst, src);
    //
    r
}

/// This mimics `libc::memset()`, same as `buf.fill(c)`.
///
/// The `memset()` function fills `buf` with the `c`.
pub fn memset(buf: &mut [u8], c: u8) {
    #[cfg(all(
        any(target_arch = "x86_64", target_arch = "x86"),
        any(target_feature = "sse2", target_feature = "avx")
    ))]
    arch::x86::_memset_impl(buf, c);
    #[cfg(any(
        not(any(target_arch = "x86_64", target_arch = "x86",)),
        not(any(target_feature = "sse2", target_feature = "avx"))
    ))]
    mem::_memset_impl(buf, c);
}

#[inline(always)]
pub(crate) fn plus_offset_from(ptr: *const u8, origin: *const u8) -> usize {
    (ptr as usize) - (origin as usize)
}

// ascii stochastics
pub(crate) const _ASCII_STOCHAS: [u8; 128] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    255, 0, 2, 0, 0, 0, 0, 0, 1, 1, 0, 3, 6, 14, 19, 1, 3, 4, 3, 2, 2, 1, 1, 1, 1, 1, 2, 0, 0, 1,
    0, 0, 0, 4, 1, 5, 2, 4, 3, 0, 1, 5, 0, 0, 2, 3, 3, 2, 5, 0, 4, 6, 6, 1, 0, 0, 0, 0, 0, 1, 0, 1,
    0, 1, 0, 39, 7, 20, 19, 69, 11, 9, 18, 39, 0, 2, 18, 12, 38, 38, 12, 1, 34, 35, 50, 13, 5, 5,
    2, 7, 0, 0, 2, 0, 0, 0,
];

/*
 * Refer.
 *   https://mmi.hatenablog.com/entry/2017/07/27/230005
 *   you should have memcpy(), memcmp(), memset() on nostd environments
*/
