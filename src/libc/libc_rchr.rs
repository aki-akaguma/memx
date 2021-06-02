use crate::plus_offset_from;

#[allow(non_camel_case_types)]
type c_int = i32;

#[allow(non_camel_case_types)]
type size_t = usize;

extern "C" {
    fn memrchr(s: *const u8, c: c_int, len: size_t) -> *const u8;
}

#[inline(always)]
pub(crate) fn _memrchr_impl(buf: &[u8], c: u8) -> Option<usize> {
    let buf_len = buf.len();
    let buf_ptr = buf.as_ptr();
    let r_ptr = unsafe { memrchr(buf_ptr, c.into(), buf_len) };
    if r_ptr.is_null() {
        None
    } else {
        Some(plus_offset_from(r_ptr, buf_ptr))
    }
}
