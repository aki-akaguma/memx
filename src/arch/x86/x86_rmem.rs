use crate::mem as basic;

#[inline(always)]
#[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
pub fn _memrmem_impl(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    #[cfg(target_feature = "avx")]
    let r = unsafe { _memrmem_avx(haystack, needle) };
    #[cfg(target_feature = "sse2")]
    let r = unsafe { _memrmem_sse2(haystack, needle) };
    #[cfg(not(any(target_feature = "avx", target_feature = "sse2")))]
    let r = _memrmem_basic(haystack, needle);
    r
}

fn _memrmem_basic(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    basic::_memrmem_impl(haystack, needle)
}

#[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
#[target_feature(enable = "sse2")]
pub unsafe fn _memrmem_sse2(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    _memrmem_sse2_impl(haystack, needle)
}

#[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
#[target_feature(enable = "avx")]
unsafe fn _memrmem_avx(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    _memrmem_avx_impl(haystack, needle)
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
        let weight_1st = crate::_ASCII_STOCHAS[byte_1st as usize];
        let weight_last = crate::_ASCII_STOCHAS[byte_last as usize];
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
fn _memrmem_avx_impl(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    let hay_len = haystack.len();
    let nee_len = needle.len();
    if hay_len < nee_len {
        return None;
    }
    let nee_last_idx = nee_len - 1;
    let nee_last_byte = needle[nee_last_idx];
    let mut curr_idx = hay_len;
    while curr_idx > 0 {
        let r = unsafe { super::_memrchr_avx(&haystack[..curr_idx], nee_last_byte) };
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
