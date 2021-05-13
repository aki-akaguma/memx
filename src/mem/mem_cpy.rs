use super::super::RangeError;

#[inline(always)]
pub fn _memcpy_impl(dst: &mut [u8], src: &[u8], n: usize) -> Result<(), RangeError> {
    if dst.len() < n || src.len() < n {
        return Err(RangeError);
    }
    for i in 0..n {
        dst[i] = src[i];
    }
    Ok(())
}
