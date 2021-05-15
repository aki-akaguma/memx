#[inline(always)]
pub fn _memchr_impl(buf: &[u8], c: u8) -> Option<usize> {
    //
    let buf_len = buf.len();
    let mut buf_ptr = buf.as_ptr();
    let start_ptr = buf_ptr;
    let end_ptr = unsafe { buf_ptr.add(buf_len) };
    //
    /*
    if buf_len <= 0b0111_usize {
        if buf_len > 0 && buf[0] == c { return Some(0); }
        if buf_len > 1 && buf[1] == c { return Some(1); }
        if buf_len > 2 && buf[2] == c { return Some(2); }
        if buf_len > 3 && buf[3] == c { return Some(3); }
        if buf_len > 4 && buf[4] == c { return Some(4); }
        if buf_len > 5 && buf[5] == c { return Some(5); }
        if buf_len > 6 && buf[6] == c { return Some(6); }
        if buf_len > 7 && buf[7] == c { return Some(7); }
        return None;
    }
    //
    let align = (buf_ptr as usize) & 0b0111_usize;
    if align > 0 {
        let align_pad = 0b0111_usize + 1 - align;
        if align_pad > 4 && buf_ptr <= unsafe { end_ptr.sub(8) } {
            let c8: u64 = _c8_value(c);
            let r = _check_c8(buf_ptr, c8, start_ptr);
            if !r.is_none() {
                return r;
            }
            buf_ptr = unsafe { buf_ptr.add(align_pad) };
        } else if align_pad > 2 && buf_ptr <= unsafe { end_ptr.sub(4) } {
            let c4: u32 = _c4_value(c);
            let r = _check_c4(buf_ptr, c4, start_ptr);
            if !r.is_none() {
                return r;
            }
            buf_ptr = unsafe { buf_ptr.add(align_pad) };
        } else {
            for _i in 0..align_pad {
                unsafe {
                    if *buf_ptr == c {
                        return Some(buf_ptr.offset_from(start_ptr) as usize);
                    }
                    buf_ptr = buf_ptr.add(1);
                }
            }
        }
    }
    */
    //
    #[cfg(target_pointer_width = "64")]
    {
        let loop_size = 8;
        let c8: u64 = _c8_value(c);
        while buf_ptr <= unsafe { end_ptr.sub(loop_size) } {
            let r = _check_c8(buf_ptr, c8, start_ptr);
            if !r.is_none() {
                return r;
            }
            buf_ptr = unsafe { buf_ptr.add(loop_size) };
        }
    }
    //
    let loop_size = 4;
    let c4: u32 = _c4_value(c);
    while buf_ptr <= unsafe { end_ptr.sub(loop_size) } {
        let r = _check_c4(buf_ptr, c4, start_ptr);
        if !r.is_none() {
            return r;
        }
        buf_ptr = unsafe { buf_ptr.add(loop_size) };
    }
    //
    while buf_ptr < end_ptr {
        unsafe {
            if *buf_ptr == c {
                return Some(buf_ptr.offset_from(start_ptr) as usize);
            }
            buf_ptr = buf_ptr.add(1);
        }
    }
    None
}

#[inline(always)]
fn _c8_value(c: u8) -> u64 {
    (c as u64) * 0x0101_0101_0101_0101_u64
}

#[inline(always)]
fn _c4_value(c: u8) -> u32 {
    (c as u32) * 0x0101_0101_u32
}

#[inline(always)]
fn _check_c8(buf_ptr: *const u8, c8: u64, start_ptr: *const u8) -> Option<usize> {
    let aa_ptr = buf_ptr as *const u64;
    let v = unsafe { *aa_ptr } ^ c8;
    let bits = v.wrapping_sub(0x0101_0101_0101_0101_u64) & !v & 0x8080_8080_8080_8080_u64;
    if bits != 0 {
        Some(
            unsafe { buf_ptr.offset_from(start_ptr) } as usize
                + (bits.trailing_zeros() / 8) as usize,
        )
    } else {
        None
    }
}

#[inline(always)]
fn _check_c4(buf_ptr: *const u8, c4: u32, start_ptr: *const u8) -> Option<usize> {
    let aa_ptr = buf_ptr as *const u32;
    let v = unsafe { *aa_ptr } ^ c4;
    let bits = v.wrapping_sub(0x0101_0101_u32) & !v & 0x8080_8080_u32;
    if bits != 0 {
        Some(
            unsafe { buf_ptr.offset_from(start_ptr) } as usize
                + (bits.trailing_zeros() / 8) as usize,
        )
    } else {
        None
    }
}

/*
 * Reference.
 * https://pzemtsov.github.io/2019/09/26/making-a-char-searcher-in-c.html
 * https://graphics.stanford.edu/~seander/bithacks.html#ZeroInWord
*/

/*
 * The simple implement:

#[inline(always)]
pub fn _memchr_impl(buf: &[u8], c: u8) -> Option<usize> {
    for i in 0..buf.len() {
        if buf[i] == c {
            return Some(i);
        }
    }
    None
}

pub fn _memchr_impl_(buf: &[u8], c: u8) -> Option<usize> {
    let buf_len = buf.len();
    let start_ptr = buf.as_ptr();
    let end_ptr = unsafe { start_ptr.add(buf_len) };
    let mut buf_ptr = start_ptr;
    while buf_ptr < end_ptr {
        let aa = unsafe { *buf_ptr };
        if aa == c {
            return Some(unsafe { buf_ptr.offset_from(start_ptr) } as usize);
        }
        buf_ptr = unsafe { buf_ptr.add(1) };
    }
    None
}
*/
