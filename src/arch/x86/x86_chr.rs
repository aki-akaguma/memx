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
use mmx::_mm_set1_epi8;

use mmx::__m256i;
use mmx::_mm256_cmpeq_epi8;
use mmx::_mm256_load_si256;
use mmx::_mm256_loadu_si256;
use mmx::_mm256_movemask_epi8;
use mmx::_mm256_set1_epi8;

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
        _memchr_avx2
    } else {
        _memchr_sse2
    };
    #[cfg(target_arch = "x86")]
    let func = if cpuid::has_avx2() {
        _memchr_avx2
    } else if cpuid::has_sse2() {
        _memchr_sse2
    } else {
        _memchr_basic
    };
    //
    FUNC_PTR_ATOM.store(func as *mut FuncType, Ordering::Relaxed);
    unsafe { func(buf, c) }
}

#[inline(always)]
pub(crate) fn _memchr_impl(buf: &[u8], c: u8) -> Option<usize> {
    let func_u = FUNC_PTR_ATOM.load(Ordering::Relaxed);
    #[allow(clippy::crosspointer_transmute)]
    let func: FuncType = unsafe { core::mem::transmute(func_u) };
    func(buf, c)
}

unsafe fn _memchr_basic(buf: &[u8], c: u8) -> Option<usize> {
    basic::_memchr_impl(buf, c)
}

#[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
#[target_feature(enable = "sse2")]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn _memchr_sse2(buf: &[u8], c: u8) -> Option<usize> {
    _memchr_sse2_impl(buf, c)
}

#[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
#[target_feature(enable = "avx2")]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn _memchr_avx2(buf: &[u8], c: u8) -> Option<usize> {
    _memchr_avx2_impl(buf, c)
}

#[inline(always)]
fn _memchr_sse2_impl(buf: &[u8], c: u8) -> Option<usize> {
    let buf_len = buf.len();
    let mut buf_ptr = buf.as_ptr();
    let start_ptr = buf_ptr;
    let end_ptr = unsafe { buf_ptr.add(buf_len) };
    buf_ptr.prefetch_read_data();
    //
    if buf_len >= 16 {
        let cc: __m128i = unsafe { _c16_value(c) };
        // to a aligned pointer
        {
            let remaining_align = 0x10_usize - ((buf_ptr as usize) & 0x0F_usize);
            if buf_ptr.is_aligned_u128() {
                let r = unsafe { _chr_c16_aa_x1(buf_ptr, cc, start_ptr) };
                if r.is_some() {
                    return r;
                }
                buf_ptr = unsafe { buf_ptr.add(remaining_align) };
            } else {
                #[cfg(not(feature = "test_alignment_check"))]
                {
                    let r = unsafe { _chr_c16_uu_x1(buf_ptr, cc, start_ptr) };
                    if r.is_some() {
                        return r;
                    }
                    buf_ptr = unsafe { buf_ptr.add(remaining_align) };
                }
                #[cfg(feature = "test_alignment_check")]
                {
                    let r = basic::_chr_to_aligned_u128(buf_ptr, c, start_ptr);
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
                    let r = unsafe { _chr_c16_aa_x8(buf_ptr, cc, start_ptr) };
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
                    let r = unsafe { _chr_c16_aa_x4(buf_ptr, cc, start_ptr) };
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
                    let r = unsafe { _chr_c16_aa_x2(buf_ptr, cc, start_ptr) };
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
                    let r = unsafe { _chr_c16_aa_x1(buf_ptr, cc, start_ptr) };
                    if r.is_some() {
                        return r;
                    }
                    buf_ptr = unsafe { buf_ptr.add(loop_size * unroll) };
                }
            }
        }
    }
    //
    let cc: u64 = _c8_value(c);
    basic::_memchr_remaining_15_bytes_impl(buf_ptr, cc, start_ptr, end_ptr)
}

#[allow(clippy::missing_safety_doc)]
#[inline(always)]
pub fn _memchr_avx2_impl(buf: &[u8], c: u8) -> Option<usize> {
    let buf_len = buf.len();
    let mut buf_ptr = buf.as_ptr();
    let start_ptr = buf_ptr;
    let end_ptr = unsafe { buf_ptr.add(buf_len) };
    buf_ptr.prefetch_read_data();
    //
    if buf_len >= 32 {
        let cc: __m256i = unsafe { _c32_value(c) };
        // to a aligned pointer
        {
            let remaining_align = 0x20_usize - ((buf_ptr as usize) & 0x1F_usize);
            if buf_ptr.is_aligned_u256() {
                let r = unsafe { _chr_c32_aa_x1(buf_ptr, cc, start_ptr) };
                if r.is_some() {
                    return r;
                }
                buf_ptr = unsafe { buf_ptr.add(remaining_align) };
            } else {
                #[cfg(not(feature = "test_alignment_check"))]
                {
                    let r = unsafe { _chr_c32_uu_x1(buf_ptr, cc, start_ptr) };
                    if r.is_some() {
                        return r;
                    }
                    buf_ptr = unsafe { buf_ptr.add(remaining_align) };
                }
                #[cfg(feature = "test_alignment_check")]
                {
                    let r = basic::_chr_to_aligned_u256(buf_ptr, c, start_ptr);
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
                    let r = unsafe { _chr_c32_aa_x8(buf_ptr, cc, start_ptr) };
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
                    let r = unsafe { _chr_c32_aa_x4(buf_ptr, cc, start_ptr) };
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
                    let r = unsafe { _chr_c32_aa_x2(buf_ptr, cc, start_ptr) };
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
                let end_ptr_32_x1 = unsafe { end_ptr.sub(loop_size * unroll) };
                while buf_ptr <= end_ptr_32_x1 {
                    let r = unsafe { _chr_c32_aa_x1(buf_ptr, cc, start_ptr) };
                    if r.is_some() {
                        return r;
                    }
                    buf_ptr = unsafe { buf_ptr.add(loop_size * unroll) };
                }
            }
        }
        {
            let cc: __m128i = unsafe { _c16_value(c) };
            let unroll = 1;
            let loop_size = 16;
            if unsafe { end_ptr.offset_from(buf_ptr) } >= (loop_size * unroll) as isize {
                let end_ptr_16_x1 = unsafe { end_ptr.sub(loop_size * unroll) };
                while buf_ptr <= end_ptr_16_x1 {
                    let r = unsafe { _chr_c16_aa_x1(buf_ptr, cc, start_ptr) };
                    if r.is_some() {
                        return r;
                    }
                    buf_ptr = unsafe { buf_ptr.add(loop_size * unroll) };
                }
            }
        }
    } else if buf_len >= 16 {
        {
            let cc: __m128i = unsafe { _c16_value(c) };
            let unroll = 1;
            let loop_size = 16;
            if unsafe { end_ptr.offset_from(buf_ptr) } >= (loop_size * unroll) as isize {
                let end_ptr_16_x1 = unsafe { end_ptr.sub(loop_size * unroll) };
                //
                if buf_ptr.is_aligned_u128() {
                    while buf_ptr <= end_ptr_16_x1 {
                        let r = unsafe { _chr_c16_aa_x1(buf_ptr, cc, start_ptr) };
                        if r.is_some() {
                            return r;
                        }
                        buf_ptr = unsafe { buf_ptr.add(loop_size * unroll) };
                    }
                } else {
                    #[cfg(not(feature = "test_alignment_check"))]
                    {
                        while buf_ptr <= end_ptr_16_x1 {
                            let r = unsafe { _chr_c16_uu_x1(buf_ptr, cc, start_ptr) };
                            if r.is_some() {
                                return r;
                            }
                            buf_ptr = unsafe { buf_ptr.add(loop_size * unroll) };
                        }
                    }
                    #[cfg(feature = "test_alignment_check")]
                    {
                        let r = basic::_chr_to_aligned_u128(buf_ptr, c, start_ptr);
                        if let Some(p) = r.0 {
                            buf_ptr = p;
                        } else if let Some(v) = r.1 {
                            return Some(v);
                        }
                        while buf_ptr <= end_ptr_16_x1 {
                            let r = unsafe { _chr_c16_aa_x1(buf_ptr, cc, start_ptr) };
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
    let cc: u64 = _c8_value(c);
    basic::_memchr_remaining_15_bytes_impl(buf_ptr, cc, start_ptr, end_ptr)
}

#[inline(always)]
unsafe fn _c16_value(c: u8) -> __m128i {
    _mm_set1_epi8(c as i8)
}

#[inline(always)]
unsafe fn _chr_c16_uu_x1(buf_ptr: *const u8, mm_c16: __m128i, st_ptr: *const u8) -> Option<usize> {
    //
    let mm_a = _mm_loadu_si128(buf_ptr as *const __m128i);
    let mm_eq = _mm_cmpeq_epi8(mm_a, mm_c16);
    let mask = _mm_movemask_epi8(mm_eq);
    if mask != 0 {
        Some(plus_offset_from(buf_ptr, st_ptr) + mask.trailing_zeros() as usize)
    } else {
        None
    }
}

#[inline(always)]
unsafe fn _chr_c16_aa_x1(buf_ptr: *const u8, mm_c16: __m128i, st_ptr: *const u8) -> Option<usize> {
    //
    let mm_0 = _mm_load_si128(buf_ptr as *const __m128i);
    let mm_0_eq = _mm_cmpeq_epi8(mm_0, mm_c16);
    let mask_0 = _mm_movemask_epi8(mm_0_eq) as u64;
    if mask_0 != 0 {
        Some(plus_offset_from(buf_ptr, st_ptr) + mask_0.trailing_zeros() as usize)
    } else {
        None
    }
}

#[inline(always)]
unsafe fn _chr_c16_aa_x2(buf_ptr: *const u8, mm_c16: __m128i, st_ptr: *const u8) -> Option<usize> {
    //
    let mm_0 = _mm_load_si128(buf_ptr as *const __m128i);
    let mm_1 = _mm_load_si128(buf_ptr.add(16) as *const __m128i);
    let mm_0_eq = _mm_cmpeq_epi8(mm_0, mm_c16);
    let mm_1_eq = _mm_cmpeq_epi8(mm_1, mm_c16);
    let mask_0 = _mm_movemask_epi8(mm_0_eq) as u64;
    let mask_1 = _mm_movemask_epi8(mm_1_eq) as u64;
    if mask_0 != 0 {
        Some(plus_offset_from(buf_ptr, st_ptr) + mask_0.trailing_zeros() as usize)
    } else if mask_1 != 0 {
        Some(plus_offset_from(buf_ptr, st_ptr) + mask_1.trailing_zeros() as usize + 16)
    } else {
        None
    }
}

#[inline(always)]
unsafe fn _chr_c16_aa_x4(buf_ptr: *const u8, mm_c16: __m128i, st_ptr: *const u8) -> Option<usize> {
    //
    let mm_0 = _mm_load_si128(buf_ptr as *const __m128i);
    let mm_1 = _mm_load_si128(buf_ptr.add(16) as *const __m128i);
    let mm_2 = _mm_load_si128(buf_ptr.add(16 * 2) as *const __m128i);
    let mm_3 = _mm_load_si128(buf_ptr.add(16 * 3) as *const __m128i);
    let mm_0_eq = _mm_cmpeq_epi8(mm_0, mm_c16);
    let mm_1_eq = _mm_cmpeq_epi8(mm_1, mm_c16);
    let mm_2_eq = _mm_cmpeq_epi8(mm_2, mm_c16);
    let mm_3_eq = _mm_cmpeq_epi8(mm_3, mm_c16);
    let mask_0 = _mm_movemask_epi8(mm_0_eq) as u64;
    let mask_1 = _mm_movemask_epi8(mm_1_eq) as u64;
    let mask_2 = _mm_movemask_epi8(mm_2_eq) as u64;
    let mask_3 = _mm_movemask_epi8(mm_3_eq) as u64;
    if mask_0 != 0 {
        Some(plus_offset_from(buf_ptr, st_ptr) + mask_0.trailing_zeros() as usize)
    } else if mask_1 != 0 {
        Some(plus_offset_from(buf_ptr, st_ptr) + mask_1.trailing_zeros() as usize + 16)
    } else if mask_2 != 0 {
        Some(plus_offset_from(buf_ptr, st_ptr) + mask_2.trailing_zeros() as usize + 16 * 2)
    } else if mask_3 != 0 {
        Some(plus_offset_from(buf_ptr, st_ptr) + mask_3.trailing_zeros() as usize + 16 * 3)
    } else {
        None
    }
}

#[inline(always)]
unsafe fn _chr_c16_aa_x8(buf_ptr: *const u8, mm_c16: __m128i, st_ptr: *const u8) -> Option<usize> {
    //
    let mm_0 = _mm_load_si128(buf_ptr as *const __m128i);
    let mm_1 = _mm_load_si128(buf_ptr.add(16) as *const __m128i);
    let mm_2 = _mm_load_si128(buf_ptr.add(16 * 2) as *const __m128i);
    let mm_3 = _mm_load_si128(buf_ptr.add(16 * 3) as *const __m128i);
    let mm_4 = _mm_load_si128(buf_ptr.add(16 * 4) as *const __m128i);
    let mm_5 = _mm_load_si128(buf_ptr.add(16 * 5) as *const __m128i);
    let mm_6 = _mm_load_si128(buf_ptr.add(16 * 6) as *const __m128i);
    let mm_7 = _mm_load_si128(buf_ptr.add(16 * 7) as *const __m128i);
    let mm_0_eq = _mm_cmpeq_epi8(mm_0, mm_c16);
    let mm_1_eq = _mm_cmpeq_epi8(mm_1, mm_c16);
    let mm_2_eq = _mm_cmpeq_epi8(mm_2, mm_c16);
    let mm_3_eq = _mm_cmpeq_epi8(mm_3, mm_c16);
    let mm_4_eq = _mm_cmpeq_epi8(mm_4, mm_c16);
    let mm_5_eq = _mm_cmpeq_epi8(mm_5, mm_c16);
    let mm_6_eq = _mm_cmpeq_epi8(mm_6, mm_c16);
    let mm_7_eq = _mm_cmpeq_epi8(mm_7, mm_c16);
    let mask_0 = _mm_movemask_epi8(mm_0_eq) as u64;
    let mask_1 = _mm_movemask_epi8(mm_1_eq) as u64;
    let mask_2 = _mm_movemask_epi8(mm_2_eq) as u64;
    let mask_3 = _mm_movemask_epi8(mm_3_eq) as u64;
    let mask_4 = _mm_movemask_epi8(mm_4_eq) as u64;
    let mask_5 = _mm_movemask_epi8(mm_5_eq) as u64;
    let mask_6 = _mm_movemask_epi8(mm_6_eq) as u64;
    let mask_7 = _mm_movemask_epi8(mm_7_eq) as u64;
    if mask_0 != 0 {
        Some(plus_offset_from(buf_ptr, st_ptr) + mask_0.trailing_zeros() as usize)
    } else if mask_1 != 0 {
        Some(plus_offset_from(buf_ptr, st_ptr) + mask_1.trailing_zeros() as usize + 16)
    } else if mask_2 != 0 {
        Some(plus_offset_from(buf_ptr, st_ptr) + mask_2.trailing_zeros() as usize + 16 * 2)
    } else if mask_3 != 0 {
        Some(plus_offset_from(buf_ptr, st_ptr) + mask_3.trailing_zeros() as usize + 16 * 3)
    } else if mask_4 != 0 {
        Some(plus_offset_from(buf_ptr, st_ptr) + mask_4.trailing_zeros() as usize + 16 * 4)
    } else if mask_5 != 0 {
        Some(plus_offset_from(buf_ptr, st_ptr) + mask_5.trailing_zeros() as usize + 16 * 5)
    } else if mask_6 != 0 {
        Some(plus_offset_from(buf_ptr, st_ptr) + mask_6.trailing_zeros() as usize + 16 * 6)
    } else if mask_7 != 0 {
        Some(plus_offset_from(buf_ptr, st_ptr) + mask_7.trailing_zeros() as usize + 16 * 7)
    } else {
        None
    }
}

#[inline(always)]
unsafe fn _c32_value(c: u8) -> __m256i {
    _mm256_set1_epi8(c as i8)
}

#[inline(always)]
unsafe fn _chr_c32_uu_x1(buf_ptr: *const u8, mm_c32: __m256i, st_ptr: *const u8) -> Option<usize> {
    //
    let mm_0 = _mm256_loadu_si256(buf_ptr as *const __m256i);
    let mm_0_eq = _mm256_cmpeq_epi8(mm_0, mm_c32);
    let mask_0 = _mm256_movemask_epi8(mm_0_eq);
    if mask_0 != 0 {
        Some(plus_offset_from(buf_ptr, st_ptr) + mask_0.trailing_zeros() as usize)
    } else {
        None
    }
}

#[inline(always)]
unsafe fn _chr_c32_aa_x1(buf_ptr: *const u8, mm_c32: __m256i, st_ptr: *const u8) -> Option<usize> {
    //
    let mm_0 = _mm256_load_si256(buf_ptr as *const __m256i);
    let mm_0_eq = _mm256_cmpeq_epi8(mm_0, mm_c32);
    let mask_0 = _mm256_movemask_epi8(mm_0_eq);
    if mask_0 != 0 {
        Some(plus_offset_from(buf_ptr, st_ptr) + mask_0.trailing_zeros() as usize)
    } else {
        None
    }
}

#[inline(always)]
unsafe fn _chr_c32_aa_x2(buf_ptr: *const u8, mm_c32: __m256i, st_ptr: *const u8) -> Option<usize> {
    //
    let mm_0 = _mm256_load_si256(buf_ptr as *const __m256i);
    let mm_1 = _mm256_load_si256(buf_ptr.add(32) as *const __m256i);
    let mm_0_eq = _mm256_cmpeq_epi8(mm_0, mm_c32);
    let mm_1_eq = _mm256_cmpeq_epi8(mm_1, mm_c32);
    let mask_0 = _mm256_movemask_epi8(mm_0_eq) as u64;
    let mask_1 = _mm256_movemask_epi8(mm_1_eq) as u64;
    if mask_0 != 0 {
        Some(plus_offset_from(buf_ptr, st_ptr) + mask_0.trailing_zeros() as usize)
    } else if mask_1 != 0 {
        Some(plus_offset_from(buf_ptr, st_ptr) + mask_1.trailing_zeros() as usize + 32)
    } else {
        None
    }
}

#[inline(always)]
unsafe fn _chr_c32_aa_x4(buf_ptr: *const u8, mm_c32: __m256i, st_ptr: *const u8) -> Option<usize> {
    //
    let mm_0 = _mm256_load_si256(buf_ptr as *const __m256i);
    let mm_1 = _mm256_load_si256(buf_ptr.add(32) as *const __m256i);
    let mm_2 = _mm256_load_si256(buf_ptr.add(32 * 2) as *const __m256i);
    let mm_3 = _mm256_load_si256(buf_ptr.add(32 * 3) as *const __m256i);
    let mm_0_eq = _mm256_cmpeq_epi8(mm_0, mm_c32);
    let mm_1_eq = _mm256_cmpeq_epi8(mm_1, mm_c32);
    let mm_2_eq = _mm256_cmpeq_epi8(mm_2, mm_c32);
    let mm_3_eq = _mm256_cmpeq_epi8(mm_3, mm_c32);
    let mask_0 = _mm256_movemask_epi8(mm_0_eq) as u64;
    let mask_1 = _mm256_movemask_epi8(mm_1_eq) as u64;
    let mask_2 = _mm256_movemask_epi8(mm_2_eq) as u64;
    let mask_3 = _mm256_movemask_epi8(mm_3_eq) as u64;
    if mask_0 != 0 {
        Some(plus_offset_from(buf_ptr, st_ptr) + mask_0.trailing_zeros() as usize)
    } else if mask_1 != 0 {
        Some(plus_offset_from(buf_ptr, st_ptr) + mask_1.trailing_zeros() as usize + 32)
    } else if mask_2 != 0 {
        Some(plus_offset_from(buf_ptr, st_ptr) + mask_2.trailing_zeros() as usize + 32 * 2)
    } else if mask_3 != 0 {
        Some(plus_offset_from(buf_ptr, st_ptr) + mask_3.trailing_zeros() as usize + 32 * 3)
    } else {
        None
    }
}

#[inline(always)]
unsafe fn _chr_c32_aa_x8(buf_ptr: *const u8, mm_c32: __m256i, st_ptr: *const u8) -> Option<usize> {
    //
    let mm_0 = _mm256_load_si256(buf_ptr as *const __m256i);
    let mm_1 = _mm256_load_si256(buf_ptr.add(32) as *const __m256i);
    let mm_2 = _mm256_load_si256(buf_ptr.add(32 * 2) as *const __m256i);
    let mm_3 = _mm256_load_si256(buf_ptr.add(32 * 3) as *const __m256i);
    let mm_4 = _mm256_load_si256(buf_ptr.add(32 * 4) as *const __m256i);
    let mm_5 = _mm256_load_si256(buf_ptr.add(32 * 5) as *const __m256i);
    let mm_6 = _mm256_load_si256(buf_ptr.add(32 * 6) as *const __m256i);
    let mm_7 = _mm256_load_si256(buf_ptr.add(32 * 7) as *const __m256i);
    let mm_0_eq = _mm256_cmpeq_epi8(mm_0, mm_c32);
    let mm_1_eq = _mm256_cmpeq_epi8(mm_1, mm_c32);
    let mm_2_eq = _mm256_cmpeq_epi8(mm_2, mm_c32);
    let mm_3_eq = _mm256_cmpeq_epi8(mm_3, mm_c32);
    let mm_4_eq = _mm256_cmpeq_epi8(mm_4, mm_c32);
    let mm_5_eq = _mm256_cmpeq_epi8(mm_5, mm_c32);
    let mm_6_eq = _mm256_cmpeq_epi8(mm_6, mm_c32);
    let mm_7_eq = _mm256_cmpeq_epi8(mm_7, mm_c32);
    let mask_0 = _mm256_movemask_epi8(mm_0_eq) as u64;
    let mask_1 = _mm256_movemask_epi8(mm_1_eq) as u64;
    let mask_2 = _mm256_movemask_epi8(mm_2_eq) as u64;
    let mask_3 = _mm256_movemask_epi8(mm_3_eq) as u64;
    let mask_4 = _mm256_movemask_epi8(mm_4_eq) as u64;
    let mask_5 = _mm256_movemask_epi8(mm_5_eq) as u64;
    let mask_6 = _mm256_movemask_epi8(mm_6_eq) as u64;
    let mask_7 = _mm256_movemask_epi8(mm_7_eq) as u64;
    if mask_0 != 0 {
        Some(plus_offset_from(buf_ptr, st_ptr) + mask_0.trailing_zeros() as usize)
    } else if mask_1 != 0 {
        Some(plus_offset_from(buf_ptr, st_ptr) + mask_1.trailing_zeros() as usize + 32)
    } else if mask_2 != 0 {
        Some(plus_offset_from(buf_ptr, st_ptr) + mask_2.trailing_zeros() as usize + 32 * 2)
    } else if mask_3 != 0 {
        Some(plus_offset_from(buf_ptr, st_ptr) + mask_3.trailing_zeros() as usize + 32 * 3)
    } else if mask_4 != 0 {
        Some(plus_offset_from(buf_ptr, st_ptr) + mask_4.trailing_zeros() as usize + 32 * 4)
    } else if mask_5 != 0 {
        Some(plus_offset_from(buf_ptr, st_ptr) + mask_5.trailing_zeros() as usize + 32 * 5)
    } else if mask_6 != 0 {
        Some(plus_offset_from(buf_ptr, st_ptr) + mask_6.trailing_zeros() as usize + 32 * 6)
    } else if mask_7 != 0 {
        Some(plus_offset_from(buf_ptr, st_ptr) + mask_7.trailing_zeros() as usize + 32 * 7)
    } else {
        None
    }
}
