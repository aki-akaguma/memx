use crate::mem as basic;
use crate::RangeError;

#[inline(always)]
#[cfg(target_arch = "x86_64")]
pub fn _memcpy_impl(dst: &mut [u8], src: &[u8]) -> Result<(), RangeError> {
    unsafe { _memcpy_sse2(dst, src) }
    /*
    if is_x86_feature_detected!("avx") {
        unsafe { _memset_avx(buf, c, n) }
    } else {
        unsafe { _memset_sse2(buf, c, n) }
    }
    */
}

#[inline(always)]
#[cfg(target_arch = "x86")]
pub fn _memcpy_impl(dst: &mut [u8], src: &[u8]) -> Result<(), RangeError> {
    _memcpy_basic(dst, src)
    /*
    if is_x86_feature_detected!("avx") {
        unsafe { _memset_avx(buf, c, n) }
    } else if is_x86_feature_detected!("sse2") {
        unsafe { _memset_sse2(buf, c, n) }
    } else {
        _memset_basic(buf, c, n)
    }
    */
}

fn _memcpy_basic(dst: &mut [u8], src: &[u8]) -> Result<(), RangeError> {
    basic::_memcpy_impl(dst, src)
}

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "sse2")]
unsafe fn _memcpy_sse2(dst: &mut [u8], src: &[u8]) -> Result<(), RangeError> {
    basic::_memcpy_impl(dst, src)
}
