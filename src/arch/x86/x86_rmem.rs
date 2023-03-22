use crate::mem as basic;

#[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
use super::cpuid;

#[inline(always)]
#[cfg(target_arch = "x86_64")]
pub(crate) fn _memrmem_impl(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    // TODO: Replace with https://github.com/rust-lang/rfcs/pull/2725
    // after stabilization
    if cpuid::has_avx2() {
        unsafe { _memrmem_avx2(haystack, needle) }
    } else {
        unsafe { _memrmem_sse2(haystack, needle) }
    }
}

#[inline(always)]
#[cfg(target_arch = "x86")]
pub(crate) fn _memrmem_impl(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    // TODO: Replace with https://github.com/rust-lang/rfcs/pull/2725
    // after stabilization
    if cpuid::has_avx2() {
        unsafe { _memrmem_avx2(haystack, needle) }
    } else if cpuid::has_sse2() {
        unsafe { _memrmem_sse2(haystack, needle) }
    } else {
        _memrmem_basic(haystack, needle)
    }
}

fn _memrmem_basic(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    basic::_memrmem_impl(haystack, needle)
}

#[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
#[target_feature(enable = "sse2")]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn _memrmem_sse2(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    _memrmem_sse2_avx2_impl(haystack, needle)
}

#[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
#[target_feature(enable = "avx2")]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn _memrmem_avx2(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    _memrmem_sse2_avx2_impl(haystack, needle)
}

#[cfg(any(target_feature = "sse2", target_feature = "avx2"))]
#[inline(always)]
fn _memrmem_sse2_avx2_impl(haystack: &[u8], needle: &[u8]) -> Option<usize> {
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
        let weight_1st = crate::_ASCII_STOCHAS[byte_1st as usize];
        let weight_last = crate::_ASCII_STOCHAS[byte_last as usize];
        if weight_1st <= weight_last {
            _memrmem_sse2_avx2_impl_1st(haystack, needle)
        } else {
            _memrmem_sse2_avx2_impl_last(haystack, needle)
        }
    } else if byte_last.is_ascii() {
        _memrmem_sse2_avx2_impl_1st(haystack, needle)
    } else {
        _memrmem_sse2_avx2_impl_last(haystack, needle)
    }
}

#[cfg(any(target_feature = "sse2", target_feature = "avx2"))]
#[inline(always)]
fn _memrmem_sse2_avx2_impl_1st(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    let hay_len = haystack.len();
    let nee_len = needle.len();
    let nee_1st_byte = needle[0];
    let mut curr_idx = hay_len - nee_len;
    while curr_idx > 0 {
        // TODO: Replace with https://github.com/rust-lang/rfcs/pull/2725
        // after stabilization
        let r = if cpuid::has_avx2() {
            unsafe { super::_memrchr_avx2(&haystack[..curr_idx], nee_1st_byte) }
        } else {
            unsafe { super::_memrchr_sse2(&haystack[..curr_idx], nee_1st_byte) }
        };
        /*
        #[cfg(target_feature = "avx")]
        let r = unsafe { super::_memrchr_avx(&haystack[..curr_idx], nee_1st_byte) };
        #[cfg(all(target_feature = "sse2", not(target_feature = "avx")))]
        let r = unsafe { super::_memrchr_sse2(&haystack[..curr_idx], nee_1st_byte) };
        */
        //
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

#[cfg(any(target_feature = "sse2", target_feature = "avx2"))]
#[inline(always)]
fn _memrmem_sse2_avx2_impl_last(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    let hay_len = haystack.len();
    let nee_len = needle.len();
    let nee_last_idx = nee_len - 1;
    let nee_last_byte = needle[nee_last_idx];
    let mut curr_idx = hay_len;
    while curr_idx > 0 {
        // TODO: Replace with https://github.com/rust-lang/rfcs/pull/2725
        // after stabilization
        let r = if cpuid::has_avx2() {
            unsafe { super::_memrchr_avx2(&haystack[..curr_idx], nee_last_byte) }
        } else {
            unsafe { super::_memrchr_sse2(&haystack[..curr_idx], nee_last_byte) }
        };
        /*
        #[cfg(target_feature = "avx")]
        let r = unsafe { super::_memrchr_avx(&haystack[..curr_idx], nee_last_byte) };
        #[cfg(all(target_feature = "sse2", not(target_feature = "avx")))]
        let r = unsafe { super::_memrchr_sse2(&haystack[..curr_idx], nee_last_byte) };
        */
        //
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
