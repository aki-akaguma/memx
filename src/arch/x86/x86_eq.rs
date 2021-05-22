use crate::mem as basic;

#[inline(always)]
#[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
pub fn _memeq_impl(a: &[u8], b: &[u8]) -> bool {
    /*
    #[cfg(target_feature = "avx")]
    let r = unsafe { _memeq_avx(a, b) };
    #[cfg(target_feature = "sse2")]
    let r = unsafe { _memeq_sse2(a, b) };
    #[cfg(not(any(target_feature = "avx", target_feature = "sse2")))]
    */
    let r = _memeq_basic(a, b);
    r
}

fn _memeq_basic(a: &[u8], b: &[u8]) -> bool {
    basic::_memeq_impl(a, b)
}

/*
#[target_feature(enable = "sse2")]
pub unsafe fn _memeq_sse2(a: &[u8], b: &[u8]) -> bool {
    basic::_memeq_impl(a, b)
}

#[target_feature(enable = "avx")]
pub unsafe fn _memeq_avx(a: &[u8], b: &[u8]) -> bool {
    basic::_memeq_impl(a, b)
}
*/
