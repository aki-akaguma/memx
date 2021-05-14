use crate::mem as basic;

#[inline(always)]
#[cfg(target_arch = "x86_64")]
pub fn _memchr_impl(buf: &[u8], c: u8) -> Option<usize> {
    _memchr_basic(buf, c)
    /*
    if is_x86_feature_detected!("avx") {
        unsafe { _memchr_avx(buf, c) }
    } else {
        unsafe { _memchr_sse2(buf, c) }
    }
    */
}

#[inline(always)]
#[cfg(target_arch = "x86")]
pub fn _memchr_impl(buf: &[u8], c: u8) -> Option<usize> {
    _memchr_basic(buf, c)
    /*
    if is_x86_feature_detected!("avx") {
        unsafe { _memchr_avx(buf, c) }
    } else if is_x86_feature_detected!("sse2") {
        unsafe { _memchr_sse2(buf, c) }
    } else {
        _memchr_basic(buf, c)
    }
    */
}

fn _memchr_basic(buf: &[u8], c: u8) -> Option<usize> {
    basic::_memchr_impl(buf, c)
}

// auto-vector: Ok
#[target_feature(enable = "sse2")]
unsafe fn _memchr_sse2(buf: &[u8], c: u8) -> Option<usize> {
    basic::_memchr_impl(buf, c)
}

// auto-vector: Ok
#[target_feature(enable = "avx")]
unsafe fn _memchr_avx(buf: &[u8], c: u8) -> Option<usize> {
    basic::_memchr_impl(buf, c)
}
