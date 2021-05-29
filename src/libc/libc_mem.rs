use crate::plus_offset_from;

#[allow(non_camel_case_types)]
type size_t = usize;

extern "C" {
    fn memmem(
        haystack: *const u8,
        haystacklen: size_t,
        needle: *const u8,
        needlelen: size_t,
    ) -> *const u8;
}

#[inline(always)]
pub fn _memmem_impl(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    let hay_len = haystack.len();
    let nee_len = needle.len();
    let hay_ptr = haystack.as_ptr();
    let nee_ptr = needle.as_ptr();
    let r_ptr = unsafe { memmem(hay_ptr, hay_len, nee_ptr, nee_len) };
    if r_ptr.is_null() {
        None
    } else {
        Some(plus_offset_from(r_ptr, hay_ptr))
    }
}
