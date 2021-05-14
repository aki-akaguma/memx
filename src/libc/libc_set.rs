use super::super::RangeError;

#[allow(non_camel_case_types)]
type c_int = i32;

#[allow(non_camel_case_types)]
type size_t = usize;

extern "C" {
    fn memset(s: *const u8, c: c_int, n: size_t) -> *const u8;
}

#[inline(always)]
pub fn _memset_impl(buf: &mut [u8], c: u8, n: usize) -> Result<(), RangeError> {
    if buf.len() < n {
        return Err(RangeError);
    }
    let buf_ptr = buf.as_ptr();
    unsafe { memset(buf_ptr, c as c_int, n) };
    Ok(())
}
