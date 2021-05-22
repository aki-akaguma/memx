use crate::mem as basic;

#[inline(always)]
#[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
pub fn _memmem_impl(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    /*
    #[cfg(target_feature = "avx")]
    let r = unsafe { _memmem_avx(haystack, needle) };
    #[cfg(target_feature = "sse2")]
    let r = unsafe { _memmem_sse2(haystack, needle) };
    #[cfg(not(any(target_feature = "avx", target_feature = "sse2")))]
    let r = _memmem_basic(haystack, needle);
    r
    */
    basic::_memmem_impl(haystack, needle)
}

/*
fn _memmem_basic(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    basic::_memmem_impl(haystack, needle)
}

#[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
#[target_feature(enable = "sse2")]
unsafe fn _memmem_sse2(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    basic::_memmem_impl(haystack, needle)
}
*/
