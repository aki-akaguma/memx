use super::super::RangeError;

#[allow(non_camel_case_types)]
type size_t = usize;

extern "C" {
    fn memcpy(dst: *const u8, src: *const u8, len: size_t) -> *const u8;
}

#[inline(always)]
pub fn _memcpy_impl(dst: &mut [u8], src: &[u8]) -> Result<(), RangeError> {
    if dst.len() < src.len() {
        return Err(RangeError);
    }
    let dst_ptr = dst.as_ptr();
    let src_ptr = src.as_ptr();
    unsafe { memcpy(dst_ptr, src_ptr, src.len()) };
    Ok(())
}
