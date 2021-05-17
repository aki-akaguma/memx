use super::super::RangeError;

#[inline(always)]
pub fn _memcpy_impl(dst: &mut [u8], src: &[u8]) -> Result<(), RangeError> {
    let dst_len = dst.len();
    let src_len = src.len();
    if dst_len < src_len {
        return Err(RangeError);
    }
    let mut dst_ptr = dst.as_mut_ptr();
    let mut src_ptr = src.as_ptr();
    let end_ptr = unsafe { dst_ptr.add(src_len) };
    //
    #[cfg(target_pointer_width = "64")]
    {
        let loop_size = 8;
        while dst_ptr <= unsafe { end_ptr.sub(loop_size) } {
            let dst_ptr_c8 = dst_ptr as *mut u64;
            let src_ptr_c8 = src_ptr as *const u64;
            unsafe {
                *dst_ptr_c8 = *src_ptr_c8;
            }
            dst_ptr = unsafe { dst_ptr.add(loop_size) };
            src_ptr = unsafe { src_ptr.add(loop_size) };
        }
    }
    //
    let loop_size = 4;
    while dst_ptr <= unsafe { end_ptr.sub(loop_size) } {
        let dst_ptr_c4 = dst_ptr as *mut u32;
        let src_ptr_c4 = src_ptr as *const u32;
        unsafe {
            *dst_ptr_c4 = *src_ptr_c4;
        }
        dst_ptr = unsafe { dst_ptr.add(loop_size) };
        src_ptr = unsafe { src_ptr.add(loop_size) };
    }
    /*
    let loop_size = 2;
    while dst_ptr <= unsafe { end_ptr.sub(loop_size) } {
        let dst_ptr_c2 = dst_ptr as *mut u16;
        let src_ptr_c2 = src_ptr as *const u16;
        unsafe {
            *dst_ptr_c2 = *src_ptr_c2;
        }
        dst_ptr = unsafe { dst_ptr.add(loop_size) };
        src_ptr = unsafe { src_ptr.add(loop_size) };
    }
    */
    //
    while dst_ptr < end_ptr {
        unsafe {
            *dst_ptr = *src_ptr;
        }
        dst_ptr = unsafe { dst_ptr.add(1) };
        src_ptr = unsafe { src_ptr.add(1) };
    }
    Ok(())
}

/*
 * The simple implement:

#[inline(always)]
pub fn _memcpy_impl(dst: &mut [u8], src: &[u8]) -> Result<(), RangeError> {
    if dst.len() < src.len() {
        return Err(RangeError);
    }
    for i in 0..src.len() {
        dst[i] = src[i];
    }
    Ok(())
}

#[inline(always)]
pub fn _memcpy_impl(dst: &mut [u8], src: &[u8]) -> Result<(), RangeError> {
    let dst_len = dst.len();
    let src_len = src.len();
    if dst_len < src_len {
        return Err(RangeError);
    }
    let mut dst_ptr = dst.as_mut_ptr();
    let mut src_ptr = src.as_ptr();
    let end_ptr = unsafe { dst_ptr.add(src_len) };
    while dst_ptr < end_ptr {
        unsafe {
            *dst_ptr = *src_ptr;
        }
        dst_ptr = unsafe { dst_ptr.add(1) };
        src_ptr = unsafe { src_ptr.add(1) };
    }
    Ok(())
}
*/
