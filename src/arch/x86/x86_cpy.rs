use crate::mem as basic;
use crate::utils::*;
use crate::RangeError;

#[cfg(target_arch = "x86")]
use core::arch::x86 as mmx;
#[cfg(target_arch = "x86_64")]
use core::arch::x86_64 as mmx;

use mmx::__m128i;
use mmx::_mm_load_si128;
use mmx::_mm_loadu_si128;
use mmx::_mm_store_si128;
use mmx::_mm_storeu_si128;
/*
use mmx::__m256i;
use mmx::_mm256_load_si256;
use mmx::_mm256_loadu_si256;
use mmx::_mm256_store_si256;
use mmx::_mm256_storeu_si256;
*/

#[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
use super::cpuid;

use core::sync::atomic::AtomicPtr;
use core::sync::atomic::Ordering;
type _FuncType = fn(&mut [u8], &[u8]) -> Result<(), RangeError>;

const _FUNC: _FuncType = _fnptr_setup_func;
static _FUNC_PTR_ATOM: AtomicPtr<_FuncType> = AtomicPtr::new(_FUNC as *mut _FuncType);

#[inline(never)]
fn _fnptr_setup_func(dst: &mut [u8], src: &[u8]) -> Result<(), RangeError> {
    #[cfg(target_arch = "x86_64")]
    let func = if cpuid::has_avx2() {
        _memcpy_avx2
    } else {
        _memcpy_sse2
    };
    #[cfg(target_arch = "x86")]
    let func = if cpuid::has_avx2() {
        _memcpy_avx2
    } else if cpuid::has_sse2() {
        _memcpy_sse2
    } else {
        _memcpy_basic
    };
    //
    _FUNC_PTR_ATOM.store(func as *mut _FuncType, Ordering::Relaxed);
    unsafe { func(dst, src) }
}

#[inline(always)]
pub(crate) fn _memcpy_impl(dst: &mut [u8], src: &[u8]) -> Result<(), RangeError> {
    let func_u = _FUNC_PTR_ATOM.load(Ordering::Relaxed);
    #[allow(clippy::crosspointer_transmute)]
    let func: _FuncType = unsafe { core::mem::transmute(func_u) };
    func(dst, src)
}

unsafe fn _memcpy_basic(dst: &mut [u8], src: &[u8]) -> Result<(), RangeError> {
    basic::_memcpy_impl(dst, src)
}

#[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
#[target_feature(enable = "sse2")]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn _memcpy_sse2(dst: &mut [u8], src: &[u8]) -> Result<(), RangeError> {
    _memcpy_sse2_impl(dst, src)
}

#[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
#[target_feature(enable = "avx2")]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn _memcpy_avx2(dst: &mut [u8], src: &[u8]) -> Result<(), RangeError> {
    _memcpy_sse2_impl(dst, src)
}

#[inline(always)]
fn _memcpy_sse2_impl(dst: &mut [u8], src: &[u8]) -> Result<(), RangeError> {
    if src.is_empty() {
        return Ok(());
    }
    let dst_len = dst.len();
    let src_len = src.len();
    if dst_len < src_len {
        return Err(RangeError);
    }
    //
    let mut a_ptr = dst.as_mut_ptr();
    let mut b_ptr = src.as_ptr();
    let end_ptr = unsafe { a_ptr.add(src_len) };
    b_ptr.prefetch_read_data();
    //
    if src_len >= 16 {
        // to a aligned pointer
        {
            if !a_ptr.is_aligned_u128() {
                #[cfg(not(feature = "test_alignment_check"))]
                {
                    unsafe { _cpy_b16_uu_x1(a_ptr, b_ptr) };
                    let remaining_align = 0x10_usize - ((a_ptr as usize) & 0x0F_usize);
                    a_ptr = unsafe { a_ptr.add(remaining_align) };
                    b_ptr = unsafe { b_ptr.add(remaining_align) };
                }
                #[cfg(feature = "test_alignment_check")]
                {
                    let (ap, bp) = basic::_cpy_to_aligned_u128(a_ptr, b_ptr);
                    a_ptr = ap;
                    b_ptr = bp;
                }
            }
        }
        // the loop
        if b_ptr.is_aligned_u128() {
            {
                let unroll = 16;
                let loop_size = 16;
                while a_ptr.is_not_over(end_ptr, loop_size * unroll) {
                    b_ptr.prefetch_read_data();
                    _cpy_b16_aa_x16(a_ptr, b_ptr);
                    a_ptr = unsafe { a_ptr.add(loop_size * unroll) };
                    b_ptr = unsafe { b_ptr.add(loop_size * unroll) };
                }
            }
            {
                let unroll = 8;
                let loop_size = 16;
                while a_ptr.is_not_over(end_ptr, loop_size * unroll) {
                    b_ptr.prefetch_read_data();
                    _cpy_b16_aa_x8(a_ptr, b_ptr);
                    a_ptr = unsafe { a_ptr.add(loop_size * unroll) };
                    b_ptr = unsafe { b_ptr.add(loop_size * unroll) };
                }
            }
            {
                let unroll = 1;
                let loop_size = 16;
                while a_ptr.is_not_over(end_ptr, loop_size * unroll) {
                    unsafe { _cpy_b16_aa_x1(a_ptr, b_ptr) };
                    a_ptr = unsafe { a_ptr.add(loop_size * unroll) };
                    b_ptr = unsafe { b_ptr.add(loop_size * unroll) };
                }
            }
        } else {
            {
                let unroll = 16;
                let loop_size = 16;
                while a_ptr.is_not_over(end_ptr, loop_size * unroll) {
                    b_ptr.prefetch_read_data();
                    _cpy_b16_au_x16(a_ptr, b_ptr);
                    a_ptr = unsafe { a_ptr.add(loop_size * unroll) };
                    b_ptr = unsafe { b_ptr.add(loop_size * unroll) };
                }
            }
            {
                let unroll = 8;
                let loop_size = 16;
                while a_ptr.is_not_over(end_ptr, loop_size * unroll) {
                    b_ptr.prefetch_read_data();
                    _cpy_b16_au_x8(a_ptr, b_ptr);
                    a_ptr = unsafe { a_ptr.add(loop_size * unroll) };
                    b_ptr = unsafe { b_ptr.add(loop_size * unroll) };
                }
            }
            {
                let unroll = 1;
                let loop_size = 16;
                while a_ptr.is_not_over(end_ptr, loop_size * unroll) {
                    unsafe { _cpy_b16_au_x1(a_ptr, b_ptr) };
                    a_ptr = unsafe { a_ptr.add(loop_size * unroll) };
                    b_ptr = unsafe { b_ptr.add(loop_size * unroll) };
                }
            }
        }
    }
    // the remaining data is the max: 15 bytes.
    basic::_memcpy_remaining_15_bytes_impl(a_ptr, b_ptr, end_ptr);
    Ok(())
}

#[inline(always)]
unsafe fn _cpy_b16_uu_x1(a_ptr: *mut u8, b_ptr: *const u8) {
    unsafe {
        let mm_0_b = _mm_loadu_si128(b_ptr as *const __m128i);
        _mm_storeu_si128(a_ptr as *mut __m128i, mm_0_b);
    }
}

#[inline(always)]
unsafe fn _cpy_b16_au_x1(a_ptr: *mut u8, b_ptr: *const u8) {
    unsafe {
        let mm_0_b = _mm_loadu_si128(b_ptr as *const __m128i);
        _mm_store_si128(a_ptr as *mut __m128i, mm_0_b);
    }
}

#[inline(always)]
fn _cpy_b16_au_x2(a_ptr: *mut u8, b_ptr: *const u8) {
    unsafe { _cpy_b16_au_x1(a_ptr, b_ptr) };
    unsafe { _cpy_b16_au_x1(a_ptr.add(16), b_ptr.add(16)) }
}

#[inline(always)]
fn _cpy_b16_au_x4(a_ptr: *mut u8, b_ptr: *const u8) {
    _cpy_b16_au_x2(a_ptr, b_ptr);
    _cpy_b16_au_x2(unsafe { a_ptr.add(16 * 2) }, unsafe { b_ptr.add(16 * 2) });
}

#[inline(always)]
fn _cpy_b16_au_x8(a_ptr: *mut u8, b_ptr: *const u8) {
    _cpy_b16_au_x4(a_ptr, b_ptr);
    _cpy_b16_au_x4(unsafe { a_ptr.add(16 * 4) }, unsafe { b_ptr.add(16 * 4) });
}

#[inline(always)]
fn _cpy_b16_au_x16(a_ptr: *mut u8, b_ptr: *const u8) {
    _cpy_b16_au_x8(a_ptr, b_ptr);
    _cpy_b16_au_x8(unsafe { a_ptr.add(16 * 8) }, unsafe { b_ptr.add(16 * 8) });
}

#[inline(always)]
unsafe fn _cpy_b16_aa_x1(a_ptr: *mut u8, b_ptr: *const u8) {
    unsafe {
        let mm_0_b = _mm_load_si128(b_ptr as *const __m128i);
        _mm_store_si128(a_ptr as *mut __m128i, mm_0_b);
    }
}

#[inline(always)]
fn _cpy_b16_aa_x2(a_ptr: *mut u8, b_ptr: *const u8) {
    unsafe { _cpy_b16_aa_x1(a_ptr, b_ptr) };
    unsafe { _cpy_b16_aa_x1(a_ptr.add(16), b_ptr.add(16)) }
}

#[inline(always)]
fn _cpy_b16_aa_x4(a_ptr: *mut u8, b_ptr: *const u8) {
    _cpy_b16_aa_x2(a_ptr, b_ptr);
    _cpy_b16_aa_x2(unsafe { a_ptr.add(16 * 2) }, unsafe { b_ptr.add(16 * 2) });
}

#[inline(always)]
fn _cpy_b16_aa_x8(a_ptr: *mut u8, b_ptr: *const u8) {
    _cpy_b16_aa_x4(a_ptr, b_ptr);
    _cpy_b16_aa_x4(unsafe { a_ptr.add(16 * 4) }, unsafe { b_ptr.add(16 * 4) });
}

#[inline(always)]
fn _cpy_b16_aa_x16(a_ptr: *mut u8, b_ptr: *const u8) {
    _cpy_b16_aa_x8(a_ptr, b_ptr);
    _cpy_b16_aa_x8(unsafe { a_ptr.add(16 * 8) }, unsafe { b_ptr.add(16 * 8) });
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
        unsafe { _memcpy_basic(a, b) }
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
