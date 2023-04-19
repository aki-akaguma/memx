use super::{MMB16Sgl, MMB32Sgl};
use crate::mem as basic;
use crate::utils::*;

#[cfg(target_arch = "x86")]
use core::arch::x86 as mmx;
#[cfg(target_arch = "x86_64")]
use core::arch::x86_64 as mmx;

use mmx::__m128i;
use mmx::_mm_store_si128;
use mmx::_mm_storeu_si128;

use mmx::__m256i;
use mmx::_mm256_store_si256;
use mmx::_mm256_storeu_si256;

#[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
use super::cpuid;

use core::sync::atomic::AtomicPtr;
use core::sync::atomic::Ordering;
type FuncType = fn(&mut [u8], u8);

const FUNC: FuncType = fnptr_setup_func;
static FUNC_PTR_ATOM: AtomicPtr<FuncType> = AtomicPtr::new(FUNC as *mut FuncType);

#[inline(never)]
fn fnptr_setup_func(buf: &mut [u8], c: u8) {
    #[cfg(target_arch = "x86_64")]
    let func = if cpuid::has_avx2() {
        _memset_avx2
    } else {
        _memset_sse2
    };
    #[cfg(target_arch = "x86")]
    let func = if cpuid::has_avx2() {
        _memset_avx2
    } else if cpuid::has_sse2() {
        _memset_sse2
    } else {
        _memset_basic
    };
    //
    FUNC_PTR_ATOM.store(func as *mut FuncType, Ordering::Relaxed);
    unsafe { func(buf, c) }
}

#[inline(always)]
pub(crate) fn _memset_impl(buf: &mut [u8], c: u8) {
    let func_u = FUNC_PTR_ATOM.load(Ordering::Relaxed);
    #[allow(clippy::crosspointer_transmute)]
    let func: FuncType = unsafe { core::mem::transmute(func_u) };
    func(buf, c)
}

unsafe fn _memset_basic(buf: &mut [u8], c: u8) {
    basic::_memset_impl(buf, c)
}

#[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
#[target_feature(enable = "sse2")]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn _memset_sse2(buf: &mut [u8], c: u8) {
    _memset_sse2_impl(buf, c)
}

#[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
#[target_feature(enable = "avx2")]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn _memset_avx2(buf: &mut [u8], c: u8) {
    _memset_avx2_impl(buf, c)
}

#[inline(always)]
fn _memset_sse2_impl(buf: &mut [u8], c1: u8) {
    let buf_len = buf.len();
    let mut buf_ptr = buf.as_mut_ptr();
    let end_ptr = unsafe { buf_ptr.add(buf_len) };
    //
    if buf_len >= 16 {
        let cc = MMB16Sgl::new(c1);
        // to a aligned pointer
        {
            if !buf_ptr.is_aligned_u128() {
                #[cfg(not(feature = "test_alignment_check"))]
                {
                    let remaining_align = 0x10_usize - ((buf_ptr as usize) & 0x0F_usize);
                    unsafe { _set_c16_uu_x1(buf_ptr, cc) };
                    buf_ptr = unsafe { buf_ptr.add(remaining_align) };
                }
                #[cfg(feature = "test_alignment_check")]
                {
                    let c = B1Sgl::new(c1);
                    buf_ptr = basic::_set_to_aligned_u128(buf_ptr, c);
                }
            }
        }
        // the loop
        {
            let unroll = 16;
            let loop_size = 16;
            while buf_ptr.is_not_over(end_ptr, loop_size * unroll) {
                unsafe { _set_c16_aa_x16(buf_ptr, cc) };
                buf_ptr = unsafe { buf_ptr.add(loop_size * unroll) };
            }
        }
        {
            let unroll = 8;
            let loop_size = 16;
            while buf_ptr.is_not_over(end_ptr, loop_size * unroll) {
                unsafe { _set_c16_aa_x8(buf_ptr, cc) };
                buf_ptr = unsafe { buf_ptr.add(loop_size * unroll) };
            }
        }
        /*
        {
            let unroll = 4;
            let loop_size = 16;
            while buf_ptr.is_not_over(end_ptr, loop_size * unroll) {
                unsafe { _set_c16_aa_x4(buf_ptr, cc) };
                buf_ptr = unsafe { buf_ptr.add(loop_size * unroll) };
            }
        }
        {
            let unroll = 2;
            let loop_size = 16;
            while buf_ptr.is_not_over(end_ptr, loop_size * unroll) {
                unsafe { _set_c16_aa_x2(buf_ptr, cc) };
                buf_ptr = unsafe { buf_ptr.add(loop_size * unroll) };
            }
        }
        */
        {
            let unroll = 1;
            let loop_size = 16;
            while buf_ptr.is_not_over(end_ptr, loop_size * unroll) {
                unsafe { _set_c16_aa_x1(buf_ptr, cc) };
                buf_ptr = unsafe { buf_ptr.add(loop_size * unroll) };
            }
        }
    }
    //
    let cc = B8Sgl::new(c1);
    basic::_memset_remaining_15_bytes_impl(buf_ptr, cc, end_ptr)
}

#[inline(always)]
fn _memset_avx2_impl(buf: &mut [u8], c1: u8) {
    let buf_len = buf.len();
    let mut buf_ptr = buf.as_mut_ptr();
    let end_ptr = unsafe { buf_ptr.add(buf_len) };
    //
    if buf_len >= 32 {
        let cc = MMB32Sgl::new(c1);
        // to a aligned pointer
        {
            if !buf_ptr.is_aligned_u256() {
                #[cfg(not(feature = "test_alignment_check"))]
                {
                    let remaining_align = 0x20_usize - ((buf_ptr as usize) & 0x1F_usize);
                    unsafe { _set_c32_uu_x1(buf_ptr, cc) };
                    buf_ptr = unsafe { buf_ptr.add(remaining_align) };
                }
                #[cfg(feature = "test_alignment_check")]
                {
                    let c = B1Sgl::new(c1);
                    buf_ptr = basic::_set_to_aligned_u256(buf_ptr, c);
                }
            }
        }
        // the loop
        {
            let unroll = 8;
            let loop_size = 32;
            while buf_ptr.is_not_over(end_ptr, loop_size * unroll) {
                unsafe { _set_c32_aa_x8(buf_ptr, cc) };
                buf_ptr = unsafe { buf_ptr.add(loop_size * unroll) };
            }
        }
        /*
        {
            let unroll = 4;
            let loop_size = 32;
            while buf_ptr.is_not_over(end_ptr, loop_size * unroll) {
                unsafe { _set_c32_aa_x4(buf_ptr, cc) };
                buf_ptr = unsafe { buf_ptr.add(loop_size * unroll) };
            }
        }
        {
            let unroll = 2;
            let loop_size = 32;
            while buf_ptr.is_not_over(end_ptr, loop_size * unroll) {
                unsafe { _set_c32_aa_x2(buf_ptr, cc) };
                buf_ptr = unsafe { buf_ptr.add(loop_size * unroll) };
            }
        }
        */
        {
            let unroll = 1;
            let loop_size = 32;
            while buf_ptr.is_not_over(end_ptr, loop_size * unroll) {
                unsafe { _set_c32_aa_x1(buf_ptr, cc) };
                buf_ptr = unsafe { buf_ptr.add(loop_size * unroll) };
            }
        }
        {
            let cc = MMB16Sgl::new(c1);
            let unroll = 1;
            let loop_size = 16;
            while buf_ptr.is_not_over(end_ptr, loop_size * unroll) {
                unsafe { _set_c16_aa_x1(buf_ptr, cc) };
                buf_ptr = unsafe { buf_ptr.add(loop_size * unroll) };
            }
        }
    } else if buf_len >= 16 {
        {
            let cc = MMB16Sgl::new(c1);
            let unroll = 1;
            let loop_size = 16;
            if buf_ptr.is_not_over(end_ptr, loop_size * unroll) {
                let end_ptr_16_x1 = unsafe { end_ptr.sub(loop_size * unroll) };
                //
                if buf_ptr.is_aligned_u128() {
                    while buf_ptr <= end_ptr_16_x1 {
                        unsafe { _set_c16_aa_x1(buf_ptr, cc) };
                        buf_ptr = unsafe { buf_ptr.add(loop_size * unroll) };
                    }
                } else {
                    #[cfg(not(feature = "test_alignment_check"))]
                    {
                        while buf_ptr <= end_ptr_16_x1 {
                            unsafe { _set_c16_uu_x1(buf_ptr, cc) };
                            buf_ptr = unsafe { buf_ptr.add(loop_size) };
                        }
                    }
                    #[cfg(feature = "test_alignment_check")]
                    {
                        let c = B1Sgl::new(c1);
                        buf_ptr = basic::_set_to_aligned_u128(buf_ptr, c);
                        while buf_ptr <= end_ptr_16_x1 {
                            unsafe { _set_c16_aa_x1(buf_ptr, cc) };
                            buf_ptr = unsafe { buf_ptr.add(loop_size * unroll) };
                        }
                    }
                }
            }
        }
    }
    //
    let cc = B8Sgl::new(c1);
    basic::_memset_remaining_15_bytes_impl(buf_ptr, cc, end_ptr)
}

#[inline(always)]
unsafe fn _set_c16_uu_x1(buf_ptr: *mut u8, mm_c16: MMB16Sgl) {
    _mm_storeu_si128(buf_ptr as *mut __m128i, mm_c16.v1);
}

#[inline(always)]
unsafe fn _set_c16_aa_x1(buf_ptr: *mut u8, mm_c16: MMB16Sgl) {
    _mm_store_si128(buf_ptr as *mut __m128i, mm_c16.v1);
}

#[inline(always)]
unsafe fn _set_c16_aa_x2(buf_ptr: *mut u8, mm_c16: MMB16Sgl) {
    _mm_store_si128(buf_ptr as *mut __m128i, mm_c16.v1);
    _mm_store_si128(buf_ptr.add(16) as *mut __m128i, mm_c16.v1);
}

#[inline(always)]
unsafe fn _set_c16_aa_x4(buf_ptr: *mut u8, mm_c16: MMB16Sgl) {
    _mm_store_si128(buf_ptr as *mut __m128i, mm_c16.v1);
    _mm_store_si128(buf_ptr.add(16) as *mut __m128i, mm_c16.v1);
    _mm_store_si128(buf_ptr.add(16 * 2) as *mut __m128i, mm_c16.v1);
    _mm_store_si128(buf_ptr.add(16 * 3) as *mut __m128i, mm_c16.v1);
}

#[inline(always)]
unsafe fn _set_c16_aa_x8(buf_ptr: *mut u8, mm_c16: MMB16Sgl) {
    _set_c16_aa_x4(buf_ptr, mm_c16);
    _set_c16_aa_x4(buf_ptr.add(16 * 4), mm_c16);
}

#[inline(always)]
unsafe fn _set_c16_aa_x16(buf_ptr: *mut u8, mm_c16: MMB16Sgl) {
    _set_c16_aa_x8(buf_ptr, mm_c16);
    _set_c16_aa_x8(buf_ptr.add(16 * 8), mm_c16);
}

#[inline(always)]
unsafe fn _set_c32_uu_x1(buf_ptr: *mut u8, mm_c32: MMB32Sgl) {
    _mm256_storeu_si256(buf_ptr as *mut __m256i, mm_c32.v1);
}

#[inline(always)]
unsafe fn _set_c32_aa_x1(buf_ptr: *mut u8, mm_c32: MMB32Sgl) {
    _mm256_store_si256(buf_ptr as *mut __m256i, mm_c32.v1);
}

#[inline(always)]
unsafe fn _set_c32_aa_x2(buf_ptr: *mut u8, mm_c32: MMB32Sgl) {
    _mm256_store_si256(buf_ptr as *mut __m256i, mm_c32.v1);
    _mm256_store_si256(buf_ptr.add(32) as *mut __m256i, mm_c32.v1);
}

#[inline(always)]
unsafe fn _set_c32_aa_x4(buf_ptr: *mut u8, mm_c32: MMB32Sgl) {
    _mm256_store_si256(buf_ptr as *mut __m256i, mm_c32.v1);
    _mm256_store_si256(buf_ptr.add(32) as *mut __m256i, mm_c32.v1);
    _mm256_store_si256(buf_ptr.add(32 * 2) as *mut __m256i, mm_c32.v1);
    _mm256_store_si256(buf_ptr.add(32 * 3) as *mut __m256i, mm_c32.v1);
}

#[inline(always)]
unsafe fn _set_c32_aa_x8(buf_ptr: *mut u8, mm_c32: MMB32Sgl) {
    _set_c32_aa_x4(buf_ptr, mm_c32);
    _set_c32_aa_x4(buf_ptr.add(32 * 4), mm_c32);
}

#[cfg(test)]
mod disasm {
    use super::*;
    //
    #[test]
    fn do_procs() {
        let mut a = b"abcdefg".to_vec();
        let a = a.as_mut_slice();
        let c = b'A';
        do_proc_basic(a, c);
        do_proc_sse2(a, c);
        do_proc_avx2(a, c);
    }
    #[inline(never)]
    fn do_proc_basic(a: &mut [u8], c: u8) {
        unsafe { _memset_basic(a, c) }
    }
    #[inline(never)]
    fn do_proc_sse2(a: &mut [u8], c: u8) {
        unsafe { _memset_sse2(a, c) }
    }
    #[inline(never)]
    fn do_proc_avx2(a: &mut [u8], c: u8) {
        unsafe { _memset_avx2(a, c) }
    }
}
