use crate::mem as basic;
use crate::RangeError;

#[cfg(target_arch = "x86_64")]
use super::cpuid_avx2;

#[cfg(target_arch = "x86")]
use super::{cpuid_avx2, cpuid_sse2};

#[inline(always)]
#[cfg(target_arch = "x86_64")]
pub fn _memcpy_impl(dst: &mut [u8], src: &[u8]) -> Result<(), RangeError> {
    // TODO: Replace with https://github.com/rust-lang/rfcs/pull/2725
    // after stabilization
    if cpuid_avx2::get() {
        unsafe { _memcpy_avx2(dst, src) }
    } else {
        unsafe { _memcpy_sse2(dst, src) }
    }
}

#[inline(always)]
#[cfg(target_arch = "x86")]
pub fn _memcpy_impl(dst: &mut [u8], src: &[u8]) -> Result<(), RangeError> {
    // TODO: Replace with https://github.com/rust-lang/rfcs/pull/2725
    // after stabilization
    if cpuid_avx2::get() {
        unsafe { _memcpy_avx2(dst, src) }
    } else if cpuid_sse2::get() {
        unsafe { _memcpy_sse2(dst, src) }
    } else {
        _memcpy_basic(dst, src)
    }
}

fn _memcpy_basic(dst: &mut [u8], src: &[u8]) -> Result<(), RangeError> {
    basic::_memcpy_impl(dst, src)
}

#[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
#[target_feature(enable = "sse2")]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn _memcpy_sse2(dst: &mut [u8], src: &[u8]) -> Result<(), RangeError> {
    basic::_memcpy_impl(dst, src)
}

#[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
#[target_feature(enable = "avx2")]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn _memcpy_avx2(dst: &mut [u8], src: &[u8]) -> Result<(), RangeError> {
    basic::_memcpy_impl(dst, src)
}

#[cfg(test)]
mod disasm {
    use super::*;
    //
    #[test]
    fn do_procs() {
        let mut a = b"       ".to_vec();
        let b = b"abcdefg".to_vec();
        let a = a.as_mut_slice();
        let b = b.as_slice();
        assert_eq!(do_proc_basic(a, b), Ok(()));
        assert_eq!(do_proc_sse2(a, b), Ok(()));
        assert_eq!(do_proc_avx2(a, b), Ok(()));
    }
    #[inline(never)]
    fn do_proc_basic(a: &mut [u8], b: &[u8]) -> Result<(), RangeError> {
        _memcpy_basic(a, b)
    }
    #[inline(never)]
    fn do_proc_sse2(a: &mut [u8], b: &[u8]) -> Result<(), RangeError> {
        unsafe { _memcpy_sse2(a, b) }
    }
    #[inline(never)]
    fn do_proc_avx2(a: &mut [u8], b: &[u8]) -> Result<(), RangeError> {
        unsafe { _memcpy_avx2(a, b) }
    }
}
