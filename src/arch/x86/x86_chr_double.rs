use crate::mem as basic;
use crate::utils::*;

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

use core::sync::atomic::AtomicPtr;
use core::sync::atomic::Ordering;
type FuncType = fn(&[u8], u8, u8) -> Option<usize>;

const FUNC: FuncType = fnptr_setup_func;
static FUNC_PTR_ATOM: AtomicPtr<FuncType> = AtomicPtr::new(FUNC as *mut FuncType);

#[inline(never)]
fn fnptr_setup_func(buf: &[u8], c1: u8, c2: u8) -> Option<usize> {
    #[cfg(target_arch = "x86_64")]
    let func = if cpuid::has_avx2() {
        _memchr_double_avx2
    } else {
        _memchr_double_sse2
    };
    #[cfg(target_arch = "x86")]
    let func = if cpuid::has_avx2() {
        _memchr_double_avx2
    } else if cpuid::has_sse2() {
        _memchr_double_sse2
    } else {
        _memchr_double_basic
    };
    //
    FUNC_PTR_ATOM.store(func as *mut FuncType, Ordering::Relaxed);
    unsafe { func(buf, c1, c2) }
}

#[inline(always)]
pub(crate) fn _memchr_double_impl(buf: &[u8], c1: u8, c2: u8) -> Option<usize> {
    let func_u = FUNC_PTR_ATOM.load(Ordering::Relaxed);
    #[allow(clippy::crosspointer_transmute)]
    let func: FuncType = unsafe { core::mem::transmute(func_u) };
    func(buf, c1, c2)
}

unsafe fn _memchr_double_basic(buf: &[u8], c1: u8, c2: u8) -> Option<usize> {
    basic::_memchr_double_impl(buf, c1, c2)
}

#[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
#[target_feature(enable = "sse2")]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn _memchr_double_sse2(buf: &[u8], c1: u8, c2: u8) -> Option<usize> {
    _memchr_double_sse2_impl(buf, c1, c2)
}

#[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
#[target_feature(enable = "avx2")]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn _memchr_double_avx2(buf: &[u8], c1: u8, c2: u8) -> Option<usize> {
    _memchr_double_avx2_impl(buf, c1, c2)
}

macro_rules! _unroll_one_chr_16_uu {
    ($a_ptr:expr, $cc1:expr, $cc2:expr, $start_ptr:expr, $loop_size:expr, $idx:expr) => {{
        let aa_ptr = unsafe { $a_ptr.add($loop_size * $idx) };
        let r = unsafe { _chr_c16_dbl_uu(aa_ptr, $cc1, $cc2, $start_ptr) };
        if !r.is_none() {
            return r;
        }
    }};
}

macro_rules! _unroll_one_chr_16_aa {
    ($a_ptr:expr, $cc1:expr, $cc2:expr, $start_ptr:expr, $loop_size:expr, $idx:expr) => {{
        let aa_ptr = unsafe { $a_ptr.add($loop_size * $idx) };
        let r = unsafe { _chr_c16_dbl_aa(aa_ptr, $cc1, $cc2, $start_ptr) };
        if !r.is_none() {
            return r;
        }
    }};
}

macro_rules! _unroll_one_chr_16_aa_x2 {
    ($a_ptr:expr, $cc1:expr, $cc2:expr, $start_ptr:expr, $loop_size:expr, $idx:expr) => {{
        let aa_ptr = unsafe { $a_ptr.add($loop_size * $idx) };
        let r = unsafe { _chr_c16_dbl_aa_x2(aa_ptr, $cc1, $cc2, $start_ptr) };
        if !r.is_none() {
            return r;
        }
    }};
}

macro_rules! _unroll_one_chr_16_aa_x4 {
    ($a_ptr:expr, $cc1:expr, $cc2:expr, $start_ptr:expr, $loop_size:expr, $idx:expr) => {{
        let aa_ptr = unsafe { $a_ptr.add($loop_size * $idx) };
        let r = unsafe { _chr_c16_dbl_aa_x4(aa_ptr, $cc1, $cc2, $start_ptr) };
        if !r.is_none() {
            return r;
        }
    }};
}

macro_rules! _unroll_one_chr_32_uu {
    ($a_ptr:expr, $cc1:expr, $cc2:expr, $start_ptr:expr, $loop_size:expr, $idx:expr) => {{
        let aa_ptr = unsafe { $a_ptr.add($loop_size * $idx) };
        let r = unsafe { _chr_c32_dbl_uu(aa_ptr, $cc1, $cc2, $start_ptr) };
        if !r.is_none() {
            return r;
        }
    }};
}

macro_rules! _unroll_one_chr_32_aa {
    ($a_ptr:expr, $cc1:expr, $cc2:expr, $start_ptr:expr, $loop_size:expr, $idx:expr) => {{
        let aa_ptr = unsafe { $a_ptr.add($loop_size * $idx) };
        let r = unsafe { _chr_c32_dbl_aa(aa_ptr, $cc1, $cc2, $start_ptr) };
        if !r.is_none() {
            return r;
        }
    }};
}

macro_rules! _unroll_one_chr_32_aa_x2 {
    ($a_ptr:expr, $cc1:expr, $cc2:expr, $start_ptr:expr, $loop_size:expr, $idx:expr) => {{
        let aa_ptr = unsafe { $a_ptr.add($loop_size * $idx) };
        let r = unsafe { _chr_c32_dbl_aa_x2(aa_ptr, $cc1, $cc2, $start_ptr) };
        if !r.is_none() {
            return r;
        }
    }};
}

#[inline(always)]
fn _memchr_double_sse2_impl(buf: &[u8], c1: u8, c2: u8) -> Option<usize> {
    let buf_len = buf.len();
    let mut buf_ptr = buf.as_ptr();
    let start_ptr = buf_ptr;
    let end_ptr = unsafe { buf_ptr.add(buf_len) };
    buf_ptr.prefetch_read_data();
    //
    if buf_len >= 16 {
        let cc1: __m128i = unsafe { _c16_value(c1) };
        let cc2: __m128i = unsafe { _c16_value(c2) };
        {
            let loop_size = 16;
            //
            #[cfg(not(feature = "test_alignment_check"))]
            {
                if buf_ptr.is_aligned_u128() {
                    _unroll_one_chr_16_aa!(buf_ptr, cc1, cc2, start_ptr, loop_size, 0);
                } else {
                    _unroll_one_chr_16_uu!(buf_ptr, cc1, cc2, start_ptr, loop_size, 0);
                }
                let remaining_align = 0x10_usize - ((buf_ptr as usize) & 0x0F_usize);
                buf_ptr = unsafe { buf_ptr.add(remaining_align) };
            }
            #[cfg(feature = "test_alignment_check")]
            {
                if buf_ptr.is_aligned_u128() {
                    _unroll_one_chr_16_aa!(buf_ptr, cc1, cc2, start_ptr, loop_size, 0);
                    let remaining_align = 0x10_usize - ((buf_ptr as usize) & 0x0F_usize);
                    buf_ptr = unsafe { buf_ptr.add(remaining_align) };
                } else {
                    let r = basic::_chr_dbl_to_aligned_u128(buf_ptr, c1, c2, start_ptr);
                    if let Some(p) = r.0 {
                        buf_ptr = p;
                    } else if let Some(v) = r.1 {
                        return Some(v);
                    }
                }
            }
        }
        //
        {
            let unroll = 4;
            let loop_size = 16;
            if unsafe { end_ptr.offset_from(buf_ptr) } >= (loop_size * unroll) as isize {
                let end_ptr_16_x4 = unsafe { end_ptr.sub(loop_size * unroll) };
                while buf_ptr <= end_ptr_16_x4 {
                    buf_ptr.prefetch_read_data();
                    _unroll_one_chr_16_aa_x4!(buf_ptr, cc1, cc2, start_ptr, loop_size, 0);
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
                    _unroll_one_chr_16_aa_x2!(buf_ptr, cc1, cc2, start_ptr, loop_size, 0);
                    //_unroll_one_chr_16_aa!(buf_ptr, cc1, cc2, start_ptr, loop_size, 1);
                    buf_ptr = unsafe { buf_ptr.add(loop_size * unroll) };
                }
            }
        }
        {
            let loop_size = 16;
            let end_ptr_16 = unsafe { end_ptr.sub(loop_size) };
            while buf_ptr <= end_ptr_16 {
                _unroll_one_chr_16_aa!(buf_ptr, cc1, cc2, start_ptr, loop_size, 0);
                //
                buf_ptr = unsafe { buf_ptr.add(loop_size) };
            }
        }
    }
    //
    let cc1: u64 = _c8_value(c1);
    let cc2: u64 = _c8_value(c2);
    basic::_memchr_double_remaining_15_bytes_impl(buf_ptr, cc1, cc2, start_ptr, end_ptr)
}

#[inline(always)]
fn _memchr_double_avx2_impl(buf: &[u8], c1: u8, c2: u8) -> Option<usize> {
    let buf_len = buf.len();
    let mut buf_ptr = buf.as_ptr();
    let start_ptr = buf_ptr;
    let end_ptr = unsafe { buf_ptr.add(buf_len) };
    buf_ptr.prefetch_read_data();
    //
    if buf_len >= 32 {
        let cc1: __m256i = unsafe { _c32_value(c1) };
        let cc2: __m256i = unsafe { _c32_value(c2) };
        {
            let loop_size = 32;
            //
            #[cfg(not(feature = "test_alignment_check"))]
            {
                if buf_ptr.is_aligned_u256() {
                    _unroll_one_chr_32_aa!(buf_ptr, cc1, cc2, start_ptr, loop_size, 0);
                } else {
                    _unroll_one_chr_32_uu!(buf_ptr, cc1, cc2, start_ptr, loop_size, 0);
                }
                let remaining_align = 0x20_usize - ((buf_ptr as usize) & 0x1F_usize);
                buf_ptr = unsafe { buf_ptr.add(remaining_align) };
            }
            #[cfg(feature = "test_alignment_check")]
            {
                if buf_ptr.is_aligned_u256() {
                    _unroll_one_chr_32_aa!(buf_ptr, cc1, cc2, start_ptr, loop_size, 0);
                    let remaining_align = 0x20_usize - ((buf_ptr as usize) & 0x1F_usize);
                    buf_ptr = unsafe { buf_ptr.add(remaining_align) };
                } else {
                    let r = basic::_chr_dbl_to_aligned_u256(buf_ptr, c1, c2, start_ptr);
                    if let Some(p) = r.0 {
                        buf_ptr = p;
                    } else if let Some(v) = r.1 {
                        return Some(v);
                    }
                }
            }
        }
        //
        {
            let unroll = 2;
            let loop_size = 32;
            if unsafe { end_ptr.offset_from(buf_ptr) } >= (loop_size * unroll) as isize {
                let end_ptr_32_x2 = unsafe { end_ptr.sub(loop_size * unroll) };
                while buf_ptr <= end_ptr_32_x2 {
                    buf_ptr.prefetch_read_data();
                    _unroll_one_chr_32_aa_x2!(buf_ptr, cc1, cc2, start_ptr, loop_size, 0);
                    buf_ptr = unsafe { buf_ptr.add(loop_size * unroll) };
                }
            }
        }
        {
            let loop_size = 32;
            let end_ptr_32 = unsafe { end_ptr.sub(loop_size) };
            while buf_ptr <= end_ptr_32 {
                _unroll_one_chr_32_aa!(buf_ptr, cc1, cc2, start_ptr, loop_size, 0);
                buf_ptr = unsafe { buf_ptr.add(loop_size) };
            }
        }
        {
            let cc1: __m128i = unsafe { _c16_value(c1) };
            let cc2: __m128i = unsafe { _c16_value(c2) };
            let loop_size = 16;
            let end_ptr_16 = unsafe { end_ptr.sub(loop_size) };
            while buf_ptr <= end_ptr_16 {
                _unroll_one_chr_16_aa!(buf_ptr, cc1, cc2, start_ptr, loop_size, 0);
                buf_ptr = unsafe { buf_ptr.add(loop_size) };
            }
        }
    } else if buf_len >= 16 {
        {
            let cc1: __m128i = unsafe { _c16_value(c1) };
            let cc2: __m128i = unsafe { _c16_value(c2) };
            let loop_size = 16;
            let end_ptr_16 = unsafe { end_ptr.sub(loop_size) };
            if buf_ptr <= end_ptr_16 {
                //
                #[cfg(not(feature = "test_alignment_check"))]
                {
                    if buf_ptr.is_aligned_u128() {
                        while buf_ptr <= end_ptr_16 {
                            _unroll_one_chr_16_aa!(buf_ptr, cc1, cc2, start_ptr, loop_size, 0);
                            buf_ptr = unsafe { buf_ptr.add(loop_size) };
                        }
                    } else {
                        while buf_ptr <= end_ptr_16 {
                            _unroll_one_chr_16_uu!(buf_ptr, cc1, cc2, start_ptr, loop_size, 0);
                            buf_ptr = unsafe { buf_ptr.add(loop_size) };
                        }
                    }
                }
                #[cfg(feature = "test_alignment_check")]
                {
                    let r = basic::_chr_dbl_to_aligned_u128(buf_ptr, c1, c2, start_ptr);
                    if let Some(p) = r.0 {
                        buf_ptr = p;
                    } else if let Some(v) = r.1 {
                        return Some(v);
                    }
                    while buf_ptr <= end_ptr_16 {
                        _unroll_one_chr_16_aa!(buf_ptr, cc1, cc2, start_ptr, loop_size, 0);
                        buf_ptr = unsafe { buf_ptr.add(loop_size) };
                    }
                }
            }
        }
    }
    //
    let cc1: u64 = _c8_value(c1);
    let cc2: u64 = _c8_value(c2);
    basic::_memchr_double_remaining_15_bytes_impl(buf_ptr, cc1, cc2, start_ptr, end_ptr)
}

#[inline(always)]
unsafe fn _c16_value(c: u8) -> __m128i {
    _mm_set1_epi8(c as i8)
}

#[inline(always)]
unsafe fn _chr_c16_dbl_uu(
    buf_ptr: *const u8,
    mm_c16_1: __m128i,
    mm_c16_2: __m128i,
    start_ptr: *const u8,
) -> Option<usize> {
    //
    let mm_a = _mm_loadu_si128(buf_ptr as *const __m128i);
    let mm_a_eq_1 = _mm_cmpeq_epi8(mm_a, mm_c16_1);
    let mm_a_eq_2 = _mm_cmpeq_epi8(mm_a, mm_c16_2);
    let mask_a_1 = _mm_movemask_epi8(mm_a_eq_1);
    let mask_a_2 = _mm_movemask_epi8(mm_a_eq_2);
    if mask_a_1 != 0 {
        if mask_a_2 != 0 {
            let base = plus_offset_from(buf_ptr, start_ptr);
            let idx1 = mask_a_1.trailing_zeros() as usize;
            let idx2 = mask_a_2.trailing_zeros() as usize;
            if idx1 < idx2 {
                Some(base + idx1)
            } else {
                Some(base + idx2)
            }
        } else {
            Some(plus_offset_from(buf_ptr, start_ptr) + mask_a_1.trailing_zeros() as usize)
        }
    } else if mask_a_2 != 0 {
        Some(plus_offset_from(buf_ptr, start_ptr) + mask_a_2.trailing_zeros() as usize)
    } else {
        None
    }
}

#[inline(always)]
unsafe fn _chr_c16_dbl_aa(
    buf_ptr: *const u8,
    mm_c16_1: __m128i,
    mm_c16_2: __m128i,
    start_ptr: *const u8,
) -> Option<usize> {
    //
    let mm_a = _mm_load_si128(buf_ptr as *const __m128i);
    let mm_a_eq_1 = _mm_cmpeq_epi8(mm_a, mm_c16_1);
    let mm_a_eq_2 = _mm_cmpeq_epi8(mm_a, mm_c16_2);
    let mask_a_1 = _mm_movemask_epi8(mm_a_eq_1);
    let mask_a_2 = _mm_movemask_epi8(mm_a_eq_2);
    if mask_a_1 != 0 {
        if mask_a_2 != 0 {
            let base = plus_offset_from(buf_ptr, start_ptr);
            let idx1 = mask_a_1.trailing_zeros() as usize;
            let idx2 = mask_a_2.trailing_zeros() as usize;
            if idx1 < idx2 {
                Some(base + idx1)
            } else {
                Some(base + idx2)
            }
        } else {
            Some(plus_offset_from(buf_ptr, start_ptr) + mask_a_1.trailing_zeros() as usize)
        }
    } else if mask_a_2 != 0 {
        Some(plus_offset_from(buf_ptr, start_ptr) + mask_a_2.trailing_zeros() as usize)
    } else {
        None
    }
}

#[inline(always)]
unsafe fn _chr_c16_dbl_aa_x2(
    buf_ptr: *const u8,
    mm_c16_1: __m128i,
    mm_c16_2: __m128i,
    start_ptr: *const u8,
) -> Option<usize> {
    //
    let mm_a = _mm_load_si128(buf_ptr as *const __m128i);
    let mm_b = _mm_load_si128(buf_ptr.add(16) as *const __m128i);
    //
    let mm_a_eq_1 = _mm_cmpeq_epi8(mm_a, mm_c16_1);
    let mm_a_eq_2 = _mm_cmpeq_epi8(mm_a, mm_c16_2);
    let mask_a_1 = _mm_movemask_epi8(mm_a_eq_1);
    let mask_a_2 = _mm_movemask_epi8(mm_a_eq_2);
    //
    let mm_b_eq_1 = _mm_cmpeq_epi8(mm_b, mm_c16_1);
    let mm_b_eq_2 = _mm_cmpeq_epi8(mm_b, mm_c16_2);
    let mask_b_1 = _mm_movemask_epi8(mm_b_eq_1);
    let mask_b_2 = _mm_movemask_epi8(mm_b_eq_2);
    //
    if mask_a_1 != 0 {
        if mask_a_2 != 0 {
            let base = plus_offset_from(buf_ptr, start_ptr);
            let idx1 = mask_a_1.trailing_zeros() as usize;
            let idx2 = mask_a_2.trailing_zeros() as usize;
            if idx1 < idx2 {
                Some(base + idx1)
            } else {
                Some(base + idx2)
            }
        } else {
            Some(plus_offset_from(buf_ptr, start_ptr) + mask_a_1.trailing_zeros() as usize)
        }
    } else if mask_a_2 != 0 {
        Some(plus_offset_from(buf_ptr, start_ptr) + mask_a_2.trailing_zeros() as usize)
    } else if mask_b_1 != 0 {
        if mask_b_2 != 0 {
            let base = plus_offset_from(buf_ptr, start_ptr);
            let idx1 = mask_b_1.trailing_zeros() as usize;
            let idx2 = mask_b_2.trailing_zeros() as usize;
            if idx1 < idx2 {
                Some(base + idx1 + 16)
            } else {
                Some(base + idx2 + 16)
            }
        } else {
            Some(plus_offset_from(buf_ptr, start_ptr) + mask_b_1.trailing_zeros() as usize + 16)
        }
    } else if mask_b_2 != 0 {
        Some(plus_offset_from(buf_ptr, start_ptr) + mask_b_2.trailing_zeros() as usize + 16)
    } else {
        None
    }
}

#[inline(always)]
unsafe fn _chr_c16_dbl_aa_x4(
    buf_ptr: *const u8,
    mm_c16_1: __m128i,
    mm_c16_2: __m128i,
    start_ptr: *const u8,
) -> Option<usize> {
    //
    let mm_a = _mm_load_si128(buf_ptr as *const __m128i);
    let mm_b = _mm_load_si128(buf_ptr.add(16) as *const __m128i);
    let mm_c = _mm_load_si128(buf_ptr.add(16 * 2) as *const __m128i);
    let mm_d = _mm_load_si128(buf_ptr.add(16 * 3) as *const __m128i);
    //
    let mm_a_eq_1 = _mm_cmpeq_epi8(mm_a, mm_c16_1);
    let mm_a_eq_2 = _mm_cmpeq_epi8(mm_a, mm_c16_2);
    let mask_a_1 = _mm_movemask_epi8(mm_a_eq_1);
    let mask_a_2 = _mm_movemask_epi8(mm_a_eq_2);
    //
    let mm_b_eq_1 = _mm_cmpeq_epi8(mm_b, mm_c16_1);
    let mm_b_eq_2 = _mm_cmpeq_epi8(mm_b, mm_c16_2);
    let mask_b_1 = _mm_movemask_epi8(mm_b_eq_1);
    let mask_b_2 = _mm_movemask_epi8(mm_b_eq_2);
    //
    let mm_c_eq_1 = _mm_cmpeq_epi8(mm_c, mm_c16_1);
    let mm_c_eq_2 = _mm_cmpeq_epi8(mm_c, mm_c16_2);
    let mask_c_1 = _mm_movemask_epi8(mm_c_eq_1);
    let mask_c_2 = _mm_movemask_epi8(mm_c_eq_2);
    //
    let mm_d_eq_1 = _mm_cmpeq_epi8(mm_d, mm_c16_1);
    let mm_d_eq_2 = _mm_cmpeq_epi8(mm_d, mm_c16_2);
    let mask_d_1 = _mm_movemask_epi8(mm_d_eq_1);
    let mask_d_2 = _mm_movemask_epi8(mm_d_eq_2);
    //
    if mask_a_1 != 0 {
        if mask_a_2 != 0 {
            let base = plus_offset_from(buf_ptr, start_ptr);
            let idx1 = mask_a_1.trailing_zeros() as usize;
            let idx2 = mask_a_2.trailing_zeros() as usize;
            if idx1 < idx2 {
                Some(base + idx1)
            } else {
                Some(base + idx2)
            }
        } else {
            Some(plus_offset_from(buf_ptr, start_ptr) + mask_a_1.trailing_zeros() as usize)
        }
    } else if mask_a_2 != 0 {
        Some(plus_offset_from(buf_ptr, start_ptr) + mask_a_2.trailing_zeros() as usize)
    } else if mask_b_1 != 0 {
        if mask_b_2 != 0 {
            let base = plus_offset_from(buf_ptr, start_ptr);
            let idx1 = mask_b_1.trailing_zeros() as usize;
            let idx2 = mask_b_2.trailing_zeros() as usize;
            if idx1 < idx2 {
                Some(base + idx1 + 16)
            } else {
                Some(base + idx2 + 16)
            }
        } else {
            Some(plus_offset_from(buf_ptr, start_ptr) + mask_b_1.trailing_zeros() as usize + 16)
        }
    } else if mask_b_2 != 0 {
        Some(plus_offset_from(buf_ptr, start_ptr) + mask_b_2.trailing_zeros() as usize + 16)
    } else if mask_c_1 != 0 {
        if mask_c_2 != 0 {
            let base = plus_offset_from(buf_ptr, start_ptr);
            let idx1 = mask_c_1.trailing_zeros() as usize;
            let idx2 = mask_c_2.trailing_zeros() as usize;
            if idx1 < idx2 {
                Some(base + idx1 + 16 * 2)
            } else {
                Some(base + idx2 + 16 * 2)
            }
        } else {
            Some(plus_offset_from(buf_ptr, start_ptr) + mask_c_1.trailing_zeros() as usize + 16 * 2)
        }
    } else if mask_c_2 != 0 {
        Some(plus_offset_from(buf_ptr, start_ptr) + mask_c_2.trailing_zeros() as usize + 16 * 2)
    } else if mask_d_1 != 0 {
        if mask_d_2 != 0 {
            let base = plus_offset_from(buf_ptr, start_ptr);
            let idx1 = mask_d_1.trailing_zeros() as usize;
            let idx2 = mask_d_2.trailing_zeros() as usize;
            if idx1 < idx2 {
                Some(base + idx1 + 16 * 3)
            } else {
                Some(base + idx2 + 16 * 3)
            }
        } else {
            Some(plus_offset_from(buf_ptr, start_ptr) + mask_d_1.trailing_zeros() as usize + 16 * 3)
        }
    } else if mask_d_2 != 0 {
        Some(plus_offset_from(buf_ptr, start_ptr) + mask_d_2.trailing_zeros() as usize + 16 * 3)
    } else {
        None
    }
}

#[inline(always)]
unsafe fn _c32_value(c: u8) -> __m256i {
    _mm256_set1_epi8(c as i8)
}

#[inline(always)]
unsafe fn _chr_c32_dbl_uu(
    buf_ptr: *const u8,
    mm_c32_1: __m256i,
    mm_c32_2: __m256i,
    start_ptr: *const u8,
) -> Option<usize> {
    //
    let mm_a = _mm256_loadu_si256(buf_ptr as *const __m256i);
    let mm_a_eq_1 = _mm256_cmpeq_epi8(mm_a, mm_c32_1);
    let mm_a_eq_2 = _mm256_cmpeq_epi8(mm_a, mm_c32_2);
    let mask_a_1 = _mm256_movemask_epi8(mm_a_eq_1);
    let mask_a_2 = _mm256_movemask_epi8(mm_a_eq_2);
    if mask_a_1 != 0 {
        if mask_a_2 != 0 {
            let base = plus_offset_from(buf_ptr, start_ptr);
            let idx1 = mask_a_1.trailing_zeros() as usize;
            let idx2 = mask_a_2.trailing_zeros() as usize;
            if idx1 < idx2 {
                Some(base + idx1)
            } else {
                Some(base + idx2)
            }
        } else {
            Some(plus_offset_from(buf_ptr, start_ptr) + mask_a_1.trailing_zeros() as usize)
        }
    } else if mask_a_2 != 0 {
        Some(plus_offset_from(buf_ptr, start_ptr) + mask_a_2.trailing_zeros() as usize)
    } else {
        None
    }
}

#[inline(always)]
unsafe fn _chr_c32_dbl_aa(
    buf_ptr: *const u8,
    mm_c32_1: __m256i,
    mm_c32_2: __m256i,
    start_ptr: *const u8,
) -> Option<usize> {
    //
    let mm_a = _mm256_load_si256(buf_ptr as *const __m256i);
    let mm_a_eq_1 = _mm256_cmpeq_epi8(mm_a, mm_c32_1);
    let mm_a_eq_2 = _mm256_cmpeq_epi8(mm_a, mm_c32_2);
    let mask_a_1 = _mm256_movemask_epi8(mm_a_eq_1);
    let mask_a_2 = _mm256_movemask_epi8(mm_a_eq_2);
    if mask_a_1 != 0 {
        if mask_a_2 != 0 {
            let base = plus_offset_from(buf_ptr, start_ptr);
            let idx1 = mask_a_1.trailing_zeros() as usize;
            let idx2 = mask_a_2.trailing_zeros() as usize;
            if idx1 < idx2 {
                Some(base + idx1)
            } else {
                Some(base + idx2)
            }
        } else {
            Some(plus_offset_from(buf_ptr, start_ptr) + mask_a_1.trailing_zeros() as usize)
        }
    } else if mask_a_2 != 0 {
        Some(plus_offset_from(buf_ptr, start_ptr) + mask_a_2.trailing_zeros() as usize)
    } else {
        None
    }
}

#[inline(always)]
unsafe fn _chr_c32_dbl_aa_x2(
    buf_ptr: *const u8,
    mm_c32_1: __m256i,
    mm_c32_2: __m256i,
    start_ptr: *const u8,
) -> Option<usize> {
    //
    let mm_a = _mm256_load_si256(buf_ptr as *const __m256i);
    let mm_b = _mm256_load_si256(buf_ptr.add(32) as *const __m256i);
    //
    let mm_a_eq_1 = _mm256_cmpeq_epi8(mm_a, mm_c32_1);
    let mm_a_eq_2 = _mm256_cmpeq_epi8(mm_a, mm_c32_2);
    let mask_a_1 = _mm256_movemask_epi8(mm_a_eq_1);
    let mask_a_2 = _mm256_movemask_epi8(mm_a_eq_2);
    //
    let mm_b_eq_1 = _mm256_cmpeq_epi8(mm_b, mm_c32_1);
    let mm_b_eq_2 = _mm256_cmpeq_epi8(mm_b, mm_c32_2);
    let mask_b_1 = _mm256_movemask_epi8(mm_b_eq_1);
    let mask_b_2 = _mm256_movemask_epi8(mm_b_eq_2);
    //
    if mask_a_1 != 0 {
        if mask_a_2 != 0 {
            let base = plus_offset_from(buf_ptr, start_ptr);
            let idx1 = mask_a_1.trailing_zeros() as usize;
            let idx2 = mask_a_2.trailing_zeros() as usize;
            if idx1 < idx2 {
                Some(base + idx1)
            } else {
                Some(base + idx2)
            }
        } else {
            Some(plus_offset_from(buf_ptr, start_ptr) + mask_a_1.trailing_zeros() as usize)
        }
    } else if mask_a_2 != 0 {
        Some(plus_offset_from(buf_ptr, start_ptr) + mask_a_2.trailing_zeros() as usize)
    } else if mask_b_1 != 0 {
        if mask_b_2 != 0 {
            let base = plus_offset_from(buf_ptr, start_ptr);
            let idx1 = mask_b_1.trailing_zeros() as usize;
            let idx2 = mask_b_2.trailing_zeros() as usize;
            if idx1 < idx2 {
                Some(base + idx1 + 32)
            } else {
                Some(base + idx2 + 32)
            }
        } else {
            Some(plus_offset_from(buf_ptr, start_ptr) + mask_b_1.trailing_zeros() as usize + 32)
        }
    } else if mask_b_2 != 0 {
        Some(plus_offset_from(buf_ptr, start_ptr) + mask_b_2.trailing_zeros() as usize + 32)
    } else {
        None
    }
}
