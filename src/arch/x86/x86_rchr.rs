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
pub(crate) fn _memrchr_impl(buf: &[u8], c: u8) -> Option<usize> {
    // TODO: Replace with https://github.com/rust-lang/rfcs/pull/2725
    // after stabilization
    if cpuid::has_avx2() {
        unsafe { _memrchr_avx2(buf, c) }
    } else {
        unsafe { _memrchr_sse2(buf, c) }
    }
}

#[inline(always)]
#[cfg(target_arch = "x86")]
pub(crate) fn _memrchr_impl(buf: &[u8], c: u8) -> Option<usize> {
    // TODO: Replace with https://github.com/rust-lang/rfcs/pull/2725
    // after stabilization
    if cpuid::has_avx2() {
        unsafe { _memrchr_avx2(buf, c) }
    } else if cpuid::has_sse2() {
        unsafe { _memrchr_sse2(buf, c) }
    } else {
        _memrchr_basic(buf, c)
    }
}

fn _memrchr_basic(buf: &[u8], c: u8) -> Option<usize> {
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

macro_rules! _unroll_one_rchr_16_uu {
    ($a_ptr:expr, $cc:expr, $start_ptr:expr, $loop_size:expr, $idx:expr) => {{
        let aa_ptr = unsafe { $a_ptr.add($loop_size * $idx) };
        let r = unsafe { _rchr_c16_uu(aa_ptr, $cc, $start_ptr) };
        if !r.is_none() {
            return r;
        }
    }};
}

macro_rules! _unroll_one_rchr_16_aa {
    ($a_ptr:expr, $cc:expr, $start_ptr:expr, $loop_size:expr, $idx:expr) => {{
        let aa_ptr = unsafe { $a_ptr.add($loop_size * $idx) };
        let r = unsafe { _rchr_c16_aa(aa_ptr, $cc, $start_ptr) };
        if !r.is_none() {
            return r;
        }
    }};
}

macro_rules! _unroll_one_rchr_16_aa_x2 {
    ($a_ptr:expr, $cc:expr, $start_ptr:expr, $loop_size:expr, $idx:expr) => {{
        let aa_ptr = unsafe { $a_ptr.add($loop_size * $idx) };
        let r = unsafe { _rchr_c16_aa_x2(aa_ptr, $cc, $start_ptr) };
        if !r.is_none() {
            return r;
        }
    }};
}

macro_rules! _unroll_one_rchr_16_aa_x4 {
    ($a_ptr:expr, $cc:expr, $start_ptr:expr, $loop_size:expr, $idx:expr) => {{
        let aa_ptr = unsafe { $a_ptr.add($loop_size * $idx) };
        let r = unsafe { _rchr_c16_aa_x4(aa_ptr, $cc, $start_ptr) };
        if !r.is_none() {
            return r;
        }
    }};
}

macro_rules! _unroll_one_rchr_32_uu {
    ($a_ptr:expr, $cc:expr, $start_ptr:expr, $loop_size:expr, $idx:expr) => {{
        let aa_ptr = unsafe { $a_ptr.add($loop_size * $idx) };
        let r = unsafe { _rchr_c32_uu(aa_ptr, $cc, $start_ptr) };
        if !r.is_none() {
            return r;
        }
    }};
}

macro_rules! _unroll_one_rchr_32_aa {
    ($a_ptr:expr, $cc:expr, $start_ptr:expr, $loop_size:expr, $idx:expr) => {{
        let aa_ptr = unsafe { $a_ptr.add($loop_size * $idx) };
        let r = unsafe { _rchr_c32_aa(aa_ptr, $cc, $start_ptr) };
        if !r.is_none() {
            return r;
        }
    }};
}

macro_rules! _unroll_one_rchr_32_aa_x2 {
    ($a_ptr:expr, $cc:expr, $start_ptr:expr, $loop_size:expr, $idx:expr) => {{
        let aa_ptr = unsafe { $a_ptr.add($loop_size * $idx) };
        let r = unsafe { _rchr_c32_aa_x2(aa_ptr, $cc, $start_ptr) };
        if !r.is_none() {
            return r;
        }
    }};
}

#[inline(always)]
fn _memrchr_sse2_impl(buf: &[u8], c: u8) -> Option<usize> {
    let buf_len = buf.len();
    let start_ptr = buf.as_ptr();
    let mut buf_ptr_cur = unsafe { start_ptr.add(buf_len) };
    //
    if buf_len >= 16 {
        let cc: __m128i = unsafe { _c16_value(c) };
        {
            let loop_size = 16;
            let buf_ptr = unsafe { buf_ptr_cur.sub(loop_size) };
            let remaining_align = 0x10_usize - ((buf_ptr as usize) & 0x0F_usize);
            _unroll_one_rchr_16_uu!(buf_ptr, cc, start_ptr, loop_size, 0);
            buf_ptr_cur = unsafe { buf_ptr.add(remaining_align) };
        }
        //
        {
            let unroll = 4;
            let loop_size = 16;
            let mut buf_ptr = buf_ptr_cur;
            if unsafe { buf_ptr.offset_from(start_ptr) } >= (loop_size * unroll) as isize {
                let min_ptr = unsafe { start_ptr.add(loop_size * unroll) };
                if buf_ptr >= min_ptr {
                    while buf_ptr >= min_ptr {
                        buf_ptr = unsafe { buf_ptr.sub(loop_size * unroll) };
                        _unroll_one_rchr_16_aa_x4!(buf_ptr, cc, start_ptr, loop_size, 0);
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
                        _unroll_one_rchr_16_aa_x2!(buf_ptr, cc, start_ptr, loop_size, 0);
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
                    _unroll_one_rchr_16_aa!(buf_ptr, cc, start_ptr, loop_size, 0);
                }
                buf_ptr_cur = buf_ptr;
            }
        }
    }
    //
    let cc = (c as u64) * 0x0101_0101_0101_0101_u64;
    basic::_memrchr_remaining_15_bytes_impl(buf_ptr_cur, cc, start_ptr)
}

#[allow(clippy::missing_safety_doc)]
#[inline(always)]
fn _memrchr_avx2_impl(buf: &[u8], c: u8) -> Option<usize> {
    let buf_len = buf.len();
    let start_ptr = buf.as_ptr();
    let mut buf_ptr_cur = unsafe { start_ptr.add(buf_len) };
    //
    if buf_len >= 32 {
        let cc: __m256i = unsafe { _c32_value(c) };
        {
            let loop_size = 32;
            let buf_ptr = unsafe { buf_ptr_cur.sub(loop_size) };
            let remaining_align = 0x20_usize - ((buf_ptr as usize) & 0x1F_usize);
            _unroll_one_rchr_32_uu!(buf_ptr, cc, start_ptr, loop_size, 0);
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
                        _unroll_one_rchr_32_aa_x2!(buf_ptr, cc, start_ptr, loop_size, 0);
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
                    _unroll_one_rchr_32_aa!(buf_ptr, cc, start_ptr, loop_size, 0);
                }
                buf_ptr_cur = buf_ptr;
            }
        }
        {
            let cc: __m128i = unsafe { _c16_value(c) };
            let loop_size = 16;
            let mut buf_ptr = buf_ptr_cur;
            let min_ptr = unsafe { start_ptr.add(loop_size) };
            if buf_ptr >= min_ptr {
                while buf_ptr >= min_ptr {
                    buf_ptr = unsafe { buf_ptr.sub(loop_size) };
                    _unroll_one_rchr_16_aa!(buf_ptr, cc, start_ptr, loop_size, 0);
                }
                buf_ptr_cur = buf_ptr;
            }
        }
    } else if buf_len >= 16 {
        {
            let cc: __m128i = unsafe { _c16_value(c) };
            let loop_size = 16;
            let mut buf_ptr = buf_ptr_cur;
            let min_ptr = unsafe { start_ptr.add(loop_size) };
            if buf_ptr >= min_ptr {
                while buf_ptr >= min_ptr {
                    buf_ptr = unsafe { buf_ptr.sub(loop_size) };
                    _unroll_one_rchr_16_uu!(buf_ptr, cc, start_ptr, loop_size, 0);
                }
                buf_ptr_cur = buf_ptr;
            }
        }
    }
    //
    let cc = (c as u64) * 0x0101_0101_0101_0101_u64;
    basic::_memrchr_remaining_15_bytes_impl(buf_ptr_cur, cc, start_ptr)
}

#[inline(always)]
unsafe fn _c16_value(c: u8) -> __m128i {
    _mm_set1_epi8(c as i8)
}

#[inline(always)]
unsafe fn _rchr_c16_uu(buf_ptr: *const u8, mm_c16: __m128i, start_ptr: *const u8) -> Option<usize> {
    //
    let mm_a = _mm_loadu_si128(buf_ptr as *const __m128i);
    let mm_eq = _mm_cmpeq_epi8(mm_a, mm_c16);
    let mask = _mm_movemask_epi8(mm_eq);
    if mask != 0 {
        Some(plus_offset_from(buf_ptr, start_ptr) + 16 - 1 - (mask as u16).leading_zeros() as usize)
    } else {
        None
    }
}

#[inline(always)]
unsafe fn _rchr_c16_aa(buf_ptr: *const u8, mm_c16: __m128i, start_ptr: *const u8) -> Option<usize> {
    //
    let mm_a = _mm_load_si128(buf_ptr as *const __m128i);
    let mm_eq = _mm_cmpeq_epi8(mm_a, mm_c16);
    let mask = _mm_movemask_epi8(mm_eq);
    if mask != 0 {
        Some(plus_offset_from(buf_ptr, start_ptr) + 16 - 1 - (mask as u16).leading_zeros() as usize)
    } else {
        None
    }
}

#[inline(always)]
unsafe fn _rchr_c16_aa_x2(
    buf_ptr: *const u8,
    mm_c16: __m128i,
    start_ptr: *const u8,
) -> Option<usize> {
    //
    let mm_a = _mm_load_si128(buf_ptr as *const __m128i);
    let mm_b = _mm_load_si128(buf_ptr.add(16) as *const __m128i);
    let mm_a_eq = _mm_cmpeq_epi8(mm_a, mm_c16);
    let mm_b_eq = _mm_cmpeq_epi8(mm_b, mm_c16);
    let mask_a = _mm_movemask_epi8(mm_a_eq);
    let mask_b = _mm_movemask_epi8(mm_b_eq);
    if mask_b != 0 {
        Some(
            plus_offset_from(buf_ptr, start_ptr) + 16
                - 1
                - (mask_b as u16).leading_zeros() as usize
                + 16,
        )
    } else if mask_a != 0 {
        Some(
            plus_offset_from(buf_ptr, start_ptr) + 16
                - 1
                - (mask_a as u16).leading_zeros() as usize,
        )
    } else {
        None
    }
}

#[inline(always)]
unsafe fn _rchr_c16_aa_x4(
    buf_ptr: *const u8,
    mm_c16: __m128i,
    start_ptr: *const u8,
) -> Option<usize> {
    //
    let mm_a = _mm_load_si128(buf_ptr as *const __m128i);
    let mm_b = _mm_load_si128(buf_ptr.add(16) as *const __m128i);
    let mm_c = _mm_load_si128(buf_ptr.add(16 * 2) as *const __m128i);
    let mm_d = _mm_load_si128(buf_ptr.add(16 * 3) as *const __m128i);
    let mm_a_eq = _mm_cmpeq_epi8(mm_a, mm_c16);
    let mm_b_eq = _mm_cmpeq_epi8(mm_b, mm_c16);
    let mm_c_eq = _mm_cmpeq_epi8(mm_c, mm_c16);
    let mm_d_eq = _mm_cmpeq_epi8(mm_d, mm_c16);
    let mask_a = _mm_movemask_epi8(mm_a_eq);
    let mask_b = _mm_movemask_epi8(mm_b_eq);
    let mask_c = _mm_movemask_epi8(mm_c_eq);
    let mask_d = _mm_movemask_epi8(mm_d_eq);
    if mask_d != 0 {
        Some(
            plus_offset_from(buf_ptr, start_ptr) + 16
                - 1
                - (mask_d as u16).leading_zeros() as usize
                + 16 * 3,
        )
    } else if mask_c != 0 {
        Some(
            plus_offset_from(buf_ptr, start_ptr) + 16
                - 1
                - (mask_c as u16).leading_zeros() as usize
                + 16 * 2,
        )
    } else if mask_b != 0 {
        Some(
            plus_offset_from(buf_ptr, start_ptr) + 16
                - 1
                - (mask_b as u16).leading_zeros() as usize
                + 16,
        )
    } else if mask_a != 0 {
        Some(
            plus_offset_from(buf_ptr, start_ptr) + 16
                - 1
                - (mask_a as u16).leading_zeros() as usize,
        )
    } else {
        None
    }
}

#[inline(always)]
unsafe fn _c32_value(c: u8) -> __m256i {
    _mm256_set1_epi8(c as i8)
}

#[inline(always)]
unsafe fn _rchr_c32_uu(buf_ptr: *const u8, mm_c32: __m256i, start_ptr: *const u8) -> Option<usize> {
    //
    let mm_a = _mm256_loadu_si256(buf_ptr as *const __m256i);
    let mm_eq = _mm256_cmpeq_epi8(mm_a, mm_c32);
    let mask = _mm256_movemask_epi8(mm_eq);
    if mask != 0 {
        Some(plus_offset_from(buf_ptr, start_ptr) + 32 - 1 - (mask as u32).leading_zeros() as usize)
    } else {
        None
    }
}

#[inline(always)]
unsafe fn _rchr_c32_aa(buf_ptr: *const u8, mm_c32: __m256i, start_ptr: *const u8) -> Option<usize> {
    //
    let mm_a = _mm256_load_si256(buf_ptr as *const __m256i);
    let mm_eq = _mm256_cmpeq_epi8(mm_a, mm_c32);
    let mask = _mm256_movemask_epi8(mm_eq);
    if mask != 0 {
        Some(plus_offset_from(buf_ptr, start_ptr) + 32 - 1 - (mask as u32).leading_zeros() as usize)
    } else {
        None
    }
}

#[inline(always)]
unsafe fn _rchr_c32_aa_x2(
    buf_ptr: *const u8,
    mm_c32: __m256i,
    start_ptr: *const u8,
) -> Option<usize> {
    //
    let mm_a = _mm256_load_si256(buf_ptr as *const __m256i);
    let mm_b = _mm256_load_si256(buf_ptr.add(32) as *const __m256i);
    let mm_a_eq = _mm256_cmpeq_epi8(mm_a, mm_c32);
    let mm_b_eq = _mm256_cmpeq_epi8(mm_b, mm_c32);
    let mask_a = _mm256_movemask_epi8(mm_a_eq);
    let mask_b = _mm256_movemask_epi8(mm_b_eq);
    if mask_b != 0 {
        Some(
            plus_offset_from(buf_ptr, start_ptr) + 32
                - 1
                - (mask_b as u32).leading_zeros() as usize
                + 32,
        )
    } else if mask_a != 0 {
        Some(
            plus_offset_from(buf_ptr, start_ptr) + 32
                - 1
                - (mask_a as u32).leading_zeros() as usize,
        )
    } else {
        None
    }
}
