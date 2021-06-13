use crate::plus_offset_from;

#[inline(always)]
pub fn _memchr_impl(buf: &[u8], c: u8) -> Option<usize> {
    if buf.len() == 0 {
        return None;
    }
    #[cfg(target_pointer_width = "128")]
    let r = _start_chr_128(buf, c);
    #[cfg(target_pointer_width = "64")]
    let r = _start_chr_64(buf, c);
    #[cfg(target_pointer_width = "32")]
    let r = _start_chr_32(buf, c);
    #[cfg(target_pointer_width = "16")]
    let r = _start_chr_16(buf, c);
    //
    r
}

macro_rules! _unroll_one_chr_16 {
    ($a_ptr:expr, $cc:expr, $start_ptr:expr, $loop_size:expr, $idx:expr) => {{
        let aa_ptr = unsafe { $a_ptr.add($loop_size * $idx) };
        let r = _chr_c16(aa_ptr, $cc, $start_ptr);
        if !r.is_none() {
            return r;
        }
    }};
}

macro_rules! _unroll_one_chr_8 {
    ($a_ptr:expr, $cc:expr, $start_ptr:expr, $loop_size:expr, $idx:expr) => {{
        let aa_ptr = unsafe { $a_ptr.add($loop_size * $idx) };
        let r = _chr_c8(aa_ptr, $cc, $start_ptr);
        if !r.is_none() {
            return r;
        }
    }};
}

macro_rules! _unroll_one_chr_4 {
    ($a_ptr:expr, $cc:expr, $start_ptr:expr, $loop_size:expr, $idx:expr) => {{
        let aa_ptr = unsafe { $a_ptr.add($loop_size * $idx) };
        let r = _chr_c4(aa_ptr, $cc, $start_ptr);
        if !r.is_none() {
            return r;
        }
    }};
}

macro_rules! _unroll_one_chr_2 {
    ($a_ptr:expr, $cc:expr, $start_ptr:expr, $loop_size:expr, $idx:expr) => {{
        let aa_ptr = unsafe { $a_ptr.add($loop_size * $idx) };
        let r = _chr_c2(aa_ptr, $cc, $start_ptr);
        if !r.is_none() {
            return r;
        }
    }};
}

macro_rules! _unroll_one_chr_1 {
    ($a_ptr:expr, $cc:expr, $start_ptr:expr, $loop_size:expr, $idx:expr) => {{
        let aa_ptr = unsafe { $a_ptr.add($loop_size * $idx) };
        let r = _chr_c1(aa_ptr, $cc, $start_ptr);
        if !r.is_none() {
            return r;
        }
    }};
}

#[cfg(target_pointer_width = "128")]
#[inline(always)]
fn _start_chr_128(buf: &[u8], c: u8) -> Option<usize> {
    //
    let buf_len = buf.len();
    let mut buf_ptr = buf.as_ptr();
    let start_ptr = buf_ptr;
    let end_ptr = unsafe { buf_ptr.add(buf_len) };
    let cc: u128 = _c16_value(c);
    //
    {
        let unroll = 8;
        let loop_size = 16;
        let end_ptr_16_8 = unsafe { end_ptr.sub(loop_size * unroll) };
        while buf_ptr <= end_ptr_16_8 {
            _unroll_one_chr_16!(buf_ptr, cc, start_ptr, loop_size, 0);
            _unroll_one_chr_16!(buf_ptr, cc, start_ptr, loop_size, 1);
            _unroll_one_chr_16!(buf_ptr, cc, start_ptr, loop_size, 2);
            _unroll_one_chr_16!(buf_ptr, cc, start_ptr, loop_size, 3);
            _unroll_one_chr_16!(buf_ptr, cc, start_ptr, loop_size, 4);
            _unroll_one_chr_16!(buf_ptr, cc, start_ptr, loop_size, 5);
            _unroll_one_chr_16!(buf_ptr, cc, start_ptr, loop_size, 6);
            _unroll_one_chr_16!(buf_ptr, cc, start_ptr, loop_size, 7);
            //
            buf_ptr = unsafe { buf_ptr.add(loop_size * unroll) };
        }
    }
    {
        let loop_size = 16;
        let end_ptr_16 = unsafe { end_ptr.sub(loop_size) };
        while buf_ptr <= end_ptr_16 {
            _unroll_one_chr_16!(buf_ptr, cc, start_ptr, loop_size, 0);
            //
            buf_ptr = unsafe { buf_ptr.add(loop_size) };
        }
    }
    // the remaining data is the max: 15 bytes.
    _memchr_remaining_15_bytes_impl(buf_ptr, c, start_ptr, end_ptr)
}

#[inline(always)]
pub(crate) fn _memchr_remaining_15_bytes_impl(
    buf_ptr: *const u8,
    c: u8,
    start_ptr: *const u8,
    end_ptr: *const u8,
) -> Option<usize> {
    let mut buf_ptr = buf_ptr;
    let cc: u64 = _c8_value(c);
    {
        let loop_size = 8;
        let end_ptr_8 = unsafe { end_ptr.sub(loop_size) };
        if buf_ptr <= end_ptr_8 {
            _unroll_one_chr_8!(buf_ptr, cc, start_ptr, loop_size, 0);
            //
            buf_ptr = unsafe { buf_ptr.add(loop_size) };
        }
    }
    {
        let loop_size = 4;
        let cc: u32 = cc as u32;
        let end_ptr_4 = unsafe { end_ptr.sub(loop_size) };
        if buf_ptr <= end_ptr_4 {
            _unroll_one_chr_4!(buf_ptr, cc, start_ptr, loop_size, 0);
            //
            buf_ptr = unsafe { buf_ptr.add(loop_size) };
        }
    }
    {
        let loop_size = 2;
        let cc: u16 = cc as u16;
        let end_ptr_2 = unsafe { end_ptr.sub(loop_size) };
        if buf_ptr <= end_ptr_2 {
            _unroll_one_chr_2!(buf_ptr, cc, start_ptr, loop_size, 0);
            //
            buf_ptr = unsafe { buf_ptr.add(loop_size) };
        }
    }
    {
        let loop_size = 1;
        let cc: u8 = c;
        let end_ptr_1 = unsafe { end_ptr.sub(loop_size) };
        if buf_ptr <= end_ptr_1 {
            _unroll_one_chr_1!(buf_ptr, cc, start_ptr, loop_size, 0);
        }
    }
    //
    None
}

#[cfg(target_pointer_width = "64")]
#[inline(always)]
fn _start_chr_64(buf: &[u8], c: u8) -> Option<usize> {
    //
    let buf_len = buf.len();
    let mut buf_ptr = buf.as_ptr();
    let start_ptr = buf_ptr;
    let end_ptr = unsafe { buf_ptr.add(buf_len) };
    let cc: u64 = _c8_value(c);
    //
    {
        let unroll = 8;
        let loop_size = 8;
        let end_ptr_8_8 = unsafe { end_ptr.sub(loop_size * unroll) };
        while buf_ptr <= end_ptr_8_8 {
            _unroll_one_chr_8!(buf_ptr, cc, start_ptr, loop_size, 0);
            _unroll_one_chr_8!(buf_ptr, cc, start_ptr, loop_size, 1);
            _unroll_one_chr_8!(buf_ptr, cc, start_ptr, loop_size, 2);
            _unroll_one_chr_8!(buf_ptr, cc, start_ptr, loop_size, 3);
            _unroll_one_chr_8!(buf_ptr, cc, start_ptr, loop_size, 4);
            _unroll_one_chr_8!(buf_ptr, cc, start_ptr, loop_size, 5);
            _unroll_one_chr_8!(buf_ptr, cc, start_ptr, loop_size, 6);
            _unroll_one_chr_8!(buf_ptr, cc, start_ptr, loop_size, 7);
            //
            buf_ptr = unsafe { buf_ptr.add(loop_size * unroll) };
        }
    }
    {
        let loop_size = 8;
        let end_ptr_8 = unsafe { end_ptr.sub(loop_size) };
        while buf_ptr <= end_ptr_8 {
            _unroll_one_chr_8!(buf_ptr, cc, start_ptr, loop_size, 0);
            //
            buf_ptr = unsafe { buf_ptr.add(loop_size) };
        }
    }
    // a rest data is a max: 7 bytes.
    _memchr_remaining_7_bytes_impl(buf_ptr, c, start_ptr, end_ptr)
}

#[inline(always)]
pub(crate) fn _memchr_remaining_7_bytes_impl(
    buf_ptr: *const u8,
    c: u8,
    start_ptr: *const u8,
    end_ptr: *const u8,
) -> Option<usize> {
    let mut buf_ptr = buf_ptr;
    let cc: u32 = _c4_value(c);
    {
        let loop_size = 4;
        let end_ptr_4 = unsafe { end_ptr.sub(loop_size) };
        if buf_ptr <= end_ptr_4 {
            _unroll_one_chr_4!(buf_ptr, cc, start_ptr, loop_size, 0);
            //
            buf_ptr = unsafe { buf_ptr.add(loop_size) };
        }
    }
    {
        let loop_size = 2;
        let cc: u16 = cc as u16;
        let end_ptr_2 = unsafe { end_ptr.sub(loop_size) };
        if buf_ptr <= end_ptr_2 {
            _unroll_one_chr_2!(buf_ptr, cc, start_ptr, loop_size, 0);
            //
            buf_ptr = unsafe { buf_ptr.add(loop_size) };
        }
    }
    {
        let loop_size = 1;
        let cc: u8 = c;
        let end_ptr_1 = unsafe { end_ptr.sub(loop_size) };
        if buf_ptr <= end_ptr_1 {
            _unroll_one_chr_1!(buf_ptr, cc, start_ptr, loop_size, 0);
        }
    }
    //
    None
}

#[cfg(target_pointer_width = "32")]
#[inline(always)]
fn _start_chr_32(buf: &[u8], c: u8) -> Option<usize> {
    //
    let buf_len = buf.len();
    let mut buf_ptr = buf.as_ptr();
    let start_ptr = buf_ptr;
    let end_ptr = unsafe { buf_ptr.add(buf_len) };
    let cc: u32 = _c4_value(c);
    //
    {
        let unroll = 8;
        let loop_size = 4;
        let end_ptr_4_8 = unsafe { end_ptr.sub(loop_size * unroll) };
        while buf_ptr <= end_ptr_4_8 {
            _unroll_one_chr_4!(buf_ptr, cc, start_ptr, loop_size, 0);
            _unroll_one_chr_4!(buf_ptr, cc, start_ptr, loop_size, 1);
            _unroll_one_chr_4!(buf_ptr, cc, start_ptr, loop_size, 2);
            _unroll_one_chr_4!(buf_ptr, cc, start_ptr, loop_size, 3);
            _unroll_one_chr_4!(buf_ptr, cc, start_ptr, loop_size, 4);
            _unroll_one_chr_4!(buf_ptr, cc, start_ptr, loop_size, 5);
            _unroll_one_chr_4!(buf_ptr, cc, start_ptr, loop_size, 6);
            _unroll_one_chr_4!(buf_ptr, cc, start_ptr, loop_size, 7);
            //
            buf_ptr = unsafe { buf_ptr.add(loop_size * unroll) };
        }
    }
    {
        let loop_size = 4;
        let end_ptr_4 = unsafe { end_ptr.sub(loop_size) };
        while buf_ptr <= end_ptr_4 {
            _unroll_one_chr_4!(buf_ptr, cc, start_ptr, loop_size, 0);
            //
            buf_ptr = unsafe { buf_ptr.add(loop_size) };
        }
    }
    // a rest data is a max: 3 bytes.
    _memchr_remaining_3_bytes_impl(buf_ptr, c, start_ptr, end_ptr)
}

#[inline(always)]
pub(crate) fn _memchr_remaining_3_bytes_impl(
    buf_ptr: *const u8,
    c: u8,
    start_ptr: *const u8,
    end_ptr: *const u8,
) -> Option<usize> {
    let mut buf_ptr = buf_ptr;
    let cc: u16 = _c2_value(c);
    {
        let loop_size = 2;
        let end_ptr_2 = unsafe { end_ptr.sub(loop_size) };
        if buf_ptr <= end_ptr_2 {
            _unroll_one_chr_2!(buf_ptr, cc, start_ptr, loop_size, 0);
            //
            buf_ptr = unsafe { buf_ptr.add(loop_size) };
        }
    }
    {
        let loop_size = 1;
        let cc: u8 = c;
        let end_ptr_1 = unsafe { end_ptr.sub(loop_size) };
        if buf_ptr <= end_ptr_1 {
            _unroll_one_chr_1!(buf_ptr, cc, start_ptr, loop_size, 0);
        }
    }
    //
    None
}

#[cfg(target_pointer_width = "16")]
#[inline(always)]
fn _start_chr_16(buf: &[u8], c: u8) -> Option<usize> {
    //
    let buf_len = buf.len();
    let mut buf_ptr = buf.as_ptr();
    let start_ptr = buf_ptr;
    let end_ptr = unsafe { buf_ptr.add(buf_len) };
    let cc: u16 = _c2_value(c);
    //
    {
        let unroll = 8;
        let loop_size = 2;
        let end_ptr_2_8 = unsafe { end_ptr.sub(loop_size * unroll) };
        while buf_ptr <= end_ptr_2_8 {
            _unroll_one_chr_2!(buf_ptr, cc, start_ptr, loop_size, 0);
            _unroll_one_chr_2!(buf_ptr, cc, start_ptr, loop_size, 1);
            _unroll_one_chr_2!(buf_ptr, cc, start_ptr, loop_size, 2);
            _unroll_one_chr_2!(buf_ptr, cc, start_ptr, loop_size, 3);
            _unroll_one_chr_2!(buf_ptr, cc, start_ptr, loop_size, 4);
            _unroll_one_chr_2!(buf_ptr, cc, start_ptr, loop_size, 5);
            _unroll_one_chr_2!(buf_ptr, cc, start_ptr, loop_size, 6);
            _unroll_one_chr_2!(buf_ptr, cc, start_ptr, loop_size, 7);
            //
            buf_ptr = unsafe { buf_ptr.add(loop_size * unroll) };
        }
    }
    {
        let loop_size = 2;
        let end_ptr_2 = unsafe { end_ptr.sub(loop_size) };
        while buf_ptr <= end_ptr_2 {
            _unroll_one_chr_2!(buf_ptr, cc, start_ptr, loop_size, 0);
            //
            buf_ptr = unsafe { buf_ptr.add(loop_size) };
        }
    }
    // a rest data is a max: 1 bytes.
    {
        let loop_size = 1;
        let cc: u8 = c;
        let end_ptr_1 = unsafe { end_ptr.sub(loop_size) };
        if buf_ptr <= end_ptr_1 {
            _unroll_one_chr_1!(buf_ptr, cc, start_ptr, loop_size, 0);
        }
    }
    //
    None
}

#[inline(always)]
fn _c16_value(c: u8) -> u128 {
    (c as u128) * 0x0101_0101_0101_0101_0101_0101_0101_0101_u128
}

#[inline(always)]
fn _chr_c16(buf_ptr: *const u8, c8: u128, start_ptr: *const u8) -> Option<usize> {
    let aa_ptr = buf_ptr as *const u128;
    let v = unsafe { *aa_ptr } ^ c8;
    let bits = v.wrapping_sub(0x0101_0101_0101_0101_0101_0101_0101_0101_u128)
        & !v
        & 0x8080_8080_8080_8080_8080_8080_8080_8080_u128;
    if bits != 0 {
        Some(plus_offset_from(buf_ptr, start_ptr) + (bits.trailing_zeros() / 8) as usize)
    } else {
        None
    }
}

#[inline(always)]
fn _c8_value(c: u8) -> u64 {
    (c as u64) * 0x0101_0101_0101_0101_u64
}

#[inline(always)]
fn _chr_c8(buf_ptr: *const u8, c8: u64, start_ptr: *const u8) -> Option<usize> {
    let aa_ptr = buf_ptr as *const u64;
    let v = unsafe { *aa_ptr } ^ c8;
    let bits = v.wrapping_sub(0x0101_0101_0101_0101_u64) & !v & 0x8080_8080_8080_8080_u64;
    if bits != 0 {
        Some(plus_offset_from(buf_ptr, start_ptr) + (bits.trailing_zeros() / 8) as usize)
    } else {
        None
    }
}

#[inline(always)]
fn _c4_value(c: u8) -> u32 {
    (c as u32) * 0x0101_0101_u32
}

#[inline(always)]
fn _chr_c4(buf_ptr: *const u8, c4: u32, start_ptr: *const u8) -> Option<usize> {
    let aa_ptr = buf_ptr as *const u32;
    let v = unsafe { *aa_ptr } ^ c4;
    let bits = v.wrapping_sub(0x0101_0101_u32) & !v & 0x8080_8080_u32;
    if bits != 0 {
        Some(plus_offset_from(buf_ptr, start_ptr) + (bits.trailing_zeros() / 8) as usize)
    } else {
        None
    }
}

#[inline(always)]
fn _c2_value(c: u8) -> u16 {
    (c as u16) * 0x0101_u16
}

#[inline(always)]
fn _chr_c2(buf_ptr: *const u8, c2: u16, start_ptr: *const u8) -> Option<usize> {
    let aa_ptr = buf_ptr as *const u16;
    let v = unsafe { *aa_ptr } ^ c2;
    let bits = v.wrapping_sub(0x0101_u16) & !v & 0x8080_u16;
    if bits != 0 {
        Some(plus_offset_from(buf_ptr, start_ptr) + (bits.trailing_zeros() / 8) as usize)
    } else {
        None
    }
}

#[inline(always)]
fn _chr_c1(buf_ptr: *const u8, c1: u8, start_ptr: *const u8) -> Option<usize> {
    let aa_ptr = buf_ptr as *const u8;
    let aac = unsafe { *aa_ptr };
    if aac == c1 {
        Some(plus_offset_from(buf_ptr, start_ptr))
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
