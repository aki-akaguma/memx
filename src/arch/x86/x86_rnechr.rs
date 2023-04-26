use super::{MMB16Sgl, MMB32Sgl};
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

use mmx::__m256i;
use mmx::_mm256_cmpeq_epi8;
use mmx::_mm256_load_si256;
use mmx::_mm256_loadu_si256;
use mmx::_mm256_movemask_epi8;

#[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
use super::cpuid;

use core::sync::atomic::AtomicPtr;
use core::sync::atomic::Ordering;
type FuncType = fn(&[u8], u8) -> Option<usize>;

const FUNC: FuncType = fnptr_setup_func;
static FUNC_PTR_ATOM: AtomicPtr<FuncType> = AtomicPtr::new(FUNC as *mut FuncType);

#[inline(never)]
fn fnptr_setup_func(buf: &[u8], c: u8) -> Option<usize> {
    #[cfg(target_arch = "x86_64")]
    let func = if cpuid::has_avx2() {
        _memrnechr_avx2
    } else {
        _memrnechr_sse2
    };
    #[cfg(target_arch = "x86")]
    let func = if cpuid::has_avx2() {
        _memrnechr_avx2
    } else if cpuid::has_sse2() {
        _memrnechr_sse2
    } else {
        _memrnechr_basic
    };
    //
    FUNC_PTR_ATOM.store(func as *mut FuncType, Ordering::Relaxed);
    unsafe { func(buf, c) }
}

#[inline(always)]
pub(crate) fn _memrnechr_impl(buf: &[u8], c: u8) -> Option<usize> {
    let func_u = FUNC_PTR_ATOM.load(Ordering::Relaxed);
    #[allow(clippy::crosspointer_transmute)]
    let func: FuncType = unsafe { core::mem::transmute(func_u) };
    func(buf, c)
}

unsafe fn _memrnechr_basic(buf: &[u8], c: u8) -> Option<usize> {
    basic::_memrnechr_impl(buf, c)
}

#[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
#[target_feature(enable = "sse2")]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn _memrnechr_sse2(buf: &[u8], c: u8) -> Option<usize> {
    _memrnechr_sse2_impl(buf, c)
}

#[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
#[target_feature(enable = "avx2")]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn _memrnechr_avx2(buf: &[u8], c: u8) -> Option<usize> {
    _memrnechr_avx2_impl(buf, c)
}

#[inline(always)]
fn _memrnechr_sse2_impl(buf: &[u8], c1: u8) -> Option<usize> {
    let buf_len = buf.len();
    let start_ptr = buf.as_ptr();
    let mut buf_ptr = unsafe { start_ptr.add(buf_len) };
    //
    if buf_len >= 16 {
        let cc = MMB16Sgl::new(c1);
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
                    let c = B1Sgl::new(c1);
                    let r = basic::_rnechr_to_aligned_u128(buf_ptr, c, start_ptr);
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
    let cc = B8Sgl::new(c1);
    basic::_memrnechr_remaining_15_bytes_impl(buf_ptr, cc, start_ptr)
}

#[inline(always)]
fn _memrnechr_avx2_impl(buf: &[u8], c1: u8) -> Option<usize> {
    let buf_len = buf.len();
    let start_ptr = buf.as_ptr();
    let mut buf_ptr = unsafe { start_ptr.add(buf_len) };
    //
    if buf_len >= 32 {
        let cc = MMB32Sgl::new(c1);
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
                    let c = B1Sgl::new(c1);
                    let r = basic::_rnechr_to_aligned_u256(buf_ptr, c, start_ptr);
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
            let cc = MMB16Sgl::new(c1);
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
            let cc = MMB16Sgl::new(c1);
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
                        let c = B1Sgl::new(c1);
                        let r = basic::_rnechr_to_aligned_u128(buf_ptr, c, start_ptr);
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
    let cc = B8Sgl::new(c1);
    basic::_memrnechr_remaining_15_bytes_impl(buf_ptr, cc, start_ptr)
}

#[inline(always)]
unsafe fn _rnechr_c16_uu_x1(
    buf_ptr: *const u8,
    mm_c16: MMB16Sgl,
    st_ptr: *const u8,
) -> Option<usize> {
    //
    let mm_a = _mm_loadu_si128(buf_ptr as *const __m128i);
    let mm_eq = _mm_cmpeq_epi8(mm_a, mm_c16.v1);
    let mask = _mm_movemask_epi8(mm_eq) as u32 & 0xFFFF_u32;
    let base = buf_ptr.usz_offset_from(st_ptr) + 16 - 1;
    //
    if mask != 0xFFFF_u32 {
        Some(base - (mask as u16).leading_ones() as usize)
    } else {
        None
    }
}

#[inline(always)]
unsafe fn _rnechr_c16_aa_x1(
    buf_ptr: *const u8,
    mm_c16: MMB16Sgl,
    st_ptr: *const u8,
) -> Option<usize> {
    //
    let mm_a = _mm_load_si128(buf_ptr as *const __m128i);
    let mm_eq = _mm_cmpeq_epi8(mm_a, mm_c16.v1);
    let mask = _mm_movemask_epi8(mm_eq) as u32 & 0xFFFF_u32;
    let base = buf_ptr.usz_offset_from(st_ptr) + 16 - 1;
    //
    if mask != 0xFFFF_u32 {
        Some(base - (mask as u16).leading_ones() as usize)
    } else {
        None
    }
}

#[inline(always)]
unsafe fn _rnechr_c16_aa_x2(
    buf_ptr: *const u8,
    mm_c16: MMB16Sgl,
    st_ptr: *const u8,
) -> Option<usize> {
    //
    let mm_a = _mm_load_si128(buf_ptr as *const __m128i);
    let mm_b = _mm_load_si128(buf_ptr.add(16) as *const __m128i);
    let mm_a_eq = _mm_cmpeq_epi8(mm_a, mm_c16.v1);
    let mm_b_eq = _mm_cmpeq_epi8(mm_b, mm_c16.v1);
    let mask_a = _mm_movemask_epi8(mm_a_eq) as u32 & 0xFFFF_u32;
    let mask_b = _mm_movemask_epi8(mm_b_eq) as u32 & 0xFFFF_u32;
    let base = buf_ptr.usz_offset_from(st_ptr) + 16 - 1;
    //
    if mask_b != 0xFFFF_u32 {
        Some(base - (mask_b as u16).leading_ones() as usize + 16)
    } else if mask_a != 0xFFFF_u32 {
        Some(base - (mask_a as u16).leading_ones() as usize)
    } else {
        None
    }
}

#[inline(always)]
unsafe fn _rnechr_c16_aa_x4(
    buf_ptr: *const u8,
    mm_c16: MMB16Sgl,
    st_ptr: *const u8,
) -> Option<usize> {
    //
    let mm_a = _mm_load_si128(buf_ptr as *const __m128i);
    let mm_b = _mm_load_si128(buf_ptr.add(16) as *const __m128i);
    let mm_c = _mm_load_si128(buf_ptr.add(16 * 2) as *const __m128i);
    let mm_d = _mm_load_si128(buf_ptr.add(16 * 3) as *const __m128i);
    let mm_a_eq = _mm_cmpeq_epi8(mm_a, mm_c16.v1);
    let mm_b_eq = _mm_cmpeq_epi8(mm_b, mm_c16.v1);
    let mm_c_eq = _mm_cmpeq_epi8(mm_c, mm_c16.v1);
    let mm_d_eq = _mm_cmpeq_epi8(mm_d, mm_c16.v1);
    let mask_a = _mm_movemask_epi8(mm_a_eq) as u32 & 0xFFFF_u32;
    let mask_b = _mm_movemask_epi8(mm_b_eq) as u32 & 0xFFFF_u32;
    let mask_c = _mm_movemask_epi8(mm_c_eq) as u32 & 0xFFFF_u32;
    let mask_d = _mm_movemask_epi8(mm_d_eq) as u32 & 0xFFFF_u32;
    let base = buf_ptr.usz_offset_from(st_ptr) + 16 - 1;
    //
    if mask_d != 0xFFFF_u32 {
        Some(base - (mask_d as u16).leading_ones() as usize + 16 * 3)
    } else if mask_c != 0xFFFF_u32 {
        Some(base - (mask_c as u16).leading_ones() as usize + 16 * 2)
    } else if mask_b != 0xFFFF_u32 {
        Some(base - (mask_b as u16).leading_ones() as usize + 16)
    } else if mask_a != 0xFFFF_u32 {
        Some(base - (mask_a as u16).leading_ones() as usize)
    } else {
        None
    }
}

#[inline(always)]
unsafe fn _rnechr_c16_aa_x8(
    buf_ptr: *const u8,
    mm_c16: MMB16Sgl,
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
    mm_c32: MMB32Sgl,
    st_ptr: *const u8,
) -> Option<usize> {
    //
    let mm_a = _mm256_loadu_si256(buf_ptr as *const __m256i);
    let mm_eq = _mm256_cmpeq_epi8(mm_a, mm_c32.v1);
    let mask = _mm256_movemask_epi8(mm_eq) as u32;
    let base = buf_ptr.usz_offset_from(st_ptr) + 32 - 1;
    //
    if mask != 0xFFFF_FFFF_u32 {
        Some(base - mask.leading_ones() as usize)
    } else {
        None
    }
}

#[inline(always)]
unsafe fn _rnechr_c32_aa_x1(
    buf_ptr: *const u8,
    mm_c32: MMB32Sgl,
    st_ptr: *const u8,
) -> Option<usize> {
    //
    let mm_a = _mm256_load_si256(buf_ptr as *const __m256i);
    let mm_eq = _mm256_cmpeq_epi8(mm_a, mm_c32.v1);
    let mask = _mm256_movemask_epi8(mm_eq) as u32;
    let base = buf_ptr.usz_offset_from(st_ptr) + 32 - 1;
    //
    if mask != 0xFFFF_FFFF_u32 {
        Some(base - mask.leading_ones() as usize)
    } else {
        None
    }
}

#[inline(always)]
unsafe fn _rnechr_c32_aa_x2(
    buf_ptr: *const u8,
    mm_c32: MMB32Sgl,
    st_ptr: *const u8,
) -> Option<usize> {
    //
    let mm_a = _mm256_load_si256(buf_ptr as *const __m256i);
    let mm_b = _mm256_load_si256(buf_ptr.add(32) as *const __m256i);
    let mm_a_eq = _mm256_cmpeq_epi8(mm_a, mm_c32.v1);
    let mm_b_eq = _mm256_cmpeq_epi8(mm_b, mm_c32.v1);
    let mask_a = _mm256_movemask_epi8(mm_a_eq) as u32;
    let mask_b = _mm256_movemask_epi8(mm_b_eq) as u32;
    let base = buf_ptr.usz_offset_from(st_ptr) + 32 - 1;
    //
    if mask_b != 0xFFFF_FFFF_u32 {
        Some(base - mask_b.leading_ones() as usize + 32)
    } else if mask_a != 0xFFFF_FFFF_u32 {
        Some(base - mask_a.leading_ones() as usize)
    } else {
        None
    }
}

#[inline(always)]
unsafe fn _rnechr_c32_aa_x4(
    buf_ptr: *const u8,
    mm_c32: MMB32Sgl,
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
    mm_c32: MMB32Sgl,
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
