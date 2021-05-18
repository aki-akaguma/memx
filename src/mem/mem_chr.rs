#[inline(always)]
pub fn _memchr_impl(buf: &[u8], c: u8) -> Option<usize> {
    //
    let buf_len = buf.len();
    let mut buf_ptr = buf.as_ptr();
    let start_ptr = buf_ptr;
    let end_ptr = unsafe { buf_ptr.add(buf_len) };
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
    /*
    let loop_size = 2;
    let c2: u16 = _c2_value(c);
    while buf_ptr <= unsafe { end_ptr.sub(loop_size) } {
        let r = _check_c2(buf_ptr, c2, start_ptr);
        if !r.is_none() {
            return r;
        }
        buf_ptr = unsafe { buf_ptr.add(loop_size) };
    }
    */
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
fn _c4_value(c: u8) -> u32 {
    (c as u32) * 0x0101_0101_u32
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

#[inline(always)]
fn _c2_value(c: u8) -> u16 {
    (c as u16) * 0x0101_u16
}

#[inline(always)]
fn _check_c2(buf_ptr: *const u8, c2: u16, start_ptr: *const u8) -> Option<usize> {
    let aa_ptr = buf_ptr as *const u16;
    let v = unsafe { *aa_ptr } ^ c2;
    let bits = v.wrapping_sub(0x0101_u16) & !v & 0x8080_u16;
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

/*
 * Why is the next code slow ??? <2021-05-17>
 * 
#[inline(always)]
pub fn _memchr_impl(buf: &[u8], c: u8) -> Option<usize> {
    let start_ptr = buf.as_ptr();
    let (buf_pre, buf_align, buf_suf) = unsafe { buf.align_to::<u32>() };
    //
    {
        let r = _memchr_impl_bytes(buf_pre, c);
        if r.is_some() {
            return r;
        }
    }
    {
        let r = _memchr_impl_u32(buf_align, c);
        if let Some(pos) = r {
            let buf_align_ptr = buf_align.as_ptr() as *const u8;
            return Some(unsafe { buf_align_ptr.offset_from(start_ptr) } as usize + pos);
        }
    }
    {
        let r = _memchr_impl_bytes(buf_suf, c);
        if let Some(pos) = r {
            let buf_suf_ptr = buf_suf.as_ptr() as *const u8;
            return Some(unsafe { buf_suf_ptr.offset_from(start_ptr) } as usize + pos);
        }
    }
    None
}

#[inline(always)]
pub fn _memchr_impl_u32(buf: &[u32], c: u8) -> Option<usize> {
    let buf_len = buf.len();
    let mut buf_ptr = buf.as_ptr();
    let start_ptr = buf_ptr;
    let end_ptr = unsafe { buf_ptr.add(buf_len) };
    //
    let c4: u32 = _c4_value(c);
    while buf_ptr < end_ptr {
        let r = _check_c4(buf_ptr as *const u8, c4, start_ptr as *const u8);
        if !r.is_none() {
            return r;
        }
        buf_ptr = unsafe { buf_ptr.add(1) };
    }
    None
}

#[inline(always)]
pub fn _memchr_impl_bytes(buf: &[u8], c: u8) -> Option<usize> {
    //
    let buf_len = buf.len();
    let mut buf_ptr = buf.as_ptr();
    let start_ptr = buf_ptr;
    let end_ptr = unsafe { buf_ptr.add(buf_len) };
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
*/
