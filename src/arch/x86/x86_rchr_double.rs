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
pub(crate) fn _memrchr_double_impl(buf: &[u8], c1: u8, c2: u8) -> Option<usize> {
    #[cfg(target_feature = "avx")]
    let r = unsafe { _memrchr_double_avx(buf, c1, c2) };
    #[cfg(all(target_feature = "sse2", not(target_feature = "avx")))]
    let r = unsafe { _memrchr_double_sse2(buf, c1, c2) };
    #[cfg(not(any(target_feature = "avx", target_feature = "sse2")))]
    let r = _memrchr_double_basic(buf, c1, c2);
    r
    /*
     * <WSID>
     * The is_x86_feature_detected!() routine is slower, and not support no_std.
     * What should i do ?
     * </WSID>
     *
    if is_x86_feature_detected!("avx") {
        unsafe { _memrchr_avx(buf, c) }
    } else {
        unsafe { _memrchr_sse2(buf, c) }
    }
    */
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

macro_rules! _unroll_one_rchr_16_uu {
    ($a_ptr:expr, $cc1:expr, $cc2:expr, $start_ptr:expr, $loop_size:expr, $idx:expr) => {{
        let aa_ptr = unsafe { $a_ptr.add($loop_size * $idx) };
        let r1 = unsafe { _rchr_c16_uu(aa_ptr, $cc1, $start_ptr) };
        let r2 = unsafe { _rchr_c16_uu(aa_ptr, $cc2, $start_ptr) };
        if !r1.is_none() && r2.is_none() {
            return r1;
        } else if r1.is_none() && !r2.is_none() {
            return r2;
        } else if !r1.is_none() && !r2.is_none() {
            let idx1 = r1.map(|a| a).unwrap();
            let idx2 = r2.map(|a| a).unwrap();
            if idx1 > idx2 {
                return Some(idx1);
            } else {
                return Some(idx2);
            }
        }
    }};
}

macro_rules! _unroll_one_rchr_16_aa {
    ($a_ptr:expr, $cc1:expr, $cc2:expr, $start_ptr:expr, $loop_size:expr, $idx:expr) => {{
        let aa_ptr = unsafe { $a_ptr.add($loop_size * $idx) };
        let r1 = unsafe { _rchr_c16_aa(aa_ptr, $cc1, $start_ptr) };
        let r2 = unsafe { _rchr_c16_aa(aa_ptr, $cc2, $start_ptr) };
        if !r1.is_none() && r2.is_none() {
            return r1;
        } else if r1.is_none() && !r2.is_none() {
            return r2;
        } else if !r1.is_none() && !r2.is_none() {
            let idx1 = r1.map(|a| a).unwrap();
            let idx2 = r2.map(|a| a).unwrap();
            if idx1 > idx2 {
                return Some(idx1);
            } else {
                return Some(idx2);
            }
        }
    }};
}

#[inline(always)]
fn _memrchr_double_sse2_impl(buf: &[u8], c1: u8, c2: u8) -> Option<usize> {
    if buf.is_empty() {
        return None;
    }
    if buf[buf.len() - 1] == c1 || buf[buf.len() - 1] == c2 {
        return Some(buf.len() - 1);
    }
    //
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
            let min_ptr = unsafe { start_ptr.add(loop_size * unroll) };
            if buf_ptr >= min_ptr {
                while buf_ptr >= min_ptr {
                    buf_ptr = unsafe { buf_ptr.sub(loop_size * unroll) };
                    _unroll_one_rchr_16_aa!(buf_ptr, cc1, cc2, start_ptr, loop_size, 7);
                    _unroll_one_rchr_16_aa!(buf_ptr, cc1, cc2, start_ptr, loop_size, 6);
                    _unroll_one_rchr_16_aa!(buf_ptr, cc1, cc2, start_ptr, loop_size, 5);
                    _unroll_one_rchr_16_aa!(buf_ptr, cc1, cc2, start_ptr, loop_size, 4);
                    _unroll_one_rchr_16_aa!(buf_ptr, cc1, cc2, start_ptr, loop_size, 3);
                    _unroll_one_rchr_16_aa!(buf_ptr, cc1, cc2, start_ptr, loop_size, 2);
                    _unroll_one_rchr_16_aa!(buf_ptr, cc1, cc2, start_ptr, loop_size, 1);
                    _unroll_one_rchr_16_aa!(buf_ptr, cc1, cc2, start_ptr, loop_size, 0);
                }
                buf_ptr_cur = buf_ptr;
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

#[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
#[target_feature(enable = "avx")]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn _memrchr_double_avx(buf: &[u8], c1: u8, c2: u8) -> Option<usize> {
    _memrchr_double_sse2_impl(buf, c1, c2)
}
/*
#[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
#[target_feature(enable = "avx")]
pub unsafe fn _memrchr_avx(buf: &[u8], c: u8) -> Option<usize> {
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
        if !r.is_none() {
            return r;
        }
        buf_ptr = buf_ptr.add(loop_size);
    }
    //
    let next_idx = plus_offset_from(buf_ptr, start_ptr);
    let r = _memrchr_sse2(&buf[next_idx..], c);
    if let Some(pos) = r {
        Some(pos + next_idx)
    } else {
        None
    }
}
*/

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
    let mask = _mm256_movemask_epi8(mm_eq);
    if mask != 0 {
        Some(plus_offset_from(buf_ptr, start_ptr) + 32 - 1 - (mask as u32).leading_zeros() as usize)
    } else {
        None
    }
}
