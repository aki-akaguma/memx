use super::{MMB16Tpl, MMB32Tpl};
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
use mmx::_mm_or_si128;

use mmx::__m256i;
use mmx::_mm256_cmpeq_epi8;
use mmx::_mm256_load_si256;
use mmx::_mm256_loadu_si256;
use mmx::_mm256_movemask_epi8;
use mmx::_mm256_or_si256;

#[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
use super::cpuid;

use core::sync::atomic::AtomicPtr;
use core::sync::atomic::Ordering;
type FuncType = fn(&[u8], B1Tpl) -> Option<usize>;

const FUNC: FuncType = fnptr_setup_func;
static FUNC_PTR_ATOM: AtomicPtr<FuncType> = AtomicPtr::new(FUNC as *mut FuncType);

#[inline(never)]
fn fnptr_setup_func(buf: &[u8], needle: B1Tpl) -> Option<usize> {
    #[cfg(target_arch = "x86_64")]
    let func = if cpuid::has_avx2() {
        _memrnechr_tpl_avx2
    } else {
        _memrnechr_tpl_sse2
    };
    #[cfg(target_arch = "x86")]
    let func = if cpuid::has_avx2() {
        _memrnechr_tpl_avx2
    } else if cpuid::has_sse2() {
        _memrnechr_tpl_sse2
    } else {
        _memrnechr_tpl_basic
    };
    //
    FUNC_PTR_ATOM.store(func as *mut FuncType, Ordering::Relaxed);
    unsafe { func(buf, needle) }
}

#[inline(always)]
pub(crate) fn _memrnechr_tpl_impl(buf: &[u8], needle: B1Tpl) -> Option<usize> {
    let func_u = FUNC_PTR_ATOM.load(Ordering::Relaxed);
    #[allow(clippy::crosspointer_transmute)]
    let func: FuncType = unsafe { core::mem::transmute(func_u) };
    func(buf, needle)
}

unsafe fn _memrnechr_tpl_basic(buf: &[u8], needle: B1Tpl) -> Option<usize> {
    basic::_memrnechr_tpl_impl(buf, needle)
}

#[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
#[target_feature(enable = "sse2")]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn _memrnechr_tpl_sse2(buf: &[u8], needle: B1Tpl) -> Option<usize> {
    _memrnechr_tpl_sse2_impl(buf, needle)
}

#[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
#[target_feature(enable = "avx2")]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn _memrnechr_tpl_avx2(buf: &[u8], needle: B1Tpl) -> Option<usize> {
    _memrnechr_tpl_avx2_impl(buf, needle)
}

#[inline(always)]
fn _memrnechr_tpl_sse2_impl(buf: &[u8], needle: B1Tpl) -> Option<usize> {
    let buf_len = buf.len();
    let start_ptr = buf.as_ptr();
    let mut buf_ptr = unsafe { start_ptr.add(buf_len) };
    //
    if buf_len >= 16 {
        let cc: MMB16Tpl = needle.into();
        // to a aligned pointer
        {
            if !buf_ptr.is_aligned_u128() {
                #[cfg(not(feature = "test_alignment_check"))]
                {
                    let loop_size = 16;
                    let buf_ptr_pre = unsafe { buf_ptr.sub(loop_size) };
                    let r = unsafe { _rnechr_c16_uu_x1(buf_ptr_pre, cc, start_ptr) };
                    if r.is_some() {
                        return r;
                    }
                    let remaining_align = 0x10_usize - ((buf_ptr as usize) & 0x0F_usize);
                    buf_ptr = unsafe { buf_ptr_pre.add(remaining_align) };
                }
                #[cfg(feature = "test_alignment_check")]
                {
                    let r = basic::_rnechr_tpl_to_aligned_u128(buf_ptr, needle, start_ptr);
                    if let Some(p) = r.0 {
                        buf_ptr = p;
                    } else if let Some(v) = r.1 {
                        return Some(v);
                    }
                }
            }
        }
        // the loop
        {
            let unroll = 4;
            let loop_size = 16;
            while buf_ptr.is_not_under(start_ptr, loop_size * unroll) {
                buf_ptr = unsafe { buf_ptr.sub(loop_size * unroll) };
                let r = unsafe { _rnechr_c16_aa_x4(buf_ptr, cc, start_ptr) };
                if r.is_some() {
                    return r;
                }
            }
        }
        {
            let unroll = 1;
            let loop_size = 16;
            while buf_ptr.is_not_under(start_ptr, loop_size * unroll) {
                buf_ptr = unsafe { buf_ptr.sub(loop_size * unroll) };
                let r = unsafe { _rnechr_c16_aa_x1(buf_ptr, cc, start_ptr) };
                if r.is_some() {
                    return r;
                }
            }
        }
    }
    //
    let cc: B8Tpl = needle.into();
    basic::_memrnechr_tpl_remaining_15_bytes_impl(buf_ptr, cc, start_ptr)
}

#[inline(always)]
fn _memrnechr_tpl_avx2_impl(buf: &[u8], needle: B1Tpl) -> Option<usize> {
    let buf_len = buf.len();
    let start_ptr = buf.as_ptr();
    let mut buf_ptr = unsafe { start_ptr.add(buf_len) };
    //
    if buf_len >= 32 {
        let cc: MMB32Tpl = needle.into();
        // to a aligned pointer
        {
            if !buf_ptr.is_aligned_u256() {
                #[cfg(not(feature = "test_alignment_check"))]
                {
                    let loop_size = 32;
                    let buf_ptr_pre = unsafe { buf_ptr.sub(loop_size) };
                    let r = unsafe { _rnechr_c32_uu_x1(buf_ptr_pre, cc, start_ptr) };
                    if r.is_some() {
                        return r;
                    }
                    let remaining_align = 0x20_usize - ((buf_ptr as usize) & 0x1F_usize);
                    buf_ptr = unsafe { buf_ptr_pre.add(remaining_align) };
                }
                #[cfg(feature = "test_alignment_check")]
                {
                    let r = basic::_rnechr_tpl_to_aligned_u256(buf_ptr, needle, start_ptr);
                    if let Some(p) = r.0 {
                        buf_ptr = p;
                    } else if let Some(v) = r.1 {
                        return Some(v);
                    }
                }
            }
        }
        // the loop
        /*
        {
            let unroll = 2;
            let loop_size = 32;
            while buf_ptr.is_not_under(start_ptr, loop_size * unroll) {
                buf_ptr = unsafe { buf_ptr.sub(loop_size * unroll) };
                let r = unsafe { _rnechr_c32_aa_x2(buf_ptr, cc, start_ptr) };
                if r.is_some() {
                    return r;
                }
            }
        }
        */
        {
            let unroll = 1;
            let loop_size = 32;
            while buf_ptr.is_not_under(start_ptr, loop_size * unroll) {
                buf_ptr = unsafe { buf_ptr.sub(loop_size * unroll) };
                let r = unsafe { _rnechr_c32_aa_x1(buf_ptr, cc, start_ptr) };
                if r.is_some() {
                    return r;
                }
            }
        }
        {
            let cc: MMB16Tpl = needle.into();
            let unroll = 1;
            let loop_size = 16;
            while buf_ptr.is_not_under(start_ptr, loop_size * unroll) {
                buf_ptr = unsafe { buf_ptr.sub(loop_size) };
                let r = unsafe { _rnechr_c16_aa_x1(buf_ptr, cc, start_ptr) };
                if r.is_some() {
                    return r;
                }
            }
        }
    } else if buf_len >= 16 {
        {
            let cc: MMB16Tpl = needle.into();
            let unroll = 1;
            let loop_size = 16;
            if buf_ptr.is_not_under(start_ptr, loop_size * unroll) {
                let min_ptr = unsafe { start_ptr.add(loop_size) };
                //
                if buf_ptr.is_aligned_u128() {
                    while buf_ptr >= min_ptr {
                        buf_ptr = unsafe { buf_ptr.sub(loop_size) };
                        let r = unsafe { _rnechr_c16_aa_x1(buf_ptr, cc, start_ptr) };
                        if r.is_some() {
                            return r;
                        }
                    }
                } else {
                    #[cfg(not(feature = "test_alignment_check"))]
                    {
                        while buf_ptr >= min_ptr {
                            buf_ptr = unsafe { buf_ptr.sub(loop_size) };
                            let r = unsafe { _rnechr_c16_uu_x1(buf_ptr, cc, start_ptr) };
                            if r.is_some() {
                                return r;
                            }
                        }
                    }
                    #[cfg(feature = "test_alignment_check")]
                    {
                        let r = basic::_rnechr_tpl_to_aligned_u128(buf_ptr, needle, start_ptr);
                        if let Some(p) = r.0 {
                            buf_ptr = p;
                        } else if let Some(v) = r.1 {
                            return Some(v);
                        }
                        while buf_ptr >= min_ptr {
                            buf_ptr = unsafe { buf_ptr.sub(loop_size) };
                            let r = unsafe { _rnechr_c16_aa_x1(buf_ptr, cc, start_ptr) };
                            if r.is_some() {
                                return r;
                            }
                        }
                    }
                }
            }
        }
    }
    //
    let cc: B8Tpl = needle.into();
    basic::_memrnechr_tpl_remaining_15_bytes_impl(buf_ptr, cc, start_ptr)
}

#[inline(always)]
fn _return_rnechr_tpl<T, PU>(base: T, mask_a: PU) -> Option<usize>
where
    T: core::ops::Sub<usize, Output = usize>,
    PU: BitOrt,
{
    if !mask_a.is_highs() {
        let idx1 = mask_a.leading_ones() as usize;
        Some(base - idx1)
    } else {
        None
    }
}

#[inline(always)]
unsafe fn _rnechr_c16_uu_x1(
    buf_ptr: *const u8,
    mm_c16: MMB16Tpl,
    st_ptr: *const u8,
) -> Option<usize> {
    let mm_0 = _mm_loadu_si128(buf_ptr as *const __m128i);
    let mm_0_eq_a = _mm_cmpeq_epi8(mm_0, mm_c16.v1);
    let mm_0_eq_b = _mm_cmpeq_epi8(mm_0, mm_c16.v2);
    let mm_0_eq_c = _mm_cmpeq_epi8(mm_0, mm_c16.v3);
    let mm_0_eq_ab = _mm_or_si128(mm_0_eq_a, mm_0_eq_b);
    let mm_0_eq_abc = _mm_or_si128(mm_0_eq_ab, mm_0_eq_c);
    let mask_0_abc = _mm_movemask_epi8(mm_0_eq_abc) as u16;
    let base = buf_ptr.usz_offset_from(st_ptr) + 16 - 1;
    //
    _return_rnechr_tpl(base, mask_0_abc)
}

#[inline(always)]
unsafe fn _rnechr_c16_aa_x1(
    buf_ptr: *const u8,
    mm_c16: MMB16Tpl,
    st_ptr: *const u8,
) -> Option<usize> {
    let mm_0 = _mm_load_si128(buf_ptr as *const __m128i);
    let mm_0_eq_a = _mm_cmpeq_epi8(mm_0, mm_c16.v1);
    let mm_0_eq_b = _mm_cmpeq_epi8(mm_0, mm_c16.v2);
    let mm_0_eq_c = _mm_cmpeq_epi8(mm_0, mm_c16.v3);
    let mm_0_eq_ab = _mm_or_si128(mm_0_eq_a, mm_0_eq_b);
    let mm_0_eq_abc = _mm_or_si128(mm_0_eq_ab, mm_0_eq_c);
    let mask_0_abc = _mm_movemask_epi8(mm_0_eq_abc) as u16;
    let base = buf_ptr.usz_offset_from(st_ptr) + 16 - 1;
    //
    _return_rnechr_tpl(base, mask_0_abc)
}

#[inline(always)]
unsafe fn _rnechr_c16_aa_x2(
    buf_ptr: *const u8,
    mm_c16: MMB16Tpl,
    st_ptr: *const u8,
) -> Option<usize> {
    let r = _rnechr_c16_aa_x1(buf_ptr.add(16), mm_c16, st_ptr);
    if r.is_some() {
        return r;
    }
    let r = _rnechr_c16_aa_x1(buf_ptr, mm_c16, st_ptr);
    if r.is_some() {
        return r;
    }
    None
}

#[inline(always)]
unsafe fn _rnechr_c16_aa_x4(
    buf_ptr: *const u8,
    mm_c16: MMB16Tpl,
    st_ptr: *const u8,
) -> Option<usize> {
    let r = _rnechr_c16_aa_x2(buf_ptr.add(16 * 2), mm_c16, st_ptr);
    if r.is_some() {
        return r;
    }
    let r = _rnechr_c16_aa_x2(buf_ptr, mm_c16, st_ptr);
    if r.is_some() {
        return r;
    }
    None
}

#[inline(always)]
unsafe fn _rnechr_c16_aa_x8(
    buf_ptr: *const u8,
    mm_c16: MMB16Tpl,
    st_ptr: *const u8,
) -> Option<usize> {
    let r = _rnechr_c16_aa_x4(buf_ptr.add(16 * 4), mm_c16, st_ptr);
    if r.is_some() {
        return r;
    }
    let r = _rnechr_c16_aa_x4(buf_ptr, mm_c16, st_ptr);
    if r.is_some() {
        return r;
    }
    None
}

#[inline(always)]
unsafe fn _rnechr_c32_uu_x1(
    buf_ptr: *const u8,
    mm_c32: MMB32Tpl,
    st_ptr: *const u8,
) -> Option<usize> {
    let mm_0 = _mm256_loadu_si256(buf_ptr as *const __m256i);
    let mm_0_eq_a = _mm256_cmpeq_epi8(mm_0, mm_c32.v1);
    let mm_0_eq_b = _mm256_cmpeq_epi8(mm_0, mm_c32.v2);
    let mm_0_eq_c = _mm256_cmpeq_epi8(mm_0, mm_c32.v3);
    let mm_0_eq_ab = _mm256_or_si256(mm_0_eq_a, mm_0_eq_b);
    let mm_0_eq_abc = _mm256_or_si256(mm_0_eq_ab, mm_0_eq_c);
    let mask_0_abc = _mm256_movemask_epi8(mm_0_eq_abc) as u32;
    let base = buf_ptr.usz_offset_from(st_ptr) + 32 - 1;
    //
    _return_rnechr_tpl(base, mask_0_abc)
}

#[inline(always)]
unsafe fn _rnechr_c32_aa_x1(
    buf_ptr: *const u8,
    mm_c32: MMB32Tpl,
    st_ptr: *const u8,
) -> Option<usize> {
    let mm_0 = _mm256_load_si256(buf_ptr as *const __m256i);
    let mm_0_eq_a = _mm256_cmpeq_epi8(mm_0, mm_c32.v1);
    let mm_0_eq_b = _mm256_cmpeq_epi8(mm_0, mm_c32.v2);
    let mm_0_eq_c = _mm256_cmpeq_epi8(mm_0, mm_c32.v3);
    let mm_0_eq_ab = _mm256_or_si256(mm_0_eq_a, mm_0_eq_b);
    let mm_0_eq_abc = _mm256_or_si256(mm_0_eq_ab, mm_0_eq_c);
    let mask_0_abc = _mm256_movemask_epi8(mm_0_eq_abc) as u32;
    let base = buf_ptr.usz_offset_from(st_ptr) + 32 - 1;
    //
    _return_rnechr_tpl(base, mask_0_abc)
}

#[inline(always)]
unsafe fn _rnechr_c32_aa_x2(
    buf_ptr: *const u8,
    mm_c32: MMB32Tpl,
    st_ptr: *const u8,
) -> Option<usize> {
    let r = _rnechr_c32_aa_x1(buf_ptr.add(32), mm_c32, st_ptr);
    if r.is_some() {
        return r;
    }
    let r = _rnechr_c32_aa_x1(buf_ptr, mm_c32, st_ptr);
    if r.is_some() {
        return r;
    }
    None
}

#[inline(always)]
unsafe fn _rnechr_c32_aa_x4(
    buf_ptr: *const u8,
    mm_c32: MMB32Tpl,
    st_ptr: *const u8,
) -> Option<usize> {
    let r = _rnechr_c32_aa_x2(buf_ptr.add(32 * 2), mm_c32, st_ptr);
    if r.is_some() {
        return r;
    }
    let r = _rnechr_c32_aa_x2(buf_ptr, mm_c32, st_ptr);
    if r.is_some() {
        return r;
    }
    None
}

#[inline(always)]
unsafe fn _rnechr_c32_aa_x8(
    buf_ptr: *const u8,
    mm_c32: MMB32Tpl,
    st_ptr: *const u8,
) -> Option<usize> {
    let r = _rnechr_c32_aa_x4(buf_ptr.add(32 * 4), mm_c32, st_ptr);
    if r.is_some() {
        return r;
    }
    let r = _rnechr_c32_aa_x4(buf_ptr, mm_c32, st_ptr);
    if r.is_some() {
        return r;
    }
    None
}
