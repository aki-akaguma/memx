use crate::mem as basic;

#[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
use super::cpuid;

use core::sync::atomic::AtomicPtr;
use core::sync::atomic::Ordering;
type FuncType = fn(&[u8], &[u8]) -> Option<usize>;

const FUNC: FuncType = fnptr_setup_func;
static FUNC_PTR_ATOM: AtomicPtr<FuncType> = AtomicPtr::new(FUNC as *mut FuncType);

#[inline(never)]
fn fnptr_setup_func(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    #[cfg(target_arch = "x86_64")]
    let func = if cpuid::has_avx2() {
        _memrmem_avx2
    } else {
        _memrmem_sse2
    };
    #[cfg(target_arch = "x86")]
    let func = if cpuid::has_avx2() {
        _memrmem_avx2
    } else if cpuid::has_sse2() {
        _memrmem_sse2
    } else {
        _memrmem_basic
    };
    //
    FUNC_PTR_ATOM.store(func as *mut FuncType, Ordering::Relaxed);
    unsafe { func(haystack, needle) }
}

#[inline(always)]
pub(crate) fn _memrmem_impl(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    let func_u = FUNC_PTR_ATOM.load(Ordering::Relaxed);
    #[allow(clippy::crosspointer_transmute)]
    let func: FuncType = unsafe { core::mem::transmute(func_u) };
    func(haystack, needle)
}

unsafe fn _memrmem_basic(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    basic::_memrmem_impl(haystack, needle)
}

#[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
#[target_feature(enable = "sse2")]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn _memrmem_sse2(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    _memrmem_sse2_impl(haystack, needle)
}

#[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
#[target_feature(enable = "avx2")]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn _memrmem_avx2(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    _memrmem_avx2_impl(haystack, needle)
}

#[inline(always)]
fn _memrmem_sse2_impl(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    let hay_len = haystack.len();
    let nee_len = needle.len();
    if hay_len < nee_len {
        return None;
    }
    if nee_len == 0 {
        return Some(hay_len);
    }
    //
    let byte_1st = needle[0];
    let byte_last = needle[nee_len - 1];
    if byte_1st.is_ascii() && byte_last.is_ascii() {
        let weight_1st = crate::_ascii_stochas(byte_1st);
        let weight_last = crate::_ascii_stochas(byte_last);
        if weight_1st <= weight_last {
            _memrmem_sse2_impl_1st(haystack, needle)
        } else {
            _memrmem_sse2_impl_last(haystack, needle)
        }
    } else if byte_last.is_ascii() {
        _memrmem_sse2_impl_1st(haystack, needle)
    } else {
        _memrmem_sse2_impl_last(haystack, needle)
    }
}

#[inline(always)]
fn _memrmem_sse2_impl_1st(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    let hay_len = haystack.len();
    let nee_len = needle.len();
    let nee_1st_byte = needle[0];
    let mut curr_idx = hay_len - nee_len;
    while curr_idx > 0 {
        let r = unsafe { super::_memrchr_sse2(&haystack[..curr_idx], nee_1st_byte) };
        if let Some(pos) = r {
            let r_idx = pos;
            if crate::mem::_memeq_impl(&haystack[r_idx..(r_idx + nee_len)], needle) {
                return Some(r_idx);
            }
            curr_idx = pos;
        } else {
            return None;
        }
    }
    None
}

#[inline(always)]
fn _memrmem_sse2_impl_last(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    let hay_len = haystack.len();
    let nee_len = needle.len();
    let nee_last_idx = nee_len - 1;
    let nee_last_byte = needle[nee_last_idx];
    let mut curr_idx = hay_len;
    while curr_idx > 0 {
        let r = unsafe { super::_memrchr_sse2(&haystack[..curr_idx], nee_last_byte) };
        if let Some(pos) = r {
            if pos >= nee_last_idx {
                let r_idx = pos - nee_last_idx;
                if crate::mem::_memeq_impl(&haystack[r_idx..(r_idx + nee_len)], needle) {
                    return Some(r_idx);
                }
                curr_idx = pos;
            } else {
                return None;
            }
        } else {
            return None;
        }
    }
    None
}

#[inline(always)]
fn _memrmem_avx2_impl(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    let hay_len = haystack.len();
    let nee_len = needle.len();
    if hay_len < nee_len {
        return None;
    }
    if nee_len == 0 {
        return Some(hay_len);
    }
    //
    let byte_1st = needle[0];
    let byte_last = needle[nee_len - 1];
    if byte_1st.is_ascii() && byte_last.is_ascii() {
        let weight_1st = crate::_ascii_stochas(byte_1st);
        let weight_last = crate::_ascii_stochas(byte_last);
        if weight_1st <= weight_last {
            _memrmem_avx2_impl_1st(haystack, needle)
        } else {
            _memrmem_avx2_impl_last(haystack, needle)
        }
    } else if byte_last.is_ascii() {
        _memrmem_avx2_impl_1st(haystack, needle)
    } else {
        _memrmem_avx2_impl_last(haystack, needle)
    }
}

#[inline(always)]
fn _memrmem_avx2_impl_1st(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    let hay_len = haystack.len();
    let nee_len = needle.len();
    let nee_1st_byte = needle[0];
    let mut curr_idx = hay_len - nee_len;
    while curr_idx > 0 {
        let r = unsafe { super::_memrchr_avx2(&haystack[..curr_idx], nee_1st_byte) };
        if let Some(pos) = r {
            let r_idx = pos;
            if crate::mem::_memeq_impl(&haystack[r_idx..(r_idx + nee_len)], needle) {
                return Some(r_idx);
            }
            curr_idx = pos;
        } else {
            return None;
        }
    }
    None
}

#[inline(always)]
fn _memrmem_avx2_impl_last(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    let hay_len = haystack.len();
    let nee_len = needle.len();
    let nee_last_idx = nee_len - 1;
    let nee_last_byte = needle[nee_last_idx];
    let mut curr_idx = hay_len;
    while curr_idx > 0 {
        let r = unsafe { super::_memrchr_avx2(&haystack[..curr_idx], nee_last_byte) };
        if let Some(pos) = r {
            if pos >= nee_last_idx {
                let r_idx = pos - nee_last_idx;
                if crate::mem::_memeq_impl(&haystack[r_idx..(r_idx + nee_len)], needle) {
                    return Some(r_idx);
                }
                curr_idx = pos;
            } else {
                return None;
            }
        } else {
            return None;
        }
    }
    None
}
