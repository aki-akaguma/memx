use crate::mem as basic;
use core::cmp::Ordering;

#[cfg(target_arch = "x86")]
use core::arch::x86 as mmx;
#[cfg(target_arch = "x86_64")]
use core::arch::x86_64 as mmx;

use mmx::__m128i;
//use mmx::_mm_load_si128;
use mmx::_mm_cmpeq_epi8;
use mmx::_mm_loadu_si128;
use mmx::_mm_movemask_epi8;
//use mmx::_mm_prefetch;
//use mmx::_MM_HINT_T1;

#[inline(always)]
#[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
pub fn _memcmp_impl(a: &[u8], b: &[u8]) -> Ordering {
    /*
    #[cfg(target_feature = "avx")]
    let r = unsafe { _memcmp_avx(a, b) };
    #[cfg(target_feature = "sse2")]
    let r = unsafe { _memcmp_sse2(a, b) };
    #[cfg(not(any(target_feature = "avx", target_feature = "sse2")))]
    let r = _memcmp_basic(a, b);
    */
    let r = unsafe { _memcmp_sse2(a, b) };
    r
}

fn _memcmp_basic(a: &[u8], b: &[u8]) -> Ordering {
    basic::_memcmp_impl(a, b)
}

macro_rules! _unroll_one_cmp_16 {
    ($a_ptr:expr, $b_ptr:expr, $loop_size:expr, $idx:expr) => {{
        let aa_ptr = unsafe { $a_ptr.add($loop_size * $idx) };
        let bb_ptr = unsafe { $b_ptr.add($loop_size * $idx) };
        //
        let aac = unsafe { *(aa_ptr as *const u128) };
        let bbc = unsafe { *(bb_ptr as *const u128) };
        if aac != bbc {
            return _cmp_bytes_16(aa_ptr, bb_ptr);
        }
    }};
}

macro_rules! _unroll_one_cmp_8 {
    ($a_ptr:expr, $b_ptr:expr, $loop_size:expr, $idx:expr) => {{
        let aa_ptr = unsafe { $a_ptr.add($loop_size * $idx) };
        let bb_ptr = unsafe { $b_ptr.add($loop_size * $idx) };
        //
        let aac = unsafe { *(aa_ptr as *const u64) };
        let bbc = unsafe { *(bb_ptr as *const u64) };
        if aac != bbc {
            return _cmp_bytes_8(aa_ptr, bb_ptr);
        }
    }};
}

macro_rules! _unroll_one_cmp_4 {
    ($a_ptr:expr, $b_ptr:expr, $loop_size:expr, $idx:expr) => {{
        let aa_ptr = unsafe { $a_ptr.add($loop_size * $idx) };
        let bb_ptr = unsafe { $b_ptr.add($loop_size * $idx) };
        //
        let aac = unsafe { *(aa_ptr as *const u32) };
        let bbc = unsafe { *(bb_ptr as *const u32) };
        if aac != bbc {
            return _cmp_bytes_4(aa_ptr, bb_ptr);
        }
    }};
}

macro_rules! _unroll_one_cmp_2 {
    ($a_ptr:expr, $b_ptr:expr, $loop_size:expr, $idx:expr) => {{
        let aa_ptr = unsafe { $a_ptr.add($loop_size * $idx) };
        let bb_ptr = unsafe { $b_ptr.add($loop_size * $idx) };
        //
        let aac = unsafe { *(aa_ptr as *const u16) };
        let bbc = unsafe { *(bb_ptr as *const u16) };
        if aac != bbc {
            return _cmp_bytes_2(aa_ptr, bb_ptr);
        }
    }};
}

macro_rules! _unroll_one_cmp_1 {
    ($a_ptr:expr, $b_ptr:expr, $loop_size:expr, $idx:expr) => {{
        let aa_ptr = unsafe { $a_ptr.add($loop_size * $idx) };
        let bb_ptr = unsafe { $b_ptr.add($loop_size * $idx) };
        //
        let aac = unsafe { *(aa_ptr as *const u8) };
        let bbc = unsafe { *(bb_ptr as *const u8) };
        if aac != bbc {
            return aac.cmp(&bbc);
        }
    }};
}

#[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
#[target_feature(enable = "sse2")]
pub unsafe fn _memcmp_sse2(a: &[u8], b: &[u8]) -> Ordering {
    _memcmp_sse2_impl(a, b)
}

#[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
#[target_feature(enable = "avx")]
pub unsafe fn _memcmp_avx(a: &[u8], b: &[u8]) -> Ordering {
    _memcmp_sse2_impl(a, b)
}

#[inline(always)]
fn _memcmp_sse2_impl(a: &[u8], b: &[u8]) -> Ordering {
    //
    let a_len = a.len();
    let b_len = b.len();
    let min_len = a_len.min(b_len);
    let mut a_ptr = a.as_ptr();
    let mut b_ptr = b.as_ptr();
    let end_ptr = unsafe { b_ptr.add(min_len) };
    //
    //unsafe { _mm_prefetch(a_ptr as *const i8, _MM_HINT_T1) };
    //unsafe { _mm_prefetch(b_ptr as *const i8, _MM_HINT_T1) };
    //
    if min_len >= 16 {
        if min_len <= 32 {
            let a0_ptr = a_ptr;
            let b0_ptr = b_ptr;
            let a1_ptr = unsafe { a_ptr.add(min_len - 16) };
            let b1_ptr = unsafe { b_ptr.add(min_len - 16) };
            //
            let aa0_ptr = a0_ptr as *mut __m128i;
            let bb0_ptr = b0_ptr as *mut __m128i;
            let aa1_ptr = a1_ptr as *mut __m128i;
            let bb1_ptr = b1_ptr as *mut __m128i;
            //
            let mm_a0 = unsafe { _mm_loadu_si128(aa0_ptr) };
            let mm_b0 = unsafe { _mm_loadu_si128(bb0_ptr) };
            let mm_eq0 = unsafe { _mm_cmpeq_epi8(mm_a0, mm_b0) };
            let mask0 = unsafe { _mm_movemask_epi8(mm_eq0) };
            //
            if mask0 != 0xffff {
                return _cmp_bytes_16_mask(mask0, a0_ptr, b0_ptr);
            }
            //
            if min_len > 16 {
                let mm_a1 = unsafe { _mm_loadu_si128(aa1_ptr) };
                let mm_b1 = unsafe { _mm_loadu_si128(bb1_ptr) };
                let mm_eq1 = unsafe { _mm_cmpeq_epi8(mm_a1, mm_b1) };
                let mask1 = unsafe { _mm_movemask_epi8(mm_eq1) };
                //
                if mask1 != 0xffff {
                    return _cmp_bytes_16_mask(mask1, a1_ptr, b1_ptr);
                }
            }
            return a_len.cmp(&b_len);
        }
        {
            let unroll = 4;
            let loop_size = 16;
            let end_ptr_16_4 = unsafe { end_ptr.sub(loop_size * unroll) };
            while b_ptr <= end_ptr_16_4 {
                let r = _eq_unroll_16x4(a_ptr, b_ptr, loop_size);
                if r != Ordering::Equal {
                    return r;
                }
                //
                a_ptr = unsafe { a_ptr.add(loop_size * unroll) };
                b_ptr = unsafe { b_ptr.add(loop_size * unroll) };
            }
        }
        {
            let unroll = 2;
            let loop_size = 16;
            let end_ptr_16_2 = unsafe { end_ptr.sub(loop_size * unroll) };
            while b_ptr <= end_ptr_16_2 {
                let r = _eq_unroll_16x2(a_ptr, b_ptr, loop_size);
                if r != Ordering::Equal {
                    return r;
                }
                //
                a_ptr = unsafe { a_ptr.add(loop_size * unroll) };
                b_ptr = unsafe { b_ptr.add(loop_size * unroll) };
            }
        }
        /*
         */
        {
            let loop_size = 16;
            let end_ptr_16 = unsafe { end_ptr.sub(loop_size) };
            while b_ptr <= end_ptr_16 {
                let aa0_ptr = a_ptr as *mut __m128i;
                let bb0_ptr = b_ptr as *mut __m128i;
                //
                let mm_a0 = unsafe { _mm_loadu_si128(aa0_ptr) };
                let mm_b0 = unsafe { _mm_loadu_si128(bb0_ptr) };
                let mm_eq0 = unsafe { _mm_cmpeq_epi8(mm_a0, mm_b0) };
                let mask0 = unsafe { _mm_movemask_epi8(mm_eq0) };
                //
                if mask0 != 0xffff {
                    return _cmp_bytes_16_mask(mask0, a_ptr, b_ptr);
                }
                //
                a_ptr = unsafe { a_ptr.add(loop_size) };
                b_ptr = unsafe { b_ptr.add(loop_size) };
            }
        }
    }
    // a rest data is a max: 15 bytes.
    {
        let loop_size = 8;
        let end_ptr_8 = unsafe { end_ptr.sub(loop_size) };
        if b_ptr <= end_ptr_8 {
            _unroll_one_cmp_8!(a_ptr, b_ptr, loop_size, 0);
            //
            a_ptr = unsafe { a_ptr.add(loop_size) };
            b_ptr = unsafe { b_ptr.add(loop_size) };
        }
    }
    {
        let loop_size = 4;
        let end_ptr_4 = unsafe { end_ptr.sub(loop_size) };
        if b_ptr <= end_ptr_4 {
            _unroll_one_cmp_4!(a_ptr, b_ptr, loop_size, 0);
            //
            a_ptr = unsafe { a_ptr.add(loop_size) };
            b_ptr = unsafe { b_ptr.add(loop_size) };
        }
    }
    {
        let loop_size = 2;
        let end_ptr_2 = unsafe { end_ptr.sub(loop_size) };
        if b_ptr <= end_ptr_2 {
            _unroll_one_cmp_2!(a_ptr, b_ptr, loop_size, 0);
            //
            a_ptr = unsafe { a_ptr.add(loop_size) };
            b_ptr = unsafe { b_ptr.add(loop_size) };
        }
    }
    {
        let loop_size = 1;
        let end_ptr_1 = unsafe { end_ptr.sub(loop_size) };
        if b_ptr <= end_ptr_1 {
            _unroll_one_cmp_1!(a_ptr, b_ptr, loop_size, 0);
        }
    }
    //
    a_len.cmp(&b_len)
}

#[inline(always)]
fn _cmp_bytes_16_mask(mask: i32, a_ptr: *const u8, b_ptr: *const u8) -> Ordering {
    if mask != 0xffff {
        let bits = !mask;
        let pos = bits.trailing_zeros() as usize;
        let aa_ptr = unsafe { a_ptr.add(pos) };
        let bb_ptr = unsafe { b_ptr.add(pos) };
        let aac = unsafe { *aa_ptr };
        let bbc = unsafe { *bb_ptr };
        return aac.cmp(&bbc);
    } else {
        Ordering::Equal
    }
}

#[inline(always)]
fn _cmp_bytes_16(a_ptr: *const u8, b_ptr: *const u8) -> Ordering {
    let aa_ptr = a_ptr as *const u128;
    let bb_ptr = b_ptr as *const u128;
    let aac = unsafe { *aa_ptr };
    let bbc = unsafe { *bb_ptr };
    let bits = aac ^ bbc;
    if bits != 0 {
        let pos = (bits.trailing_zeros() / 8) as usize;
        let aa_ptr = unsafe { a_ptr.add(pos) };
        let bb_ptr = unsafe { b_ptr.add(pos) };
        let aac = unsafe { *aa_ptr };
        let bbc = unsafe { *bb_ptr };
        return aac.cmp(&bbc);
    } else {
        Ordering::Equal
    }
}

#[inline(always)]
fn _cmp_bytes_8(a_ptr: *const u8, b_ptr: *const u8) -> Ordering {
    let aa_ptr = a_ptr as *const u64;
    let bb_ptr = b_ptr as *const u64;
    let aac = unsafe { *aa_ptr };
    let bbc = unsafe { *bb_ptr };
    let bits = aac ^ bbc;
    if bits != 0 {
        let pos = (bits.trailing_zeros() / 8) as usize;
        let aa_ptr = unsafe { a_ptr.add(pos) };
        let bb_ptr = unsafe { b_ptr.add(pos) };
        let aac = unsafe { *aa_ptr };
        let bbc = unsafe { *bb_ptr };
        return aac.cmp(&bbc);
    } else {
        Ordering::Equal
    }
}

#[inline(always)]
fn _cmp_bytes_4(a_ptr: *const u8, b_ptr: *const u8) -> Ordering {
    let aa_ptr = a_ptr as *const u32;
    let bb_ptr = b_ptr as *const u32;
    let aac = unsafe { *aa_ptr };
    let bbc = unsafe { *bb_ptr };
    let bits = aac ^ bbc;
    if bits != 0 {
        let pos = (bits.trailing_zeros() / 8) as usize;
        let aa_ptr = unsafe { a_ptr.add(pos) };
        let bb_ptr = unsafe { b_ptr.add(pos) };
        let aac = unsafe { *aa_ptr };
        let bbc = unsafe { *bb_ptr };
        return aac.cmp(&bbc);
    } else {
        Ordering::Equal
    }
}

#[inline(always)]
fn _cmp_bytes_2(a_ptr: *const u8, b_ptr: *const u8) -> Ordering {
    let aa_ptr = a_ptr as *const u16;
    let bb_ptr = b_ptr as *const u16;
    let aac = unsafe { *aa_ptr };
    let bbc = unsafe { *bb_ptr };
    let bits = aac ^ bbc;
    if bits != 0 {
        let pos = (bits.trailing_zeros() / 8) as usize;
        let aa_ptr = unsafe { a_ptr.add(pos) };
        let bb_ptr = unsafe { b_ptr.add(pos) };
        let aac = unsafe { *aa_ptr };
        let bbc = unsafe { *bb_ptr };
        return aac.cmp(&bbc);
    } else {
        Ordering::Equal
    }
}

#[inline(always)]
fn _eq_unroll_16x8(a_ptr: *const u8, b_ptr: *const u8, loop_size: usize) -> Ordering {
    let aa0_ptr = unsafe { a_ptr.add(loop_size * 0) } as *mut __m128i;
    let bb0_ptr = unsafe { b_ptr.add(loop_size * 0) } as *mut __m128i;
    let aa1_ptr = unsafe { a_ptr.add(loop_size * 1) } as *mut __m128i;
    let bb1_ptr = unsafe { b_ptr.add(loop_size * 1) } as *mut __m128i;
    let aa2_ptr = unsafe { a_ptr.add(loop_size * 2) } as *mut __m128i;
    let bb2_ptr = unsafe { b_ptr.add(loop_size * 2) } as *mut __m128i;
    let aa3_ptr = unsafe { a_ptr.add(loop_size * 3) } as *mut __m128i;
    let bb3_ptr = unsafe { b_ptr.add(loop_size * 3) } as *mut __m128i;
    let aa4_ptr = unsafe { a_ptr.add(loop_size * 4) } as *mut __m128i;
    let bb4_ptr = unsafe { b_ptr.add(loop_size * 4) } as *mut __m128i;
    let aa5_ptr = unsafe { a_ptr.add(loop_size * 5) } as *mut __m128i;
    let bb5_ptr = unsafe { b_ptr.add(loop_size * 5) } as *mut __m128i;
    let aa6_ptr = unsafe { a_ptr.add(loop_size * 6) } as *mut __m128i;
    let bb6_ptr = unsafe { b_ptr.add(loop_size * 6) } as *mut __m128i;
    let aa7_ptr = unsafe { a_ptr.add(loop_size * 7) } as *mut __m128i;
    let bb7_ptr = unsafe { b_ptr.add(loop_size * 7) } as *mut __m128i;
    //
    let mm_a0 = unsafe { _mm_loadu_si128(aa0_ptr) };
    let mm_a1 = unsafe { _mm_loadu_si128(aa1_ptr) };
    let mm_a2 = unsafe { _mm_loadu_si128(aa2_ptr) };
    let mm_a3 = unsafe { _mm_loadu_si128(aa3_ptr) };
    let mm_a4 = unsafe { _mm_loadu_si128(aa4_ptr) };
    let mm_a5 = unsafe { _mm_loadu_si128(aa5_ptr) };
    let mm_a6 = unsafe { _mm_loadu_si128(aa6_ptr) };
    let mm_a7 = unsafe { _mm_loadu_si128(aa7_ptr) };
    let mm_b0 = unsafe { _mm_loadu_si128(bb0_ptr) };
    let mm_b1 = unsafe { _mm_loadu_si128(bb1_ptr) };
    let mm_b2 = unsafe { _mm_loadu_si128(bb2_ptr) };
    let mm_b3 = unsafe { _mm_loadu_si128(bb3_ptr) };
    let mm_b4 = unsafe { _mm_loadu_si128(bb4_ptr) };
    let mm_b5 = unsafe { _mm_loadu_si128(bb5_ptr) };
    let mm_b6 = unsafe { _mm_loadu_si128(bb6_ptr) };
    let mm_b7 = unsafe { _mm_loadu_si128(bb7_ptr) };
    //
    let mm_eq0 = unsafe { _mm_cmpeq_epi8(mm_a0, mm_b0) };
    let mask0 = unsafe { _mm_movemask_epi8(mm_eq0) };
    let mm_eq1 = unsafe { _mm_cmpeq_epi8(mm_a1, mm_b1) };
    let mask1 = unsafe { _mm_movemask_epi8(mm_eq1) };
    let mm_eq2 = unsafe { _mm_cmpeq_epi8(mm_a2, mm_b2) };
    let mask2 = unsafe { _mm_movemask_epi8(mm_eq2) };
    let mm_eq3 = unsafe { _mm_cmpeq_epi8(mm_a3, mm_b3) };
    let mask3 = unsafe { _mm_movemask_epi8(mm_eq3) };
    let mm_eq4 = unsafe { _mm_cmpeq_epi8(mm_a4, mm_b4) };
    let mask4 = unsafe { _mm_movemask_epi8(mm_eq4) };
    let mm_eq5 = unsafe { _mm_cmpeq_epi8(mm_a5, mm_b5) };
    let mask5 = unsafe { _mm_movemask_epi8(mm_eq5) };
    let mm_eq6 = unsafe { _mm_cmpeq_epi8(mm_a6, mm_b6) };
    let mask6 = unsafe { _mm_movemask_epi8(mm_eq6) };
    let mm_eq7 = unsafe { _mm_cmpeq_epi8(mm_a7, mm_b7) };
    let mask7 = unsafe { _mm_movemask_epi8(mm_eq7) };
    //
    if mask0 != 0xffff {
        return _cmp_bytes_16_mask(mask0, aa0_ptr as *const u8, bb0_ptr as *const u8);
    }
    if mask1 != 0xffff {
        return _cmp_bytes_16_mask(mask1, aa1_ptr as *const u8, bb1_ptr as *const u8);
    }
    if mask2 != 0xffff {
        return _cmp_bytes_16_mask(mask2, aa2_ptr as *const u8, bb2_ptr as *const u8);
    }
    if mask3 != 0xffff {
        return _cmp_bytes_16_mask(mask3, aa3_ptr as *const u8, bb3_ptr as *const u8);
    }
    if mask4 != 0xffff {
        return _cmp_bytes_16_mask(mask4, aa4_ptr as *const u8, bb4_ptr as *const u8);
    }
    if mask5 != 0xffff {
        return _cmp_bytes_16_mask(mask5, aa5_ptr as *const u8, bb5_ptr as *const u8);
    }
    if mask6 != 0xffff {
        return _cmp_bytes_16_mask(mask6, aa6_ptr as *const u8, bb6_ptr as *const u8);
    }
    if mask7 != 0xffff {
        return _cmp_bytes_16_mask(mask7, aa7_ptr as *const u8, bb7_ptr as *const u8);
    }
    //
    Ordering::Equal
}

#[inline(always)]
fn _eq_unroll_16x4(a_ptr: *const u8, b_ptr: *const u8, loop_size: usize) -> Ordering {
    let aa0_ptr = unsafe { a_ptr.add(loop_size * 0) } as *mut __m128i;
    let bb0_ptr = unsafe { b_ptr.add(loop_size * 0) } as *mut __m128i;
    let aa1_ptr = unsafe { a_ptr.add(loop_size * 1) } as *mut __m128i;
    let bb1_ptr = unsafe { b_ptr.add(loop_size * 1) } as *mut __m128i;
    let aa2_ptr = unsafe { a_ptr.add(loop_size * 2) } as *mut __m128i;
    let bb2_ptr = unsafe { b_ptr.add(loop_size * 2) } as *mut __m128i;
    let aa3_ptr = unsafe { a_ptr.add(loop_size * 3) } as *mut __m128i;
    let bb3_ptr = unsafe { b_ptr.add(loop_size * 3) } as *mut __m128i;
    //
    let mm_a0 = unsafe { _mm_loadu_si128(aa0_ptr) };
    let mm_a1 = unsafe { _mm_loadu_si128(aa1_ptr) };
    let mm_a2 = unsafe { _mm_loadu_si128(aa2_ptr) };
    let mm_a3 = unsafe { _mm_loadu_si128(aa3_ptr) };
    let mm_b0 = unsafe { _mm_loadu_si128(bb0_ptr) };
    let mm_b1 = unsafe { _mm_loadu_si128(bb1_ptr) };
    let mm_b2 = unsafe { _mm_loadu_si128(bb2_ptr) };
    let mm_b3 = unsafe { _mm_loadu_si128(bb3_ptr) };
    //
    let mm_eq0 = unsafe { _mm_cmpeq_epi8(mm_a0, mm_b0) };
    let mask0 = unsafe { _mm_movemask_epi8(mm_eq0) };
    let mm_eq1 = unsafe { _mm_cmpeq_epi8(mm_a1, mm_b1) };
    let mask1 = unsafe { _mm_movemask_epi8(mm_eq1) };
    let mm_eq2 = unsafe { _mm_cmpeq_epi8(mm_a2, mm_b2) };
    let mask2 = unsafe { _mm_movemask_epi8(mm_eq2) };
    let mm_eq3 = unsafe { _mm_cmpeq_epi8(mm_a3, mm_b3) };
    let mask3 = unsafe { _mm_movemask_epi8(mm_eq3) };
    //
    if mask0 != 0xffff {
        return _cmp_bytes_16_mask(mask0, aa0_ptr as *const u8, bb0_ptr as *const u8);
    }
    if mask1 != 0xffff {
        return _cmp_bytes_16_mask(mask1, aa1_ptr as *const u8, bb1_ptr as *const u8);
    }
    if mask2 != 0xffff {
        return _cmp_bytes_16_mask(mask2, aa2_ptr as *const u8, bb2_ptr as *const u8);
    }
    if mask3 != 0xffff {
        return _cmp_bytes_16_mask(mask3, aa3_ptr as *const u8, bb3_ptr as *const u8);
    }
    //
    Ordering::Equal
}

#[inline(always)]
fn _eq_unroll_16x2(a_ptr: *const u8, b_ptr: *const u8, loop_size: usize) -> Ordering {
    let aa0_ptr = unsafe { a_ptr.add(loop_size * 0) } as *mut __m128i;
    let bb0_ptr = unsafe { b_ptr.add(loop_size * 0) } as *mut __m128i;
    let aa1_ptr = unsafe { a_ptr.add(loop_size * 1) } as *mut __m128i;
    let bb1_ptr = unsafe { b_ptr.add(loop_size * 1) } as *mut __m128i;
    //
    let mm_a0 = unsafe { _mm_loadu_si128(aa0_ptr) };
    let mm_a1 = unsafe { _mm_loadu_si128(aa1_ptr) };
    let mm_b0 = unsafe { _mm_loadu_si128(bb0_ptr) };
    let mm_b1 = unsafe { _mm_loadu_si128(bb1_ptr) };
    //
    let mm_eq0 = unsafe { _mm_cmpeq_epi8(mm_a0, mm_b0) };
    let mask0 = unsafe { _mm_movemask_epi8(mm_eq0) };
    let mm_eq1 = unsafe { _mm_cmpeq_epi8(mm_a1, mm_b1) };
    let mask1 = unsafe { _mm_movemask_epi8(mm_eq1) };
    //
    if mask0 != 0xffff {
        return _cmp_bytes_16_mask(mask0, aa0_ptr as *const u8, bb0_ptr as *const u8);
    }
    if mask1 != 0xffff {
        return _cmp_bytes_16_mask(mask1, aa1_ptr as *const u8, bb1_ptr as *const u8);
    }
    //
    Ordering::Equal
}

//
// Why is the next routine slow ??? <2021-05-17>
/*
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
