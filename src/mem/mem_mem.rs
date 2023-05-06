use crate::utils::B1Sgl;
use crate::utils::_ascii_stochas;

#[inline(never)]
pub fn _memmem_impl(haystack: &[u8], needle: &[u8]) -> Option<usize> {
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
        let weight_1st = _ascii_stochas(byte_1st);
        let weight_last = _ascii_stochas(byte_last);
        if weight_1st <= weight_last {
            _memmem_impl_1st(haystack, needle)
        } else {
            _memmem_impl_last(haystack, needle)
        }
    } else if byte_last.is_ascii() {
        _memmem_impl_1st(haystack, needle)
    } else {
        _memmem_impl_last(haystack, needle)
    }
}

#[inline(always)]
fn _memmem_impl_1st(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    let hay_len = haystack.len();
    let nee_len = needle.len();
    let nee_1st_byte = needle[0];
    let mut curr_idx = 0;
    while curr_idx < hay_len {
        let r = super::_memchr_impl(&haystack[curr_idx..], B1Sgl::new(nee_1st_byte));
        if let Some(pos) = r {
            let r_idx = curr_idx + pos;
            if r_idx + nee_len > hay_len {
                break;
            }
            if super::_memeq_impl(&haystack[r_idx..(r_idx + nee_len)], needle) {
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
fn _memmem_impl_last(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    let hay_len = haystack.len();
    let nee_len = needle.len();
    let nee_last_idx = nee_len - 1;
    let nee_last_byte = needle[nee_last_idx];
    let mut curr_idx = nee_last_idx;
    while curr_idx < hay_len {
        let r = super::_memchr_impl(&haystack[curr_idx..], B1Sgl::new(nee_last_byte));
        if let Some(pos) = r {
            let r_idx = curr_idx + pos - nee_last_idx;
            if super::_memeq_impl(&haystack[r_idx..(r_idx + nee_len)], needle) {
                return Some(r_idx);
            }
            curr_idx = curr_idx + pos + 1;
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
pub fn _memmem_impl(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    let hay_len = haystack.len();
    let nee_len = needle.len();
    if hay_len < nee_len {
        return None;
    }
    for i in 0..(hay_len - nee_len) {
        if &haystack[i..(i + nee_len)] == needle {
            return Some(i);
        }
    }
    None
}
*/
