#[inline(always)]
pub fn _memset_impl(buf: &mut [u8], c: u8) {
    if buf.is_empty() {
        return;
    }
    // `_start_set_1()` is a fastest by compiler optimaization
    _start_set_1(buf, c);
}

#[inline(always)]
fn _start_set_1(buf: &mut [u8], c: u8) {
    let buf_len = buf.len();
    let mut a_ptr = buf.as_mut_ptr();
    let end_ptr = unsafe { a_ptr.add(buf_len) };
    //
    while a_ptr < end_ptr {
        unsafe { *a_ptr = c };
        a_ptr = unsafe { a_ptr.add(1) };
    }
}

#[inline(always)]
pub(crate) fn _memset_remaining_15_bytes_impl(buf_ptr: *const u8, cc: u128, end_ptr: *const u8) {
    let mut a_ptr = buf_ptr;
    {
        let loop_size = 8;
        if unsafe { end_ptr.offset_from(a_ptr) } >= loop_size as isize {
            let cc: u64 = cc as u64;
            let aa_ptr = a_ptr as *mut u64;
            unsafe { aa_ptr.write_unaligned(cc) };
            //
            a_ptr = unsafe { a_ptr.add(loop_size) };
        }
    }
    // the remaining data is the max: 7 bytes.
    _memset_remaining_7_bytes_impl(a_ptr, cc as u64, end_ptr)
}

#[inline(always)]
pub(crate) fn _memset_remaining_7_bytes_impl(buf_ptr: *const u8, cc: u64, end_ptr: *const u8) {
    let mut a_ptr = buf_ptr;
    {
        let loop_size = 4;
        if unsafe { end_ptr.offset_from(a_ptr) } >= loop_size as isize {
            let cc: u32 = cc as u32;
            let aa_ptr = a_ptr as *mut u32;
            unsafe { aa_ptr.write_unaligned(cc) };
            //
            a_ptr = unsafe { a_ptr.add(loop_size) };
        }
    }
    // the remaining data is the max: 3 bytes.
    _memset_remaining_3_bytes_impl(a_ptr, cc as u32, end_ptr)
}

#[inline(always)]
pub(crate) fn _memset_remaining_3_bytes_impl(buf_ptr: *const u8, cc: u32, end_ptr: *const u8) {
    let mut a_ptr = buf_ptr;
    {
        let loop_size = 2;
        if unsafe { end_ptr.offset_from(a_ptr) } >= loop_size as isize {
            let cc: u16 = cc as u16;
            let aa_ptr = a_ptr as *mut u16;
            unsafe { aa_ptr.write_unaligned(cc) };
            //
            a_ptr = unsafe { a_ptr.add(loop_size) };
        }
    }
    {
        let loop_size = 1;
        if unsafe { end_ptr.offset_from(a_ptr) } >= loop_size as isize {
            let c: u8 = cc as u8;
            let aa_ptr = a_ptr as *mut u8;
            unsafe { *aa_ptr = c };
        }
    }
}

/*
 * The simple implement:

#[inline(always)]
pub fn _memset_impl(buf: &mut [u8], c: u8, n: usize) -> Result<(), RangeError> {
    if buf.len() < n {
        return Err(RangeError);
    }
    for i in 0..n {
        buf[i] = c;
    }
    Ok(())
}
*/

#[cfg(test)]
mod disasm {
    use super::*;
    //
    #[test]
    fn do_procs() {
        let mut a = b"abcdefg".to_vec();
        let a = a.as_mut_slice();
        let c = b'A';
        do_proc_basic(a, c);
    }
    #[inline(never)]
    fn do_proc_basic(a: &mut [u8], c: u8) {
        _memset_impl(a, c)
    }
}
