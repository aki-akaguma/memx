use crate::mem as basic;
use crate::RangeError;

#[inline(always)]
#[cfg(target_arch = "x86_64")]
pub fn _memcpy_impl(dst: &mut [u8], src: &[u8]) -> Result<(), RangeError> {
    unsafe { _memcpy_sse2(dst, src) }
}

#[inline(always)]
#[cfg(target_arch = "x86")]
pub fn _memcpy_impl(dst: &mut [u8], src: &[u8]) -> Result<(), RangeError> {
    _memcpy_basic(dst, src)
}

fn _memcpy_basic(dst: &mut [u8], src: &[u8]) -> Result<(), RangeError> {
    basic::_memcpy_impl(dst, src)
}

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "sse2")]
unsafe fn _memcpy_sse2(dst: &mut [u8], src: &[u8]) -> Result<(), RangeError> {
    basic::_memcpy_impl(dst, src)
}
