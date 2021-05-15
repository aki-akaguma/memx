use std::cmp::Ordering;

mod arch;
mod libc;
mod mem;

#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
pub struct RangeError;

pub fn memchr(buf: &[u8], c: u8) -> Option<usize> {
    #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
    let r = arch::x86::_memchr_impl(buf, c);
    //
    #[cfg(not(any(target_arch = "x86_64", target_arch = "x86",)))]
    let r = mem::_memchr_impl(buf, c);
    //
    r
}

pub fn memchr_basic(buf: &[u8], c: u8) -> Option<usize> {
    mem::_memchr_impl(buf, c)
}

pub fn memchr_libc(buf: &[u8], c: u8) -> Option<usize> {
    libc::_memchr_impl(buf, c)
}

pub fn memcmp(a: &[u8], b: &[u8]) -> Ordering {
    #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
    let r = arch::x86::_memcmp_impl(a, b);
    //
    #[cfg(target_arch = "aarch64")]
    let r = libc::_memcmp_impl(a, b);
    //
    #[cfg(not(any(
        target_arch = "x86_64",
        target_arch = "x86",
        target_arch = "aarch64",
    )))]
    let r = mem::_memcmp_impl(a, b);
    //
    r
}

pub fn memcmp_basic(a: &[u8], b: &[u8]) -> Ordering {
    mem::_memcmp_impl(a, b)
}

pub fn memcmp_libc(a: &[u8], b: &[u8]) -> Ordering {
    libc::_memcmp_impl(a, b)
}

pub fn memeq(a: &[u8], b: &[u8]) -> bool {
    #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
    let r = arch::x86::_memeq_impl(a, b);
    //
    #[cfg(target_arch = "aarch64")]
    let r = libc::_memeq_impl(a, b);
    //
    #[cfg(not(any(
        target_arch = "x86_64",
        target_arch = "x86",
        target_arch = "aarch64",
    )))]
    let r = mem::_memeq_impl(a, b);
    //
    r
}

pub fn memeq_basic(a: &[u8], b: &[u8]) -> bool {
    mem::_memeq_impl(a, b)
}

pub fn memeq_libc(a: &[u8], b: &[u8]) -> bool {
    libc::_memeq_impl(a, b)
}

pub fn memset(buf: &mut [u8], c: u8, n: usize) -> Result<(), RangeError> {
    #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
    let r = arch::x86::_memset_impl(buf, c, n);
    //
    #[cfg(target_arch = "arm")]
    let r = mem::_memset_impl(buf, c, n);
    //
    #[cfg(not(any(target_arch = "x86_64", target_arch = "x86", target_arch = "arm")))]
    let r = libc::_memset_impl(buf, c, n);
    //
    r
}

pub fn memset_basic(buf: &mut [u8], c: u8, n: usize) -> Result<(), RangeError> {
    mem::_memset_impl(buf, c, n)
}

pub fn memset_libc(buf: &mut [u8], c: u8, n: usize) -> Result<(), RangeError> {
    libc::_memset_impl(buf, c, n)
}

/*
pub fn memchr(buf: &[u8], c: u8) -> Option<usize> {
    if is_x86_feature_detected!("avx") {
        unsafe { memchr_avx(buf, c) }
    } else if is_x86_feature_detected!("sse2") {
        unsafe {  memchr_sse2(buf, c) }
    } else {
        memchr_basic(buf, c)
    }
}

pub fn memchr_basic(buf: &[u8], c: u8) -> Option<usize> {
    basic::memchr_impl(buf, c)
}

#[target_feature(enable = "sse2")]
pub unsafe fn memchr_sse2(buf: &[u8], c: u8) -> Option<usize> {
    basic::memchr_impl(buf, c)
}

#[target_feature(enable = "avx")]
pub unsafe fn memchr_avx(buf: &[u8], c: u8) -> Option<usize> {
    basic::memchr_impl(buf, c)
}

pub fn memmem(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    if is_x86_feature_detected!("avx") {
        unsafe { memmem_avx(haystack, needle) }
    } else if is_x86_feature_detected!("sse2") {
        unsafe {  memmem_sse2(haystack, needle) }
    } else {
        memmem_basic(haystack, needle)
    }
}

pub fn memmem_basic(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    basic::memmem_impl(haystack, needle)
}

#[target_feature(enable = "sse2")]
pub unsafe fn memmem_sse2(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    basic::memmem_impl(haystack, needle)
}

#[target_feature(enable = "avx")]
pub unsafe fn memmem_avx(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    basic::memmem_impl(haystack, needle)
}

pub fn memcpy(dst: &mut [u8], src: &[u8], n: usize) -> Result<(), RangeError> {
    if is_x86_feature_detected!("avx") {
        unsafe { memcpy_avx(dst, src, n) }
    } else if is_x86_feature_detected!("sse2") {
        unsafe {  memcpy_sse2(dst, src, n) }
    } else {
        memcpy_basic(dst, src, n)
    }
}

pub fn memcpy_basic(dst: &mut [u8], src: &[u8], n: usize) -> Result<(), RangeError> {
    basic::memcpy_impl(dst, src, n)
}

#[target_feature(enable = "sse2")]
pub unsafe fn memcpy_sse2(dst: &mut [u8], src: &[u8], n: usize) -> Result<(), RangeError> {
    basic::memcpy_impl(dst, src, n)
}

#[target_feature(enable = "avx")]
pub unsafe fn memcpy_avx(dst: &mut [u8], src: &[u8], n: usize) -> Result<(), RangeError> {
    basic::memcpy_impl(dst, src, n)
}
*/

/*
 * Refer.
 *   https://mmi.hatenablog.com/entry/2017/07/27/230005
 *   you should have memcpy(), memcmp(), memset() on nostd environments
*/
