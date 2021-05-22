use crate::mem as basic;

#[cfg(target_arch = "x86")]
use std::arch::x86 as mmx;
#[cfg(target_arch = "x86_64")]
use std::arch::x86_64 as mmx;

use mmx::__m128i;
use mmx::_mm_cmpeq_epi8;
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
pub(crate) fn _memchr_impl(buf: &[u8], c: u8) -> Option<usize> {
    #[cfg(target_feature = "avx")]
    let r = unsafe { _memchr_avx(buf, c) };
    #[cfg(target_feature = "sse2")]
    let r = unsafe { _memchr_sse2(buf, c) };
    #[cfg(not(any(target_feature = "avx", target_feature = "sse2")))]
    let r = _memchr_basic(buf, c);
    r
    /*
     * <WSID>
     * The is_x86_feature_detected!() routine is slower, and not support no_std.
     * What should i do ?
     * </WSID>
     *
    if is_x86_feature_detected!("avx") {
        unsafe { _memchr_avx(buf, c) }
    } else {
        unsafe { _memchr_sse2(buf, c) }
    }
    */
}

fn _memchr_basic(buf: &[u8], c: u8) -> Option<usize> {
    basic::_memchr_impl(buf, c)
}

#[target_feature(enable = "sse2")]
pub unsafe fn _memchr_sse2(buf: &[u8], c: u8) -> Option<usize> {
    //
    let buf_len = buf.len();
    let mut buf_ptr = buf.as_ptr();
    let start_ptr = buf_ptr;
    let end_ptr = buf_ptr.add(buf_len);
    //
    let loop_size = 16;
    let c16: __m128i = _c16_value(c);
    while buf_ptr <= end_ptr.sub(loop_size) {
        let r = _check_c16_uu(buf_ptr, c16, start_ptr);
        if !r.is_none() {
            return r;
        }
        buf_ptr = buf_ptr.add(loop_size);
    }
    //
    let next_idx = buf_ptr.offset_from(start_ptr) as usize;
    basic::_memchr_impl(&buf[next_idx..], c)
}

#[target_feature(enable = "avx")]
pub unsafe fn _memchr_avx(buf: &[u8], c: u8) -> Option<usize> {
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
    let next_idx = buf_ptr.offset_from(start_ptr) as usize;
    _memchr_sse2(&buf[next_idx..], c)
}

#[inline(always)]
unsafe fn _c16_value(c: u8) -> __m128i {
    _mm_set1_epi8(c as i8)
}

#[inline(always)]
unsafe fn _check_c16_uu(
    buf_ptr: *const u8,
    mm_c16: __m128i,
    start_ptr: *const u8,
) -> Option<usize> {
    //
    let mm_a = _mm_loadu_si128(buf_ptr as *const __m128i);
    let mm_eq = _mm_cmpeq_epi8(mm_a, mm_c16);
    let mask = _mm_movemask_epi8(mm_eq);
    if mask != 0 {
        Some(buf_ptr.offset_from(start_ptr) as usize + mask.trailing_zeros() as usize)
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
        Some(buf_ptr.offset_from(start_ptr) as usize + mask.trailing_zeros() as usize)
    } else {
        None
    }
}
