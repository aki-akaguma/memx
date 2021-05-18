use crate::mem as basic;

#[inline(always)]
#[cfg(target_arch = "x86_64")]
pub fn _memmem_impl(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    unsafe { _memmem_sse2(haystack, needle) }
}

#[inline(always)]
#[cfg(target_arch = "x86")]
pub fn _memmem_impl(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    _memmem_basic(haystack, needle)
}

fn _memmem_basic(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    basic::_memmem_impl(haystack, needle)
}

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "sse2")]
unsafe fn _memmem_sse2(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    basic::_memmem_impl(haystack, needle)
}
