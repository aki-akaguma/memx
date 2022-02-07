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
use mmx::_mm256_loadu_si256;
use mmx::_mm256_movemask_epi8;
use mmx::_mm256_set1_epi8;

#[inline(always)]
#[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
pub(crate) fn _memnechr_impl(buf: &[u8], c: u8) -> Option<usize> {
    #[cfg(target_feature = "avx")]
    let r = unsafe { _memnechr_avx(buf, c) };
    #[cfg(all(target_feature = "sse2", not(target_feature = "avx")))]
    let r = unsafe { _memnechr_sse2(buf, c) };
    #[cfg(not(any(target_feature = "avx", target_feature = "sse2")))]
    let r = _memnechr_basic(buf, c);
    r
    /*
     * <WSID>
     * The is_x86_feature_detected!() routine is slower, and not support no_std.
     * What should i do ?
     * </WSID>
     *
    if is_x86_feature_detected!("avx") {
        unsafe { _memnechr_avx(buf, c) }
    } else {
        unsafe { _memnechr_sse2(buf, c) }
    }
    */
}

fn _memnechr_basic(buf: &[u8], c: u8) -> Option<usize> {
    basic::_memnechr_impl(buf, c)
}

#[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
#[target_feature(enable = "sse2")]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn _memnechr_sse2(buf: &[u8], c: u8) -> Option<usize> {
    _memnechr_sse2_impl(buf, c)
}

macro_rules! _unroll_one_nechr_16_uu {
    ($a_ptr:expr, $cc:expr, $start_ptr:expr, $loop_size:expr, $idx:expr) => {{
        let aa_ptr = unsafe { $a_ptr.add($loop_size * $idx) };
        let r = unsafe { _nechr_c16_uu(aa_ptr, $cc, $start_ptr) };
        if !r.is_none() {
            return r;
        }
    }};
}

macro_rules! _unroll_one_nechr_16_aa {
    ($a_ptr:expr, $cc:expr, $start_ptr:expr, $loop_size:expr, $idx:expr) => {{
        let aa_ptr = unsafe { $a_ptr.add($loop_size * $idx) };
        let r = unsafe { _nechr_c16_aa(aa_ptr, $cc, $start_ptr) };
        if !r.is_none() {
            return r;
        }
    }};
}

#[inline(always)]
fn _memnechr_sse2_impl(buf: &[u8], c: u8) -> Option<usize> {
    if buf.is_empty() {
        return None;
    }
    if buf[0] != c {
        return Some(0);
    }
    //
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
            _unroll_one_nechr_16_uu!(buf_ptr, cc, start_ptr, loop_size, 0);
            //
            buf_ptr = unsafe { buf_ptr.add(remaining_align) };
        }
        //
        {
            let unroll = 8;
            let loop_size = 16;
            let end_ptr_16_8 = unsafe { end_ptr.sub(loop_size * unroll) };
            while buf_ptr <= end_ptr_16_8 {
                _unroll_one_nechr_16_aa!(buf_ptr, cc, start_ptr, loop_size, 0);
                _unroll_one_nechr_16_aa!(buf_ptr, cc, start_ptr, loop_size, 1);
                _unroll_one_nechr_16_aa!(buf_ptr, cc, start_ptr, loop_size, 2);
                _unroll_one_nechr_16_aa!(buf_ptr, cc, start_ptr, loop_size, 3);
                _unroll_one_nechr_16_aa!(buf_ptr, cc, start_ptr, loop_size, 4);
                _unroll_one_nechr_16_aa!(buf_ptr, cc, start_ptr, loop_size, 5);
                _unroll_one_nechr_16_aa!(buf_ptr, cc, start_ptr, loop_size, 6);
                _unroll_one_nechr_16_aa!(buf_ptr, cc, start_ptr, loop_size, 7);
                //
                buf_ptr = unsafe { buf_ptr.add(loop_size * unroll) };
            }
        }
        {
            let loop_size = 16;
            let end_ptr_16 = unsafe { end_ptr.sub(loop_size) };
            while buf_ptr <= end_ptr_16 {
                _unroll_one_nechr_16_aa!(buf_ptr, cc, start_ptr, loop_size, 0);
                //
                buf_ptr = unsafe { buf_ptr.add(loop_size) };
            }
        }
    }
    //
    let cc = (c as u64) * 0x0101_0101_0101_0101_u64;
    basic::_memnechr_remaining_15_bytes_impl(buf_ptr, cc, start_ptr, end_ptr)
}

#[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
#[target_feature(enable = "avx")]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn _memnechr_avx(buf: &[u8], c: u8) -> Option<usize> {
    if buf.is_empty() {
        return None;
    }
    if buf[0] != c {
        return Some(0);
    }
    //
    let buf_len = buf.len();
    let mut buf_ptr = buf.as_ptr();
    let start_ptr = buf_ptr;
    let end_ptr = buf_ptr.add(buf_len);
    //
    let loop_size = 32;
    let c32: __m256i = _c32_value(c);
    while buf_ptr <= end_ptr.sub(loop_size) {
        let r = _check_c32_uu(buf_ptr, c32, start_ptr);
        if r.is_some() {
            return r;
        }
        buf_ptr = buf_ptr.add(loop_size);
    }
    //
    let next_idx = plus_offset_from(buf_ptr, start_ptr);
    let r = _memnechr_sse2(&buf[next_idx..], c);
    r.map(|pos| pos + next_idx)
}

#[inline(always)]
unsafe fn _c16_value(c: u8) -> __m128i {
    _mm_set1_epi8(c as i8)
}

#[inline(always)]
unsafe fn _nechr_c16_uu(
    buf_ptr: *const u8,
    mm_c16: __m128i,
    start_ptr: *const u8,
) -> Option<usize> {
    //
    let mm_a = _mm_loadu_si128(buf_ptr as *const __m128i);
    let mm_eq = _mm_cmpeq_epi8(mm_a, mm_c16);
    let mask = _mm_movemask_epi8(mm_eq) as u32 & 0xFFFF_u32;
    if mask != 0xFFFF_u32 {
        Some(plus_offset_from(buf_ptr, start_ptr) + mask.trailing_ones() as usize)
    } else {
        None
    }
}

#[inline(always)]
unsafe fn _nechr_c16_aa(
    buf_ptr: *const u8,
    mm_c16: __m128i,
    start_ptr: *const u8,
) -> Option<usize> {
    //
    let mm_a = _mm_load_si128(buf_ptr as *const __m128i);
    let mm_eq = _mm_cmpeq_epi8(mm_a, mm_c16);
    let mask = _mm_movemask_epi8(mm_eq) as u32 & 0xFFFF_u32;
    if mask != 0xFFFF_u32 {
        Some(plus_offset_from(buf_ptr, start_ptr) + mask.trailing_ones() as usize)
    } else {
        None
    }
}

#[inline(always)]
unsafe fn _c32_value(c: u8) -> __m256i {
    _mm256_set1_epi8(c as i8)
}

#[inline(always)]
unsafe fn _check_c32_uu(
    buf_ptr: *const u8,
    mm_c32: __m256i,
    start_ptr: *const u8,
) -> Option<usize> {
    //
    let mm_a = _mm256_loadu_si256(buf_ptr as *const __m256i);
    let mm_eq = _mm256_cmpeq_epi8(mm_a, mm_c32);
    let mask = _mm256_movemask_epi8(mm_eq) as u32;
    if mask != 0xFFFF_FFFF_u32 {
        Some(plus_offset_from(buf_ptr, start_ptr) + mask.trailing_ones() as usize)
    } else {
        None
    }
}
