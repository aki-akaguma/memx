use crate::mem as basic;

#[inline(always)]
#[cfg(target_arch = "x86_64")]
pub fn _memmem_impl(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    unsafe { _memmem_sse2(haystack, needle) }
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
pub fn _memmem_impl(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    _memmem_basic(haystack, needle)
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

fn _memmem_basic(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    basic::_memmem_impl(haystack, needle)
}

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "sse2")]
unsafe fn _memmem_sse2(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    basic::_memmem_impl(haystack, needle)
}
