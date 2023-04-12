use crate::utils::_ascii_stochas;

#[inline(always)]
pub fn _memrmem_impl(haystack: &[u8], needle: &[u8]) -> Option<usize> {
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
        let weight_1st = _ascii_stochas(byte_1st);
        let weight_last = _ascii_stochas(byte_last);
        if weight_1st <= weight_last {
            _memrmem_impl_1st(haystack, needle)
        } else {
            _memrmem_impl_last(haystack, needle)
        }
    } else if byte_last.is_ascii() {
        _memrmem_impl_1st(haystack, needle)
    } else {
        _memrmem_impl_last(haystack, needle)
    }
}

#[inline(always)]
fn _memrmem_impl_1st(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    let hay_len = haystack.len();
    let nee_len = needle.len();
    let nee_1st_byte = needle[0];
    let mut curr_idx = hay_len - nee_len;
    while curr_idx > 0 {
        let r = super::_memrchr_impl(&haystack[..curr_idx], nee_1st_byte);
        if let Some(pos) = r {
            let r_idx = pos;
            if super::_memeq_impl(&haystack[r_idx..(r_idx + nee_len)], needle) {
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
fn _memrmem_impl_last(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    let hay_len = haystack.len();
    let nee_len = needle.len();
    let nee_last_idx = nee_len - 1;
    let nee_last_byte = needle[nee_last_idx];
    let mut curr_idx = hay_len;
    while curr_idx > 0 {
        let r = super::_memrchr_impl(&haystack[..curr_idx], nee_last_byte);
        if let Some(pos) = r {
            if pos >= nee_last_idx {
                let r_idx = pos - nee_last_idx;
                if super::_memeq_impl(&haystack[r_idx..(r_idx + nee_len)], needle) {
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

/*
 * Reference:
 * The naive optimize algorithm for utf8 sequence.
 *   https://github.com/aki-akaguma/naive_opt
*/

/*
 * The simple implement:

#[inline(always)]
pub fn _memrmem_impl(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    let hay_len = haystack.len();
    let nee_len = needle.len();
    if hay_len < nee_len {
        return None;
    }
    let max_i = hay_len - nee_len;
    for i in 0..max_i {
        let j = max_i - i;
        if &haystack[j..(j + nee_len)] == needle {
            return Some(j);
        }
    }
    None
}
*/
