use crate::mem as basic;
use crate::utils::*;

#[cfg(target_arch = "x86")]
use core::arch::x86 as mmx;
#[cfg(target_arch = "x86_64")]
use core::arch::x86_64 as mmx;

use mmx::__m128i;
use mmx::_mm_cmpeq_epi8;
use mmx::_mm_load_si128;
use mmx::_mm_movemask_epi8;
use mmx::_mm_or_si128;
use mmx::_mm_setzero_si128;
use mmx::_mm_xor_si128;
/*
use mmx::_mm_set1_epi8;
use mmx::__m256i;
use mmx::_mm256_cmpeq_epi8;
use mmx::_mm256_load_si256;
use mmx::_mm256_loadu_si256;
use mmx::_mm256_movemask_epi8;
use mmx::_mm256_set1_epi8;
*/

#[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
use super::cpuid;

use core::sync::atomic::AtomicPtr;
use core::sync::atomic::Ordering as AtomicOrdering;
type _FuncType = fn(&[u8], &[u8]) -> bool;

const _FUNC: _FuncType = _fnptr_setup_func;
static _FUNC_PTR_ATOM: AtomicPtr<_FuncType> = AtomicPtr::new(_FUNC as *mut _FuncType);

#[inline(never)]
fn _fnptr_setup_func(a: &[u8], b: &[u8]) -> bool {
    #[cfg(target_arch = "x86_64")]
    let func = if cpuid::has_avx2() {
        _memeq_avx2
    } else {
        _memeq_sse2
    };
    #[cfg(target_arch = "x86")]
    let func = if cpuid::has_avx2() {
        _memeq_avx2
    } else if cpuid::has_sse2() {
        _memeq_sse2
    } else {
        _memeq_basic
    };
    //
    _FUNC_PTR_ATOM.store(func as *mut _FuncType, AtomicOrdering::Relaxed);
    unsafe { func(a, b) }
}

#[inline(always)]
pub(crate) fn _memeq_impl(a: &[u8], b: &[u8]) -> bool {
    let func_u = _FUNC_PTR_ATOM.load(AtomicOrdering::Relaxed);
    #[allow(clippy::crosspointer_transmute)]
    let func: _FuncType = unsafe { core::mem::transmute(func_u) };
    func(a, b)
}

unsafe fn _memeq_basic(a: &[u8], b: &[u8]) -> bool {
    basic::_memeq_impl(a, b)
}

#[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
#[target_feature(enable = "sse2")]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn _memeq_sse2(a: &[u8], b: &[u8]) -> bool {
    _memeq_sse2_impl(a, b)
    //basic::_memeq_impl(a, b)
}

#[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
#[target_feature(enable = "avx2")]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn _memeq_avx2(a: &[u8], b: &[u8]) -> bool {
    _memeq_sse2_impl(a, b)
    //basic::_memeq_impl(a, b)
}

macro_rules! _unroll_one_eq_16_aa {
    ($a_ptr:expr, $b_ptr:expr, $loop_size:expr, $idx:expr) => {{
        let aa_ptr = unsafe { $a_ptr.add($loop_size * $idx) };
        let bb_ptr = unsafe { $b_ptr.add($loop_size * $idx) };
        //
        let r = unsafe { _eq_c16_aa(aa_ptr, bb_ptr) };
        if !r {
            return false;
        }
    }};
}

macro_rules! _unroll_one_eq_16_aa_x2 {
    ($a_ptr:expr, $b_ptr:expr, $loop_size:expr, $idx:expr) => {{
        let aa_ptr = unsafe { $a_ptr.add($loop_size * $idx) };
        let bb_ptr = unsafe { $b_ptr.add($loop_size * $idx) };
        //
        let r = unsafe { _eq_c16_aa_x2(aa_ptr, bb_ptr) };
        if !r {
            return false;
        }
    }};
}

macro_rules! _unroll_one_eq_16_aa_x4 {
    ($a_ptr:expr, $b_ptr:expr, $loop_size:expr, $idx:expr) => {{
        let aa_ptr = unsafe { $a_ptr.add($loop_size * $idx) };
        let bb_ptr = unsafe { $b_ptr.add($loop_size * $idx) };
        //
        let r = unsafe { _eq_c16_aa_x4(aa_ptr, bb_ptr) };
        if !r {
            return false;
        }
    }};
}

#[inline(always)]
fn _memeq_sse2_impl(a: &[u8], b: &[u8]) -> bool {
    //
    let a_len = a.len();
    let b_len = b.len();
    if a_len != b_len {
        return false;
    }
    let mut a_ptr = a.as_ptr();
    let mut b_ptr = b.as_ptr();
    let end_ptr = unsafe { a_ptr.add(a_len) };
    a_ptr.prefetch_read_data();
    b_ptr.prefetch_read_data();
    //
    if a_len >= 16 {
        {
            if !a_ptr.is_aligned_u128() {
                let r = basic::_eq_to_aligned_u128(a_ptr, b_ptr);
                if let Some((ap, bp)) = r.0 {
                    a_ptr = ap;
                    b_ptr = bp;
                } else if let Some(v) = r.1 {
                    return v;
                }
            }
        }
        if b_ptr.is_aligned_u128() {
            {
                let unroll = 4;
                let loop_size = 16;
                if a_len >= loop_size * unroll {
                    let eend_ptr = unsafe { end_ptr.sub(loop_size * unroll) };
                    while a_ptr <= eend_ptr {
                        a_ptr.prefetch_read_data();
                        b_ptr.prefetch_read_data();
                        _unroll_one_eq_16_aa_x4!(a_ptr, b_ptr, loop_size, 0);
                        //
                        a_ptr = unsafe { a_ptr.add(loop_size * unroll) };
                        b_ptr = unsafe { b_ptr.add(loop_size * unroll) };
                    }
                }
            }
            {
                let unroll = 2;
                let loop_size = 16;
                if a_len >= loop_size * unroll {
                    let eend_ptr = unsafe { end_ptr.sub(loop_size * unroll) };
                    while a_ptr <= eend_ptr {
                        _unroll_one_eq_16_aa_x2!(a_ptr, b_ptr, loop_size, 0);
                        //
                        a_ptr = unsafe { a_ptr.add(loop_size * unroll) };
                        b_ptr = unsafe { b_ptr.add(loop_size * unroll) };
                    }
                }
            }
            {
                let loop_size = 16;
                let eend_ptr = unsafe { end_ptr.sub(loop_size) };
                while a_ptr <= eend_ptr {
                    _unroll_one_eq_16_aa!(a_ptr, b_ptr, loop_size, 0);
                    a_ptr = unsafe { a_ptr.add(loop_size) };
                    b_ptr = unsafe { b_ptr.add(loop_size) };
                }
            }
        }
    }
    // the remaining data is the max: 15 bytes.
    basic::_memeq_remaining_15_bytes_impl(a_ptr, b_ptr, end_ptr)
}

#[inline(always)]
unsafe fn _eq_c16_aa(a_ptr: *const u8, b_ptr: *const u8) -> bool {
    //
    let mm_a = _mm_load_si128(a_ptr as *const __m128i);
    let mm_b = _mm_load_si128(b_ptr as *const __m128i);
    let mm_eq = _mm_cmpeq_epi8(mm_a, mm_b);
    let mask = _mm_movemask_epi8(mm_eq);
    mask == 0xFFFF
}

#[inline(always)]
unsafe fn _eq_c16_aa_x2(a_ptr: *const u8, b_ptr: *const u8) -> bool {
    //
    let mm_a_0 = _mm_load_si128(a_ptr as *const __m128i);
    let mm_a_1 = _mm_load_si128(a_ptr.add(16) as *const __m128i);
    let mm_b_0 = _mm_load_si128(b_ptr as *const __m128i);
    let mm_b_1 = _mm_load_si128(b_ptr.add(16) as *const __m128i);
    //
    let mm_0 = _mm_setzero_si128();
    let mm_xor_0 = _mm_xor_si128(mm_a_0, mm_b_0);
    let mm_xor_1 = _mm_xor_si128(mm_a_1, mm_b_1);
    //
    let mm_or_01 = _mm_or_si128(mm_xor_0, mm_xor_1);
    //
    let mm_eq_0 = _mm_cmpeq_epi8(mm_or_01, mm_0);
    let mask = _mm_movemask_epi8(mm_eq_0);
    mask == 0xFFFF
}

#[inline(always)]
unsafe fn _eq_c16_aa_x4(a_ptr: *const u8, b_ptr: *const u8) -> bool {
    //
    let mm_a_0 = _mm_load_si128(a_ptr as *const __m128i);
    let mm_a_1 = _mm_load_si128(a_ptr.add(16) as *const __m128i);
    let mm_a_2 = _mm_load_si128(a_ptr.add(16 * 2) as *const __m128i);
    let mm_a_3 = _mm_load_si128(a_ptr.add(16 * 3) as *const __m128i);
    let mm_b_0 = _mm_load_si128(b_ptr as *const __m128i);
    let mm_b_1 = _mm_load_si128(b_ptr.add(16) as *const __m128i);
    let mm_b_2 = _mm_load_si128(b_ptr.add(16 * 2) as *const __m128i);
    let mm_b_3 = _mm_load_si128(b_ptr.add(16 * 3) as *const __m128i);
    //
    let mm_0 = _mm_setzero_si128();
    let mm_xor_0 = _mm_xor_si128(mm_a_0, mm_b_0);
    let mm_xor_1 = _mm_xor_si128(mm_a_1, mm_b_1);
    let mm_xor_2 = _mm_xor_si128(mm_a_2, mm_b_2);
    let mm_xor_3 = _mm_xor_si128(mm_a_3, mm_b_3);
    //
    let mm_or_01 = _mm_or_si128(mm_xor_0, mm_xor_1);
    let mm_or_23 = _mm_or_si128(mm_xor_2, mm_xor_3);
    //
    let mm_or_0123 = _mm_or_si128(mm_or_01, mm_or_23);
    //
    let mm_eq_0 = _mm_cmpeq_epi8(mm_or_0123, mm_0);
    let mask = _mm_movemask_epi8(mm_eq_0);
    mask == 0xFFFF
}

#[cfg(test)]
mod disasm {
    use super::*;
    //
    #[test]
    fn do_procs() {
        let a = b"abcdefg".to_vec();
        let b = b"abcdefg".to_vec();
        let a = a.as_slice();
        let b = b.as_slice();
        assert!(do_proc_basic(a, b));
        assert!(do_proc_sse2(a, b));
        assert!(do_proc_avx2(a, b));
    }
    fn do_proc_basic(a: &[u8], b: &[u8]) -> bool {
        unsafe { _memeq_basic(a, b) }
    }
    fn do_proc_sse2(a: &[u8], b: &[u8]) -> bool {
        unsafe { _memeq_sse2(a, b) }
    }
    fn do_proc_avx2(a: &[u8], b: &[u8]) -> bool {
        unsafe { _memeq_avx2(a, b) }
    }
}
