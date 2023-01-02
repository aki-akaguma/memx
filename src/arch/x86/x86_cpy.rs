use crate::mem as basic;
use crate::RangeError;

#[cfg(target_arch = "x86")]
use core::arch::x86 as mmx;
#[cfg(target_arch = "x86_64")]
use core::arch::x86_64 as mmx;

use mmx::__m128i;
use mmx::_mm_loadu_si128;
use mmx::_mm_store_si128;

/*
use mmx::__m256i;
use mmx::_mm256_store_si256;
use mmx::_mm256_storeu_si256;
use mmx::_mm256_loadu_si256;
*/

#[inline(always)]
#[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
pub fn _memcpy_impl(dst: &mut [u8], src: &[u8]) -> Result<(), RangeError> {
    #[cfg(target_feature = "avx")]
    let r = unsafe { _memcpy_avx(dst, src) };
    #[cfg(all(target_feature = "sse2", not(target_feature = "avx")))]
    let r = unsafe { _memcpy_sse2(dst, src) };
    #[cfg(not(any(target_feature = "sse2", target_feature = "avx")))]
    let r = _memcpy_basic(dst, src);
    r
}

fn _memcpy_basic(dst: &mut [u8], src: &[u8]) -> Result<(), RangeError> {
    basic::_memcpy_impl(dst, src)
}

#[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
#[target_feature(enable = "sse2")]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn _memcpy_sse2(dst: &mut [u8], src: &[u8]) -> Result<(), RangeError> {
    _memcpy_sse2_impl(dst, src)
}

#[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
#[target_feature(enable = "avx")]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn _memcpy_avx(dst: &mut [u8], src: &[u8]) -> Result<(), RangeError> {
    _memcpy_sse2_impl(dst, src)
}

#[inline(always)]
fn _memcpy_sse2_impl(dst: &mut [u8], src: &[u8]) -> Result<(), RangeError> {
    let dst_len = dst.len();
    let src_len = src.len();
    if dst_len < src_len {
        return Err(RangeError);
    }
    if src_len == 0 {
        return Ok(());
    }
    let mut a_ptr = dst.as_mut_ptr();
    let mut b_ptr = src.as_ptr();
    let end_ptr = unsafe { b_ptr.add(src_len) };
    //
    if src_len < 64 {
        return _cpy_small(a_ptr, b_ptr, end_ptr);
    }
    {
        let (aa_ptr, bb_ptr) = if src_len < 128 {
            _cpy_medium(a_ptr, b_ptr, src_len, end_ptr)
        } else {
            _cpy_large(a_ptr, b_ptr, src_len, end_ptr)
        };
        a_ptr = aa_ptr;
        b_ptr = bb_ptr;
    }
    // the remaining data is the max: 15 bytes.
    basic::_memcpy_remaining_15_bytes_impl(a_ptr, b_ptr, end_ptr)
}

//#[inline(never)]
#[inline(always)]
fn _cpy_small(dst_ptr: *mut u8, src_ptr: *const u8, end_ptr: *const u8) -> Result<(), RangeError> {
    let mut a_ptr = dst_ptr;
    let mut b_ptr = src_ptr;
    /*
    {
        let loop_size = 8;
        if unsafe { end_ptr.offset_from(b_ptr) } >= (loop_size) as isize {
            let end_ptr_8 = unsafe { end_ptr.sub(loop_size) };
            while b_ptr <= end_ptr_8 {
                let aa_ptr = a_ptr as *mut u64;
                let bb_ptr = b_ptr as *const u64;
                unsafe { *aa_ptr = *bb_ptr };
                //
                a_ptr = unsafe { a_ptr.add(loop_size) };
                b_ptr = unsafe { b_ptr.add(loop_size) };
            }
        }
    }
    */
    {
        let loop_size = 1;
        if unsafe { end_ptr.offset_from(b_ptr) } >= (loop_size) as isize {
            let end_ptr_8 = unsafe { end_ptr.sub(loop_size) };
            while b_ptr <= end_ptr_8 {
                let aa_ptr = a_ptr as *mut u8;
                let bb_ptr = b_ptr as *const u8;
                unsafe { *aa_ptr = *bb_ptr };
                //
                a_ptr = unsafe { a_ptr.add(loop_size) };
                b_ptr = unsafe { b_ptr.add(loop_size) };
            }
        }
    }
    // the remaining data is the max: 7 bytes.
    basic::_memcpy_remaining_7_bytes_impl(a_ptr, b_ptr, end_ptr)
}

//#[inline(never)]
#[inline(always)]
fn _cpy_medium(
    dst_ptr: *mut u8,
    src_ptr: *const u8,
    src_len: usize,
    end_ptr: *const u8,
) -> (*mut u8, *const u8) {
    let mut a_ptr = dst_ptr;
    let mut b_ptr = src_ptr;
    let remaining_len = {
        let remaining_align = 0x10_usize - ((a_ptr as usize) & 0x0F_usize);
        if remaining_align < 16 {
            _cpy_tiny_0_15(a_ptr, b_ptr, remaining_align);
            //
            a_ptr = unsafe { a_ptr.add(remaining_align) };
            b_ptr = unsafe { b_ptr.add(remaining_align) };
            src_len - remaining_align
        } else {
            src_len
        }
    };
    if remaining_len >= 16 {
        if remaining_len >= 16 * 6 {
            let unroll = 6;
            let loop_size = 16;
            let end_ptr_16_6 = unsafe { end_ptr.sub(loop_size * unroll) };
            while b_ptr <= end_ptr_16_6 {
                _cpy_unroll_16x6(a_ptr, b_ptr, loop_size);
                //
                a_ptr = unsafe { a_ptr.add(loop_size * unroll) };
                b_ptr = unsafe { b_ptr.add(loop_size * unroll) };
            }
        }
        if remaining_len >= 16 * 4 {
            let unroll = 4;
            let loop_size = 16;
            let end_ptr_16_4 = unsafe { end_ptr.sub(loop_size * unroll) };
            while b_ptr <= end_ptr_16_4 {
                _cpy_unroll_16x4(a_ptr, b_ptr, loop_size);
                //
                a_ptr = unsafe { a_ptr.add(loop_size * unroll) };
                b_ptr = unsafe { b_ptr.add(loop_size * unroll) };
            }
        }
        /*
        if remaining_len >= 16 * 2 {
            let unroll = 2;
            let loop_size = 16;
            let end_ptr_16_2 = unsafe { end_ptr.sub(loop_size * unroll) };
            while b_ptr <= end_ptr_16_2 {
                _cpy_unroll_16x2(a_ptr, b_ptr, loop_size);
                //
                a_ptr = unsafe { a_ptr.add(loop_size * unroll) };
                b_ptr = unsafe { b_ptr.add(loop_size * unroll) };
            }
        }
        */
        if remaining_len >= 16 {
            let loop_size = 16;
            let end_ptr_16 = unsafe { end_ptr.sub(loop_size) };
            while b_ptr <= end_ptr_16 {
                let aa_ptr = a_ptr as *mut __m128i;
                let bb_ptr = b_ptr as *mut __m128i;
                let mm_b = unsafe { _mm_loadu_si128(bb_ptr) };
                unsafe { _mm_store_si128(aa_ptr, mm_b) };
                //
                a_ptr = unsafe { a_ptr.add(loop_size) };
                b_ptr = unsafe { b_ptr.add(loop_size) };
            }
        }
    }
    (a_ptr, b_ptr)
}

#[inline(never)]
//#[inline(always)]
fn _cpy_large(
    dst_ptr: *mut u8,
    src_ptr: *const u8,
    src_len: usize,
    end_ptr: *const u8,
) -> (*mut u8, *const u8) {
    let mut a_ptr = dst_ptr;
    let mut b_ptr = src_ptr;
    let remaining_len = {
        let remaining_align = 0x10_usize - ((a_ptr as usize) & 0x0F_usize);
        if remaining_align < 16 {
            _cpy_tiny_0_15(a_ptr, b_ptr, remaining_align);
            //
            a_ptr = unsafe { a_ptr.add(remaining_align) };
            b_ptr = unsafe { b_ptr.add(remaining_align) };
            src_len - remaining_align
        } else {
            src_len
        }
    };
    if remaining_len >= 16 {
        if remaining_len >= 16 * 8 {
            let unroll = 8;
            let loop_size = 16;
            let end_ptr_16_8 = unsafe { end_ptr.sub(loop_size * unroll) };
            while b_ptr <= end_ptr_16_8 {
                _cpy_unroll_16x8(a_ptr, b_ptr, loop_size);
                //
                a_ptr = unsafe { a_ptr.add(loop_size * unroll) };
                b_ptr = unsafe { b_ptr.add(loop_size * unroll) };
            }
        }
        if remaining_len >= 16 * 4 {
            let unroll = 4;
            let loop_size = 16;
            let end_ptr_16_4 = unsafe { end_ptr.sub(loop_size * unroll) };
            while b_ptr <= end_ptr_16_4 {
                _cpy_unroll_16x4(a_ptr, b_ptr, loop_size);
                //
                a_ptr = unsafe { a_ptr.add(loop_size * unroll) };
                b_ptr = unsafe { b_ptr.add(loop_size * unroll) };
            }
        }
        if remaining_len >= 16 {
            let loop_size = 16;
            let end_ptr_16 = unsafe { end_ptr.sub(loop_size) };
            while b_ptr <= end_ptr_16 {
                let aa_ptr = a_ptr as *mut __m128i;
                let bb_ptr = b_ptr as *mut __m128i;
                let mm_b = unsafe { _mm_loadu_si128(bb_ptr) };
                unsafe { _mm_store_si128(aa_ptr, mm_b) };
                //
                a_ptr = unsafe { a_ptr.add(loop_size) };
                b_ptr = unsafe { b_ptr.add(loop_size) };
            }
        }
    }
    (a_ptr, b_ptr)
}

#[inline(always)]
fn _cpy_unroll_16x8(a_ptr: *mut u8, b_ptr: *const u8, loop_size: usize) {
    let aa0_ptr = a_ptr as *mut __m128i;
    let bb0_ptr = b_ptr as *mut __m128i;
    let aa1_ptr = unsafe { a_ptr.add(loop_size) } as *mut __m128i;
    let bb1_ptr = unsafe { b_ptr.add(loop_size) } as *mut __m128i;
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
    let mm_b0 = unsafe { _mm_loadu_si128(bb0_ptr) };
    let mm_b1 = unsafe { _mm_loadu_si128(bb1_ptr) };
    let mm_b2 = unsafe { _mm_loadu_si128(bb2_ptr) };
    let mm_b3 = unsafe { _mm_loadu_si128(bb3_ptr) };
    let mm_b4 = unsafe { _mm_loadu_si128(bb4_ptr) };
    let mm_b5 = unsafe { _mm_loadu_si128(bb5_ptr) };
    let mm_b6 = unsafe { _mm_loadu_si128(bb6_ptr) };
    let mm_b7 = unsafe { _mm_loadu_si128(bb7_ptr) };
    //
    unsafe { _mm_store_si128(aa0_ptr, mm_b0) };
    unsafe { _mm_store_si128(aa1_ptr, mm_b1) };
    unsafe { _mm_store_si128(aa2_ptr, mm_b2) };
    unsafe { _mm_store_si128(aa3_ptr, mm_b3) };
    unsafe { _mm_store_si128(aa4_ptr, mm_b4) };
    unsafe { _mm_store_si128(aa5_ptr, mm_b5) };
    unsafe { _mm_store_si128(aa6_ptr, mm_b6) };
    unsafe { _mm_store_si128(aa7_ptr, mm_b7) };
}

#[inline(always)]
fn _cpy_unroll_16x6(a_ptr: *mut u8, b_ptr: *const u8, loop_size: usize) {
    let aa0_ptr = a_ptr as *mut __m128i;
    let bb0_ptr = b_ptr as *mut __m128i;
    let aa1_ptr = unsafe { a_ptr.add(loop_size) } as *mut __m128i;
    let bb1_ptr = unsafe { b_ptr.add(loop_size) } as *mut __m128i;
    let aa2_ptr = unsafe { a_ptr.add(loop_size * 2) } as *mut __m128i;
    let bb2_ptr = unsafe { b_ptr.add(loop_size * 2) } as *mut __m128i;
    let aa3_ptr = unsafe { a_ptr.add(loop_size * 3) } as *mut __m128i;
    let bb3_ptr = unsafe { b_ptr.add(loop_size * 3) } as *mut __m128i;
    let aa4_ptr = unsafe { a_ptr.add(loop_size * 4) } as *mut __m128i;
    let bb4_ptr = unsafe { b_ptr.add(loop_size * 4) } as *mut __m128i;
    let aa5_ptr = unsafe { a_ptr.add(loop_size * 5) } as *mut __m128i;
    let bb5_ptr = unsafe { b_ptr.add(loop_size * 5) } as *mut __m128i;
    //
    let mm_b0 = unsafe { _mm_loadu_si128(bb0_ptr) };
    let mm_b1 = unsafe { _mm_loadu_si128(bb1_ptr) };
    let mm_b2 = unsafe { _mm_loadu_si128(bb2_ptr) };
    let mm_b3 = unsafe { _mm_loadu_si128(bb3_ptr) };
    let mm_b4 = unsafe { _mm_loadu_si128(bb4_ptr) };
    let mm_b5 = unsafe { _mm_loadu_si128(bb5_ptr) };
    //
    unsafe { _mm_store_si128(aa0_ptr, mm_b0) };
    unsafe { _mm_store_si128(aa1_ptr, mm_b1) };
    unsafe { _mm_store_si128(aa2_ptr, mm_b2) };
    unsafe { _mm_store_si128(aa3_ptr, mm_b3) };
    unsafe { _mm_store_si128(aa4_ptr, mm_b4) };
    unsafe { _mm_store_si128(aa5_ptr, mm_b5) };
}

#[inline(always)]
fn _cpy_unroll_16x4(a_ptr: *mut u8, b_ptr: *const u8, loop_size: usize) {
    let aa0_ptr = a_ptr as *mut __m128i;
    let bb0_ptr = b_ptr as *mut __m128i;
    let aa1_ptr = unsafe { a_ptr.add(loop_size) } as *mut __m128i;
    let bb1_ptr = unsafe { b_ptr.add(loop_size) } as *mut __m128i;
    let aa2_ptr = unsafe { a_ptr.add(loop_size * 2) } as *mut __m128i;
    let bb2_ptr = unsafe { b_ptr.add(loop_size * 2) } as *mut __m128i;
    let aa3_ptr = unsafe { a_ptr.add(loop_size * 3) } as *mut __m128i;
    let bb3_ptr = unsafe { b_ptr.add(loop_size * 3) } as *mut __m128i;
    //
    let mm_b0 = unsafe { _mm_loadu_si128(bb0_ptr) };
    let mm_b1 = unsafe { _mm_loadu_si128(bb1_ptr) };
    let mm_b2 = unsafe { _mm_loadu_si128(bb2_ptr) };
    let mm_b3 = unsafe { _mm_loadu_si128(bb3_ptr) };
    //
    unsafe { _mm_store_si128(aa0_ptr, mm_b0) };
    unsafe { _mm_store_si128(aa1_ptr, mm_b1) };
    unsafe { _mm_store_si128(aa2_ptr, mm_b2) };
    unsafe { _mm_store_si128(aa3_ptr, mm_b3) };
}

#[inline(always)]
fn _cpy_unroll_16x2(a_ptr: *mut u8, b_ptr: *const u8, loop_size: usize) {
    let aa0_ptr = a_ptr as *mut __m128i;
    let bb0_ptr = b_ptr as *mut __m128i;
    let aa1_ptr = unsafe { a_ptr.add(loop_size) } as *mut __m128i;
    let bb1_ptr = unsafe { b_ptr.add(loop_size) } as *mut __m128i;
    //
    let mm_b0 = unsafe { _mm_loadu_si128(bb0_ptr) };
    let mm_b1 = unsafe { _mm_loadu_si128(bb1_ptr) };
    //
    unsafe { _mm_store_si128(aa0_ptr, mm_b0) };
    unsafe { _mm_store_si128(aa1_ptr, mm_b1) };
}

/**/
#[inline(always)]
fn _cpy_tiny_0_15(dst_ptr: *mut u8, src_ptr: *const u8, len: usize) {
    match len {
        0 => {}
        1 => _copy_1_bytes(dst_ptr, src_ptr),
        2 => _copy_2_bytes(dst_ptr, src_ptr),
        3 => {
            _copy_1_bytes(dst_ptr, src_ptr);
            _copy_2_bytes(unsafe { dst_ptr.add(1) }, unsafe { src_ptr.add(1) });
        }
        4 => _copy_4_bytes(dst_ptr, src_ptr),
        5 => {
            _copy_1_bytes(dst_ptr, src_ptr);
            _copy_4_bytes(unsafe { dst_ptr.add(1) }, unsafe { src_ptr.add(1) });
        }
        6 => {
            _copy_2_bytes(dst_ptr, src_ptr);
            _copy_4_bytes(unsafe { dst_ptr.add(2) }, unsafe { src_ptr.add(2) });
        }
        7 => {
            _copy_1_bytes(dst_ptr, src_ptr);
            _copy_2_bytes(unsafe { dst_ptr.add(1) }, unsafe { src_ptr.add(1) });
            _copy_4_bytes(unsafe { dst_ptr.add(1 + 2) }, unsafe { src_ptr.add(1 + 2) });
        }
        8 => _copy_8_bytes(dst_ptr, src_ptr),
        9 => {
            _copy_1_bytes(dst_ptr, src_ptr);
            _copy_8_bytes(unsafe { dst_ptr.add(1) }, unsafe { src_ptr.add(1) });
        }
        10 => {
            _copy_2_bytes(dst_ptr, src_ptr);
            _copy_8_bytes(unsafe { dst_ptr.add(2) }, unsafe { src_ptr.add(2) });
        }
        11 => {
            _copy_1_bytes(dst_ptr, src_ptr);
            _copy_2_bytes(unsafe { dst_ptr.add(1) }, unsafe { src_ptr.add(1) });
            _copy_8_bytes(unsafe { dst_ptr.add(1 + 2) }, unsafe { src_ptr.add(1 + 2) });
        }
        12 => {
            _copy_4_bytes(dst_ptr, src_ptr);
            _copy_8_bytes(unsafe { dst_ptr.add(4) }, unsafe { src_ptr.add(4) });
        }
        13 => {
            _copy_1_bytes(dst_ptr, src_ptr);
            _copy_4_bytes(unsafe { dst_ptr.add(1) }, unsafe { src_ptr.add(1) });
            _copy_8_bytes(unsafe { dst_ptr.add(1 + 4) }, unsafe { src_ptr.add(1 + 4) });
        }
        14 => {
            _copy_2_bytes(dst_ptr, src_ptr);
            _copy_4_bytes(unsafe { dst_ptr.add(2) }, unsafe { src_ptr.add(2) });
            _copy_8_bytes(unsafe { dst_ptr.add(2 + 4) }, unsafe { src_ptr.add(2 + 4) });
        }
        15 => {
            _copy_1_bytes(dst_ptr, src_ptr);
            _copy_2_bytes(unsafe { dst_ptr.add(1) }, unsafe { src_ptr.add(1) });
            _copy_4_bytes(unsafe { dst_ptr.add(1 + 2) }, unsafe { src_ptr.add(1 + 2) });
            _copy_8_bytes(unsafe { dst_ptr.add(1 + 2 + 4) }, unsafe {
                src_ptr.add(1 + 2 + 4)
            });
        }
        _ => {
            //assert_eq!(len, 0);
            unreachable!();
        }
    }
}
/**/
/*
#[inline(always)]
fn _cpy_tiny_0_15(dst_ptr: *mut u8, src_ptr: *const u8, len: usize) {
    match len {
        0 => {}
        1 => _copy_1_bytes(dst_ptr, src_ptr),
        2 => _copy_2_bytes(dst_ptr, src_ptr),
        3 => {
            _copy_2_bytes(dst_ptr, src_ptr);
            _copy_1_bytes(unsafe { dst_ptr.add(2) }, unsafe { src_ptr.add(2) });
        }
        4 => _copy_4_bytes(dst_ptr, src_ptr),
        5 => {
            _copy_4_bytes(dst_ptr, src_ptr);
            _copy_1_bytes(unsafe { dst_ptr.add(4) }, unsafe { src_ptr.add(4) });
        }
        6 => {
            _copy_4_bytes(dst_ptr, src_ptr);
            _copy_2_bytes(unsafe { dst_ptr.add(4) }, unsafe { src_ptr.add(4) });
        }
        7 => {
            _copy_4_bytes(dst_ptr, src_ptr);
            _copy_2_bytes(unsafe { dst_ptr.add(4) }, unsafe { src_ptr.add(4) });
            _copy_1_bytes(unsafe { dst_ptr.add(4 + 2) }, unsafe { src_ptr.add(4 + 2) });
        }
        8 => _copy_8_bytes(dst_ptr, src_ptr),
        9 => {
            _copy_8_bytes(dst_ptr, src_ptr);
            _copy_1_bytes(unsafe { dst_ptr.add(8) }, unsafe { src_ptr.add(8) });
        }
        10 => {
            _copy_8_bytes(dst_ptr, src_ptr);
            _copy_2_bytes(unsafe { dst_ptr.add(8) }, unsafe { src_ptr.add(8) });
        }
        11 => {
            _copy_8_bytes(dst_ptr, src_ptr);
            _copy_2_bytes(unsafe { dst_ptr.add(8) }, unsafe { src_ptr.add(8) });
            _copy_1_bytes(unsafe { dst_ptr.add(8 + 2) }, unsafe { src_ptr.add(8 + 2) });
        }
        12 => {
            _copy_8_bytes(dst_ptr, src_ptr);
            _copy_4_bytes(unsafe { dst_ptr.add(8) }, unsafe { src_ptr.add(8) });
        }
        13 => {
            _copy_1_bytes(dst_ptr, src_ptr);
            _copy_4_bytes(unsafe { dst_ptr.add(1) }, unsafe { src_ptr.add(1) });
            _copy_8_bytes(unsafe { dst_ptr.add(1 + 4) }, unsafe { src_ptr.add(1 + 4) });
        }
        14 => {
            _copy_8_bytes(dst_ptr, src_ptr);
            _copy_4_bytes(unsafe { dst_ptr.add(8) }, unsafe { src_ptr.add(8) });
            _copy_2_bytes(unsafe { dst_ptr.add(8 + 4) }, unsafe { src_ptr.add(8 + 4) });
        }
        15 => {
            _copy_8_bytes(dst_ptr, src_ptr);
            _copy_4_bytes(unsafe { dst_ptr.add(8) }, unsafe { src_ptr.add(8) });
            _copy_2_bytes(unsafe { dst_ptr.add(8 + 4) }, unsafe { src_ptr.add(8 + 4) });
            _copy_1_bytes(unsafe { dst_ptr.add(8 + 4 + 2) }, unsafe {
                src_ptr.add(8 + 4 + 2)
            });
        }
        _ => {
            //assert_eq!(len, 0);
            unreachable!();
        },
    }
}
*/

#[inline(always)]
fn _copy_8_bytes(dst_ptr: *mut u8, src_ptr: *const u8) {
    /* error on miri
    let a_ptr = dst_ptr as *mut u64;
    let b_ptr = src_ptr as *const u64;
    unsafe { *a_ptr = *b_ptr };
    */
    let a_ptr = dst_ptr as *mut u8;
    let b_ptr = src_ptr as *const u8;
    unsafe { *a_ptr = *b_ptr };
    let a_ptr = unsafe { a_ptr.add(1) };
    let b_ptr = unsafe { b_ptr.add(1) };
    unsafe { *a_ptr = *b_ptr };
    let a_ptr = unsafe { a_ptr.add(1) };
    let b_ptr = unsafe { b_ptr.add(1) };
    unsafe { *a_ptr = *b_ptr };
    let a_ptr = unsafe { a_ptr.add(1) };
    let b_ptr = unsafe { b_ptr.add(1) };
    unsafe { *a_ptr = *b_ptr };
    let a_ptr = unsafe { a_ptr.add(1) };
    let b_ptr = unsafe { b_ptr.add(1) };
    unsafe { *a_ptr = *b_ptr };
    let a_ptr = unsafe { a_ptr.add(1) };
    let b_ptr = unsafe { b_ptr.add(1) };
    unsafe { *a_ptr = *b_ptr };
    let a_ptr = unsafe { a_ptr.add(1) };
    let b_ptr = unsafe { b_ptr.add(1) };
    unsafe { *a_ptr = *b_ptr };
    let a_ptr = unsafe { a_ptr.add(1) };
    let b_ptr = unsafe { b_ptr.add(1) };
    unsafe { *a_ptr = *b_ptr };
}

#[inline(always)]
fn _copy_4_bytes(dst_ptr: *mut u8, src_ptr: *const u8) {
    /* error on miri
    let a_ptr = dst_ptr as *mut u32;
    let b_ptr = src_ptr as *const u32;
    unsafe { *a_ptr = *b_ptr };
    */
    /*
    let a_ptr = dst_ptr as *mut u16;
    let b_ptr = src_ptr as *const u16;
    unsafe { *a_ptr = *b_ptr };
    let a_ptr = unsafe { a_ptr.add(1) };
    let b_ptr = unsafe { b_ptr.add(1) };
    unsafe { *a_ptr = *b_ptr };
    */
    let a_ptr = dst_ptr as *mut u8;
    let b_ptr = src_ptr as *const u8;
    unsafe { *a_ptr = *b_ptr };
    let a_ptr = unsafe { a_ptr.add(1) };
    let b_ptr = unsafe { b_ptr.add(1) };
    unsafe { *a_ptr = *b_ptr };
    let a_ptr = unsafe { a_ptr.add(1) };
    let b_ptr = unsafe { b_ptr.add(1) };
    unsafe { *a_ptr = *b_ptr };
    let a_ptr = unsafe { a_ptr.add(1) };
    let b_ptr = unsafe { b_ptr.add(1) };
    unsafe { *a_ptr = *b_ptr };
}

#[inline(always)]
fn _copy_2_bytes(dst_ptr: *mut u8, src_ptr: *const u8) {
    /* error on miri
    let a_ptr = dst_ptr as *mut u16;
    let b_ptr = src_ptr as *const u16;
    unsafe { *a_ptr = *b_ptr };
     */
    let a_ptr = dst_ptr as *mut u8;
    let b_ptr = src_ptr as *const u8;
    unsafe { *a_ptr = *b_ptr };
    let a_ptr = unsafe { a_ptr.add(1) };
    let b_ptr = unsafe { b_ptr.add(1) };
    unsafe { *a_ptr = *b_ptr };
}

#[inline(always)]
fn _copy_1_bytes(dst_ptr: *mut u8, src_ptr: *const u8) {
    let a_ptr = dst_ptr as *mut u8;
    let b_ptr = src_ptr as *const u8;
    unsafe { *a_ptr = *b_ptr };
}
