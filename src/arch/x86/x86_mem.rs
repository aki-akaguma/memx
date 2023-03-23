use crate::mem as basic;

#[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
use super::cpuid;

#[inline(always)]
#[cfg(target_arch = "x86_64")]
pub(crate) fn _memmem_impl(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    // TODO: Replace with https://github.com/rust-lang/rfcs/pull/2725
    // after stabilization
    if cpuid::has_avx2() {
        unsafe { _memmem_avx2(haystack, needle) }
    } else {
        unsafe { _memmem_sse2(haystack, needle) }
    }
}

#[inline(always)]
#[cfg(target_arch = "x86")]
pub(crate) fn _memmem_impl(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    // TODO: Replace with https://github.com/rust-lang/rfcs/pull/2725
    // after stabilization
    if cpuid::has_avx2() {
        unsafe { _memmem_avx2(haystack, needle) }
    } else if cpuid::has_sse2() {
        unsafe { _memmem_sse2(haystack, needle) }
    } else {
        _memmem_basic(haystack, needle)
    }
}

fn _memmem_basic(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    basic::_memmem_impl(haystack, needle)
}

#[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
#[target_feature(enable = "sse2")]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn _memmem_sse2(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    _memmem_sse2_impl(haystack, needle)
}

#[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
#[target_feature(enable = "avx2")]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn _memmem_avx2(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    _memmem_avx2_impl(haystack, needle)
}

#[inline(always)]
fn _memmem_sse2_impl(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    let hay_len = haystack.len();
    let nee_len = needle.len();
    if hay_len < nee_len {
        return None;
    }
    if nee_len == 0 {
        return Some(0);
    }
    //
    let byte_1st = needle[0];
    let byte_last = needle[nee_len - 1];
    if byte_1st.is_ascii() && byte_last.is_ascii() {
        let weight_1st = crate::_ASCII_STOCHAS[byte_1st as usize];
        let weight_last = crate::_ASCII_STOCHAS[byte_last as usize];
        if weight_1st <= weight_last {
            _memmem_sse2_impl_1st(haystack, needle)
        } else {
            _memmem_sse2_impl_last(haystack, needle)
        }
    } else if byte_last.is_ascii() {
        _memmem_sse2_impl_1st(haystack, needle)
    } else {
        _memmem_sse2_impl_last(haystack, needle)
    }
}

#[inline(always)]
fn _memmem_sse2_impl_1st(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    let hay_len = haystack.len();
    let nee_len = needle.len();
    let nee_1st_byte = needle[0];
    let mut curr_idx = 0;
    while curr_idx < hay_len {
        let r = unsafe { super::_memchr_sse2(&haystack[curr_idx..], nee_1st_byte) };
        if let Some(pos) = r {
            let r_idx = curr_idx + pos;
            if r_idx + nee_len > hay_len {
                break;
            }
            if crate::mem::_memeq_impl(&haystack[r_idx..(r_idx + nee_len)], needle) {
                return Some(r_idx);
            }
            curr_idx = curr_idx + pos + 1;
        } else {
            return None;
        }
    }
    None
}

#[inline(always)]
fn _memmem_sse2_impl_last(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    let hay_len = haystack.len();
    let nee_len = needle.len();
    let nee_last_idx = nee_len - 1;
    let nee_last_byte = needle[nee_last_idx];
    let mut curr_idx = nee_last_idx;
    while curr_idx < hay_len {
        let r = unsafe { super::_memchr_sse2(&haystack[curr_idx..], nee_last_byte) };
        if let Some(pos) = r {
            let r_idx = curr_idx + pos - nee_last_idx;
            if crate::mem::_memeq_impl(&haystack[r_idx..(r_idx + nee_len)], needle) {
                return Some(r_idx);
            }
            curr_idx = curr_idx + pos + 1;
        } else {
            return None;
        }
    }
    None
}

#[inline(always)]
fn _memmem_avx2_impl(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    let hay_len = haystack.len();
    let nee_len = needle.len();
    if hay_len < nee_len {
        return None;
    }
    if nee_len == 0 {
        return Some(0);
    }
    //
    let byte_1st = needle[0];
    let byte_last = needle[nee_len - 1];
    if byte_1st.is_ascii() && byte_last.is_ascii() {
        let weight_1st = crate::_ASCII_STOCHAS[byte_1st as usize];
        let weight_last = crate::_ASCII_STOCHAS[byte_last as usize];
        if weight_1st <= weight_last {
            _memmem_avx2_impl_1st(haystack, needle)
        } else {
            _memmem_avx2_impl_last(haystack, needle)
        }
    } else if byte_last.is_ascii() {
        _memmem_avx2_impl_1st(haystack, needle)
    } else {
        _memmem_avx2_impl_last(haystack, needle)
    }
}

#[inline(always)]
fn _memmem_avx2_impl_1st(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    let hay_len = haystack.len();
    let nee_len = needle.len();
    let nee_1st_byte = needle[0];
    let mut curr_idx = 0;
    while curr_idx < hay_len {
        let r = unsafe { super::_memchr_avx2(&haystack[curr_idx..], nee_1st_byte) };
        if let Some(pos) = r {
            let r_idx = curr_idx + pos;
            if r_idx + nee_len > hay_len {
                break;
            }
            if crate::mem::_memeq_impl(&haystack[r_idx..(r_idx + nee_len)], needle) {
                return Some(r_idx);
            }
            curr_idx = curr_idx + pos + 1;
        } else {
            return None;
        }
    }
    None
}

#[inline(always)]
fn _memmem_avx2_impl_last(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    let hay_len = haystack.len();
    let nee_len = needle.len();
    let nee_last_idx = nee_len - 1;
    let nee_last_byte = needle[nee_last_idx];
    let mut curr_idx = nee_last_idx;
    while curr_idx < hay_len {
        let r = unsafe { super::_memchr_avx2(&haystack[curr_idx..], nee_last_byte) };
        if let Some(pos) = r {
            let r_idx = curr_idx + pos - nee_last_idx;
            if crate::mem::_memeq_impl(&haystack[r_idx..(r_idx + nee_len)], needle) {
                return Some(r_idx);
            }
            curr_idx = curr_idx + pos + 1;
        } else {
            return None;
        }
    }
    None
}
