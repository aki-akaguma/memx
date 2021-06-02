use crate::plus_offset_from;

#[inline(always)]
pub fn _memrchr_impl(buf: &[u8], c: u8) -> Option<usize> {
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
    let start_ptr = buf.as_ptr();
    let mut buf_ptr_cur = unsafe { start_ptr.add(buf_len) };
    let cc: u128 = _c16_value(c);
    //
    {
        let unroll = 8;
        let loop_size = 16;
        let mut buf_ptr = unsafe { buf_ptr_cur.sub(loop_size * unroll) };
        if buf_ptr >= start_ptr {
            while buf_ptr >= start_ptr {
                _unroll_one_chr_16!(buf_ptr, cc, start_ptr, loop_size, 7);
                _unroll_one_chr_16!(buf_ptr, cc, start_ptr, loop_size, 6);
                _unroll_one_chr_16!(buf_ptr, cc, start_ptr, loop_size, 5);
                _unroll_one_chr_16!(buf_ptr, cc, start_ptr, loop_size, 4);
                _unroll_one_chr_16!(buf_ptr, cc, start_ptr, loop_size, 3);
                _unroll_one_chr_16!(buf_ptr, cc, start_ptr, loop_size, 2);
                _unroll_one_chr_16!(buf_ptr, cc, start_ptr, loop_size, 1);
                _unroll_one_chr_16!(buf_ptr, cc, start_ptr, loop_size, 0);
                //
                buf_ptr = unsafe { buf_ptr.sub(loop_size * unroll) };
            }
            buf_ptr_cur = unsafe { buf_ptr.add(loop_size * unroll) };
        }
    }
    {
        let loop_size = 16;
        let mut buf_ptr = unsafe { buf_ptr_cur.sub(loop_size) };
        if buf_ptr >= start_ptr {
            while buf_ptr >= start_ptr {
                _unroll_one_chr_16!(buf_ptr, cc, start_ptr, loop_size, 0);
                //
                buf_ptr = unsafe { buf_ptr.sub(loop_size) };
            }
            buf_ptr_cur = unsafe { buf_ptr.add(loop_size) };
        }
    }
    // the remaining data is the max: 15 bytes.
    _memrchr_remaining_15_bytes_impl(buf_ptr_cur, c, start_ptr)
}

#[inline(always)]
pub(crate) fn _memrchr_remaining_15_bytes_impl(
    buf_ptr_cur: *const u8,
    c: u8,
    start_ptr: *const u8,
) -> Option<usize> {
    let mut buf_ptr_cur = buf_ptr_cur;
    let cc: u64 = _c8_value(c);
    {
        let loop_size = 8;
        let buf_ptr = unsafe { buf_ptr_cur.sub(loop_size) };
        if buf_ptr >= start_ptr {
            _unroll_one_chr_8!(buf_ptr, cc, start_ptr, loop_size, 0);
            //
            buf_ptr_cur = buf_ptr;
        }
    }
    {
        let loop_size = 4;
        let cc: u32 = cc as u32;
        let buf_ptr = unsafe { buf_ptr_cur.sub(loop_size) };
        if buf_ptr >= start_ptr {
            _unroll_one_chr_4!(buf_ptr, cc, start_ptr, loop_size, 0);
            //
            buf_ptr_cur = buf_ptr;
        }
    }
    {
        let loop_size = 2;
        let cc: u16 = cc as u16;
        let buf_ptr = unsafe { buf_ptr_cur.sub(loop_size) };
        if buf_ptr >= start_ptr {
            _unroll_one_chr_2!(buf_ptr, cc, start_ptr, loop_size, 0);
            //
            buf_ptr_cur = buf_ptr;
        }
    }
    {
        let loop_size = 1;
        let cc: u8 = c;
        let buf_ptr = unsafe { buf_ptr_cur.sub(loop_size) };
        if buf_ptr >= start_ptr {
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
    let start_ptr = buf.as_ptr();
    let mut buf_ptr_cur = unsafe { start_ptr.add(buf_len) };
    let cc: u64 = _c8_value(c);
    //
    {
        let unroll = 8;
        let loop_size = 8;
        let mut buf_ptr = buf_ptr_cur;
        let min_ptr = unsafe { start_ptr.add(loop_size * unroll) };
        if buf_ptr >= min_ptr {
            while buf_ptr >= min_ptr {
                buf_ptr = unsafe { buf_ptr.sub(loop_size * unroll) };
                _unroll_one_chr_8!(buf_ptr, cc, start_ptr, loop_size, 7);
                _unroll_one_chr_8!(buf_ptr, cc, start_ptr, loop_size, 6);
                _unroll_one_chr_8!(buf_ptr, cc, start_ptr, loop_size, 5);
                _unroll_one_chr_8!(buf_ptr, cc, start_ptr, loop_size, 4);
                _unroll_one_chr_8!(buf_ptr, cc, start_ptr, loop_size, 3);
                _unroll_one_chr_8!(buf_ptr, cc, start_ptr, loop_size, 2);
                _unroll_one_chr_8!(buf_ptr, cc, start_ptr, loop_size, 1);
                _unroll_one_chr_8!(buf_ptr, cc, start_ptr, loop_size, 0);
            }
            buf_ptr_cur = buf_ptr;
        }
    }
    {
        let loop_size = 8;
        let mut buf_ptr = buf_ptr_cur;
        let min_ptr = unsafe { start_ptr.add(loop_size) };
        if buf_ptr >= min_ptr {
            while buf_ptr >= min_ptr {
                buf_ptr = unsafe { buf_ptr.sub(loop_size) };
                _unroll_one_chr_8!(buf_ptr, cc, start_ptr, loop_size, 0);
            }
            buf_ptr_cur = buf_ptr;
        }
    }
    // a rest data is a max: 7 bytes.
    _memrchr_remaining_7_bytes_impl(buf_ptr_cur, c, start_ptr)
}

#[inline(always)]
pub(crate) fn _memrchr_remaining_7_bytes_impl(
    buf_ptr_cur: *const u8,
    c: u8,
    start_ptr: *const u8,
) -> Option<usize> {
    let mut buf_ptr_cur = buf_ptr_cur;
    let cc: u32 = _c4_value(c);
    {
        let loop_size = 4;
        let buf_ptr = unsafe { buf_ptr_cur.sub(loop_size) };
        if buf_ptr >= start_ptr {
            _unroll_one_chr_4!(buf_ptr, cc, start_ptr, loop_size, 0);
            //
            buf_ptr_cur = buf_ptr;
        }
    }
    {
        let loop_size = 2;
        let cc: u16 = cc as u16;
        let buf_ptr = unsafe { buf_ptr_cur.sub(loop_size) };
        if buf_ptr >= start_ptr {
            _unroll_one_chr_2!(buf_ptr, cc, start_ptr, loop_size, 0);
            //
            buf_ptr_cur = buf_ptr;
        }
    }
    {
        let loop_size = 1;
        let cc: u8 = c;
        let buf_ptr = unsafe { buf_ptr_cur.sub(loop_size) };
        if buf_ptr >= start_ptr {
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
    let start_ptr = buf.as_ptr();
    let mut buf_ptr_cur = unsafe { start_ptr.add(buf_len) };
    let cc: u32 = _c4_value(c);
    //
    {
        let unroll = 8;
        let loop_size = 4;
        let mut buf_ptr = unsafe { buf_ptr_cur.sub(loop_size * unroll) };
        if buf_ptr >= start_ptr {
            while buf_ptr >= start_ptr {
                _unroll_one_chr_4!(buf_ptr, cc, start_ptr, loop_size, 7);
                _unroll_one_chr_4!(buf_ptr, cc, start_ptr, loop_size, 6);
                _unroll_one_chr_4!(buf_ptr, cc, start_ptr, loop_size, 5);
                _unroll_one_chr_4!(buf_ptr, cc, start_ptr, loop_size, 4);
                _unroll_one_chr_4!(buf_ptr, cc, start_ptr, loop_size, 3);
                _unroll_one_chr_4!(buf_ptr, cc, start_ptr, loop_size, 2);
                _unroll_one_chr_4!(buf_ptr, cc, start_ptr, loop_size, 1);
                _unroll_one_chr_4!(buf_ptr, cc, start_ptr, loop_size, 0);
                //
                buf_ptr = unsafe { buf_ptr.sub(loop_size * unroll) };
            }
            buf_ptr_cur = unsafe { buf_ptr.add(loop_size * unroll) };
        }
    }
    {
        let loop_size = 4;
        let mut buf_ptr = unsafe { buf_ptr_cur.sub(loop_size) };
        if buf_ptr >= start_ptr {
            while buf_ptr >= start_ptr {
                _unroll_one_chr_4!(buf_ptr, cc, start_ptr, loop_size, 0);
                //
                buf_ptr = unsafe { buf_ptr.sub(loop_size) };
            }
            buf_ptr_cur = unsafe { buf_ptr.add(loop_size) };
        }
    }
    // a rest data is a max: 3 bytes.
    _memrchr_remaining_3_bytes_impl(buf_ptr_cur, c, start_ptr)
}

#[inline(always)]
pub(crate) fn _memrchr_remaining_3_bytes_impl(
    buf_ptr_cur: *const u8,
    c: u8,
    start_ptr: *const u8,
) -> Option<usize> {
    let mut buf_ptr_cur = buf_ptr_cur;
    let cc: u16 = _c2_value(c);
    {
        let loop_size = 2;
        let buf_ptr = unsafe { buf_ptr_cur.sub(loop_size) };
        if buf_ptr >= start_ptr {
            _unroll_one_chr_2!(buf_ptr, cc, start_ptr, loop_size, 0);
            //
            buf_ptr_cur = buf_ptr;
        }
    }
    {
        let loop_size = 1;
        let cc: u8 = c;
        let buf_ptr = unsafe { buf_ptr_cur.sub(loop_size) };
        if buf_ptr >= start_ptr {
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
    let start_ptr = buf.as_ptr();
    let mut buf_ptr_cur = unsafe { start_ptr.add(buf_len) };
    let cc: u16 = _c2_value(c);
    //
    {
        let unroll = 8;
        let loop_size = 2;
        let mut buf_ptr = unsafe { buf_ptr_cur.sub(loop_size * unroll) };
        if buf_ptr >= start_ptr {
            while buf_ptr >= start_ptr {
                _unroll_one_chr_2!(buf_ptr, cc, start_ptr, loop_size, 7);
                _unroll_one_chr_2!(buf_ptr, cc, start_ptr, loop_size, 6);
                _unroll_one_chr_2!(buf_ptr, cc, start_ptr, loop_size, 5);
                _unroll_one_chr_2!(buf_ptr, cc, start_ptr, loop_size, 4);
                _unroll_one_chr_2!(buf_ptr, cc, start_ptr, loop_size, 3);
                _unroll_one_chr_2!(buf_ptr, cc, start_ptr, loop_size, 2);
                _unroll_one_chr_2!(buf_ptr, cc, start_ptr, loop_size, 1);
                _unroll_one_chr_2!(buf_ptr, cc, start_ptr, loop_size, 0);
                //
                buf_ptr = unsafe { buf_ptr.sub(loop_size * unroll) };
            }
            buf_ptr_cur = unsafe { buf_ptr.add(loop_size * unroll) };
        }
    }
    {
        let loop_size = 2;
        let mut buf_ptr = unsafe { buf_ptr_cur.sub(loop_size) };
        if buf_ptr >= start_ptr {
            while buf_ptr >= start_ptr {
                _unroll_one_chr_2!(buf_ptr, cc, start_ptr, loop_size, 0);
                //
                buf_ptr = unsafe { buf_ptr.sub(loop_size) };
            }
            buf_ptr_cur = unsafe { buf_ptr.add(loop_size) };
        }
    }
    // a rest data is a max: 1 bytes.
    {
        let loop_size = 1;
        let cc: u8 = c;
        let buf_ptr = unsafe { buf_ptr_cur.sub(loop_size) };
        if buf_ptr >= start_ptr {
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
        Some(
            plus_offset_from(buf_ptr, start_ptr) + 16
                - 1
                - ((bits as u128).leading_zeros() / 8) as usize,
        )
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
        Some(
            plus_offset_from(buf_ptr, start_ptr) + 8
                - 1
                - ((bits as u64).leading_zeros() / 8) as usize,
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
fn _chr_c4(buf_ptr: *const u8, c4: u32, start_ptr: *const u8) -> Option<usize> {
    let aa_ptr = buf_ptr as *const u32;
    let v = unsafe { *aa_ptr } ^ c4;
    let bits = v.wrapping_sub(0x0101_0101_u32) & !v & 0x8080_8080_u32;
    if bits != 0 {
        Some(
            plus_offset_from(buf_ptr, start_ptr) + 4
                - 1
                - ((bits as u32).leading_zeros() / 8) as usize,
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
fn _chr_c2(buf_ptr: *const u8, c2: u16, start_ptr: *const u8) -> Option<usize> {
    let aa_ptr = buf_ptr as *const u16;
    let v = unsafe { *aa_ptr } ^ c2;
    let bits = v.wrapping_sub(0x0101_u16) & !v & 0x8080_u16;
    if bits != 0 {
        Some(
            plus_offset_from(buf_ptr, start_ptr) + 2
                - 1
                - ((bits as u16).leading_zeros() / 8) as usize,
        )
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
pub fn _memrchr_impl(buf: &[u8], c: u8) -> Option<usize> {
    for i in 0..buf.len() {
        let j = buf.len()-i-1;
        if buf[j] == c {
            return Some(j);
        }
    }
    None
}

pub fn _memrchr_impl(buf: &[u8], c: u8) -> Option<usize> {
    let buf_len = buf.len();
    let start_ptr = buf.as_ptr();
    let mut buf_ptr = unsafe { start_ptr.add(buf_len-1) };
    while buf_ptr >= start_ptr {
        let aa = unsafe { *buf_ptr };
        if aa == c {
            return Some(plus_offset_from(buf_ptr, start_ptr));
        }
        buf_ptr = unsafe { buf_ptr.sub(1) };
    }
    None
}
*/