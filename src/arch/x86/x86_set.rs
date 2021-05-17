use crate::mem as basic;
use crate::RangeError;

#[inline(always)]
#[cfg(target_arch = "x86_64")]
pub fn _memset_impl(buf: &mut [u8], c: u8, n: usize) -> Result<(), RangeError> {
    unsafe { _memset_sse2(buf, c, n) }
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
pub fn _memset_impl(buf: &mut [u8], c: u8, n: usize) -> Result<(), RangeError> {
    _memset_basic(buf, c, n)
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

fn _memset_basic(buf: &mut [u8], c: u8, n: usize) -> Result<(), RangeError> {
    basic::_memset_impl(buf, c, n)
}

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "sse2")]
unsafe fn _memset_sse2(buf: &mut [u8], c: u8, n: usize) -> Result<(), RangeError> {
    basic::_memset_impl(buf, c, n)
}

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx")]
unsafe fn _memset_avx(buf: &mut [u8], c: u8, n: usize) -> Result<(), RangeError> {
    basic::_memset_impl(buf, c, n)
}
