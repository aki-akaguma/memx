#[allow(non_camel_case_types)]
type c_int = i32;

#[allow(non_camel_case_types)]
type size_t = usize;

extern "C" {
    fn memcmp(b1: *const u8, b2: *const u8, len: size_t) -> c_int;
}

#[inline(always)]
pub fn _memeq_impl(a: &[u8], b: &[u8]) -> bool {
    let a_len = a.len();
    let b_len = b.len();
    if a_len != b_len {
        return false;
    }
    let a_ptr = a.as_ptr();
    let b_ptr = b.as_ptr();
    let r = unsafe { memcmp(a_ptr, b_ptr, a_len) };
    r == 0
}
