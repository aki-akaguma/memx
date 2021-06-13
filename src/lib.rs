use core::cmp::Ordering;

pub mod arch;
pub mod iter;
//pub mod libc;
pub mod mem;

#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
pub struct RangeError;

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

pub fn memeq(a: &[u8], b: &[u8]) -> bool {
    let r = mem::_memeq_impl(a, b);
    //
    r
}

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

pub fn memset(buf: &mut [u8], c: u8, n: usize) -> Result<(), RangeError> {
    #[cfg(all(
        any(target_arch = "x86_64", target_arch = "x86"),
        any(target_feature = "sse2", target_feature = "avx")
    ))]
    let r = arch::x86::_memset_impl(buf, c, n);
    #[cfg(any(
        not(any(target_arch = "x86_64", target_arch = "x86",)),
        not(any(target_feature = "sse2", target_feature = "avx"))
    ))]
    let r = mem::_memset_impl(buf, c, n);
    //
    r
}

#[inline(always)]
pub(crate) fn plus_offset_from(ptr: *const u8, origin: *const u8) -> usize {
    (ptr as usize) - (origin as usize)
}

/*
 * Refer.
 *   https://mmi.hatenablog.com/entry/2017/07/27/230005
 *   you should have memcpy(), memcmp(), memset() on nostd environments
*/
