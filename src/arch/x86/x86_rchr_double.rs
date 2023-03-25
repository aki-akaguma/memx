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

#[inline(always)]
#[cfg(target_arch = "x86_64")]
pub(crate) fn _memrchr_double_impl(buf: &[u8], c1: u8, c2: u8) -> Option<usize> {
    // TODO: Replace with https://github.com/rust-lang/rfcs/pull/2725
    // after stabilization
    if cpuid::has_avx2() {
        unsafe { _memrchr_double_avx2(buf, c1, c2) }
    } else {
        unsafe { _memrchr_double_sse2(buf, c1, c2) }
    }
}

#[inline(always)]
#[cfg(target_arch = "x86")]
pub(crate) fn _memrchr_double_impl(buf: &[u8], c1: u8, c2: u8) -> Option<usize> {
    // TODO: Replace with https://github.com/rust-lang/rfcs/pull/2725
    // after stabilization
    if cpuid::has_avx2() {
        unsafe { _memrchr_double_avx2(buf, c1, c2) }
    } else if cpuid::has_sse2() {
        unsafe { _memrchr_double_sse2(buf, c1, c2) }
    } else {
        _memrchr_double_basic(buf, c1, c2)
    }
}

fn _memrchr_double_basic(buf: &[u8], c1: u8, c2: u8) -> Option<usize> {
    basic::_memrchr_double_impl(buf, c1, c2)
}

#[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
#[target_feature(enable = "sse2")]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn _memrchr_double_sse2(buf: &[u8], c1: u8, c2: u8) -> Option<usize> {
    _memrchr_double_sse2_impl(buf, c1, c2)
}

#[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
#[target_feature(enable = "avx2")]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn _memrchr_double_avx2(buf: &[u8], c1: u8, c2: u8) -> Option<usize> {
    _memrchr_double_avx2_impl(buf, c1, c2)
}

macro_rules! _unroll_one_rchr_16_uu {
    ($a_ptr:expr, $cc1:expr, $cc2:expr, $start_ptr:expr, $loop_size:expr, $idx:expr) => {{
        let aa_ptr = unsafe { $a_ptr.add($loop_size * $idx) };
        let r = unsafe { _rchr_c16_dbl_uu(aa_ptr, $cc1, $cc2, $start_ptr) };
        if !r.is_none() {
            return r;
        }
    }};
}

macro_rules! _unroll_one_rchr_16_aa {
    ($a_ptr:expr, $cc1:expr, $cc2:expr, $start_ptr:expr, $loop_size:expr, $idx:expr) => {{
        let aa_ptr = unsafe { $a_ptr.add($loop_size * $idx) };
        let r = unsafe { _rchr_c16_dbl_aa(aa_ptr, $cc1, $cc2, $start_ptr) };
        if !r.is_none() {
            return r;
        }
    }};
}

macro_rules! _unroll_one_rchr_16_aa_x2 {
    ($a_ptr:expr, $cc1:expr, $cc2:expr, $start_ptr:expr, $loop_size:expr, $idx:expr) => {{
        let aa_ptr = unsafe { $a_ptr.add($loop_size * $idx) };
        let r = unsafe { _rchr_c16_dbl_aa_x2(aa_ptr, $cc1, $cc2, $start_ptr) };
        if !r.is_none() {
            return r;
        }
    }};
}

macro_rules! _unroll_one_rchr_16_aa_x4 {
    ($a_ptr:expr, $cc1:expr, $cc2:expr, $start_ptr:expr, $loop_size:expr, $idx:expr) => {{
        let aa_ptr = unsafe { $a_ptr.add($loop_size * $idx) };
        let r = unsafe { _rchr_c16_dbl_aa_x4(aa_ptr, $cc1, $cc2, $start_ptr) };
        if !r.is_none() {
            return r;
        }
    }};
}

macro_rules! _unroll_one_rchr_32_uu {
    ($a_ptr:expr, $cc1:expr, $cc2:expr, $start_ptr:expr, $loop_size:expr, $idx:expr) => {{
        let aa_ptr = unsafe { $a_ptr.add($loop_size * $idx) };
        let r = unsafe { _rchr_c32_dbl_uu(aa_ptr, $cc1, $cc2, $start_ptr) };
        if !r.is_none() {
            return r;
        }
    }};
}

macro_rules! _unroll_one_rchr_32_aa {
    ($a_ptr:expr, $cc1:expr, $cc2:expr, $start_ptr:expr, $loop_size:expr, $idx:expr) => {{
        let aa_ptr = unsafe { $a_ptr.add($loop_size * $idx) };
        let r = unsafe { _rchr_c32_dbl_aa(aa_ptr, $cc1, $cc2, $start_ptr) };
        if !r.is_none() {
            return r;
        }
    }};
}

macro_rules! _unroll_one_rchr_32_aa_x2 {
    ($a_ptr:expr, $cc1:expr, $cc2:expr, $start_ptr:expr, $loop_size:expr, $idx:expr) => {{
        let aa_ptr = unsafe { $a_ptr.add($loop_size * $idx) };
        let r = unsafe { _rchr_c32_dbl_aa_x2(aa_ptr, $cc1, $cc2, $start_ptr) };
        if !r.is_none() {
            return r;
        }
    }};
}

#[inline(always)]
fn _memrchr_double_sse2_impl(buf: &[u8], c1: u8, c2: u8) -> Option<usize> {
    let buf_len = buf.len();
    let start_ptr = buf.as_ptr();
    let mut buf_ptr_cur = unsafe { start_ptr.add(buf_len) };
    //
    if buf_len >= 16 {
        let cc1: __m128i = unsafe { _c16_value(c1) };
        let cc2: __m128i = unsafe { _c16_value(c2) };
        {
            let loop_size = 16;
            let buf_ptr = unsafe { buf_ptr_cur.sub(loop_size) };
            let remaining_align = 0x10_usize - ((buf_ptr as usize) & 0x0F_usize);
            _unroll_one_rchr_16_uu!(buf_ptr, cc1, cc2, start_ptr, loop_size, 0);
            //
            buf_ptr_cur = unsafe { buf_ptr.add(remaining_align) };
        }
        //
        {
            let unroll = 8;
            let loop_size = 16;
            let mut buf_ptr = buf_ptr_cur;
            if unsafe { buf_ptr.offset_from(start_ptr) } >= (loop_size * unroll) as isize {
                let min_ptr = unsafe { start_ptr.add(loop_size * unroll) };
                if buf_ptr >= min_ptr {
                    while buf_ptr >= min_ptr {
                        buf_ptr = unsafe { buf_ptr.sub(loop_size * unroll) };
                        _unroll_one_rchr_16_aa_x4!(buf_ptr, cc1, cc2, start_ptr, loop_size, 4);
                        _unroll_one_rchr_16_aa_x4!(buf_ptr, cc1, cc2, start_ptr, loop_size, 0);
                    }
                    buf_ptr_cur = buf_ptr;
                }
            }
        }
        {
            let unroll = 4;
            let loop_size = 16;
            let mut buf_ptr = buf_ptr_cur;
            if unsafe { buf_ptr.offset_from(start_ptr) } >= (loop_size * unroll) as isize {
                let min_ptr = unsafe { start_ptr.add(loop_size * unroll) };
                if buf_ptr >= min_ptr {
                    while buf_ptr >= min_ptr {
                        buf_ptr = unsafe { buf_ptr.sub(loop_size * unroll) };
                        _unroll_one_rchr_16_aa_x4!(buf_ptr, cc1, cc2, start_ptr, loop_size, 0);
                    }
                    buf_ptr_cur = buf_ptr;
                }
            }
        }
        {
            let unroll = 2;
            let loop_size = 16;
            let mut buf_ptr = buf_ptr_cur;
            if unsafe { buf_ptr.offset_from(start_ptr) } >= (loop_size * unroll) as isize {
                let min_ptr = unsafe { start_ptr.add(loop_size * unroll) };
                if buf_ptr >= min_ptr {
                    while buf_ptr >= min_ptr {
                        buf_ptr = unsafe { buf_ptr.sub(loop_size * unroll) };
                        _unroll_one_rchr_16_aa_x2!(buf_ptr, cc1, cc2, start_ptr, loop_size, 0);
                    }
                    buf_ptr_cur = buf_ptr;
                }
            }
        }
        {
            let loop_size = 16;
            let mut buf_ptr = buf_ptr_cur;
            let min_ptr = unsafe { start_ptr.add(loop_size) };
            if buf_ptr >= min_ptr {
                while buf_ptr >= min_ptr {
                    buf_ptr = unsafe { buf_ptr.sub(loop_size) };
                    _unroll_one_rchr_16_aa!(buf_ptr, cc1, cc2, start_ptr, loop_size, 0);
                }
                buf_ptr_cur = buf_ptr;
            }
        }
    }
    //
    let cc1 = (c1 as u64) * 0x0101_0101_0101_0101_u64;
    let cc2 = (c2 as u64) * 0x0101_0101_0101_0101_u64;
    basic::_memrchr_double_remaining_15_bytes_impl(buf_ptr_cur, cc1, cc2, start_ptr)
}

#[inline(always)]
fn _memrchr_double_avx2_impl(buf: &[u8], c1: u8, c2: u8) -> Option<usize> {
    let buf_len = buf.len();
    let start_ptr = buf.as_ptr();
    let mut buf_ptr_cur = unsafe { start_ptr.add(buf_len) };
    //
    if buf_len >= 32 {
        let cc1: __m256i = unsafe { _c32_value(c1) };
        let cc2: __m256i = unsafe { _c32_value(c2) };
        {
            let loop_size = 32;
            let buf_ptr = unsafe { buf_ptr_cur.sub(loop_size) };
            let remaining_align = 0x20_usize - ((buf_ptr as usize) & 0x1F_usize);
            _unroll_one_rchr_32_uu!(buf_ptr, cc1, cc2, start_ptr, loop_size, 0);
            //
            buf_ptr_cur = unsafe { buf_ptr.add(remaining_align) };
        }
        //
        {
            let unroll = 2;
            let loop_size = 32;
            let mut buf_ptr = buf_ptr_cur;
            if unsafe { buf_ptr.offset_from(start_ptr) } >= (loop_size * unroll) as isize {
                let min_ptr = unsafe { start_ptr.add(loop_size * unroll) };
                if buf_ptr >= min_ptr {
                    while buf_ptr >= min_ptr {
                        buf_ptr = unsafe { buf_ptr.sub(loop_size * unroll) };
                        _unroll_one_rchr_32_aa_x2!(buf_ptr, cc1, cc2, start_ptr, loop_size, 0);
                    }
                    buf_ptr_cur = buf_ptr;
                }
            }
        }
        {
            let loop_size = 32;
            let mut buf_ptr = buf_ptr_cur;
            let min_ptr = unsafe { start_ptr.add(loop_size) };
            if buf_ptr >= min_ptr {
                while buf_ptr >= min_ptr {
                    buf_ptr = unsafe { buf_ptr.sub(loop_size) };
                    _unroll_one_rchr_32_aa!(buf_ptr, cc1, cc2, start_ptr, loop_size, 0);
                }
                buf_ptr_cur = buf_ptr;
            }
        }
        {
            let cc1: __m128i = unsafe { _c16_value(c1) };
            let cc2: __m128i = unsafe { _c16_value(c2) };
            let loop_size = 16;
            let mut buf_ptr = buf_ptr_cur;
            let min_ptr = unsafe { start_ptr.add(loop_size) };
            if buf_ptr >= min_ptr {
                while buf_ptr >= min_ptr {
                    buf_ptr = unsafe { buf_ptr.sub(loop_size) };
                    _unroll_one_rchr_16_aa!(buf_ptr, cc1, cc2, start_ptr, loop_size, 0);
                }
                buf_ptr_cur = buf_ptr;
            }
        }
    } else if buf_len >= 16 {
        {
            let cc1: __m128i = unsafe { _c16_value(c1) };
            let cc2: __m128i = unsafe { _c16_value(c2) };
            let loop_size = 16;
            let mut buf_ptr = buf_ptr_cur;
            let min_ptr = unsafe { start_ptr.add(loop_size) };
            if buf_ptr >= min_ptr {
                while buf_ptr >= min_ptr {
                    buf_ptr = unsafe { buf_ptr.sub(loop_size) };
                    _unroll_one_rchr_16_uu!(buf_ptr, cc1, cc2, start_ptr, loop_size, 0);
                }
                buf_ptr_cur = buf_ptr;
            }
        }
    }
    //
    let cc1 = (c1 as u64) * 0x0101_0101_0101_0101_u64;
    let cc2 = (c2 as u64) * 0x0101_0101_0101_0101_u64;
    basic::_memrchr_double_remaining_15_bytes_impl(buf_ptr_cur, cc1, cc2, start_ptr)
}

#[inline(always)]
unsafe fn _c16_value(c: u8) -> __m128i {
    _mm_set1_epi8(c as i8)
}

#[inline(always)]
unsafe fn _rchr_c16_dbl_uu(
    buf_ptr: *const u8,
    mm_c16_1: __m128i,
    mm_c16_2: __m128i,
    start_ptr: *const u8,
) -> Option<usize> {
    //
    let mm_a = _mm_loadu_si128(buf_ptr as *const __m128i);
    let mm_a_eq_1 = _mm_cmpeq_epi8(mm_a, mm_c16_1);
    let mm_a_eq_2 = _mm_cmpeq_epi8(mm_a, mm_c16_2);
    let mask_a_1 = _mm_movemask_epi8(mm_a_eq_1);
    let mask_a_2 = _mm_movemask_epi8(mm_a_eq_2);
    let base = plus_offset_from(buf_ptr, start_ptr) + 16 - 1;
    //
    if mask_a_1 != 0 {
        if mask_a_2 != 0 {
            let idx1 = (mask_a_1 as u16).leading_zeros() as usize;
            let idx2 = (mask_a_2 as u16).leading_zeros() as usize;
            if idx1 < idx2 {
                Some(base - idx1)
            } else {
                Some(base - idx2)
            }
        } else {
            Some(base - (mask_a_1 as u16).leading_zeros() as usize)
        }
    } else if mask_a_2 != 0 {
        Some(base - (mask_a_2 as u16).leading_zeros() as usize)
    } else {
        None
    }
}

#[inline(always)]
unsafe fn _rchr_c16_dbl_aa(
    buf_ptr: *const u8,
    mm_c16_1: __m128i,
    mm_c16_2: __m128i,
    start_ptr: *const u8,
) -> Option<usize> {
    //
    let mm_a = _mm_load_si128(buf_ptr as *const __m128i);
    let mm_a_eq_1 = _mm_cmpeq_epi8(mm_a, mm_c16_1);
    let mm_a_eq_2 = _mm_cmpeq_epi8(mm_a, mm_c16_2);
    let mask_a_1 = _mm_movemask_epi8(mm_a_eq_1);
    let mask_a_2 = _mm_movemask_epi8(mm_a_eq_2);
    let base = plus_offset_from(buf_ptr, start_ptr) + 16 - 1;
    //
    if mask_a_1 != 0 {
        if mask_a_2 != 0 {
            let idx1 = (mask_a_1 as u16).leading_zeros() as usize;
            let idx2 = (mask_a_2 as u16).leading_zeros() as usize;
            if idx1 < idx2 {
                Some(base - idx1)
            } else {
                Some(base - idx2)
            }
        } else {
            Some(base - (mask_a_1 as u16).leading_zeros() as usize)
        }
    } else if mask_a_2 != 0 {
        Some(base - (mask_a_2 as u16).leading_zeros() as usize)
    } else {
        None
    }
}

#[inline(always)]
unsafe fn _rchr_c16_dbl_aa_x2(
    buf_ptr: *const u8,
    mm_c16_1: __m128i,
    mm_c16_2: __m128i,
    start_ptr: *const u8,
) -> Option<usize> {
    //
    let mm_a = _mm_load_si128(buf_ptr as *const __m128i);
    let mm_b = _mm_load_si128(buf_ptr.add(16) as *const __m128i);
    //
    let mm_a_eq_1 = _mm_cmpeq_epi8(mm_a, mm_c16_1);
    let mm_a_eq_2 = _mm_cmpeq_epi8(mm_a, mm_c16_2);
    let mask_a_1 = _mm_movemask_epi8(mm_a_eq_1);
    let mask_a_2 = _mm_movemask_epi8(mm_a_eq_2);
    //
    let mm_b_eq_1 = _mm_cmpeq_epi8(mm_b, mm_c16_1);
    let mm_b_eq_2 = _mm_cmpeq_epi8(mm_b, mm_c16_2);
    let mask_b_1 = _mm_movemask_epi8(mm_b_eq_1);
    let mask_b_2 = _mm_movemask_epi8(mm_b_eq_2);
    //
    let base = plus_offset_from(buf_ptr, start_ptr) + 16 - 1;
    //
    if mask_b_1 != 0 {
        if mask_b_2 != 0 {
            let idx1 = (mask_b_1 as u16).leading_zeros() as usize;
            let idx2 = (mask_b_2 as u16).leading_zeros() as usize;
            if idx1 < idx2 {
                Some(base - idx1 + 16)
            } else {
                Some(base - idx2 + 16)
            }
        } else {
            Some(base - (mask_b_1 as u16).leading_zeros() as usize + 16)
        }
    } else if mask_b_2 != 0 {
        Some(base - (mask_b_2 as u16).leading_zeros() as usize + 16)
    } else if mask_a_1 != 0 {
        if mask_a_2 != 0 {
            let idx1 = (mask_a_1 as u16).leading_zeros() as usize;
            let idx2 = (mask_a_2 as u16).leading_zeros() as usize;
            if idx1 < idx2 {
                Some(base - idx1)
            } else {
                Some(base - idx2)
            }
        } else {
            Some(base - (mask_a_1 as u16).leading_zeros() as usize)
        }
    } else if mask_a_2 != 0 {
        Some(base - (mask_a_2 as u16).leading_zeros() as usize)
    } else {
        None
    }
}

#[inline(always)]
unsafe fn _rchr_c16_dbl_aa_x4(
    buf_ptr: *const u8,
    mm_c16_1: __m128i,
    mm_c16_2: __m128i,
    start_ptr: *const u8,
) -> Option<usize> {
    //
    let mm_a = _mm_load_si128(buf_ptr as *const __m128i);
    let mm_b = _mm_load_si128(buf_ptr.add(16) as *const __m128i);
    let mm_c = _mm_load_si128(buf_ptr.add(16 * 2) as *const __m128i);
    let mm_d = _mm_load_si128(buf_ptr.add(16 * 3) as *const __m128i);
    //
    let mm_a_eq_1 = _mm_cmpeq_epi8(mm_a, mm_c16_1);
    let mm_a_eq_2 = _mm_cmpeq_epi8(mm_a, mm_c16_2);
    let mask_a_1 = _mm_movemask_epi8(mm_a_eq_1);
    let mask_a_2 = _mm_movemask_epi8(mm_a_eq_2);
    //
    let mm_b_eq_1 = _mm_cmpeq_epi8(mm_b, mm_c16_1);
    let mm_b_eq_2 = _mm_cmpeq_epi8(mm_b, mm_c16_2);
    let mask_b_1 = _mm_movemask_epi8(mm_b_eq_1);
    let mask_b_2 = _mm_movemask_epi8(mm_b_eq_2);
    //
    let mm_c_eq_1 = _mm_cmpeq_epi8(mm_c, mm_c16_1);
    let mm_c_eq_2 = _mm_cmpeq_epi8(mm_c, mm_c16_2);
    let mask_c_1 = _mm_movemask_epi8(mm_c_eq_1);
    let mask_c_2 = _mm_movemask_epi8(mm_c_eq_2);
    //
    let mm_d_eq_1 = _mm_cmpeq_epi8(mm_d, mm_c16_1);
    let mm_d_eq_2 = _mm_cmpeq_epi8(mm_d, mm_c16_2);
    let mask_d_1 = _mm_movemask_epi8(mm_d_eq_1);
    let mask_d_2 = _mm_movemask_epi8(mm_d_eq_2);
    //
    let base = plus_offset_from(buf_ptr, start_ptr) + 16 - 1;
    //
    if mask_d_1 != 0 {
        if mask_d_2 != 0 {
            let idx1 = (mask_d_1 as u16).leading_zeros() as usize;
            let idx2 = (mask_d_2 as u16).leading_zeros() as usize;
            if idx1 < idx2 {
                Some(base - idx1 + 16 * 3)
            } else {
                Some(base - idx2 + 16 * 3)
            }
        } else {
            Some(base - (mask_d_1 as u16).leading_zeros() as usize + 16 * 3)
        }
    } else if mask_d_2 != 0 {
        Some(base - (mask_d_2 as u16).leading_zeros() as usize + 16 * 3)
    } else if mask_c_1 != 0 {
        if mask_c_2 != 0 {
            let idx1 = (mask_c_1 as u16).leading_zeros() as usize;
            let idx2 = (mask_c_2 as u16).leading_zeros() as usize;
            if idx1 < idx2 {
                Some(base - idx1 + 16 * 2)
            } else {
                Some(base - idx2 + 16 * 2)
            }
        } else {
            Some(base - (mask_c_1 as u16).leading_zeros() as usize + 16 * 2)
        }
    } else if mask_c_2 != 0 {
        Some(base - (mask_c_2 as u16).leading_zeros() as usize + 16 * 2)
    } else if mask_b_1 != 0 {
        if mask_b_2 != 0 {
            let idx1 = (mask_b_1 as u16).leading_zeros() as usize;
            let idx2 = (mask_b_2 as u16).leading_zeros() as usize;
            if idx1 < idx2 {
                Some(base - idx1 + 16)
            } else {
                Some(base - idx2 + 16)
            }
        } else {
            Some(base - (mask_b_1 as u16).leading_zeros() as usize + 16)
        }
    } else if mask_b_2 != 0 {
        Some(base - (mask_b_2 as u16).leading_zeros() as usize + 16)
    } else if mask_a_1 != 0 {
        if mask_a_2 != 0 {
            let idx1 = (mask_a_1 as u16).leading_zeros() as usize;
            let idx2 = (mask_a_2 as u16).leading_zeros() as usize;
            if idx1 < idx2 {
                Some(base - idx1)
            } else {
                Some(base - idx2)
            }
        } else {
            Some(base - (mask_a_1 as u16).leading_zeros() as usize)
        }
    } else if mask_a_2 != 0 {
        Some(base - (mask_a_2 as u16).leading_zeros() as usize)
    } else {
        None
    }
}

#[inline(always)]
unsafe fn _c32_value(c: u8) -> __m256i {
    _mm256_set1_epi8(c as i8)
}

#[inline(always)]
unsafe fn _rchr_c32_dbl_uu(
    buf_ptr: *const u8,
    mm_c32_1: __m256i,
    mm_c32_2: __m256i,
    start_ptr: *const u8,
) -> Option<usize> {
    //
    let mm_a = _mm256_loadu_si256(buf_ptr as *const __m256i);
    let mm_a_eq_1 = _mm256_cmpeq_epi8(mm_a, mm_c32_1);
    let mm_a_eq_2 = _mm256_cmpeq_epi8(mm_a, mm_c32_2);
    let mask_a_1 = _mm256_movemask_epi8(mm_a_eq_1);
    let mask_a_2 = _mm256_movemask_epi8(mm_a_eq_2);
    let base = plus_offset_from(buf_ptr, start_ptr) + 32 - 1;
    //
    if mask_a_1 != 0 {
        if mask_a_2 != 0 {
            let idx1 = (mask_a_1 as u32).leading_zeros() as usize;
            let idx2 = (mask_a_2 as u32).leading_zeros() as usize;
            if idx1 < idx2 {
                Some(base - idx1)
            } else {
                Some(base - idx2)
            }
        } else {
            Some(base - (mask_a_1 as u32).leading_zeros() as usize)
        }
    } else if mask_a_2 != 0 {
        Some(base - (mask_a_2 as u32).leading_zeros() as usize)
    } else {
        None
    }
}

#[inline(always)]
unsafe fn _rchr_c32_dbl_aa(
    buf_ptr: *const u8,
    mm_c32_1: __m256i,
    mm_c32_2: __m256i,
    start_ptr: *const u8,
) -> Option<usize> {
    //
    let mm_a = _mm256_load_si256(buf_ptr as *const __m256i);
    let mm_a_eq_1 = _mm256_cmpeq_epi8(mm_a, mm_c32_1);
    let mm_a_eq_2 = _mm256_cmpeq_epi8(mm_a, mm_c32_2);
    let mask_a_1 = _mm256_movemask_epi8(mm_a_eq_1);
    let mask_a_2 = _mm256_movemask_epi8(mm_a_eq_2);
    let base = plus_offset_from(buf_ptr, start_ptr) + 32 - 1;
    //
    if mask_a_1 != 0 {
        if mask_a_2 != 0 {
            let idx1 = (mask_a_1 as u32).leading_zeros() as usize;
            let idx2 = (mask_a_2 as u32).leading_zeros() as usize;
            if idx1 < idx2 {
                Some(base - idx1)
            } else {
                Some(base - idx2)
            }
        } else {
            Some(base - (mask_a_1 as u32).leading_zeros() as usize)
        }
    } else if mask_a_2 != 0 {
        Some(base - (mask_a_2 as u32).leading_zeros() as usize)
    } else {
        None
    }
}

#[inline(always)]
unsafe fn _rchr_c32_dbl_aa_x2(
    buf_ptr: *const u8,
    mm_c32_1: __m256i,
    mm_c32_2: __m256i,
    start_ptr: *const u8,
) -> Option<usize> {
    //
    let mm_a = _mm256_load_si256(buf_ptr as *const __m256i);
    let mm_b = _mm256_load_si256(buf_ptr.add(32) as *const __m256i);
    //
    let mm_a_eq_1 = _mm256_cmpeq_epi8(mm_a, mm_c32_1);
    let mm_a_eq_2 = _mm256_cmpeq_epi8(mm_a, mm_c32_2);
    let mask_a_1 = _mm256_movemask_epi8(mm_a_eq_1);
    let mask_a_2 = _mm256_movemask_epi8(mm_a_eq_2);
    //
    let mm_b_eq_1 = _mm256_cmpeq_epi8(mm_b, mm_c32_1);
    let mm_b_eq_2 = _mm256_cmpeq_epi8(mm_b, mm_c32_2);
    let mask_b_1 = _mm256_movemask_epi8(mm_b_eq_1);
    let mask_b_2 = _mm256_movemask_epi8(mm_b_eq_2);
    //
    let base = plus_offset_from(buf_ptr, start_ptr) + 32 - 1;
    //
    if mask_b_1 != 0 {
        if mask_b_2 != 0 {
            let idx1 = (mask_b_1 as u32).leading_zeros() as usize;
            let idx2 = (mask_b_2 as u32).leading_zeros() as usize;
            if idx1 < idx2 {
                Some(base - idx1 + 32)
            } else {
                Some(base - idx2 + 32)
            }
        } else {
            Some(base - (mask_b_1 as u32).leading_zeros() as usize + 32)
        }
    } else if mask_b_2 != 0 {
        Some(base - (mask_b_2 as u32).leading_zeros() as usize + 32)
    } else if mask_a_1 != 0 {
        if mask_a_2 != 0 {
            let idx1 = (mask_a_1 as u32).leading_zeros() as usize;
            let idx2 = (mask_a_2 as u32).leading_zeros() as usize;
            if idx1 < idx2 {
                Some(base - idx1)
            } else {
                Some(base - idx2)
            }
        } else {
            Some(base - (mask_a_1 as u32).leading_zeros() as usize)
        }
    } else if mask_a_2 != 0 {
        Some(base - (mask_a_2 as u32).leading_zeros() as usize)
    } else {
        None
    }
}
