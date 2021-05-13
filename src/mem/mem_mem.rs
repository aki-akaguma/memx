
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
