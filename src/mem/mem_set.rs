use super::super::RangeError;

#[inline(always)]
pub fn _memset_impl(buf: &mut [u8], c: u8, n: usize) -> Result<(), RangeError> {
    if buf.len() < n {
        return Err(RangeError);
    }
    for i in 0..n {
        buf[i] = c;
        //
        // should not use unsafe for auto-vector
        // let item = unsafe { buf.get_unchecked_mut(i) };
        // *item = c;
        //
    }
    Ok(())
}
