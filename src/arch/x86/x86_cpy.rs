use crate::mem as basic;
use crate::RangeError;

#[inline(always)]
#[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
pub fn _memcpy_impl(dst: &mut [u8], src: &[u8]) -> Result<(), RangeError> {
    /*
    #[cfg(target_feature = "avx")]
    let r = unsafe { _memcpy_avx(dst, src) };
    */
    #[cfg(target_feature = "sse2")]
    let r = unsafe { _memcpy_sse2(dst, src) };
    #[cfg(not(any(target_feature = "avx", target_feature = "sse2")))]
    let r = _memcpy_basic(dst, src);
    r
}

fn _memcpy_basic(dst: &mut [u8], src: &[u8]) -> Result<(), RangeError> {
    basic::_memcpy_impl(dst, src)
}

#[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
#[target_feature(enable = "sse2")]
pub unsafe fn _memcpy_sse2(dst: &mut [u8], src: &[u8]) -> Result<(), RangeError> {
    basic::_memcpy_impl(dst, src)
}
