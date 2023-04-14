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
        _memnechr_avx2
    } else {
        _memnechr_sse2
    };
    #[cfg(target_arch = "x86")]
    let func = if cpuid::has_avx2() {
        _memnechr_avx2
    } else if cpuid::has_sse2() {
        _memnechr_sse2
    } else {
        _memnechr_basic
    };
    //
    FUNC_PTR_ATOM.store(func as *mut FuncType, Ordering::Relaxed);
    unsafe { func(buf, c) }
}

#[inline(always)]
pub(crate) fn _memnechr_impl(buf: &[u8], c: u8) -> Option<usize> {
    let func_u = FUNC_PTR_ATOM.load(Ordering::Relaxed);
    #[allow(clippy::crosspointer_transmute)]
    let func: FuncType = unsafe { core::mem::transmute(func_u) };
    func(buf, c)
}

unsafe fn _memnechr_basic(buf: &[u8], c: u8) -> Option<usize> {
    basic::_memnechr_impl(buf, c)
}

#[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
#[target_feature(enable = "sse2")]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn _memnechr_sse2(buf: &[u8], c: u8) -> Option<usize> {
    _memnechr_sse2_impl(buf, c)
}

#[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
#[target_feature(enable = "avx2")]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn _memnechr_avx2(buf: &[u8], c: u8) -> Option<usize> {
    _memnechr_avx2_impl(buf, c)
}

#[inline(always)]
fn _memnechr_sse2_impl(buf: &[u8], c1: u8) -> Option<usize> {
    let buf_len = buf.len();
    let mut buf_ptr = buf.as_ptr();
    let start_ptr = buf_ptr;
    let end_ptr = unsafe { buf_ptr.add(buf_len) };
    buf_ptr.prefetch_read_data();
    //
    if buf_len >= 16 {
        let cc = MMB16Sgl::new(c1);
        // to a aligned pointer
        {
            let remaining_align = 0x10_usize - ((buf_ptr as usize) & 0x0F_usize);
            if buf_ptr.is_aligned_u128() {
                let r = unsafe { _nechr_c16_aa_x1(buf_ptr, cc, start_ptr) };
                if r.is_some() {
                    return r;
                }
                buf_ptr = unsafe { buf_ptr.add(remaining_align) };
            } else {
                #[cfg(not(feature = "test_alignment_check"))]
                {
                    let r = unsafe { _nechr_c16_uu_x1(buf_ptr, cc, start_ptr) };
                    if r.is_some() {
                        return r;
                    }
                    buf_ptr = unsafe { buf_ptr.add(remaining_align) };
                }
                #[cfg(feature = "test_alignment_check")]
                {
                    let c = B1Sgl::new(c1);
                    let r = basic::_nechr_to_aligned_u128(buf_ptr, c, start_ptr);
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
            let unroll = 8;
            let loop_size = 16;
            if unsafe { end_ptr.offset_from(buf_ptr) } >= (loop_size * unroll) as isize {
                let end_ptr_16_x8 = unsafe { end_ptr.sub(loop_size * unroll) };
                while buf_ptr <= end_ptr_16_x8 {
                    buf_ptr.prefetch_read_data();
                    let r = unsafe { _nechr_c16_aa_x8(buf_ptr, cc, start_ptr) };
                    if r.is_some() {
                        return r;
                    }
                    buf_ptr = unsafe { buf_ptr.add(loop_size * unroll) };
                }
            }
        }
        {
            let unroll = 4;
            let loop_size = 16;
            if unsafe { end_ptr.offset_from(buf_ptr) } >= (loop_size * unroll) as isize {
                let end_ptr_16_x4 = unsafe { end_ptr.sub(loop_size * unroll) };
                while buf_ptr <= end_ptr_16_x4 {
                    buf_ptr.prefetch_read_data();
                    let r = unsafe { _nechr_c16_aa_x4(buf_ptr, cc, start_ptr) };
                    if r.is_some() {
                        return r;
                    }
                    buf_ptr = unsafe { buf_ptr.add(loop_size * unroll) };
                }
            }
        }
        {
            let unroll = 2;
            let loop_size = 16;
            if unsafe { end_ptr.offset_from(buf_ptr) } >= (loop_size * unroll) as isize {
                let end_ptr_16_x2 = unsafe { end_ptr.sub(loop_size * unroll) };
                while buf_ptr <= end_ptr_16_x2 {
                    let r = unsafe { _nechr_c16_aa_x2(buf_ptr, cc, start_ptr) };
                    if r.is_some() {
                        return r;
                    }
                    buf_ptr = unsafe { buf_ptr.add(loop_size * unroll) };
                }
            }
        }
        {
            let unroll = 1;
            let loop_size = 16;
            if unsafe { end_ptr.offset_from(buf_ptr) } >= (loop_size * unroll) as isize {
                let end_ptr_16_x1 = unsafe { end_ptr.sub(loop_size * unroll) };
                while buf_ptr <= end_ptr_16_x1 {
                    let r = unsafe { _nechr_c16_aa_x1(buf_ptr, cc, start_ptr) };
                    if r.is_some() {
                        return r;
                    }
                    buf_ptr = unsafe { buf_ptr.add(loop_size * unroll) };
                }
            }
        }
    }
    //
    let cc = B8Sgl::new(c1);
    basic::_memnechr_remaining_15_bytes_impl(buf_ptr, cc, start_ptr, end_ptr)
}

#[inline(always)]
fn _memnechr_avx2_impl(buf: &[u8], c1: u8) -> Option<usize> {
    let buf_len = buf.len();
    let mut buf_ptr = buf.as_ptr();
    let start_ptr = buf_ptr;
    let end_ptr = unsafe { buf_ptr.add(buf_len) };
    buf_ptr.prefetch_read_data();
    //
    if buf_len >= 32 {
        let cc = MMB32Sgl::new(c1);
        // to a aligned pointer
        {
            let remaining_align = 0x20_usize - ((buf_ptr as usize) & 0x1F_usize);
            if buf_ptr.is_aligned_u256() {
                let r = unsafe { _nechr_c32_aa_x1(buf_ptr, cc, start_ptr) };
                if r.is_some() {
                    return r;
                }
                buf_ptr = unsafe { buf_ptr.add(remaining_align) };
            } else {
                #[cfg(not(feature = "test_alignment_check"))]
                {
                    let r = unsafe { _nechr_c32_uu_x1(buf_ptr, cc, start_ptr) };
                    if r.is_some() {
                        return r;
                    }
                    buf_ptr = unsafe { buf_ptr.add(remaining_align) };
                }
                #[cfg(feature = "test_alignment_check")]
                {
                    let c = B1Sgl::new(c1);
                    let r = basic::_nechr_to_aligned_u256(buf_ptr, c, start_ptr);
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
            let unroll = 8;
            let loop_size = 32;
            if unsafe { end_ptr.offset_from(buf_ptr) } >= (loop_size * unroll) as isize {
                let end_ptr_32_x8 = unsafe { end_ptr.sub(loop_size * unroll) };
                while buf_ptr <= end_ptr_32_x8 {
                    buf_ptr.prefetch_read_data();
                    let r = unsafe { _nechr_c32_aa_x8(buf_ptr, cc, start_ptr) };
                    if r.is_some() {
                        return r;
                    }
                    buf_ptr = unsafe { buf_ptr.add(loop_size * unroll) };
                }
            }
        }
        {
            let unroll = 4;
            let loop_size = 32;
            if unsafe { end_ptr.offset_from(buf_ptr) } >= (loop_size * unroll) as isize {
                let end_ptr_32_x4 = unsafe { end_ptr.sub(loop_size * unroll) };
                while buf_ptr <= end_ptr_32_x4 {
                    buf_ptr.prefetch_read_data();
                    let r = unsafe { _nechr_c32_aa_x4(buf_ptr, cc, start_ptr) };
                    if r.is_some() {
                        return r;
                    }
                    buf_ptr = unsafe { buf_ptr.add(loop_size * unroll) };
                }
            }
        }
        {
            let unroll = 2;
            let loop_size = 32;
            if unsafe { end_ptr.offset_from(buf_ptr) } >= (loop_size * unroll) as isize {
                let end_ptr_32_x2 = unsafe { end_ptr.sub(loop_size * unroll) };
                while buf_ptr <= end_ptr_32_x2 {
                    buf_ptr.prefetch_read_data();
                    let r = unsafe { _nechr_c32_aa_x2(buf_ptr, cc, start_ptr) };
                    if r.is_some() {
                        return r;
                    }
                    buf_ptr = unsafe { buf_ptr.add(loop_size * unroll) };
                }
            }
        }
        {
            let unroll = 1;
            let loop_size = 32;
            if unsafe { end_ptr.offset_from(buf_ptr) } >= (loop_size * unroll) as isize {
                let end_ptr_32_x1 = unsafe { end_ptr.sub(loop_size) };
                while buf_ptr <= end_ptr_32_x1 {
                    let r = unsafe { _nechr_c32_aa_x1(buf_ptr, cc, start_ptr) };
                    if r.is_some() {
                        return r;
                    }
                    buf_ptr = unsafe { buf_ptr.add(loop_size) };
                }
            }
        }
        {
            let cc = MMB16Sgl::new(c1);
            let unroll = 1;
            let loop_size = 16;
            if unsafe { end_ptr.offset_from(buf_ptr) } >= (loop_size * unroll) as isize {
                let end_ptr_16_x1 = unsafe { end_ptr.sub(loop_size) };
                while buf_ptr <= end_ptr_16_x1 {
                    let r = unsafe { _nechr_c16_aa_x1(buf_ptr, cc, start_ptr) };
                    if r.is_some() {
                        return r;
                    }
                    buf_ptr = unsafe { buf_ptr.add(loop_size) };
                }
            }
        }
    } else if buf_len >= 16 {
        {
            let cc = MMB16Sgl::new(c1);
            let unroll = 1;
            let loop_size = 16;
            if unsafe { end_ptr.offset_from(buf_ptr) } >= (loop_size * unroll) as isize {
                let end_ptr_16_x1 = unsafe { end_ptr.sub(loop_size * unroll) };
                //
                if buf_ptr.is_aligned_u128() {
                    while buf_ptr <= end_ptr_16_x1 {
                        let r = unsafe { _nechr_c16_aa_x1(buf_ptr, cc, start_ptr) };
                        if r.is_some() {
                            return r;
                        }
                        buf_ptr = unsafe { buf_ptr.add(loop_size * unroll) };
                    }
                } else {
                    #[cfg(not(feature = "test_alignment_check"))]
                    {
                        while buf_ptr <= end_ptr_16_x1 {
                            let r = unsafe { _nechr_c16_uu_x1(buf_ptr, cc, start_ptr) };
                            if r.is_some() {
                                return r;
                            }
                            buf_ptr = unsafe { buf_ptr.add(loop_size * unroll) };
                        }
                    }
                    #[cfg(feature = "test_alignment_check")]
                    {
                        let c = B1Sgl::new(c1);
                        let r = basic::_nechr_to_aligned_u128(buf_ptr, c, start_ptr);
                        if let Some(p) = r.0 {
                            buf_ptr = p;
                        } else if let Some(v) = r.1 {
                            return Some(v);
                        }
                        while buf_ptr <= end_ptr_16_x1 {
                            let r = unsafe { _nechr_c16_aa_x1(buf_ptr, cc, start_ptr) };
                            if r.is_some() {
                                return r;
                            }
                            buf_ptr = unsafe { buf_ptr.add(loop_size * unroll) };
                        }
                    }
                }
            }
        }
    }
    //
    let cc = B8Sgl::new(c1);
    basic::_memnechr_remaining_15_bytes_impl(buf_ptr, cc, start_ptr, end_ptr)
}

#[inline(always)]
unsafe fn _nechr_c16_uu_x1(
    buf_ptr: *const u8,
    mm_c16: MMB16Sgl,
    st_ptr: *const u8,
) -> Option<usize> {
    //
    let mm_0 = _mm_loadu_si128(buf_ptr as *const __m128i);
    let mm_0_eq = _mm_cmpeq_epi8(mm_0, mm_c16.a);
    let mask_0 = _mm_movemask_epi8(mm_0_eq) as u32 & 0xFFFF_u32;
    if mask_0 != 0xFFFF_u32 {
        Some(buf_ptr.usz_offset_from(st_ptr) + mask_0.trailing_ones() as usize)
    } else {
        None
    }
}

#[inline(always)]
unsafe fn _nechr_c16_aa_x1(
    buf_ptr: *const u8,
    mm_c16: MMB16Sgl,
    st_ptr: *const u8,
) -> Option<usize> {
    //
    let mm_0 = _mm_load_si128(buf_ptr as *const __m128i);
    let mm_0_eq = _mm_cmpeq_epi8(mm_0, mm_c16.a);
    let mask_0 = _mm_movemask_epi8(mm_0_eq) as u32 & 0xFFFF_u32;
    if mask_0 != 0xFFFF_u32 {
        Some(buf_ptr.usz_offset_from(st_ptr) + mask_0.trailing_ones() as usize)
    } else {
        None
    }
}

#[inline(always)]
unsafe fn _nechr_c16_aa_x2(
    buf_ptr: *const u8,
    mm_c16: MMB16Sgl,
    st_ptr: *const u8,
) -> Option<usize> {
    //
    let mm_0 = _mm_load_si128(buf_ptr as *const __m128i);
    let mm_1 = _mm_load_si128(buf_ptr.add(16) as *const __m128i);
    let mm_0_eq = _mm_cmpeq_epi8(mm_0, mm_c16.a);
    let mm_1_eq = _mm_cmpeq_epi8(mm_1, mm_c16.a);
    let mask_0 = _mm_movemask_epi8(mm_0_eq) as u32 & 0xFFFF_u32;
    let mask_1 = _mm_movemask_epi8(mm_1_eq) as u32 & 0xFFFF_u32;
    if mask_0 != 0xFFFF_u32 {
        Some(buf_ptr.usz_offset_from(st_ptr) + mask_0.trailing_ones() as usize)
    } else if mask_1 != 0xFFFF_u32 {
        Some(buf_ptr.usz_offset_from(st_ptr) + mask_1.trailing_ones() as usize + 16)
    } else {
        None
    }
}

#[inline(always)]
unsafe fn _nechr_c16_aa_x4(
    buf_ptr: *const u8,
    mm_c16: MMB16Sgl,
    st_ptr: *const u8,
) -> Option<usize> {
    let r = _nechr_c16_aa_x2(buf_ptr, mm_c16, st_ptr);
    if r.is_some() {
        return r;
    }
    let r = _nechr_c16_aa_x2(buf_ptr.add(16 * 2), mm_c16, st_ptr);
    if r.is_some() {
        return r;
    }
    None
}

#[inline(always)]
unsafe fn _nechr_c16_aa_x8(
    buf_ptr: *const u8,
    mm_c16: MMB16Sgl,
    st_ptr: *const u8,
) -> Option<usize> {
    let r = _nechr_c16_aa_x4(buf_ptr, mm_c16, st_ptr);
    if r.is_some() {
        return r;
    }
    let r = _nechr_c16_aa_x4(buf_ptr.add(16 * 4), mm_c16, st_ptr);
    if r.is_some() {
        return r;
    }
    None
}

#[inline(always)]
unsafe fn _nechr_c32_uu_x1(
    buf_ptr: *const u8,
    mm_c32: MMB32Sgl,
    st_ptr: *const u8,
) -> Option<usize> {
    //
    let mm_0 = _mm256_loadu_si256(buf_ptr as *const __m256i);
    let mm_0_eq = _mm256_cmpeq_epi8(mm_0, mm_c32.a);
    let mask_0 = _mm256_movemask_epi8(mm_0_eq) as u32;
    if mask_0 != 0xFFFF_FFFF_u32 {
        Some(buf_ptr.usz_offset_from(st_ptr) + mask_0.trailing_ones() as usize)
    } else {
        None
    }
}

#[inline(always)]
unsafe fn _nechr_c32_aa_x1(
    buf_ptr: *const u8,
    mm_c32: MMB32Sgl,
    st_ptr: *const u8,
) -> Option<usize> {
    //
    let mm_0 = _mm256_load_si256(buf_ptr as *const __m256i);
    let mm_0_eq = _mm256_cmpeq_epi8(mm_0, mm_c32.a);
    let mask_0 = _mm256_movemask_epi8(mm_0_eq) as u32;
    if mask_0 != 0xFFFF_FFFF_u32 {
        Some(buf_ptr.usz_offset_from(st_ptr) + mask_0.trailing_ones() as usize)
    } else {
        None
    }
}

#[inline(always)]
unsafe fn _nechr_c32_aa_x2(
    buf_ptr: *const u8,
    mm_c32: MMB32Sgl,
    st_ptr: *const u8,
) -> Option<usize> {
    //
    let mm_0 = _mm256_load_si256(buf_ptr as *const __m256i);
    let mm_1 = _mm256_load_si256(buf_ptr.add(32) as *const __m256i);
    let mm_0_eq = _mm256_cmpeq_epi8(mm_0, mm_c32.a);
    let mm_1_eq = _mm256_cmpeq_epi8(mm_1, mm_c32.a);
    let mask_0 = _mm256_movemask_epi8(mm_0_eq) as u32;
    let mask_1 = _mm256_movemask_epi8(mm_1_eq) as u32;
    if mask_0 != 0xFFFF_FFFF_u32 {
        Some(buf_ptr.usz_offset_from(st_ptr) + mask_0.trailing_ones() as usize)
    } else if mask_1 != 0xFFFF_FFFF_u32 {
        Some(buf_ptr.usz_offset_from(st_ptr) + mask_1.trailing_ones() as usize + 32)
    } else {
        None
    }
}

#[inline(always)]
unsafe fn _nechr_c32_aa_x4(
    buf_ptr: *const u8,
    mm_c32: MMB32Sgl,
    st_ptr: *const u8,
) -> Option<usize> {
    let r = _nechr_c32_aa_x2(buf_ptr, mm_c32, st_ptr);
    if r.is_some() {
        return r;
    }
    let r = _nechr_c32_aa_x2(buf_ptr.add(32 * 2), mm_c32, st_ptr);
    if r.is_some() {
        return r;
    }
    None
}

#[inline(always)]
unsafe fn _nechr_c32_aa_x8(
    buf_ptr: *const u8,
    mm_c32: MMB32Sgl,
    st_ptr: *const u8,
) -> Option<usize> {
    let r = _nechr_c32_aa_x4(buf_ptr, mm_c32, st_ptr);
    if r.is_some() {
        return r;
    }
    let r = _nechr_c32_aa_x4(buf_ptr.add(32 * 4), mm_c32, st_ptr);
    if r.is_some() {
        return r;
    }
    None
}
