use crate::mem as basic;
use crate::RangeError;

#[inline(always)]
#[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
pub fn _memset_impl(buf: &mut [u8], c: u8, n: usize) -> Result<(), RangeError> {
    #[cfg(target_feature = "avx")]
    let r = unsafe { _memset_avx(buf, c, n) };
    #[cfg(target_feature = "sse2")]
    let r = unsafe { _memset_sse2(buf, c, n) };
    #[cfg(not(any(target_feature = "avx", target_feature = "sse2")))]
    let r = _memset_basic(buf, c, n);
    r
}

fn _memset_basic(buf: &mut [u8], c: u8, n: usize) -> Result<(), RangeError> {
    basic::_memset_impl(buf, c, n)
}

#[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
#[target_feature(enable = "sse2")]
pub unsafe fn _memset_sse2(buf: &mut [u8], c: u8, n: usize) -> Result<(), RangeError> {
    basic::_memset_impl(buf, c, n)
}

#[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
#[target_feature(enable = "avx")]
pub unsafe fn _memset_avx(buf: &mut [u8], c: u8, n: usize) -> Result<(), RangeError> {
    basic::_memset_impl(buf, c, n)
}
