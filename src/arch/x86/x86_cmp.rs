use crate::mem as basic;
use std::cmp::Ordering;

#[inline(always)]
#[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
pub fn _memcmp_impl(a: &[u8], b: &[u8]) -> Ordering {
    /*
    #[cfg(target_feature = "avx")]
    let r = unsafe { _memeq_avx(a, b) };
    #[cfg(target_feature = "sse2")]
    let r = unsafe { _memeq_sse2(a, b) };
    #[cfg(not(any(target_feature = "avx", target_feature = "sse2")))]
    */
    let r = _memcmp_basic(a, b);
    r
}

fn _memcmp_basic(a: &[u8], b: &[u8]) -> Ordering {
    basic::_memcmp_impl(a, b)
}

//
// Why is the next routine slow ??? <2021-05-17>
/*
#[cfg(target_arch = "x86_64")]
#[cfg(target_feature = "sse2")]
fn _memcmp_sse2(a: &[u8], b: &[u8]) -> Ordering {
    use std::arch::x86_64::__m128i;
    //
    let (a_u_pre, a_a, a_u_suf) = unsafe { a.align_to::<__m128i>() };
    let a_u_pre_len = a_u_pre.len();
    let a_a_bytes_len = a_a.len() * std::mem::size_of::<__m128i>();
    let a_u_suf_len = a_u_suf.len();
    //
    {
        let r = basic::_memcmp_impl(a_u_pre, &b[0..a_u_pre_len]);
        if r != Ordering::Equal {
            return r;
        }
    }
    {
        let b_start_idx = a_u_pre_len;
        let r = _memcmp_sse2_au(a_a, &b[b_start_idx..(b_start_idx + a_a_bytes_len)]);
        if r != Ordering::Equal {
            return r;
        }
    }
    {
        let b_start_idx = a_u_pre_len + a_a_bytes_len;
        let r = basic::_memcmp_impl(a_u_suf, &b[b_start_idx..(b_start_idx + a_u_suf_len)]);
        if r != Ordering::Equal {
            return r;
        }
    }
    Ordering::Equal
}

// Why is the next routine slow ??? <2021-05-17>
#[cfg(target_arch = "x86_64")]
#[cfg(target_feature = "sse2")]
#[inline]
fn _memcmp_sse2_uu(a: &[u8], b: &[u8]) -> Ordering {
    //
    let a_len = a.len();
    let b_len = b.len();
    let min_len = a_len.min(b_len);
    let mut a_ptr = a.as_ptr();
    let mut b_ptr = b.as_ptr();
    let a_start_ptr = a_ptr;
    let end_ptr = unsafe { a_ptr.add(min_len) };
    //
    let loop_size = 16;
    while a_ptr <= unsafe { end_ptr.sub(loop_size) } {
        let r = unsafe { _cmp_bytes_16_uu(a_ptr, b_ptr) };
        if !r {
            let next_idx = unsafe { a_ptr.offset_from(a_start_ptr) } as usize;
            return basic::_memcmp_impl(
                &a[next_idx..(next_idx + 16)],
                &b[next_idx..(next_idx + 16)],
            );
        }
        a_ptr = unsafe { a_ptr.add(loop_size) };
        b_ptr = unsafe { b_ptr.add(loop_size) };
    }
    //
    let next_idx = unsafe { a_ptr.offset_from(a_start_ptr) } as usize;
    basic::_memcmp_impl(&a[next_idx..], &b[next_idx..])
}

// Why is the next routine slow ??? <2021-05-17>
#[cfg(target_arch = "x86_64")]
#[cfg(target_feature = "sse2")]
#[inline]
fn _memcmp_sse2_au(a: &[std::arch::x86_64::__m128i], b: &[u8]) -> Ordering {
    //
    let a_len = a.len() * std::mem::size_of::<std::arch::x86_64::__m128i>();
    let b_len = b.len();
    let min_len = a_len.min(b_len);
    let mut a_ptr = a.as_ptr() as *const u8;
    let mut b_ptr = b.as_ptr();
    let a_start_ptr = a_ptr;
    let end_ptr = unsafe { a_ptr.add(min_len) };
    //
    let loop_size = 16;
    while a_ptr <= unsafe { end_ptr.sub(loop_size) } {
        let r = unsafe { _cmp_bytes_16_au(a_ptr, b_ptr) };
        if !r {
            let next_idx = unsafe { a_ptr.offset_from(a_start_ptr) } as usize;
            let aa: &[u8] = unsafe { std::slice::from_raw_parts(a_ptr, 16) };
            return basic::_memcmp_impl(aa, &b[next_idx..(next_idx + 16)]);
        }
        a_ptr = unsafe { a_ptr.add(loop_size) };
        b_ptr = unsafe { b_ptr.add(loop_size) };
    }
    //
    let next_idx = unsafe { a_ptr.offset_from(a_start_ptr) } as usize;
    let aa: &[u8] = unsafe { std::slice::from_raw_parts(a_start_ptr, a_len) };
    basic::_memcmp_impl(&aa[next_idx..], &b[next_idx..])
}

#[cfg(target_arch = "x86_64")]
#[cfg(target_feature = "sse2")]
#[inline]
unsafe fn _cmp_bytes_16_uu(a_ptr: *const u8, b_ptr: *const u8) -> bool {
    use std::arch::x86_64::__m128i;
    use std::arch::x86_64::_mm_cmpeq_epi8;
    use std::arch::x86_64::_mm_loadu_si128;
    use std::arch::x86_64::_mm_movemask_epi8;
    let mm_a = _mm_loadu_si128(a_ptr as *const __m128i);
    let mm_b = _mm_loadu_si128(b_ptr as *const __m128i);
    let mm_eq = _mm_cmpeq_epi8(mm_a, mm_b);
    let mask = _mm_movemask_epi8(mm_eq);
    mask == 0xffff
}

#[cfg(target_arch = "x86_64")]
#[cfg(target_feature = "sse2")]
#[inline]
unsafe fn _cmp_bytes_16_au(a_ptr: *const u8, b_ptr: *const u8) -> bool {
    use std::arch::x86_64::__m128i;
    use std::arch::x86_64::_mm_cmpeq_epi8;
    use std::arch::x86_64::_mm_load_si128;
    use std::arch::x86_64::_mm_loadu_si128;
    use std::arch::x86_64::_mm_movemask_epi8;
    let mm_a = _mm_load_si128(a_ptr as *const __m128i);
    let mm_b = _mm_loadu_si128(b_ptr as *const __m128i);
    let mm_eq = _mm_cmpeq_epi8(mm_a, mm_b);
    let mask = _mm_movemask_epi8(mm_eq);
    mask == 0xffff
}
*/
