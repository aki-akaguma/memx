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
    #[cfg(target_env = "gnu")]
    if src_len <= 32 {
        unsafe {
            dst.as_mut_ptr()
                .copy_from_nonoverlapping(src.as_ptr(), src_len)
        };
        return Ok(());
    }
    _start_cpy_1(dst, src);
    /*
    #[cfg(target_arch = "arm")]
    {
        // measures to prevent bus errors
        #[cfg(target_pointer_width = "32")]
        let r = _start_cpy_32_no_unroll(dst, src);
        #[cfg(target_pointer_width = "16")]
        let r = _start_cpy_1(dst, src);
        //
        r
    };
    #[cfg(not(target_arch = "arm"))]
    {
        #[cfg(any(target_arch = "x86_64", target_arch = "x86", target_arch = "aarch64"))]
        _start_cpy_128(dst, src);
        #[cfg(not(any(target_arch = "x86_64", target_arch = "x86", target_arch = "aarch64")))]
        {
            #[cfg(target_pointer_width = "128")]
            _start_cpy_128(dst, src);
            #[cfg(target_pointer_width = "64")]
            _start_cpy_64(dst, src);
            #[cfg(target_pointer_width = "32")]
            _start_cpy_32(dst, src);
            #[cfg(target_pointer_width = "16")]
            _start_cpy_16(dst, src);
        };
    };
    */
    //
    Ok(())
}

#[cfg(any(
    target_pointer_width = "128",
    target_arch = "x86_64",
    target_arch = "x86",
    target_arch = "aarch64"
))]
#[inline(always)]
fn _start_cpy_128(dst: &mut [u8], src: &[u8]) {
    let src_len = src.len();
    let mut a_ptr = dst.as_mut_ptr();
    let mut b_ptr = src.as_ptr();
    let end_ptr = unsafe { b_ptr.add(src_len) };
    //
    let remaining_align = 0x10_usize - ((a_ptr as usize) & 0x0F_usize);
    dst[..remaining_align].copy_from_slice(&src[..remaining_align]);
    a_ptr = unsafe { a_ptr.add(remaining_align) };
    b_ptr = unsafe { b_ptr.add(remaining_align) };
    //
    /**/
    {
        let mut aa_ptr = a_ptr;
        let mut bb_ptr = b_ptr;
        while unsafe { bb_ptr.add(16) } < end_ptr {
            let aaa_ptr = aa_ptr as *mut u128;
            let bbc = unsafe {
                crate::utils::_read_a_native_endian_u128(core::slice::from_raw_parts(bb_ptr, 16))
            };
            unsafe { aaa_ptr.write(bbc) };
            aa_ptr = unsafe { aa_ptr.add(16) };
            bb_ptr = unsafe { bb_ptr.add(16) };
        }
        while bb_ptr < end_ptr {
            unsafe {
                *aa_ptr = *bb_ptr;
            }
            aa_ptr = unsafe { aa_ptr.add(1) };
            bb_ptr = unsafe { bb_ptr.add(1) };
        }
    }
}

#[cfg(any(
    target_pointer_width = "128",
    target_arch = "x86_64",
    target_arch = "x86",
    target_arch = "aarch64"
))]
#[inline(always)]
fn _start_cpy_64(dst: &mut [u8], src: &[u8]) {
    let src_len = src.len();
    let mut a_ptr = dst.as_mut_ptr();
    let mut b_ptr = src.as_ptr();
    let end_ptr = unsafe { b_ptr.add(src_len) };
    //
    let remaining_align = 0x09_usize - ((a_ptr as usize) & 0x08_usize);
    dst[..remaining_align].copy_from_slice(&src[..remaining_align]);
    a_ptr = unsafe { a_ptr.add(remaining_align) };
    b_ptr = unsafe { b_ptr.add(remaining_align) };
    //
    /**/
    {
        let mut aa_ptr = a_ptr;
        let mut bb_ptr = b_ptr;
        while unsafe { bb_ptr.add(8) } < end_ptr {
            let aaa_ptr = aa_ptr as *mut u64;
            let bbc = unsafe {
                crate::utils::_read_a_native_endian_u64(core::slice::from_raw_parts(bb_ptr, 8))
            };
            unsafe { aaa_ptr.write(bbc) };
            aa_ptr = unsafe { aa_ptr.add(8) };
            bb_ptr = unsafe { bb_ptr.add(8) };
        }
        while bb_ptr < end_ptr {
            unsafe {
                *aa_ptr = *bb_ptr;
            }
            aa_ptr = unsafe { aa_ptr.add(1) };
            bb_ptr = unsafe { bb_ptr.add(1) };
        }
    }
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

#[inline(always)]
pub(crate) fn _memcpy_remaining_15_bytes_impl(
    dst_ptr: *mut u8,
    src_ptr: *const u8,
    end_ptr: *const u8,
) -> Result<(), RangeError> {
    let mut a_ptr = dst_ptr;
    let mut b_ptr = src_ptr;
    {
        while b_ptr < end_ptr {
            unsafe {
                *a_ptr = *b_ptr;
            }
            //
            a_ptr = unsafe { a_ptr.add(1) };
            b_ptr = unsafe { b_ptr.add(1) };
        }
    }
    Ok(())
}

#[inline(always)]
pub(crate) fn _memcpy_remaining_7_bytes_impl(
    dst_ptr: *mut u8,
    src_ptr: *const u8,
    end_ptr: *const u8,
) -> Result<(), RangeError> {
    let mut a_ptr = dst_ptr;
    let mut b_ptr = src_ptr;
    {
        while b_ptr < end_ptr {
            unsafe {
                *a_ptr = *b_ptr;
            }
            //
            a_ptr = unsafe { a_ptr.add(1) };
            b_ptr = unsafe { b_ptr.add(1) };
        }
    }
    Ok(())
}

#[inline(always)]
pub(crate) fn _memcpy_remaining_3_bytes_impl(
    dst_ptr: *mut u8,
    src_ptr: *const u8,
    end_ptr: *const u8,
) -> Result<(), RangeError> {
    let mut a_ptr = dst_ptr;
    let mut b_ptr = src_ptr;
    {
        while b_ptr < end_ptr {
            unsafe {
                *a_ptr = *b_ptr;
            }
            //
            a_ptr = unsafe { a_ptr.add(1) };
            b_ptr = unsafe { b_ptr.add(1) };
        }
    }
    Ok(())
}
