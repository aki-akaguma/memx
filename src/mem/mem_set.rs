#[inline(always)]
pub fn _memset_impl(buf: &mut [u8], c: u8) {
    if buf.len() == 0 {
        return;
    }
    #[cfg(target_pointer_width = "128")]
    _start_set_64(buf, c);
    #[cfg(target_pointer_width = "64")]
    _start_set_64(buf, c);
    #[cfg(target_pointer_width = "32")]
    _start_set_32(buf, c);
    #[cfg(target_pointer_width = "16")]
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
    unsafe { *aa_ptr = cc as u64 };
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
            let end_ptr_8_8 = unsafe { end_ptr.sub(loop_size * unroll) };
            while a_ptr <= end_ptr_8_8 {
                for i in 0..unroll {
                    let aa_ptr = unsafe { a_ptr.add(loop_size * i) } as *mut u64;
                    unsafe { *aa_ptr = cc };
                }
                a_ptr = unsafe { a_ptr.add(loop_size * unroll) };
            }
        }
        {
            let unroll = 4;
            let loop_size = 8;
            let end_ptr_8_4 = unsafe { end_ptr.sub(loop_size * unroll) };
            while a_ptr <= end_ptr_8_4 {
                for i in 0..unroll {
                    let aa_ptr = unsafe { a_ptr.add(loop_size * i) } as *mut u64;
                    unsafe { *aa_ptr = cc };
                }
                a_ptr = unsafe { a_ptr.add(loop_size * unroll) };
            }
        }
        {
            let loop_size = 8;
            let end_ptr_8 = unsafe { end_ptr.sub(loop_size) };
            while a_ptr <= end_ptr_8 {
                let aa_ptr = a_ptr as *mut u64;
                unsafe { *aa_ptr = cc };
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
    if buf_len > 7 {
        let loop_size = 8;
        let end_ptr_8 = unsafe { end_ptr.sub(loop_size) };
        while a_ptr <= end_ptr_8 {
            let aa_ptr = a_ptr as *mut u64;
            unsafe { *aa_ptr = cc };
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
            let end_ptr_4_8 = unsafe { end_ptr.sub(loop_size * unroll) };
            while a_ptr <= end_ptr_4_8 {
                for i in 0..unroll {
                    let aa_ptr = unsafe { a_ptr.add(loop_size * i) } as *mut u32;
                    unsafe { *aa_ptr = cc };
                }
                a_ptr = unsafe { a_ptr.add(loop_size * unroll) };
            }
        }
        {
            let unroll = 4;
            let loop_size = 4;
            let end_ptr_4_4 = unsafe { end_ptr.sub(loop_size * unroll) };
            while a_ptr <= end_ptr_4_4 {
                for i in 0..unroll {
                    let aa_ptr = unsafe { a_ptr.add(loop_size * i) } as *mut u32;
                    unsafe { *aa_ptr = cc };
                }
                a_ptr = unsafe { a_ptr.add(loop_size * unroll) };
            }
        }
        {
            let loop_size = 4;
            let end_ptr_4 = unsafe { end_ptr.sub(loop_size) };
            while a_ptr <= end_ptr_4 {
                let aa_ptr = a_ptr as *mut u32;
                unsafe { *aa_ptr = cc };
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
        let end_ptr_4 = unsafe { end_ptr.sub(loop_size) };
        while a_ptr <= end_ptr_4 {
            let aa_ptr = a_ptr as *mut u32;
            unsafe { *aa_ptr = cc };
            a_ptr = unsafe { a_ptr.add(loop_size) };
        }
    }
    // the remaining data is the max: 3 bytes.
    _memset_remaining_3_bytes_impl(a_ptr, cc, end_ptr)
}

#[inline(always)]
pub(crate) fn _memset_remaining_15_bytes_impl(buf_ptr: *const u8, cc: u64, end_ptr: *const u8) {
    let mut a_ptr = buf_ptr;
    {
        let loop_size = 8;
        let end_ptr_4 = unsafe { end_ptr.sub(loop_size) };
        if a_ptr <= end_ptr_4 {
            let cc: u64 = cc as u64;
            let aa_ptr = a_ptr as *mut u64;
            unsafe { *aa_ptr = cc };
            //
            a_ptr = unsafe { a_ptr.add(loop_size) };
        }
    }
    // the remaining data is the max: 7 bytes.
    _memset_remaining_7_bytes_impl(a_ptr, cc, end_ptr)
}

#[inline(always)]
pub(crate) fn _memset_remaining_7_bytes_impl(buf_ptr: *const u8, cc: u64, end_ptr: *const u8) {
    let mut a_ptr = buf_ptr;
    {
        let loop_size = 4;
        let end_ptr_4 = unsafe { end_ptr.sub(loop_size) };
        if a_ptr <= end_ptr_4 {
            let cc: u32 = cc as u32;
            let aa_ptr = a_ptr as *mut u32;
            unsafe { *aa_ptr = cc };
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
        let end_ptr_2 = unsafe { end_ptr.sub(loop_size) };
        if a_ptr <= end_ptr_2 {
            let cc: u16 = cc as u16;
            let aa_ptr = a_ptr as *mut u16;
            unsafe { *aa_ptr = cc };
            //
            a_ptr = unsafe { a_ptr.add(loop_size) };
        }
    }
    {
        let loop_size = 1;
        let end_ptr_1 = unsafe { end_ptr.sub(loop_size) };
        if a_ptr <= end_ptr_1 {
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
