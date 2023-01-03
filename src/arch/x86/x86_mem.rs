use crate::mem as basic;

#[cfg(any(target_feature = "sse2", target_feature = "avx"))]
use super::cpuid_avx;

#[inline(always)]
#[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
pub fn _memmem_impl(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    #[cfg(any(target_feature = "sse2", target_feature = "avx"))]
    let r = unsafe { _memmem_sse2_avx(haystack, needle) };
    #[cfg(not(any(target_feature = "avx", target_feature = "sse2")))]
    let r = _memmem_basic(haystack, needle);
    r
}

fn _memmem_basic(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    basic::_memmem_impl(haystack, needle)
}

#[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
#[cfg(any(target_feature = "sse2", target_feature = "avx"))]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn _memmem_sse2_avx(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    _memmem_sse2_avx_impl(haystack, needle)
}

#[cfg(any(target_feature = "sse2", target_feature = "avx"))]
#[inline(always)]
fn _memmem_sse2_avx_impl(haystack: &[u8], needle: &[u8]) -> Option<usize> {
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
            _memmem_sse2_avx_impl_1st(haystack, needle)
        } else {
            _memmem_sse2_avx_impl_last(haystack, needle)
        }
    } else if byte_last.is_ascii() {
        _memmem_sse2_avx_impl_1st(haystack, needle)
    } else {
        _memmem_sse2_avx_impl_last(haystack, needle)
    }
}

#[cfg(any(target_feature = "sse2", target_feature = "avx"))]
#[inline(always)]
fn _memmem_sse2_avx_impl_1st(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    let hay_len = haystack.len();
    let nee_len = needle.len();
    let nee_1st_byte = needle[0];
    let mut curr_idx = 0;
    while curr_idx < hay_len {
        // TODO: Replace with https://github.com/rust-lang/rfcs/pull/2725
        // after stabilization
        let r = if cpuid_avx::get() {
            unsafe { super::_memchr_avx(&haystack[curr_idx..], nee_1st_byte) }
        } else {
            unsafe { super::_memchr_sse2(&haystack[curr_idx..], nee_1st_byte) }
        };
        /*
        #[cfg(target_feature = "avx")]
        let r = unsafe { super::_memchr_avx(&haystack[curr_idx..], nee_1st_byte) };
        #[cfg(all(target_feature = "sse2", not(target_feature = "avx")))]
        let r = unsafe { super::_memchr_sse2(&haystack[curr_idx..], nee_1st_byte) };
        */
        //
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

#[cfg(any(target_feature = "sse2", target_feature = "avx"))]
#[inline(always)]
fn _memmem_sse2_avx_impl_last(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    let hay_len = haystack.len();
    let nee_len = needle.len();
    let nee_last_idx = nee_len - 1;
    let nee_last_byte = needle[nee_last_idx];
    let mut curr_idx = nee_last_idx;
    while curr_idx < hay_len {
        // TODO: Replace with https://github.com/rust-lang/rfcs/pull/2725
        // after stabilization
        let r = if cpuid_avx::get() {
            unsafe { super::_memchr_avx(&haystack[curr_idx..], nee_last_byte) }
        } else {
            unsafe { super::_memchr_sse2(&haystack[curr_idx..], nee_last_byte) }
        };
        /*
        #[cfg(target_feature = "avx")]
        let r = unsafe { super::_memchr_avx(&haystack[curr_idx..], nee_last_byte) };
        #[cfg(all(target_feature = "sse2", not(target_feature = "avx")))]
        let r = unsafe { super::_memchr_sse2(&haystack[curr_idx..], nee_last_byte) };
        */
        //
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
