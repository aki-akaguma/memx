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
        _memrchr_avx2
    } else {
        _memrchr_sse2
    };
    #[cfg(target_arch = "x86")]
    let func = if cpuid::has_avx2() {
        _memrchr_avx2
    } else if cpuid::has_sse2() {
        _memrchr_sse2
    } else {
        _memrchr_basic
    };
    //
    FUNC_PTR_ATOM.store(func as *mut FuncType, Ordering::Relaxed);
    unsafe { func(buf, c) }
}

#[inline(always)]
pub(crate) fn _memrchr_impl(buf: &[u8], c: u8) -> Option<usize> {
    let func_u = FUNC_PTR_ATOM.load(Ordering::Relaxed);
    #[allow(clippy::crosspointer_transmute)]
    let func: FuncType = unsafe { core::mem::transmute(func_u) };
    func(buf, c)
}

unsafe fn _memrchr_basic(buf: &[u8], c: u8) -> Option<usize> {
    basic::_memrchr_impl(buf, c)
}

#[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
#[target_feature(enable = "sse2")]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn _memrchr_sse2(buf: &[u8], c: u8) -> Option<usize> {
    _memrchr_sse2_impl(buf, c)
}

#[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
#[target_feature(enable = "avx2")]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn _memrchr_avx2(buf: &[u8], c: u8) -> Option<usize> {
    _memrchr_avx2_impl(buf, c)
}

#[inline(always)]
fn _memrchr_sse2_impl(buf: &[u8], c1: u8) -> Option<usize> {
    let buf_len = buf.len();
    let start_ptr = buf.as_ptr();
    let mut buf_ptr_cur = unsafe { start_ptr.add(buf_len) };
    //
    if buf_len >= 16 {
        let cc = MMB16Sgl::new(c1);
        // to a aligned pointer
        {
            let loop_size = 16;
            let buf_ptr = unsafe { buf_ptr_cur.sub(loop_size) };
            buf_ptr.prefetch_read_data();
            let remaining_align = 0x10_usize - ((buf_ptr as usize) & 0x0F_usize);
            if buf_ptr.is_aligned_u128() {
                let r = unsafe { _rchr_c16_aa_x1(buf_ptr, cc, start_ptr) };
                if r.is_some() {
                    return r;
                }
                buf_ptr_cur = unsafe { buf_ptr.add(remaining_align) };
            } else {
                #[cfg(not(feature = "test_alignment_check"))]
                {
                    let r = unsafe { _rchr_c16_uu_x1(buf_ptr, cc, start_ptr) };
                    if r.is_some() {
                        return r;
                    }
                    buf_ptr_cur = unsafe { buf_ptr.add(remaining_align) };
                }
                #[cfg(feature = "test_alignment_check")]
                {
                    let c = B1Sgl::new(c1);
                    let r = basic::_rchr_to_aligned_u128(buf_ptr_cur, c, start_ptr);
                    if let Some(p) = r.0 {
                        buf_ptr_cur = p;
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
            let mut buf_ptr = buf_ptr_cur;
            if unsafe { buf_ptr.offset_from(start_ptr) } >= (loop_size * unroll) as isize {
                let min_ptr = unsafe { start_ptr.add(loop_size * unroll) };
                while buf_ptr >= min_ptr {
                    buf_ptr = unsafe { buf_ptr.sub(loop_size * unroll) };
                    buf_ptr.prefetch_read_data();
                    let r = unsafe { _rchr_c16_aa_x8(buf_ptr, cc, start_ptr) };
                    if r.is_some() {
                        return r;
                    }
                }
                buf_ptr_cur = buf_ptr;
            }
        }
        {
            let unroll = 4;
            let loop_size = 16;
            let mut buf_ptr = buf_ptr_cur;
            if unsafe { buf_ptr.offset_from(start_ptr) } >= (loop_size * unroll) as isize {
                let min_ptr = unsafe { start_ptr.add(loop_size * unroll) };
                while buf_ptr >= min_ptr {
                    buf_ptr = unsafe { buf_ptr.sub(loop_size * unroll) };
                    buf_ptr.prefetch_read_data();
                    let r = unsafe { _rchr_c16_aa_x4(buf_ptr, cc, start_ptr) };
                    if r.is_some() {
                        return r;
                    }
                }
                buf_ptr_cur = buf_ptr;
            }
        }
        {
            let unroll = 2;
            let loop_size = 16;
            let mut buf_ptr = buf_ptr_cur;
            if unsafe { buf_ptr.offset_from(start_ptr) } >= (loop_size * unroll) as isize {
                let min_ptr = unsafe { start_ptr.add(loop_size * unroll) };
                while buf_ptr >= min_ptr {
                    buf_ptr = unsafe { buf_ptr.sub(loop_size * unroll) };
                    let r = unsafe { _rchr_c16_aa_x2(buf_ptr, cc, start_ptr) };
                    if r.is_some() {
                        return r;
                    }
                }
                buf_ptr_cur = buf_ptr;
            }
        }
        {
            let unroll = 1;
            let loop_size = 16;
            let mut buf_ptr = buf_ptr_cur;
            if unsafe { buf_ptr.offset_from(start_ptr) } >= (loop_size * unroll) as isize {
                let min_ptr = unsafe { start_ptr.add(loop_size * unroll) };
                while buf_ptr >= min_ptr {
                    buf_ptr = unsafe { buf_ptr.sub(loop_size * unroll) };
                    let r = unsafe { _rchr_c16_aa_x1(buf_ptr, cc, start_ptr) };
                    if r.is_some() {
                        return r;
                    }
                }
                buf_ptr_cur = buf_ptr;
            }
        }
    }
    start_ptr.prefetch_read_data();
    //
    let cc = B8Sgl::new(c1);
    basic::_memrchr_remaining_15_bytes_impl(buf_ptr_cur, cc, start_ptr)
}

#[allow(clippy::missing_safety_doc)]
#[inline(always)]
fn _memrchr_avx2_impl(buf: &[u8], c1: u8) -> Option<usize> {
    let buf_len = buf.len();
    let start_ptr = buf.as_ptr();
    let mut buf_ptr_cur = unsafe { start_ptr.add(buf_len) };
    //
    if buf_len >= 32 {
        let cc = MMB32Sgl::new(c1);
        // to a aligned pointer
        {
            let loop_size = 32;
            let buf_ptr = unsafe { buf_ptr_cur.sub(loop_size) };
            buf_ptr.prefetch_read_data();
            let remaining_align = 0x20_usize - ((buf_ptr as usize) & 0x1F_usize);
            if buf_ptr.is_aligned_u256() {
                let r = unsafe { _rchr_c32_aa_x1(buf_ptr, cc, start_ptr) };
                if r.is_some() {
                    return r;
                }
                buf_ptr_cur = unsafe { buf_ptr.add(remaining_align) };
            } else {
                #[cfg(not(feature = "test_alignment_check"))]
                {
                    let r = unsafe { _rchr_c32_uu_x1(buf_ptr, cc, start_ptr) };
                    if r.is_some() {
                        return r;
                    }
                    buf_ptr_cur = unsafe { buf_ptr.add(remaining_align) };
                }
                #[cfg(feature = "test_alignment_check")]
                {
                    let c = B1Sgl::new(c1);
                    let r = basic::_rchr_to_aligned_u256(buf_ptr_cur, c, start_ptr);
                    if let Some(p) = r.0 {
                        buf_ptr_cur = p;
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
            let mut buf_ptr = buf_ptr_cur;
            if unsafe { buf_ptr.offset_from(start_ptr) } >= (loop_size * unroll) as isize {
                let min_ptr = unsafe { start_ptr.add(loop_size * unroll) };
                while buf_ptr >= min_ptr {
                    buf_ptr = unsafe { buf_ptr.sub(loop_size * unroll) };
                    buf_ptr.prefetch_read_data();
                    let r = unsafe { _rchr_c32_aa_x8(buf_ptr, cc, start_ptr) };
                    if r.is_some() {
                        return r;
                    }
                }
                buf_ptr_cur = buf_ptr;
            }
        }
        {
            let unroll = 4;
            let loop_size = 32;
            let mut buf_ptr = buf_ptr_cur;
            if unsafe { buf_ptr.offset_from(start_ptr) } >= (loop_size * unroll) as isize {
                let min_ptr = unsafe { start_ptr.add(loop_size * unroll) };
                while buf_ptr >= min_ptr {
                    buf_ptr = unsafe { buf_ptr.sub(loop_size * unroll) };
                    buf_ptr.prefetch_read_data();
                    let r = unsafe { _rchr_c32_aa_x4(buf_ptr, cc, start_ptr) };
                    if r.is_some() {
                        return r;
                    }
                }
                buf_ptr_cur = buf_ptr;
            }
        }
        {
            let unroll = 2;
            let loop_size = 32;
            let mut buf_ptr = buf_ptr_cur;
            if unsafe { buf_ptr.offset_from(start_ptr) } >= (loop_size * unroll) as isize {
                let min_ptr = unsafe { start_ptr.add(loop_size * unroll) };
                while buf_ptr >= min_ptr {
                    buf_ptr = unsafe { buf_ptr.sub(loop_size * unroll) };
                    buf_ptr.prefetch_read_data();
                    let r = unsafe { _rchr_c32_aa_x2(buf_ptr, cc, start_ptr) };
                    if r.is_some() {
                        return r;
                    }
                }
                buf_ptr_cur = buf_ptr;
            }
        }
        {
            let unroll = 1;
            let loop_size = 32;
            let mut buf_ptr = buf_ptr_cur;
            if unsafe { buf_ptr.offset_from(start_ptr) } >= (loop_size * unroll) as isize {
                let min_ptr = unsafe { start_ptr.add(loop_size * unroll) };
                while buf_ptr >= min_ptr {
                    buf_ptr = unsafe { buf_ptr.sub(loop_size * unroll) };
                    let r = unsafe { _rchr_c32_aa_x1(buf_ptr, cc, start_ptr) };
                    if r.is_some() {
                        return r;
                    }
                }
                buf_ptr_cur = buf_ptr;
            }
        }
        {
            let cc = MMB16Sgl::new(c1);
            let unroll = 1;
            let loop_size = 16;
            let mut buf_ptr = buf_ptr_cur;
            if unsafe { buf_ptr.offset_from(start_ptr) } >= (loop_size * unroll) as isize {
                let min_ptr = unsafe { start_ptr.add(loop_size) };
                while buf_ptr >= min_ptr {
                    buf_ptr = unsafe { buf_ptr.sub(loop_size) };
                    let r = unsafe { _rchr_c16_aa_x1(buf_ptr, cc, start_ptr) };
                    if r.is_some() {
                        return r;
                    }
                }
                buf_ptr_cur = buf_ptr;
            }
        }
    } else if buf_len >= 16 {
        start_ptr.prefetch_read_data();
        {
            let cc = MMB16Sgl::new(c1);
            let loop_size = 16;
            let mut buf_ptr = buf_ptr_cur;
            let min_ptr = unsafe { start_ptr.add(loop_size) };
            if buf_ptr >= min_ptr {
                if buf_ptr.is_aligned_u128() {
                    while buf_ptr >= min_ptr {
                        buf_ptr = unsafe { buf_ptr.sub(loop_size) };
                        let r = unsafe { _rchr_c16_aa_x1(buf_ptr, cc, start_ptr) };
                        if r.is_some() {
                            return r;
                        }
                    }
                    buf_ptr_cur = buf_ptr;
                } else {
                    #[cfg(not(feature = "test_alignment_check"))]
                    {
                        while buf_ptr >= min_ptr {
                            buf_ptr = unsafe { buf_ptr.sub(loop_size) };
                            let r = unsafe { _rchr_c16_uu_x1(buf_ptr, cc, start_ptr) };
                            if r.is_some() {
                                return r;
                            }
                        }
                        buf_ptr_cur = buf_ptr;
                    }
                    #[cfg(feature = "test_alignment_check")]
                    {
                        let c = B1Sgl::new(c1);
                        let r = basic::_rchr_to_aligned_u128(buf_ptr_cur, c, start_ptr);
                        if let Some(p) = r.0 {
                            buf_ptr = p;
                        } else if let Some(v) = r.1 {
                            return Some(v);
                        }
                        while buf_ptr >= min_ptr {
                            buf_ptr = unsafe { buf_ptr.sub(loop_size) };
                            let r = unsafe { _rchr_c16_aa_x1(buf_ptr, cc, start_ptr) };
                            if r.is_some() {
                                return r;
                            }
                        }
                        buf_ptr_cur = buf_ptr;
                    }
                }
            }
        }
    }
    start_ptr.prefetch_read_data();
    //
    let cc = B8Sgl::new(c1);
    basic::_memrchr_remaining_15_bytes_impl(buf_ptr_cur, cc, start_ptr)
}

#[inline(always)]
unsafe fn _rchr_c16_uu_x1(buf_ptr: *const u8, mm_c16: MMB16Sgl, st_ptr: *const u8) -> Option<usize> {
    //
    let mm_0 = _mm_loadu_si128(buf_ptr as *const __m128i);
    let mm_0_eq = _mm_cmpeq_epi8(mm_0, mm_c16.a);
    let mask_0 = _mm_movemask_epi8(mm_0_eq);
    if mask_0 != 0 {
        Some(buf_ptr.usz_offset_from(st_ptr) + 16 - 1 - (mask_0 as u16).leading_zeros() as usize)
    } else {
        None
    }
}

#[inline(always)]
unsafe fn _rchr_c16_aa_x1(buf_ptr: *const u8, mm_c16: MMB16Sgl, st_ptr: *const u8) -> Option<usize> {
    //
    let mm_0 = _mm_load_si128(buf_ptr as *const __m128i);
    let mm_0_eq = _mm_cmpeq_epi8(mm_0, mm_c16.a);
    let mask_0 = _mm_movemask_epi8(mm_0_eq);
    if mask_0 != 0 {
        Some(buf_ptr.usz_offset_from(st_ptr) + 16 - 1 - (mask_0 as u16).leading_zeros() as usize)
    } else {
        None
    }
}

#[inline(always)]
unsafe fn _rchr_c16_aa_x2(buf_ptr: *const u8, mm_c16: MMB16Sgl, st_ptr: *const u8) -> Option<usize> {
    //
    let mm_0 = _mm_load_si128(buf_ptr as *const __m128i);
    let mm_1 = _mm_load_si128(buf_ptr.add(16) as *const __m128i);
    let mm_0_eq = _mm_cmpeq_epi8(mm_0, mm_c16.a);
    let mm_1_eq = _mm_cmpeq_epi8(mm_1, mm_c16.a);
    let mask_0 = _mm_movemask_epi8(mm_0_eq);
    let mask_1 = _mm_movemask_epi8(mm_1_eq);
    if mask_1 != 0 {
        Some(
            buf_ptr.usz_offset_from(st_ptr) + 16 - 1 - (mask_1 as u16).leading_zeros() as usize
                + 16,
        )
    } else if mask_0 != 0 {
        Some(buf_ptr.usz_offset_from(st_ptr) + 16 - 1 - (mask_0 as u16).leading_zeros() as usize)
    } else {
        None
    }
}

#[inline(always)]
unsafe fn _rchr_c16_aa_x4(buf_ptr: *const u8, mm_c16: MMB16Sgl, st_ptr: *const u8) -> Option<usize> {
    //
    let mm_0 = _mm_load_si128(buf_ptr as *const __m128i);
    let mm_1 = _mm_load_si128(buf_ptr.add(16) as *const __m128i);
    let mm_2 = _mm_load_si128(buf_ptr.add(16 * 2) as *const __m128i);
    let mm_3 = _mm_load_si128(buf_ptr.add(16 * 3) as *const __m128i);
    let mm_0_eq = _mm_cmpeq_epi8(mm_0, mm_c16.a);
    let mm_1_eq = _mm_cmpeq_epi8(mm_1, mm_c16.a);
    let mm_2_eq = _mm_cmpeq_epi8(mm_2, mm_c16.a);
    let mm_3_eq = _mm_cmpeq_epi8(mm_3, mm_c16.a);
    let mask_0 = _mm_movemask_epi8(mm_0_eq);
    let mask_1 = _mm_movemask_epi8(mm_1_eq);
    let mask_2 = _mm_movemask_epi8(mm_2_eq);
    let mask_3 = _mm_movemask_epi8(mm_3_eq);
    if mask_3 != 0 {
        Some(
            buf_ptr.usz_offset_from(st_ptr) + 16 - 1 - (mask_3 as u16).leading_zeros() as usize
                + 16 * 3,
        )
    } else if mask_2 != 0 {
        Some(
            buf_ptr.usz_offset_from(st_ptr) + 16 - 1 - (mask_2 as u16).leading_zeros() as usize
                + 16 * 2,
        )
    } else if mask_1 != 0 {
        Some(
            buf_ptr.usz_offset_from(st_ptr) + 16 - 1 - (mask_1 as u16).leading_zeros() as usize
                + 16,
        )
    } else if mask_0 != 0 {
        Some(buf_ptr.usz_offset_from(st_ptr) + 16 - 1 - (mask_0 as u16).leading_zeros() as usize)
    } else {
        None
    }
}

#[inline(always)]
unsafe fn _rchr_c16_aa_x8(buf_ptr: *const u8, mm_c16: MMB16Sgl, st_ptr: *const u8) -> Option<usize> {
    let r = _rchr_c16_aa_x4(buf_ptr.add(16 * 4), mm_c16, st_ptr);
    if r.is_some() {
        return r;
    }
    let r = _rchr_c16_aa_x4(buf_ptr, mm_c16, st_ptr);
    if r.is_some() {
        return r;
    }
    None
}

#[inline(always)]
unsafe fn _rchr_c32_uu_x1(buf_ptr: *const u8, mm_c32: MMB32Sgl, st_ptr: *const u8) -> Option<usize> {
    //
    let mm_0 = _mm256_loadu_si256(buf_ptr as *const __m256i);
    let mm_0_eq = _mm256_cmpeq_epi8(mm_0, mm_c32.a);
    let mask_0 = _mm256_movemask_epi8(mm_0_eq);
    if mask_0 != 0 {
        Some(buf_ptr.usz_offset_from(st_ptr) + 32 - 1 - (mask_0 as u32).leading_zeros() as usize)
    } else {
        None
    }
}

#[inline(always)]
unsafe fn _rchr_c32_aa_x1(buf_ptr: *const u8, mm_c32: MMB32Sgl, st_ptr: *const u8) -> Option<usize> {
    //
    let mm_0 = _mm256_load_si256(buf_ptr as *const __m256i);
    let mm_0_eq = _mm256_cmpeq_epi8(mm_0, mm_c32.a);
    let mask_0 = _mm256_movemask_epi8(mm_0_eq);
    if mask_0 != 0 {
        Some(buf_ptr.usz_offset_from(st_ptr) + 32 - 1 - (mask_0 as u32).leading_zeros() as usize)
    } else {
        None
    }
}

#[inline(always)]
unsafe fn _rchr_c32_aa_x2(buf_ptr: *const u8, mm_c32: MMB32Sgl, st_ptr: *const u8) -> Option<usize> {
    //
    let mm_0 = _mm256_load_si256(buf_ptr as *const __m256i);
    let mm_1 = _mm256_load_si256(buf_ptr.add(32) as *const __m256i);
    let mm_0_eq = _mm256_cmpeq_epi8(mm_0, mm_c32.a);
    let mm_1_eq = _mm256_cmpeq_epi8(mm_1, mm_c32.a);
    let mask_0 = _mm256_movemask_epi8(mm_0_eq);
    let mask_1 = _mm256_movemask_epi8(mm_1_eq);
    if mask_1 != 0 {
        Some(
            buf_ptr.usz_offset_from(st_ptr) + 32 - 1 - (mask_1 as u32).leading_zeros() as usize
                + 32,
        )
    } else if mask_0 != 0 {
        Some(buf_ptr.usz_offset_from(st_ptr) + 32 - 1 - (mask_0 as u32).leading_zeros() as usize)
    } else {
        None
    }
}

#[inline(always)]
unsafe fn _rchr_c32_aa_x4(buf_ptr: *const u8, mm_c32: MMB32Sgl, st_ptr: *const u8) -> Option<usize> {
    let r = _rchr_c32_aa_x2(buf_ptr.add(32 * 2), mm_c32, st_ptr);
    if r.is_some() {
        return r;
    }
    let r = _rchr_c32_aa_x2(buf_ptr, mm_c32, st_ptr);
    if r.is_some() {
        return r;
    }
    None
}

#[inline(always)]
unsafe fn _rchr_c32_aa_x8(buf_ptr: *const u8, mm_c32: MMB32Sgl, st_ptr: *const u8) -> Option<usize> {
    let r = _rchr_c32_aa_x4(buf_ptr.add(32 * 4), mm_c32, st_ptr);
    if r.is_some() {
        return r;
    }
    let r = _rchr_c32_aa_x4(buf_ptr, mm_c32, st_ptr);
    if r.is_some() {
        return r;
    }
    None
}
