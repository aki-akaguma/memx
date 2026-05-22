use super::{MMB16Qpl, MMB32Qpl};
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

use mmx::__m256i;
use mmx::_mm256_cmpeq_epi8;
use mmx::_mm256_load_si256;
use mmx::_mm256_loadu_si256;
use mmx::_mm256_movemask_epi8;

#[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
use super::cpuid;

use core::sync::atomic::AtomicPtr;
use core::sync::atomic::Ordering;
type FuncType = fn(&[u8], B1Qpl) -> Option<usize>;

const FUNC: FuncType = fnptr_setup_func;
static FUNC_PTR_ATOM: AtomicPtr<FuncType> = AtomicPtr::new(FUNC as *mut FuncType);

#[inline(never)]
fn fnptr_setup_func(buf: &[u8], needle: B1Qpl) -> Option<usize> {
    #[cfg(target_arch = "x86_64")]
    let func = if cpuid::has_avx2() {
        _memrchr_qpl_avx2
    } else {
        _memrchr_qpl_sse2
    };
    #[cfg(target_arch = "x86")]
    let func = if cpuid::has_avx2() {
        _memrchr_qpl_avx2
    } else if cpuid::has_sse2() {
        _memrchr_qpl_sse2
    } else {
        _memrchr_qpl_basic
    };
    //
    FUNC_PTR_ATOM.store(func as *mut FuncType, Ordering::Relaxed);
    unsafe { func(buf, needle) }
}

#[inline(always)]
pub(crate) fn _memrchr_qpl_impl(buf: &[u8], needle: B1Qpl) -> Option<usize> {
    let func_u = FUNC_PTR_ATOM.load(Ordering::Relaxed);
    #[allow(clippy::crosspointer_transmute)]
    let func: FuncType = unsafe { core::mem::transmute(func_u) };
    func(buf, needle)
}

unsafe fn _memrchr_qpl_basic(buf: &[u8], needle: B1Qpl) -> Option<usize> {
    basic::_memrchr_qpl_impl(buf, needle)
}

#[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
#[target_feature(enable = "sse2")]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn _memrchr_qpl_sse2(buf: &[u8], needle: B1Qpl) -> Option<usize> {
    _memrchr_qpl_sse2_impl(buf, needle)
}

#[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
#[target_feature(enable = "avx2")]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn _memrchr_qpl_avx2(buf: &[u8], needle: B1Qpl) -> Option<usize> {
    _memrchr_qpl_avx2_impl(buf, needle)
}

#[inline(always)]
fn _memrchr_qpl_sse2_impl(buf: &[u8], needle: B1Qpl) -> Option<usize> {
    let buf_len = buf.len();
    let start_ptr = buf.as_ptr();
    let mut buf_ptr = unsafe { start_ptr.add(buf_len) };
    //
    if buf_len >= 16 {
        let cc: MMB16Qpl = needle.into();
        // to a aligned pointer
        {
            if !buf_ptr.is_aligned_u128() {
                #[cfg(not(feature = "test_alignment_check"))]
                {
                    let loop_size = 16;
                    let buf_ptr_pre = unsafe { buf_ptr.sub(loop_size) };
                    let r = unsafe { _rchr_qpl_c16_uu_x1(buf_ptr_pre, cc, start_ptr) };
                    if r.is_some() {
                        return r;
                    }
                    let remaining_align = (buf_ptr as usize) & 0x0F_usize;
                    buf_ptr = unsafe { buf_ptr.sub(remaining_align) };
                }
                #[cfg(feature = "test_alignment_check")]
                {
                    let r = basic::_rchr_qpl_to_aligned_u128(buf_ptr, needle, start_ptr);
                    if let Some(p) = r.0 {
                        buf_ptr = p;
                    } else if let Some(v) = r.1 {
                        return Some(v);
                    }
                }
            }
        }
        // the loop
        {
            let (r, p) = _unroll_loop_backward_with_prefetch::<4, 16, _>(buf_ptr, start_ptr, |p| {
                unsafe { _rchr_qpl_c16_aa_x1(p, cc, start_ptr) }
            });
            if r.is_some() {
                return r;
            }
            buf_ptr = p;
        }
        {
            let (r, p) = _unroll_loop_backward::<1, 16, _>(buf_ptr, start_ptr, |p| {
                unsafe { _rchr_qpl_c16_aa_x1(p, cc, start_ptr) }
            });
            if r.is_some() {
                return r;
            }
            buf_ptr = p;
        }
    }
    //
    let cc: B8Qpl = needle.into();
    basic::_memrchr_qpl_remaining_15_bytes_impl(buf_ptr, cc, start_ptr)
}

#[inline(always)]
fn _memrchr_qpl_avx2_impl(buf: &[u8], needle: B1Qpl) -> Option<usize> {
    let buf_len = buf.len();
    let start_ptr = buf.as_ptr();
    let mut buf_ptr = unsafe { start_ptr.add(buf_len) };
    //
    if buf_len >= 32 {
        let cc: MMB32Qpl = needle.into();
        {
            if !buf_ptr.is_aligned_u256() {
                #[cfg(not(feature = "test_alignment_check"))]
                {
                    let loop_size = 32;
                    let buf_ptr_pre = unsafe { buf_ptr.sub(loop_size) };
                    let r = unsafe { _rchr_qpl_c32_uu_x1(buf_ptr_pre, cc, start_ptr) };
                    if r.is_some() {
                        return r;
                    }
                    let remaining_align = (buf_ptr as usize) & 0x1F_usize;
                    buf_ptr = unsafe { buf_ptr.sub(remaining_align) };
                }
                #[cfg(feature = "test_alignment_check")]
                {
                    let r = basic::_rchr_qpl_to_aligned_u256(buf_ptr, needle, start_ptr);
                    if let Some(p) = r.0 {
                        buf_ptr = p;
                    } else if let Some(v) = r.1 {
                        return Some(v);
                    }
                }
            }
        }
        // the loop
        {
            let (r, p) = _unroll_loop_backward::<1, 32, _>(buf_ptr, start_ptr, |p| {
                unsafe { _rchr_qpl_c32_aa_x1(p, cc, start_ptr) }
            });
            if r.is_some() {
                return r;
            }
            buf_ptr = p;
        }
        {
            let cc: MMB16Qpl = needle.into();
            let (r, p) = _unroll_loop_backward::<1, 16, _>(buf_ptr, start_ptr, |p| {
                unsafe { _rchr_qpl_c16_aa_x1(p, cc, start_ptr) }
            });
            if r.is_some() {
                return r;
            }
            buf_ptr = p;
        }
    } else if buf_len >= 16 {
        {
            let cc: MMB16Qpl = needle.into();
            if buf_ptr.is_aligned_u128() {
                let (r, p) = _unroll_loop_backward::<1, 16, _>(buf_ptr, start_ptr, |p| {
                    unsafe { _rchr_qpl_c16_aa_x1(p, cc, start_ptr) }
                });
                if r.is_some() {
                    return r;
                }
                buf_ptr = p;
            } else {
                #[cfg(not(feature = "test_alignment_check"))]
                {
                    let (r, p) = _unroll_loop_backward::<1, 16, _>(buf_ptr, start_ptr, |p| {
                        unsafe { _rchr_qpl_c16_uu_x1(p, cc, start_ptr) }
                    });
                    if r.is_some() {
                        return r;
                    }
                    buf_ptr = p;
                }
                #[cfg(feature = "test_alignment_check")]
                {
                    let r = basic::_rchr_qpl_to_aligned_u128(buf_ptr, needle, start_ptr);
                    if let Some(p) = r.0 {
                        buf_ptr = p;
                    } else if let Some(v) = r.1 {
                        return Some(v);
                    }
                    let (r, p) = _unroll_loop_backward::<1, 16, _>(buf_ptr, start_ptr, |p| {
                        unsafe { _rchr_qpl_c16_aa_x1(p, cc, start_ptr) }
                    });
                    if r.is_some() {
                        return r;
                    }
                    buf_ptr = p;
                }
            }
        }
    }
    //
    let cc: B8Qpl = needle.into();
    basic::_memrchr_qpl_remaining_15_bytes_impl(buf_ptr, cc, start_ptr)
}

#[inline(always)]
fn _return_rchr_qpl<T, PU>(base: T, mask_a: PU, mask_b: PU, mask_c: PU, mask_d: PU) -> Option<usize>
where
    T: core::ops::Sub<usize, Output = usize>,
    PU: BitOrt,
{
    if !mask_a.is_zeros() {
        let idx1 = mask_a.leading_zeros() as usize;
        if !mask_b.is_zeros() {
            let idx2 = mask_b.leading_zeros() as usize;
            if !mask_c.is_zeros() {
                let idx3 = mask_c.leading_zeros() as usize;
                if !mask_d.is_zeros() {
                    let idx4 = mask_d.leading_zeros() as usize;
                    Some(base - idx1.min(idx2.min(idx3.min(idx4))))
                } else {
                    Some(base - idx1.min(idx2.min(idx3)))
                }
            } else if !mask_d.is_zeros() {
                let idx4 = mask_d.leading_zeros() as usize;
                Some(base - idx1.min(idx2.min(idx4)))
            } else {
                Some(base - idx1.min(idx2))
            }
        } else if !mask_c.is_zeros() {
            let idx3 = mask_c.leading_zeros() as usize;
            if !mask_d.is_zeros() {
                let idx4 = mask_d.leading_zeros() as usize;
                Some(base - idx1.min(idx3.min(idx4)))
            } else {
                Some(base - idx1.min(idx3))
            }
        } else if !mask_d.is_zeros() {
            let idx4 = mask_d.leading_zeros() as usize;
            Some(base - idx1.min(idx4))
        } else {
            Some(base - idx1)
        }
    } else if !mask_b.is_zeros() {
        let idx2 = mask_b.leading_zeros() as usize;
        if !mask_c.is_zeros() {
            let idx3 = mask_c.leading_zeros() as usize;
            if !mask_d.is_zeros() {
                let idx4 = mask_d.leading_zeros() as usize;
                Some(base - idx2.min(idx3.min(idx4)))
            } else {
                Some(base - idx2.min(idx3))
            }
        } else if !mask_d.is_zeros() {
            let idx4 = mask_d.leading_zeros() as usize;
            Some(base - idx2.min(idx4))
        } else {
            Some(base - idx2)
        }
    } else if !mask_c.is_zeros() {
        let idx3 = mask_c.leading_zeros() as usize;
        if !mask_d.is_zeros() {
            let idx4 = mask_d.leading_zeros() as usize;
            Some(base - idx3.min(idx4))
        } else {
            Some(base - idx3)
        }
    } else if !mask_d.is_zeros() {
        let idx4 = mask_d.leading_zeros() as usize;
        Some(base - idx4)
    } else {
        None
    }
}

#[inline(always)]
unsafe fn _rchr_qpl_c16_uu_x1(
    buf_ptr: *const u8,
    mm_c16: MMB16Qpl,
    st_ptr: *const u8,
) -> Option<usize> {
    unsafe {
        let mm_0 = _mm_loadu_si128(buf_ptr as *const __m128i);
        let mm_0_eq_a = _mm_cmpeq_epi8(mm_0, mm_c16.v1);
        let mm_0_eq_b = _mm_cmpeq_epi8(mm_0, mm_c16.v2);
        let mm_0_eq_c = _mm_cmpeq_epi8(mm_0, mm_c16.v3);
        let mm_0_eq_d = _mm_cmpeq_epi8(mm_0, mm_c16.v4);
        let mask_0_a = _mm_movemask_epi8(mm_0_eq_a) as u16;
        let mask_0_b = _mm_movemask_epi8(mm_0_eq_b) as u16;
        let mask_0_c = _mm_movemask_epi8(mm_0_eq_c) as u16;
        let mask_0_d = _mm_movemask_epi8(mm_0_eq_d) as u16;
        let base = buf_ptr.usz_offset_from(st_ptr) + 16 - 1;
        //
        _return_rchr_qpl(base, mask_0_a, mask_0_b, mask_0_c, mask_0_d)
    }
}

#[inline(always)]
unsafe fn _rchr_qpl_c16_aa_x1(
    buf_ptr: *const u8,
    mm_c16: MMB16Qpl,
    st_ptr: *const u8,
) -> Option<usize> {
    unsafe {
        let mm_0 = _mm_load_si128(buf_ptr as *const __m128i);
        let mm_0_eq_a = _mm_cmpeq_epi8(mm_0, mm_c16.v1);
        let mm_0_eq_b = _mm_cmpeq_epi8(mm_0, mm_c16.v2);
        let mm_0_eq_c = _mm_cmpeq_epi8(mm_0, mm_c16.v3);
        let mm_0_eq_d = _mm_cmpeq_epi8(mm_0, mm_c16.v4);
        let mask_0_a = _mm_movemask_epi8(mm_0_eq_a) as u16;
        let mask_0_b = _mm_movemask_epi8(mm_0_eq_b) as u16;
        let mask_0_c = _mm_movemask_epi8(mm_0_eq_c) as u16;
        let mask_0_d = _mm_movemask_epi8(mm_0_eq_d) as u16;
        let base = buf_ptr.usz_offset_from(st_ptr) + 16 - 1;
        //
        _return_rchr_qpl(base, mask_0_a, mask_0_b, mask_0_c, mask_0_d)
    }
}

#[inline(always)]
unsafe fn _rchr_qpl_c32_uu_x1(
    buf_ptr: *const u8,
    mm_c32: MMB32Qpl,
    st_ptr: *const u8,
) -> Option<usize> {
    unsafe {
        let mm_0 = _mm256_loadu_si256(buf_ptr as *const __m256i);
        let mm_0_eq_a = _mm256_cmpeq_epi8(mm_0, mm_c32.v1);
        let mm_0_eq_b = _mm256_cmpeq_epi8(mm_0, mm_c32.v2);
        let mm_0_eq_c = _mm256_cmpeq_epi8(mm_0, mm_c32.v3);
        let mm_0_eq_d = _mm256_cmpeq_epi8(mm_0, mm_c32.v4);
        let mask_0_a = _mm256_movemask_epi8(mm_0_eq_a) as u32;
        let mask_0_b = _mm256_movemask_epi8(mm_0_eq_b) as u32;
        let mask_0_c = _mm256_movemask_epi8(mm_0_eq_c) as u32;
        let mask_0_d = _mm256_movemask_epi8(mm_0_eq_d) as u32;
        let base = buf_ptr.usz_offset_from(st_ptr) + 32 - 1;
        //
        _return_rchr_qpl(base, mask_0_a, mask_0_b, mask_0_c, mask_0_d)
    }
}

#[inline(always)]
unsafe fn _rchr_qpl_c32_aa_x1(
    buf_ptr: *const u8,
    mm_c32: MMB32Qpl,
    st_ptr: *const u8,
) -> Option<usize> {
    unsafe {
        let mm_0 = _mm256_load_si256(buf_ptr as *const __m256i);
        let mm_0_eq_a = _mm256_cmpeq_epi8(mm_0, mm_c32.v1);
        let mm_0_eq_b = _mm256_cmpeq_epi8(mm_0, mm_c32.v2);
        let mm_0_eq_c = _mm256_cmpeq_epi8(mm_0, mm_c32.v3);
        let mm_0_eq_d = _mm256_cmpeq_epi8(mm_0, mm_c32.v4);
        let mask_0_a = _mm256_movemask_epi8(mm_0_eq_a) as u32;
        let mask_0_b = _mm256_movemask_epi8(mm_0_eq_b) as u32;
        let mask_0_c = _mm256_movemask_epi8(mm_0_eq_c) as u32;
        let mask_0_d = _mm256_movemask_epi8(mm_0_eq_d) as u32;
        let base = buf_ptr.usz_offset_from(st_ptr) + 32 - 1;
        //
        _return_rchr_qpl(base, mask_0_a, mask_0_b, mask_0_c, mask_0_d)
    }
}

