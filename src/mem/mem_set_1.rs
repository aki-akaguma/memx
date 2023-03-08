#![tarpaulin::skip]

#[inline(always)]
pub fn _memset_impl(buf: &mut [u8], c: u8) {
    if buf.is_empty() {
        return;
    }
    #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
    {
        _start_set_1(buf, c);
    }
    #[cfg(not(any(target_arch = "x86_64", target_arch = "x86")))]
    {
        #[cfg(target_pointer_width = "128")]
        _start_set_64(buf, c);
        #[cfg(target_pointer_width = "64")]
        _start_set_64(buf, c);
        #[cfg(target_pointer_width = "32")]
        _start_set_32(buf, c);
        #[cfg(target_pointer_width = "16")]
        _start_set_1(buf, c);
    }
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

fn _align_unroll_one_1(a_ptr: *mut u8, cc: u64, offset: usize) {
    let aa_ptr = unsafe { a_ptr.add(offset) } as *mut u8;
    unsafe { *aa_ptr = cc as u8 };
}

fn _align_unroll_one_2(a_ptr: *mut u8, cc: u64, offset: usize) {
    let aa_ptr = unsafe { a_ptr.add(offset) } as *mut u16;
    unsafe { *aa_ptr = cc as u16 };
}

fn _align_unroll_one_4(a_ptr: *mut u8, cc: u64, offset: usize) {
    let aa_ptr = unsafe { a_ptr.add(offset) } as *mut u32;
    unsafe { *aa_ptr = cc as u32 };
}

fn _align_unroll_one_8(a_ptr: *mut u8, cc: u64, offset: usize) {
    let aa_ptr = unsafe { a_ptr.add(offset) } as *mut u64;
    unsafe { *aa_ptr = cc };
}

#[inline(always)]
fn _align_2(a_ptr: *mut u8, cc: u64) -> *mut u8 {
    let mut a_ptr = a_ptr;
    let align = 0x02_usize - ((a_ptr as usize) & 0x01_usize);
    match align {
        1 => {
            _align_unroll_one_1(a_ptr, cc, 0);
            a_ptr = unsafe { a_ptr.add(1) };
        }
        2 => {}
        _ => unreachable!(),
    }
    a_ptr
}

#[inline(always)]
fn _align_4(a_ptr: *mut u8, cc: u64) -> *mut u8 {
    let mut a_ptr = a_ptr;
    let align = 0x04_usize - ((a_ptr as usize) & 0x03_usize);
    match align {
        1 => {
            _align_unroll_one_1(a_ptr, cc, 0);
            a_ptr = unsafe { a_ptr.add(1) };
        }
        2 => {
            _align_unroll_one_2(a_ptr, cc, 0);
            a_ptr = unsafe { a_ptr.add(2) };
        }
        3 => {
            _align_unroll_one_1(a_ptr, cc, 0);
            _align_unroll_one_2(a_ptr, cc, 1);
            a_ptr = unsafe { a_ptr.add(1 + 2) };
        }
        4 => {}
        _ => unreachable!(),
    }
    a_ptr
}

#[inline(always)]
fn _align_8(a_ptr: *mut u8, cc: u64) -> *mut u8 {
    let mut a_ptr = a_ptr;
    let align = 0x08_usize - ((a_ptr as usize) & 0x07_usize);
    match align {
        1 => {
            _align_unroll_one_1(a_ptr, cc, 0);
            a_ptr = unsafe { a_ptr.add(1) };
        }
        2 => {
            _align_unroll_one_2(a_ptr, cc, 0);
            a_ptr = unsafe { a_ptr.add(2) };
        }
        3 => {
            _align_unroll_one_1(a_ptr, cc, 0);
            _align_unroll_one_2(a_ptr, cc, 1);
            a_ptr = unsafe { a_ptr.add(1 + 2) };
        }
        4 => {
            _align_unroll_one_4(a_ptr, cc, 0);
            a_ptr = unsafe { a_ptr.add(4) };
        }
        5 => {
            _align_unroll_one_1(a_ptr, cc, 0);
            _align_unroll_one_4(a_ptr, cc, 1);
            a_ptr = unsafe { a_ptr.add(1 + 4) };
        }
        6 => {
            _align_unroll_one_2(a_ptr, cc, 0);
            _align_unroll_one_4(a_ptr, cc, 2);
            a_ptr = unsafe { a_ptr.add(2 + 4) };
        }
        7 => {
            _align_unroll_one_1(a_ptr, cc, 0);
            _align_unroll_one_2(a_ptr, cc, 1);
            _align_unroll_one_4(a_ptr, cc, 1 + 2);
            a_ptr = unsafe { a_ptr.add(1 + 2 + 4) };
        }
        8 => {}
        _ => unreachable!(),
    }
    a_ptr
}

#[inline(always)]
fn _align_16(a_ptr: *mut u8, cc: u64) -> *mut u8 {
    let mut a_ptr = a_ptr;
    let align = 0x0010_usize - ((a_ptr as usize) & 0x000F_usize);
    match align {
        1 => {
            _align_unroll_one_1(a_ptr, cc, 0);
            a_ptr = unsafe { a_ptr.add(1) };
        }
        2 => {
            _align_unroll_one_2(a_ptr, cc, 0);
            a_ptr = unsafe { a_ptr.add(2) };
        }
        3 => {
            _align_unroll_one_1(a_ptr, cc, 0);
            _align_unroll_one_2(a_ptr, cc, 1);
            a_ptr = unsafe { a_ptr.add(1 + 2) };
        }
        4 => {
            _align_unroll_one_4(a_ptr, cc, 0);
            a_ptr = unsafe { a_ptr.add(4) };
        }
        5 => {
            _align_unroll_one_1(a_ptr, cc, 0);
            _align_unroll_one_4(a_ptr, cc, 1);
            a_ptr = unsafe { a_ptr.add(1 + 4) };
        }
        6 => {
            _align_unroll_one_2(a_ptr, cc, 0);
            _align_unroll_one_4(a_ptr, cc, 2);
            a_ptr = unsafe { a_ptr.add(2 + 4) };
        }
        7 => {
            _align_unroll_one_1(a_ptr, cc, 0);
            _align_unroll_one_2(a_ptr, cc, 1);
            _align_unroll_one_4(a_ptr, cc, 1 + 2);
            a_ptr = unsafe { a_ptr.add(1 + 2 + 4) };
        }
        8 => {
            _align_unroll_one_8(a_ptr, cc, 0);
            a_ptr = unsafe { a_ptr.add(8) };
        }
        //
        9 => {
            _align_unroll_one_1(a_ptr, cc, 0);
            _align_unroll_one_8(a_ptr, cc, 1);
            a_ptr = unsafe { a_ptr.add(1 + 8) };
        }
        10 => {
            _align_unroll_one_2(a_ptr, cc, 0);
            _align_unroll_one_8(a_ptr, cc, 2);
            a_ptr = unsafe { a_ptr.add(2 + 8) };
        }
        11 => {
            _align_unroll_one_1(a_ptr, cc, 0);
            _align_unroll_one_2(a_ptr, cc, 1);
            _align_unroll_one_8(a_ptr, cc, 1 + 2);
            a_ptr = unsafe { a_ptr.add(1 + 2 + 8) };
        }
        12 => {
            _align_unroll_one_4(a_ptr, cc, 0);
            _align_unroll_one_8(a_ptr, cc, 4);
            a_ptr = unsafe { a_ptr.add(4 + 8) };
        }
        13 => {
            _align_unroll_one_1(a_ptr, cc, 0);
            _align_unroll_one_4(a_ptr, cc, 1);
            _align_unroll_one_8(a_ptr, cc, 1 + 4);
            a_ptr = unsafe { a_ptr.add(1 + 4 + 8) };
        }
        14 => {
            _align_unroll_one_2(a_ptr, cc, 0);
            _align_unroll_one_4(a_ptr, cc, 2);
            _align_unroll_one_8(a_ptr, cc, 2 + 4);
            a_ptr = unsafe { a_ptr.add(2 + 4 + 8) };
        }
        15 => {
            _align_unroll_one_1(a_ptr, cc, 0);
            _align_unroll_one_2(a_ptr, cc, 1);
            _align_unroll_one_4(a_ptr, cc, 1 + 2);
            _align_unroll_one_8(a_ptr, cc, 1 + 2 + 4);
            a_ptr = unsafe { a_ptr.add(1 + 2 + 4 + 8) };
        }
        16 => {}
        //
        _ => unreachable!(),
    }
    a_ptr
}

#[inline(always)]
pub(crate) fn _start_set_128(buf: &mut [u8], c: u8) {
    let buf_len = buf.len();
    let mut a_ptr = buf.as_mut_ptr();
    let end_ptr = unsafe { a_ptr.add(buf_len) };
    //
    let cc: u128 = c as u128 * 0x0101_0101_0101_0101_0101_0101_0101_0101_u128;
    if buf_len >= 16 {
        a_ptr = _align_16(a_ptr, cc as u64);
        {
            let unroll = 8;
            let loop_size = 16;
            while unsafe { end_ptr.offset_from(a_ptr) } >= (loop_size * unroll) as isize {
                for i in 0..unroll {
                    let aa_ptr = unsafe { a_ptr.add(loop_size * i) } as *mut u128;
                    unsafe { aa_ptr.write(cc) };
                }
                a_ptr = unsafe { a_ptr.add(loop_size * unroll) };
            }
        }
        {
            let unroll = 4;
            let loop_size = 16;
            while unsafe { end_ptr.offset_from(a_ptr) } >= (loop_size * unroll) as isize {
                for i in 0..unroll {
                    let aa_ptr = unsafe { a_ptr.add(loop_size * i) } as *mut u128;
                    unsafe { aa_ptr.write(cc) };
                }
                a_ptr = unsafe { a_ptr.add(loop_size * unroll) };
            }
        }
        {
            let loop_size = 16;
            while unsafe { end_ptr.offset_from(a_ptr) } >= loop_size as isize {
                let aa_ptr = a_ptr as *mut u128;
                unsafe { aa_ptr.write(cc) };
                a_ptr = unsafe { a_ptr.add(loop_size) };
            }
        }
    }
    // the remaining data is the max: 15 bytes.
    _memset_remaining_15_bytes_impl(a_ptr, cc, end_ptr)
}

#[inline(always)]
pub(crate) fn _start_set_64(buf: &mut [u8], c: u8) {
    let buf_len = buf.len();
    let mut a_ptr = buf.as_mut_ptr();
    let end_ptr = unsafe { a_ptr.add(buf_len) };
    //
    let cc: u64 = c as u64 * 0x0101_0101_0101_0101_u64;
    if buf_len >= 8 {
        a_ptr = _align_8(a_ptr, cc);
        {
            let unroll = 8;
            let loop_size = 8;
            while unsafe { end_ptr.offset_from(a_ptr) } >= (loop_size * unroll) as isize {
                for i in 0..unroll {
                    let aa_ptr = unsafe { a_ptr.add(loop_size * i) } as *mut u64;
                    unsafe { aa_ptr.write(cc) };
                }
                a_ptr = unsafe { a_ptr.add(loop_size * unroll) };
            }
        }
        {
            let unroll = 4;
            let loop_size = 8;
            while unsafe { end_ptr.offset_from(a_ptr) } >= (loop_size * unroll) as isize {
                for i in 0..unroll {
                    let aa_ptr = unsafe { a_ptr.add(loop_size * i) } as *mut u64;
                    unsafe { aa_ptr.write(cc) };
                }
                a_ptr = unsafe { a_ptr.add(loop_size * unroll) };
            }
        }
        {
            let loop_size = 8;
            while unsafe { end_ptr.offset_from(a_ptr) } >= loop_size as isize {
                let aa_ptr = a_ptr as *mut u64;
                unsafe { aa_ptr.write(cc) };
                a_ptr = unsafe { a_ptr.add(loop_size) };
            }
        }
    }
    // the remaining data is the max: 7 bytes.
    _memset_remaining_7_bytes_impl(a_ptr, cc, end_ptr)
}

#[inline(always)]
fn _start_set_64_no_unroll(buf: &mut [u8], c: u8) {
    let buf_len = buf.len();
    let mut a_ptr = buf.as_mut_ptr();
    let end_ptr = unsafe { a_ptr.add(buf_len) };
    //
    let cc: u64 = c as u64 * 0x0101_0101_0101_0101_u64;
    a_ptr = _align_8(a_ptr, cc);
    if buf_len > 7 {
        let loop_size = 8;
        while unsafe { end_ptr.offset_from(a_ptr) } >= loop_size as isize {
            let aa_ptr = a_ptr as *mut u64;
            unsafe { aa_ptr.write(cc) };
            a_ptr = unsafe { a_ptr.add(loop_size) };
        }
    }
    // the remaining data is the max: 7 bytes.
    _memset_remaining_7_bytes_impl(a_ptr, cc, end_ptr)
}

#[inline(always)]
fn _start_set_32(buf: &mut [u8], c: u8) {
    let buf_len = buf.len();
    let mut a_ptr = buf.as_mut_ptr();
    let end_ptr = unsafe { a_ptr.add(buf_len) };
    //
    let cc: u32 = c as u32 * 0x0101_0101_u32;
    if buf_len >= 4 {
        if buf_len >= 8 {
            a_ptr = _align_8(a_ptr, cc as u64);
        } else {
            a_ptr = _align_4(a_ptr, cc as u64);
        }
        //
        {
            let unroll = 8;
            let loop_size = 4;
            while unsafe { end_ptr.offset_from(a_ptr) } >= (loop_size * unroll) as isize {
                for i in 0..unroll {
                    let aa_ptr = unsafe { a_ptr.add(loop_size * i) } as *mut u32;
                    unsafe { aa_ptr.write(cc) };
                }
                a_ptr = unsafe { a_ptr.add(loop_size * unroll) };
            }
        }
        {
            let unroll = 4;
            let loop_size = 4;
            while unsafe { end_ptr.offset_from(a_ptr) } >= (loop_size * unroll) as isize {
                for i in 0..unroll {
                    let aa_ptr = unsafe { a_ptr.add(loop_size * i) } as *mut u32;
                    unsafe { aa_ptr.write(cc) };
                }
                a_ptr = unsafe { a_ptr.add(loop_size * unroll) };
            }
        }
        {
            let loop_size = 4;
            while unsafe { end_ptr.offset_from(a_ptr) } >= loop_size as isize {
                let aa_ptr = a_ptr as *mut u32;
                unsafe { aa_ptr.write(cc) };
                a_ptr = unsafe { a_ptr.add(loop_size) };
            }
        }
    }
    // the remaining data is the max: 3 bytes.
    _memset_remaining_3_bytes_impl(a_ptr, cc, end_ptr)
}

//#[cfg(target_pointer_width = "32")]
#[inline(always)]
fn _start_set_32_no_unroll(buf: &mut [u8], c: u8) {
    let buf_len = buf.len();
    let mut a_ptr = buf.as_mut_ptr();
    let end_ptr = unsafe { a_ptr.add(buf_len) };
    //
    let cc: u32 = c as u32 * 0x0101_0101_u32;
    a_ptr = _align_4(a_ptr, cc as u64);
    if buf_len > 3 {
        let loop_size = 4;
        while unsafe { end_ptr.offset_from(a_ptr) } >= loop_size as isize {
            let aa_ptr = a_ptr as *mut u32;
            unsafe { aa_ptr.write(cc) };
            a_ptr = unsafe { a_ptr.add(loop_size) };
        }
    }
    // the remaining data is the max: 3 bytes.
    _memset_remaining_3_bytes_impl(a_ptr, cc, end_ptr)
}

#[inline(always)]
fn _start_set_16(buf: &mut [u8], c: u8) {
    let buf_len = buf.len();
    let mut a_ptr = buf.as_mut_ptr();
    let end_ptr = unsafe { a_ptr.add(buf_len) };
    //
    let cc: u16 = c as u16 * 0x0101_u16;
    if buf_len >= 2 {
        a_ptr = _align_2(a_ptr, cc as u64);
        //
        {
            let unroll = 8;
            let loop_size = 2;
            while unsafe { end_ptr.offset_from(a_ptr) } >= (loop_size * unroll) as isize {
                for i in 0..unroll {
                    let aa_ptr = unsafe { a_ptr.add(loop_size * i) } as *mut u16;
                    unsafe { aa_ptr.write(cc) };
                }
                a_ptr = unsafe { a_ptr.add(loop_size * unroll) };
            }
        }
        {
            let unroll = 4;
            let loop_size = 2;
            while unsafe { end_ptr.offset_from(a_ptr) } >= (loop_size * unroll) as isize {
                for i in 0..unroll {
                    let aa_ptr = unsafe { a_ptr.add(loop_size * i) } as *mut u16;
                    unsafe { aa_ptr.write(cc) };
                }
                a_ptr = unsafe { a_ptr.add(loop_size * unroll) };
            }
        }
        {
            let loop_size = 2;
            while unsafe { end_ptr.offset_from(a_ptr) } >= loop_size as isize {
                let aa_ptr = a_ptr as *mut u16;
                unsafe { aa_ptr.write(cc) };
                a_ptr = unsafe { a_ptr.add(loop_size) };
            }
        }
    }
    // the remaining data is the max: 1 bytes.
    _memset_remaining_1_bytes_impl(a_ptr, cc, end_ptr)
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

#[inline(always)]
pub(crate) fn _memset_remaining_1_bytes_impl(buf_ptr: *const u8, cc: u16, end_ptr: *const u8) {
    let a_ptr = buf_ptr;
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
        #[cfg(target_pointer_width = "128")]
        do_proc_128(a, c);
        #[cfg(target_pointer_width = "64")]
        do_proc_64(a, c);
        #[cfg(target_pointer_width = "32")]
        do_proc_32(a, c);
    }
    #[inline(never)]
    fn do_proc_basic(a: &mut [u8], c: u8) {
        _memset_impl(a, c)
    }
    #[cfg(target_pointer_width = "128")]
    #[inline(never)]
    fn do_proc_128(a: &mut [u8], c: u8) {
        _start_set_64(a, c)
    }
    #[cfg(target_pointer_width = "64")]
    #[inline(never)]
    fn do_proc_64(a: &mut [u8], c: u8) {
        _start_set_64(a, c)
    }
    #[cfg(target_pointer_width = "32")]
    #[inline(never)]
    fn do_proc_32(a: &mut [u8], c: u8) {
        _start_set_32(a, c)
    }
}
