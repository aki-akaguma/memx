use crate::mem as basic;

#[inline(always)]
#[cfg(target_arch = "x86_64")]
pub fn _memchr_impl(buf: &[u8], c: u8) -> Option<usize> {
    unsafe { _memchr_sse2(buf, c) }
    /*
    if is_x86_feature_detected!("avx") {
        unsafe { _memchr_avx(buf, c) }
    } else {
        unsafe { _memchr_sse2(buf, c) }
    }
    */
}

#[inline(always)]
#[cfg(target_arch = "x86")]
pub fn _memchr_impl(buf: &[u8], c: u8) -> Option<usize> {
    _memchr_basic(buf, c)
    /*
    if is_x86_feature_detected!("avx") {
        unsafe { _memchr_avx(buf, c) }
    } else if is_x86_feature_detected!("sse2") {
        unsafe { _memchr_sse2(buf, c) }
    } else {
        _memchr_basic(buf, c)
    }
    */
}

fn _memchr_basic(buf: &[u8], c: u8) -> Option<usize> {
    basic::_memchr_impl(buf, c)
}

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "sse2")]
unsafe fn _memchr_sse2(buf: &[u8], c: u8) -> Option<usize> {
    //
    let buf_len = buf.len();
    let mut buf_ptr = buf.as_ptr();
    let start_ptr = buf_ptr;
    let end_ptr = buf_ptr.add(buf_len);
    //
    let loop_size = 16;
    let c16: std::arch::x86_64::__m128i = _c16_value(c);
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

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx")]
unsafe fn _memchr_avx(buf: &[u8], c: u8) -> Option<usize> {
    //
    let buf_len = buf.len();
    let mut buf_ptr = buf.as_ptr();
    let start_ptr = buf_ptr;
    let end_ptr = buf_ptr.add(buf_len);
    //
    let loop_size = 32;
    let c32: std::arch::x86_64::__m256i = _c32_value(c);
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

#[cfg(target_arch = "x86_64")]
#[inline(always)]
unsafe fn _c16_value(c: u8) -> std::arch::x86_64::__m128i {
    std::arch::x86_64::_mm_set1_epi8(c as i8)
}

#[cfg(target_arch = "x86_64")]
#[inline]
unsafe fn _check_c16_uu(
    buf_ptr: *const u8,
    mm_c16: std::arch::x86_64::__m128i,
    start_ptr: *const u8,
) -> Option<usize> {
    use std::arch::x86_64::__m128i;
    use std::arch::x86_64::_mm_cmpeq_epi8;
    use std::arch::x86_64::_mm_loadu_si128;
    use std::arch::x86_64::_mm_movemask_epi8;
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

#[cfg(target_arch = "x86_64")]
#[inline(always)]
unsafe fn _c32_value(c: u8) -> std::arch::x86_64::__m256i {
    std::arch::x86_64::_mm256_set1_epi8(c as i8)
}

#[cfg(target_arch = "x86_64")]
#[inline]
unsafe fn _check_c32_uu(
    buf_ptr: *const u8,
    mm_c32: std::arch::x86_64::__m256i,
    start_ptr: *const u8,
) -> Option<usize> {
    use std::arch::x86_64::__m256i;
    use std::arch::x86_64::_mm256_cmpeq_epi8;
    use std::arch::x86_64::_mm256_loadu_si256;
    use std::arch::x86_64::_mm256_movemask_epi8;
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
