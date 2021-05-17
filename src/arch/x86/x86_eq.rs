use crate::mem as basic;

#[inline(always)]
#[cfg(target_arch = "x86_64")]
pub fn _memeq_impl(a: &[u8], b: &[u8]) -> bool {
    unsafe { _memeq_sse2(a, b) }
}

#[inline(always)]
#[cfg(target_arch = "x86")]
pub fn _memeq_impl(a: &[u8], b: &[u8]) -> bool {
    _memeq_basic(a, b)
}

fn _memeq_basic(a: &[u8], b: &[u8]) -> bool {
    basic::_memeq_impl(a, b)
}

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "sse2")]
unsafe fn _memeq_sse2(a: &[u8], b: &[u8]) -> bool {
    basic::_memeq_impl(a, b)
}
