use crate::mem as basic;

#[cfg(target_arch = "x86")]
use core::arch::x86 as mmx;
#[cfg(target_arch = "x86_64")]
use core::arch::x86_64 as mmx;

use mmx::__m128i;
use mmx::_mm_set1_epi8;
use mmx::_mm_store_si128;
use mmx::_mm_storeu_si128;

use mmx::__m256i;
use mmx::_mm256_set1_epi8;
use mmx::_mm256_store_si256;
use mmx::_mm256_storeu_si256;

use super::{cpuid_avx, cpuid_sse2};

#[inline(always)]
#[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
pub fn _memset_impl(buf: &mut [u8], c: u8) {
    // TODO: Replace with https://github.com/rust-lang/rfcs/pull/2725
    // after stabilization
    if cpuid_avx::get() {
        unsafe { _memset_avx(buf, c) };
    } else if cpuid_sse2::get() {
        unsafe { _memset_sse2(buf, c) };
    } else {
        _memset_basic(buf, c);
    }
    /*
    #[cfg(target_feature = "avx")]
    unsafe { _memset_avx(buf, c) };
    #[cfg(all(target_feature = "sse2", not(target_feature = "avx")))]
    unsafe { _memset_sse2(buf, c) };
    #[cfg(not(any(target_feature = "sse2", target_feature = "avx")))]
    _memset_basic(buf, c);
    */
}

fn _memset_basic(buf: &mut [u8], c: u8) {
    basic::_memset_impl(buf, c)
}

#[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
#[target_feature(enable = "sse2")]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn _memset_sse2(buf: &mut [u8], c: u8) {
    _memset_sse2_impl(buf, c)
}

#[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
#[target_feature(enable = "avx")]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn _memset_avx(buf: &mut [u8], c: u8) {
    _memset_avx_impl(buf, c)
}

#[inline(always)]
fn _memset_sse2_impl(buf: &mut [u8], c: u8) {
    let buf_len = buf.len();
    if buf_len == 0 {
        return;
    }
    //
    let mut a_ptr = buf.as_mut_ptr();
    let end_ptr = unsafe { a_ptr.add(buf_len) };
    //
    if buf_len >= 16 {
        let mcc: __m128i = unsafe { _mm_set1_epi8(c as i8) };
        {
            let remaining_align = 0x10_usize - ((a_ptr as usize) & 0x0F_usize);
            //
            let aa_ptr = a_ptr as *mut __m128i;
            unsafe { _mm_storeu_si128(aa_ptr, mcc) };
            //
            a_ptr = unsafe { a_ptr.add(remaining_align) };
        }
        {
            let unroll = 8;
            let loop_size = 16;
            let end_ptr_16_8 = unsafe { end_ptr.sub(loop_size * unroll) };
            while a_ptr <= end_ptr_16_8 {
                for i in 0..unroll {
                    let aa_ptr = unsafe { a_ptr.add(loop_size * i) } as *mut __m128i;
                    unsafe { _mm_store_si128(aa_ptr, mcc) };
                }
                a_ptr = unsafe { a_ptr.add(loop_size * unroll) };
            }
        }
        {
            let loop_size = 16;
            let end_ptr_8 = unsafe { end_ptr.sub(loop_size) };
            while a_ptr <= end_ptr_8 {
                let aa_ptr = a_ptr as *mut __m128i;
                unsafe { _mm_store_si128(aa_ptr, mcc) };
                a_ptr = unsafe { a_ptr.add(loop_size) };
            }
        }
    }
    let cc: u64 = c as u64 * 0x0101_0101_0101_0101_u64;
    // the remaining data is the max: 15 bytes.
    basic::_memset_remaining_15_bytes_impl(a_ptr, cc, end_ptr)
}

#[inline(always)]
fn _memset_avx_impl(buf: &mut [u8], c: u8) {
    let buf_len = buf.len();
    if buf_len == 0 {
        return;
    }
    //
    let mut a_ptr = buf.as_mut_ptr();
    let end_ptr = unsafe { a_ptr.add(buf_len) };
    //
    if buf_len >= 32 {
        let mcc: __m256i = unsafe { _mm256_set1_epi8(c as i8) };
        {
            let remaining_align = 0x20_usize - ((a_ptr as usize) & 0x1F_usize);
            //
            let aa_ptr = a_ptr as *mut __m256i;
            unsafe { _mm256_storeu_si256(aa_ptr, mcc) };
            //
            a_ptr = unsafe { a_ptr.add(remaining_align) };
        }
        {
            let unroll = 8;
            let loop_size = 32;
            let end_ptr_32_8 = unsafe { end_ptr.sub(loop_size * unroll) };
            while a_ptr <= end_ptr_32_8 {
                for i in 0..unroll {
                    let aa_ptr = unsafe { a_ptr.add(loop_size * i) } as *mut __m256i;
                    unsafe { _mm256_store_si256(aa_ptr, mcc) };
                }
                a_ptr = unsafe { a_ptr.add(loop_size * unroll) };
            }
        }
        {
            let loop_size = 32;
            let end_ptr_16 = unsafe { end_ptr.sub(loop_size) };
            while a_ptr <= end_ptr_16 {
                let aa_ptr = a_ptr as *mut __m256i;
                unsafe { _mm256_store_si256(aa_ptr, mcc) };
                a_ptr = unsafe { a_ptr.add(loop_size) };
            }
        }
    }
    {
        let loop_size = 16;
        let end_ptr_8 = unsafe { end_ptr.sub(loop_size) };
        if a_ptr <= end_ptr_8 {
            let mcc: __m128i = unsafe { _mm_set1_epi8(c as i8) };
            let aa_ptr = a_ptr as *mut __m128i;
            unsafe { _mm_storeu_si128(aa_ptr, mcc) };
            a_ptr = unsafe { a_ptr.add(loop_size) };
        }
    }
    let cc: u64 = c as u64 * 0x0101_0101_0101_0101_u64;
    // the remaining data is the max: 15 bytes.
    basic::_memset_remaining_15_bytes_impl(a_ptr, cc, end_ptr)
}
