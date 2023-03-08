use super::super::RangeError;

#[inline(always)]
pub fn _memcpy_impl(dst: &mut [u8], src: &[u8]) -> Result<(), RangeError> {
    if src.is_empty() {
        return Ok(());
    }
    let dst_len = dst.len();
    let src_len = src.len();
    if dst_len < src_len {
        return Err(RangeError);
    }
    #[cfg(all(target_env = "gnu", not(tarpaulin)))]
    if src_len <= 32 {
        unsafe {
            dst.as_mut_ptr()
                .copy_from_nonoverlapping(src.as_ptr(), src_len)
        };
        return Ok(());
    }
    _start_cpy_1(dst, src);
    //
    Ok(())
}

#[inline(always)]
fn _start_cpy_1(dst: &mut [u8], src: &[u8]) {
    let src_len = src.len();
    let a_ptr = dst.as_mut_ptr();
    let b_ptr = src.as_ptr();
    let end_ptr = unsafe { b_ptr.add(src_len) };
    //
    {
        let mut aa_ptr = a_ptr;
        let mut bb_ptr = b_ptr;
        while bb_ptr < end_ptr {
            unsafe {
                *aa_ptr = *bb_ptr;
            }
            aa_ptr = unsafe { aa_ptr.add(1) };
            bb_ptr = unsafe { bb_ptr.add(1) };
        }
    }
}

// You need this function that does nothing because of the bug of tarpaulin.
// Without this, the line count in other files will be strange.
#[cfg(tarpaulin)]
fn _start_cpy_64(dst: &mut [u8], src: &[u8]) {
    dst[..src.len()].copy_from_slice(src);
}
