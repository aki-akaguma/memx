use crate::mem as basic;
use crate::plus_offset_from;

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
pub(crate) fn _memchr_impl(buf: &[u8], c: u8) -> Option<usize> {
    // TODO: Replace with https://github.com/rust-lang/rfcs/pull/2725
    // after stabilization
    if cpuid::has_avx2() {
        unsafe { _memchr_avx2(buf, c) }
    } else {
        unsafe { _memchr_sse2(buf, c) }
    }
}

#[inline(always)]
#[cfg(target_arch = "x86")]
pub(crate) fn _memchr_impl(buf: &[u8], c: u8) -> Option<usize> {
    // TODO: Replace with https://github.com/rust-lang/rfcs/pull/2725
    // after stabilization
    if cpuid::has_avx2() {
        unsafe { _memchr_avx2(buf, c) }
    } else if cpuid::has_sse2() {
        unsafe { _memchr_sse2(buf, c) }
    } else {
        _memchr_basic(buf, c)
    }
}

fn _memchr_basic(buf: &[u8], c: u8) -> Option<usize> {
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

macro_rules! _unroll_one_chr_16_uu {
    ($a_ptr:expr, $cc:expr, $start_ptr:expr, $loop_size:expr, $idx:expr) => {{
        let aa_ptr = unsafe { $a_ptr.add($loop_size * $idx) };
        let r = unsafe { _chr_c16_uu(aa_ptr, $cc, $start_ptr) };
        if !r.is_none() {
            return r;
        }
    }};
}

macro_rules! _unroll_one_chr_16_aa {
    ($a_ptr:expr, $cc:expr, $start_ptr:expr, $loop_size:expr, $idx:expr) => {{
        let aa_ptr = unsafe { $a_ptr.add($loop_size * $idx) };
        let r = unsafe { _chr_c16_aa(aa_ptr, $cc, $start_ptr) };
        if !r.is_none() {
            return r;
        }
    }};
}

macro_rules! _unroll_one_chr_16_aa_x2 {
    ($a_ptr:expr, $cc:expr, $start_ptr:expr, $loop_size:expr, $idx:expr) => {{
        let aa_ptr = unsafe { $a_ptr.add($loop_size * $idx) };
        let r = unsafe { _chr_c16_aa_x2(aa_ptr, $cc, $start_ptr) };
        if !r.is_none() {
            return r;
        }
    }};
}

macro_rules! _unroll_one_chr_16_aa_x4 {
    ($a_ptr:expr, $cc:expr, $start_ptr:expr, $loop_size:expr, $idx:expr) => {{
        let aa_ptr = unsafe { $a_ptr.add($loop_size * $idx) };
        let r = unsafe { _chr_c16_aa_x4(aa_ptr, $cc, $start_ptr) };
        if !r.is_none() {
            return r;
        }
    }};
}

macro_rules! _unroll_one_chr_32_uu {
    ($a_ptr:expr, $cc:expr, $start_ptr:expr, $loop_size:expr, $idx:expr) => {{
        let aa_ptr = unsafe { $a_ptr.add($loop_size * $idx) };
        let r = unsafe { _chr_c32_uu(aa_ptr, $cc, $start_ptr) };
        if !r.is_none() {
            return r;
        }
    }};
}

macro_rules! _unroll_one_chr_32_aa {
    ($a_ptr:expr, $cc:expr, $start_ptr:expr, $loop_size:expr, $idx:expr) => {{
        let aa_ptr = unsafe { $a_ptr.add($loop_size * $idx) };
        let r = unsafe { _chr_c32_aa(aa_ptr, $cc, $start_ptr) };
        if !r.is_none() {
            return r;
        }
    }};
}

macro_rules! _unroll_one_chr_32_aa_x2 {
    ($a_ptr:expr, $cc:expr, $start_ptr:expr, $loop_size:expr, $idx:expr) => {{
        let aa_ptr = unsafe { $a_ptr.add($loop_size * $idx) };
        let r = unsafe { _chr_c32_aa_x2(aa_ptr, $cc, $start_ptr) };
        if !r.is_none() {
            return r;
        }
    }};
}

#[inline(always)]
fn _memchr_sse2_impl(buf: &[u8], c: u8) -> Option<usize> {
    let buf_len = buf.len();
    let mut buf_ptr = buf.as_ptr();
    let start_ptr = buf_ptr;
    let end_ptr = unsafe { buf_ptr.add(buf_len) };
    //
    if buf_len >= 16 {
        let cc: __m128i = unsafe { _c16_value(c) };
        {
            let remaining_align = 0x10_usize - ((buf_ptr as usize) & 0x0F_usize);
            let loop_size = 16;
            _unroll_one_chr_16_uu!(buf_ptr, cc, start_ptr, loop_size, 0);
            buf_ptr = unsafe { buf_ptr.add(remaining_align) };
        }
        //
        {
            let unroll = 4;
            let loop_size = 16;
            if unsafe { end_ptr.offset_from(buf_ptr) } >= (loop_size * unroll) as isize {
                let end_ptr_16_x4 = unsafe { end_ptr.sub(loop_size * unroll) };
                while buf_ptr <= end_ptr_16_x4 {
                    _unroll_one_chr_16_aa_x4!(buf_ptr, cc, start_ptr, loop_size, 0);
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
                    _unroll_one_chr_16_aa_x2!(buf_ptr, cc, start_ptr, loop_size, 0);
                    //_unroll_one_chr_16_aa!(buf_ptr, cc, start_ptr, loop_size, 0);
                    //_unroll_one_chr_16_aa!(buf_ptr, cc, start_ptr, loop_size, 1);
                    buf_ptr = unsafe { buf_ptr.add(loop_size * unroll) };
                }
            }
        }
        {
            let loop_size = 16;
            let end_ptr_16 = unsafe { end_ptr.sub(loop_size) };
            while buf_ptr <= end_ptr_16 {
                _unroll_one_chr_16_aa!(buf_ptr, cc, start_ptr, loop_size, 0);
                buf_ptr = unsafe { buf_ptr.add(loop_size) };
            }
        }
    }
    //
    let cc = (c as u64) * 0x0101_0101_0101_0101_u64;
    basic::_memchr_remaining_15_bytes_impl(buf_ptr, cc, start_ptr, end_ptr)
}

#[allow(clippy::missing_safety_doc)]
#[inline(always)]
pub fn _memchr_avx2_impl(buf: &[u8], c: u8) -> Option<usize> {
    let buf_len = buf.len();
    let mut buf_ptr = buf.as_ptr();
    let start_ptr = buf_ptr;
    let end_ptr = unsafe { buf_ptr.add(buf_len) };
    //
    if buf_len >= 32 {
        let cc: __m256i = unsafe { _c32_value(c) };
        {
            let remaining_align = 0x20_usize - ((buf_ptr as usize) & 0x1F_usize);
            let loop_size = 32;
            _unroll_one_chr_32_uu!(buf_ptr, cc, start_ptr, loop_size, 0);
            buf_ptr = unsafe { buf_ptr.add(remaining_align) };
        }
        //
        {
            let unroll = 2;
            let loop_size = 32;
            if unsafe { end_ptr.offset_from(buf_ptr) } >= (loop_size * unroll) as isize {
                let end_ptr_32_x2 = unsafe { end_ptr.sub(loop_size * unroll) };
                while buf_ptr <= end_ptr_32_x2 {
                    _unroll_one_chr_32_aa_x2!(buf_ptr, cc, start_ptr, loop_size, 0);
                    buf_ptr = unsafe { buf_ptr.add(loop_size * unroll) };
                }
            }
        }
        {
            let loop_size = 32;
            let end_ptr_32 = unsafe { end_ptr.sub(loop_size) };
            while buf_ptr <= end_ptr_32 {
                _unroll_one_chr_32_aa!(buf_ptr, cc, start_ptr, loop_size, 0);
                buf_ptr = unsafe { buf_ptr.add(loop_size) };
            }
        }
        {
            let cc: __m128i = unsafe { _c16_value(c) };
            let loop_size = 16;
            let end_ptr_16 = unsafe { end_ptr.sub(loop_size) };
            while buf_ptr <= end_ptr_16 {
                _unroll_one_chr_16_aa!(buf_ptr, cc, start_ptr, loop_size, 0);
                buf_ptr = unsafe { buf_ptr.add(loop_size) };
            }
        }
    } else if buf_len >= 16 {
        {
            let cc: __m128i = unsafe { _c16_value(c) };
            let loop_size = 16;
            let end_ptr_16 = unsafe { end_ptr.sub(loop_size) };
            while buf_ptr <= end_ptr_16 {
                _unroll_one_chr_16_uu!(buf_ptr, cc, start_ptr, loop_size, 0);
                buf_ptr = unsafe { buf_ptr.add(loop_size) };
            }
        }
    }
    //
    let cc = (c as u64) * 0x0101_0101_0101_0101_u64;
    basic::_memchr_remaining_15_bytes_impl(buf_ptr, cc, start_ptr, end_ptr)
}

#[inline(always)]
unsafe fn _c16_value(c: u8) -> __m128i {
    _mm_set1_epi8(c as i8)
}

#[inline(always)]
unsafe fn _chr_c16_uu(buf_ptr: *const u8, mm_c16: __m128i, start_ptr: *const u8) -> Option<usize> {
    //
    let mm_a = _mm_loadu_si128(buf_ptr as *const __m128i);
    let mm_eq = _mm_cmpeq_epi8(mm_a, mm_c16);
    let mask = _mm_movemask_epi8(mm_eq);
    if mask != 0 {
        Some(plus_offset_from(buf_ptr, start_ptr) + mask.trailing_zeros() as usize)
    } else {
        None
    }
}

#[inline(always)]
unsafe fn _chr_c16_aa(buf_ptr: *const u8, mm_c16: __m128i, start_ptr: *const u8) -> Option<usize> {
    //
    let mm_a = _mm_load_si128(buf_ptr as *const __m128i);
    let mm_eq = _mm_cmpeq_epi8(mm_a, mm_c16);
    let mask = _mm_movemask_epi8(mm_eq);
    if mask != 0 {
        Some(plus_offset_from(buf_ptr, start_ptr) + mask.trailing_zeros() as usize)
    } else {
        None
    }
}

#[inline(always)]
unsafe fn _chr_c16_aa_x2(
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
    if mask_a != 0 {
        Some(plus_offset_from(buf_ptr, start_ptr) + mask_a.trailing_zeros() as usize)
    } else if mask_b != 0 {
        Some(plus_offset_from(buf_ptr, start_ptr) + mask_b.trailing_zeros() as usize + 16)
    } else {
        None
    }
    /*
    let mask = mask_b << 16 | mask_a;
    if mask != 0 {
        Some(plus_offset_from(buf_ptr, start_ptr) + mask.trailing_zeros() as usize)
    } else {
        None
    }
    */
}

#[inline(always)]
unsafe fn _chr_c16_aa_x4(
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
    let mask_a = _mm_movemask_epi8(mm_a_eq) as u64;
    let mask_b = _mm_movemask_epi8(mm_b_eq) as u64;
    let mask_c = _mm_movemask_epi8(mm_c_eq) as u64;
    let mask_d = _mm_movemask_epi8(mm_d_eq) as u64;
    if mask_a != 0 {
        Some(plus_offset_from(buf_ptr, start_ptr) + mask_a.trailing_zeros() as usize)
    } else if mask_b != 0 {
        Some(plus_offset_from(buf_ptr, start_ptr) + mask_b.trailing_zeros() as usize + 16)
    } else if mask_c != 0 {
        Some(plus_offset_from(buf_ptr, start_ptr) + mask_c.trailing_zeros() as usize + 16 * 2)
    } else if mask_d != 0 {
        Some(plus_offset_from(buf_ptr, start_ptr) + mask_d.trailing_zeros() as usize + 16 * 3)
    } else {
        None
    }
    /*
    let mask = mask_d << (16 * 3) | mask_c << (16 * 2) | mask_b << 16 | mask_a;
    if mask != 0 {
        Some(plus_offset_from(buf_ptr, start_ptr) + mask.trailing_zeros() as usize)
    } else {
        None
    }
    */
}

#[inline(always)]
unsafe fn _c32_value(c: u8) -> __m256i {
    _mm256_set1_epi8(c as i8)
}

#[inline(always)]
unsafe fn _chr_c32_uu(buf_ptr: *const u8, mm_c32: __m256i, start_ptr: *const u8) -> Option<usize> {
    //
    let mm_a = _mm256_loadu_si256(buf_ptr as *const __m256i);
    let mm_eq = _mm256_cmpeq_epi8(mm_a, mm_c32);
    let mask = _mm256_movemask_epi8(mm_eq);
    if mask != 0 {
        Some(plus_offset_from(buf_ptr, start_ptr) + mask.trailing_zeros() as usize)
    } else {
        None
    }
}

#[inline(always)]
unsafe fn _chr_c32_aa(buf_ptr: *const u8, mm_c32: __m256i, start_ptr: *const u8) -> Option<usize> {
    //
    let mm_a = _mm256_load_si256(buf_ptr as *const __m256i);
    let mm_eq = _mm256_cmpeq_epi8(mm_a, mm_c32);
    let mask = _mm256_movemask_epi8(mm_eq);
    if mask != 0 {
        Some(plus_offset_from(buf_ptr, start_ptr) + mask.trailing_zeros() as usize)
    } else {
        None
    }
}

#[inline(always)]
unsafe fn _chr_c32_aa_x2(
    buf_ptr: *const u8,
    mm_c32: __m256i,
    start_ptr: *const u8,
) -> Option<usize> {
    //
    let mm_a = _mm256_load_si256(buf_ptr as *const __m256i);
    let mm_b = _mm256_load_si256(buf_ptr.add(32) as *const __m256i);
    let mm_a_eq = _mm256_cmpeq_epi8(mm_a, mm_c32);
    let mm_b_eq = _mm256_cmpeq_epi8(mm_b, mm_c32);
    let mask_a = _mm256_movemask_epi8(mm_a_eq) as u64;
    let mask_b = _mm256_movemask_epi8(mm_b_eq) as u64;
    if mask_a != 0 {
        Some(plus_offset_from(buf_ptr, start_ptr) + mask_a.trailing_zeros() as usize)
    } else if mask_b != 0 {
        Some(plus_offset_from(buf_ptr, start_ptr) + mask_b.trailing_zeros() as usize + 32)
    } else {
        None
    }
    /*
    let mask = mask_b << 32 | mask_a;
    if mask != 0 {
        Some(plus_offset_from(buf_ptr, start_ptr) + mask.trailing_zeros() as usize)
    } else {
        None
    }
    */
}
