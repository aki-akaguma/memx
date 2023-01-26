use crate::mem as basic;
use core::cmp::Ordering;

use super::{cpuid_avx2, cpuid_sse2};

// why is sse2 slower ?

#[inline(always)]
#[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
pub fn _memcmp_impl(a: &[u8], b: &[u8]) -> Ordering {
    // TODO: Replace with https://github.com/rust-lang/rfcs/pull/2725
    // after stabilization
    if cpuid_avx2::get() {
        unsafe { _memcmp_avx2(a, b) }
    } else if cpuid_sse2::get() {
        unsafe { _memcmp_sse2(a, b) }
    } else {
        _memcmp_basic(a, b)
    }
}

fn _memcmp_basic(a: &[u8], b: &[u8]) -> Ordering {
    basic::_memcmp_impl(a, b)
}

#[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
#[target_feature(enable = "sse2")]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn _memcmp_sse2(a: &[u8], b: &[u8]) -> Ordering {
    basic::_memcmp_impl(a, b)
}

#[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
#[target_feature(enable = "avx2")]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn _memcmp_avx2(a: &[u8], b: &[u8]) -> Ordering {
    basic::_memcmp_impl(a, b)
}
