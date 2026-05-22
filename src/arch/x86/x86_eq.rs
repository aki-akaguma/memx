use crate::mem as basic;
use crate::utils::*;

#[cfg(target_arch = "x86")]
use core::arch::x86 as mmx;
#[cfg(target_arch = "x86_64")]
use core::arch::x86_64 as mmx;

use mmx::__m128i;
use mmx::_mm_cmpeq_epi8;
use mmx::_mm_load_si128;
use mmx::_mm_loadu_si128;
use mmx::_mm_movemask_epi8;

/*
use mmx::_mm_set1_epi8;
use mmx::__m256i;
use mmx::_mm256_cmpeq_epi8;
use mmx::_mm256_load_si256;
use mmx::_mm256_loadu_si256;
use mmx::_mm256_movemask_epi8;
use mmx::_mm256_set1_epi8;
*/

#[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
use super::cpuid;

use core::sync::atomic::AtomicPtr;
use core::sync::atomic::Ordering as AtomicOrdering;
type _FuncType = fn(&[u8], &[u8]) -> bool;

const _FUNC: _FuncType = _fnptr_setup_func;
static _FUNC_PTR_ATOM: AtomicPtr<_FuncType> = AtomicPtr::new(_FUNC as *mut _FuncType);

#[inline(never)]
fn _fnptr_setup_func(a: &[u8], b: &[u8]) -> bool {
    #[cfg(target_arch = "x86_64")]
    let func = if cpuid::has_avx2() {
        _memeq_avx2
    } else {
        _memeq_sse2
    };
    #[cfg(target_arch = "x86")]
    let func = if cpuid::has_avx2() {
        _memeq_avx2
    } else if cpuid::has_sse2() {
        _memeq_sse2
    } else {
        _memeq_basic
    };
    //
    _FUNC_PTR_ATOM.store(func as *mut _FuncType, AtomicOrdering::Relaxed);
    unsafe { func(a, b) }
}

#[inline(always)]
pub(crate) fn _memeq_impl(a: &[u8], b: &[u8]) -> bool {
    let func_u = _FUNC_PTR_ATOM.load(AtomicOrdering::Relaxed);
    #[allow(clippy::crosspointer_transmute)]
    let func: _FuncType = unsafe { core::mem::transmute(func_u) };
    func(a, b)
}

unsafe fn _memeq_basic(a: &[u8], b: &[u8]) -> bool {
    basic::_memeq_impl(a, b)
}

#[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
#[target_feature(enable = "sse2")]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn _memeq_sse2(a: &[u8], b: &[u8]) -> bool {
    _memeq_sse2_impl(a, b)
}

#[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
#[target_feature(enable = "avx2")]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn _memeq_avx2(a: &[u8], b: &[u8]) -> bool {
    _memeq_sse2_impl(a, b)
}

#[inline(always)]
fn _memeq_sse2_impl(a: &[u8], b: &[u8]) -> bool {
    let a_len = a.len();
    let b_len = b.len();
    if a_len != b_len {
        return false;
    }
    if a_len < 16 {
        basic::_start_eq_64(a, b)
    } else {
        let mut a_ptr = a.as_ptr();
        let mut b_ptr = b.as_ptr();
        let end_ptr = unsafe { a_ptr.add(a_len) };
        a_ptr.prefetch_read_data();
        b_ptr.prefetch_read_data();
        // to a aligned pointer
        {
            if !a_ptr.is_aligned_u128() {
                #[cfg(not(feature = "test_alignment_check"))]
                {
                    let r = unsafe { _eq_b16_uu_x1(a_ptr, b_ptr) };
                    if !r {
                        return r;
                    }
                    let remaining_align = 0x10_usize - ((a_ptr as usize) & 0x0F_usize);
                    a_ptr = unsafe { a_ptr.add(remaining_align) };
                    b_ptr = unsafe { b_ptr.add(remaining_align) };
                }
                #[cfg(feature = "test_alignment_check")]
                {
                    let r = basic::_eq_to_aligned_u128(a_ptr, b_ptr);
                    if let Some((ap, bp)) = r.0 {
                        a_ptr = ap;
                        b_ptr = bp;
                    } else if let Some(v) = r.1 {
                        return v;
                    }
                }
            }
        }
        // the loop
        if b_ptr.is_aligned_u128() {
            let (r, ap, bp) = _unroll_loop_dual_with_prefetch::<4, 16, bool, _>(a_ptr, b_ptr, end_ptr, |ap, bp| {
                let r = unsafe { _eq_b16_aa_x1(ap, bp) };
                if r {
                    None
                } else {
                    Some(false)
                }
            });
            if let Some(res) = r {
                return res;
            }
            a_ptr = ap;
            b_ptr = bp;

            let (r, ap, bp) = _unroll_loop_dual::<1, 16, bool, _>(a_ptr, b_ptr, end_ptr, |ap, bp| {
                let r = unsafe { _eq_b16_aa_x1(ap, bp) };
                if r {
                    None
                } else {
                    Some(false)
                }
            });
            if let Some(res) = r {
                return res;
            }
            a_ptr = ap;
            b_ptr = bp;
        } else {
            let (r, ap, bp) = _unroll_loop_dual_with_prefetch::<4, 16, bool, _>(a_ptr, b_ptr, end_ptr, |ap, bp| {
                let r = unsafe { _eq_b16_au_x1(ap, bp) };
                if r {
                    None
                } else {
                    Some(false)
                }
            });
            if let Some(res) = r {
                return res;
            }
            a_ptr = ap;
            b_ptr = bp;

            let (r, ap, bp) = _unroll_loop_dual::<1, 16, bool, _>(a_ptr, b_ptr, end_ptr, |ap, bp| {
                let r = unsafe { _eq_b16_au_x1(ap, bp) };
                if r {
                    None
                } else {
                    Some(false)
                }
            });
            if let Some(res) = r {
                return res;
            }
            a_ptr = ap;
            b_ptr = bp;
        }
        // the remaining data is the max: 15 bytes.
        basic::_memeq_remaining_15_bytes_impl(a_ptr, b_ptr, end_ptr)
    }
}

#[inline(always)]
unsafe fn _eq_b16_uu_x1(a_ptr: *const u8, b_ptr: *const u8) -> bool {
    let mm_0_a = unsafe { _mm_loadu_si128(a_ptr as *const __m128i) };
    let mm_0_b = unsafe { _mm_loadu_si128(b_ptr as *const __m128i) };
    let mm_0_eq = unsafe { _mm_cmpeq_epi8(mm_0_a, mm_0_b) };
    let mask_0 = unsafe { _mm_movemask_epi8(mm_0_eq) } as u32;
    mask_0 == 0xffff
}

#[inline(always)]
unsafe fn _eq_b16_au_x1(a_ptr: *const u8, b_ptr: *const u8) -> bool {
    let mm_0_a = unsafe { _mm_load_si128(a_ptr as *const __m128i) };
    let mm_0_b = unsafe { _mm_loadu_si128(b_ptr as *const __m128i) };
    let mm_0_eq = unsafe { _mm_cmpeq_epi8(mm_0_a, mm_0_b) };
    let mask_0 = unsafe { _mm_movemask_epi8(mm_0_eq) } as u32;
    mask_0 == 0xffff
}

#[inline(always)]
unsafe fn _eq_b16_aa_x1(a_ptr: *const u8, b_ptr: *const u8) -> bool {
    let mm_0_a = unsafe { _mm_load_si128(a_ptr as *const __m128i) };
    let mm_0_b = unsafe { _mm_load_si128(b_ptr as *const __m128i) };
    let mm_0_eq = unsafe { _mm_cmpeq_epi8(mm_0_a, mm_0_b) };
    let mask_0 = unsafe { _mm_movemask_epi8(mm_0_eq) } as u32;
    mask_0 == 0xffff
}


#[cfg(test)]
mod disasm {
    use super::*;
    //
    #[test]
    fn do_procs() {
        let a = b"abcdefg".to_vec();
        let b = b"abcdefg".to_vec();
        let a = a.as_slice();
        let b = b.as_slice();
        assert!(do_proc_basic(a, b));
        assert!(do_proc_sse2(a, b));
        assert!(do_proc_avx2(a, b));
    }
    fn do_proc_basic(a: &[u8], b: &[u8]) -> bool {
        unsafe { _memeq_basic(a, b) }
    }
    fn do_proc_sse2(a: &[u8], b: &[u8]) -> bool {
        unsafe { _memeq_sse2(a, b) }
    }
    fn do_proc_avx2(a: &[u8], b: &[u8]) -> bool {
        unsafe { _memeq_avx2(a, b) }
    }
}
