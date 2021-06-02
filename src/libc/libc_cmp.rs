use core::cmp::Ordering;

#[allow(non_camel_case_types)]
type c_int = i32;

#[allow(non_camel_case_types)]
type size_t = usize;

extern "C" {
    fn memcmp(b1: *const u8, b2: *const u8, len: size_t) -> c_int;
}

#[inline(always)]
pub fn _memcmp_impl(a: &[u8], b: &[u8]) -> Ordering {
    let a_len = a.len();
    let b_len = b.len();
    let min_len = a_len.min(b_len);
    let a_ptr = a.as_ptr();
    let b_ptr = b.as_ptr();
    let r = unsafe { memcmp(a_ptr, b_ptr, min_len) };
    if r == 0 {
        if a_len == b_len {
            Ordering::Equal
        } else if a_len < b_len {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    } else if r < 0 {
        Ordering::Less
    } else {
        Ordering::Greater
    }
}
