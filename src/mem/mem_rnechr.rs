use crate::{plus_offset_from, propagate_a_high_bit};

#[inline(always)]
pub fn _memrnechr_impl(buf: &[u8], c: u8) -> Option<usize> {
    if buf.is_empty() {
        return None;
    }
    if buf[buf.len() - 1] != c {
        return Some(buf.len() - 1);
    }
    #[cfg(target_pointer_width = "128")]
    let r = _start_rnechr_128(buf, c);
    #[cfg(target_pointer_width = "64")]
    let r = _start_rnechr_64(buf, c);
    #[cfg(target_pointer_width = "32")]
    let r = _start_rnechr_32(buf, c);
    #[cfg(target_pointer_width = "16")]
    let r = _start_rnechr_16(buf, c);
    //
    r
}

macro_rules! _unroll_one_rnechr_16 {
    ($a_ptr:expr, $cc:expr, $start_ptr:expr, $loop_size:expr, $idx:expr) => {{
        let aa_ptr = unsafe { $a_ptr.add($loop_size * $idx) };
        let r = _rnechr_c16(aa_ptr, $cc, $start_ptr);
        if !r.is_none() {
            return r;
        }
    }};
}

macro_rules! _unroll_one_rnechr_8 {
    ($a_ptr:expr, $cc:expr, $start_ptr:expr, $loop_size:expr, $idx:expr) => {{
        let aa_ptr = unsafe { $a_ptr.add($loop_size * $idx) };
        let r = _rnechr_c8(aa_ptr, $cc, $start_ptr);
        if !r.is_none() {
            return r;
        }
    }};
}

macro_rules! _unroll_one_rnechr_4 {
    ($a_ptr:expr, $cc:expr, $start_ptr:expr, $loop_size:expr, $idx:expr) => {{
        let aa_ptr = unsafe { $a_ptr.add($loop_size * $idx) };
        let r = _rnechr_c4(aa_ptr, $cc, $start_ptr);
        if !r.is_none() {
            return r;
        }
    }};
}

macro_rules! _unroll_one_rnechr_2 {
    ($a_ptr:expr, $cc:expr, $start_ptr:expr, $loop_size:expr, $idx:expr) => {{
        let aa_ptr = unsafe { $a_ptr.add($loop_size * $idx) };
        let r = _rnechr_c2(aa_ptr, $cc, $start_ptr);
        if !r.is_none() {
            return r;
        }
    }};
}

macro_rules! _unroll_one_rnechr_1 {
    ($a_ptr:expr, $cc:expr, $start_ptr:expr, $loop_size:expr, $idx:expr) => {{
        let aa_ptr = unsafe { $a_ptr.add($loop_size * $idx) };
        let r = _rnechr_c1(aa_ptr, $cc, $start_ptr);
        if !r.is_none() {
            return r;
        }
    }};
}

#[cfg(target_pointer_width = "128")]
#[inline(always)]
fn _start_rnechr_128(buf: &[u8], c: u8) -> Option<usize> {
    //
    let buf_len = buf.len();
    let start_ptr = buf.as_ptr();
    let mut buf_ptr_cur = unsafe { start_ptr.add(buf_len) };
    let cc: u128 = _c16_value(c);
    //
    {
        let unroll = 8;
        let loop_size = 16;
        if unsafe { buf_ptr_cur.offset_from(start_ptr) } >= (loop_size * unroll) as isize {
            let mut buf_ptr = buf_ptr_cur;
            while unsafe { buf_ptr.offset_from(start_ptr) } >= (loop_size * unroll) as isize {
                buf_ptr = unsafe { buf_ptr.sub(loop_size * unroll) };
                _unroll_one_rnechr_16!(buf_ptr, cc, start_ptr, loop_size, 7);
                _unroll_one_rnechr_16!(buf_ptr, cc, start_ptr, loop_size, 6);
                _unroll_one_rnechr_16!(buf_ptr, cc, start_ptr, loop_size, 5);
                _unroll_one_rnechr_16!(buf_ptr, cc, start_ptr, loop_size, 4);
                _unroll_one_rnechr_16!(buf_ptr, cc, start_ptr, loop_size, 3);
                _unroll_one_rnechr_16!(buf_ptr, cc, start_ptr, loop_size, 2);
                _unroll_one_rnechr_16!(buf_ptr, cc, start_ptr, loop_size, 1);
                _unroll_one_rnechr_16!(buf_ptr, cc, start_ptr, loop_size, 0);
            }
            buf_ptr_cur = buf_ptr;
        }
    }
    {
        let loop_size = 16;
        if unsafe { buf_ptr_cur.offset_from(start_ptr) } >= loop_size as isize {
            let mut buf_ptr = buf_ptr_cur;
            while unsafe { buf_ptr.offset_from(start_ptr) } >= loop_size as isize {
                buf_ptr = unsafe { buf_ptr.sub(loop_size) };
                _unroll_one_rnechr_16!(buf_ptr, cc, start_ptr, loop_size, 0);
            }
            buf_ptr_cur = buf_ptr;
        }
    }
    // the remaining data is the max: 15 bytes.
    _memrnechr_remaining_15_bytes_impl(buf_ptr_cur, cc as u64, start_ptr)
}

#[cfg(target_pointer_width = "64")]
#[inline(always)]
fn _start_rnechr_64(buf: &[u8], c: u8) -> Option<usize> {
    //
    let buf_len = buf.len();
    let start_ptr = buf.as_ptr();
    let mut buf_ptr_cur = unsafe { start_ptr.add(buf_len) };
    let cc: u64 = _c8_value(c);
    //
    {
        let unroll = 8;
        let loop_size = 8;
        if unsafe { buf_ptr_cur.offset_from(start_ptr) } >= (loop_size * unroll) as isize {
            let mut buf_ptr = buf_ptr_cur;
            while unsafe { buf_ptr.offset_from(start_ptr) } >= (loop_size * unroll) as isize {
                buf_ptr = unsafe { buf_ptr.sub(loop_size * unroll) };
                _unroll_one_rnechr_8!(buf_ptr, cc, start_ptr, loop_size, 7);
                _unroll_one_rnechr_8!(buf_ptr, cc, start_ptr, loop_size, 6);
                _unroll_one_rnechr_8!(buf_ptr, cc, start_ptr, loop_size, 5);
                _unroll_one_rnechr_8!(buf_ptr, cc, start_ptr, loop_size, 4);
                _unroll_one_rnechr_8!(buf_ptr, cc, start_ptr, loop_size, 3);
                _unroll_one_rnechr_8!(buf_ptr, cc, start_ptr, loop_size, 2);
                _unroll_one_rnechr_8!(buf_ptr, cc, start_ptr, loop_size, 1);
                _unroll_one_rnechr_8!(buf_ptr, cc, start_ptr, loop_size, 0);
            }
            buf_ptr_cur = buf_ptr;
        }
    }
    {
        let loop_size = 8;
        if unsafe { buf_ptr_cur.offset_from(start_ptr) } >= loop_size as isize {
            let mut buf_ptr = buf_ptr_cur;
            while unsafe { buf_ptr.offset_from(start_ptr) } >= loop_size as isize {
                buf_ptr = unsafe { buf_ptr.sub(loop_size) };
                _unroll_one_rnechr_8!(buf_ptr, cc, start_ptr, loop_size, 0);
            }
            buf_ptr_cur = buf_ptr;
        }
    }
    // the remaining data is the max: 7 bytes.
    _memrnechr_remaining_7_bytes_impl(buf_ptr_cur, cc as u32, start_ptr)
}

#[cfg(target_pointer_width = "32")]
#[inline(always)]
fn _start_rnechr_32(buf: &[u8], c: u8) -> Option<usize> {
    //
    let buf_len = buf.len();
    let start_ptr = buf.as_ptr();
    let mut buf_ptr_cur = unsafe { start_ptr.add(buf_len) };
    let cc: u32 = _c4_value(c);
    //
    {
        let unroll = 8;
        let loop_size = 4;
        if unsafe { buf_ptr_cur.offset_from(start_ptr) } >= (loop_size * unroll) as isize {
            let mut buf_ptr = buf_ptr_cur;
            while unsafe { buf_ptr.offset_from(start_ptr) } >= (loop_size * unroll) as isize {
                buf_ptr = unsafe { buf_ptr.sub(loop_size * unroll) };
                _unroll_one_rnechr_4!(buf_ptr, cc, start_ptr, loop_size, 7);
                _unroll_one_rnechr_4!(buf_ptr, cc, start_ptr, loop_size, 6);
                _unroll_one_rnechr_4!(buf_ptr, cc, start_ptr, loop_size, 5);
                _unroll_one_rnechr_4!(buf_ptr, cc, start_ptr, loop_size, 4);
                _unroll_one_rnechr_4!(buf_ptr, cc, start_ptr, loop_size, 3);
                _unroll_one_rnechr_4!(buf_ptr, cc, start_ptr, loop_size, 2);
                _unroll_one_rnechr_4!(buf_ptr, cc, start_ptr, loop_size, 1);
                _unroll_one_rnechr_4!(buf_ptr, cc, start_ptr, loop_size, 0);
            }
            buf_ptr_cur = buf_ptr;
        }
    }
    {
        let loop_size = 4;
        if unsafe { buf_ptr_cur.offset_from(start_ptr) } >= loop_size as isize {
            let mut buf_ptr = buf_ptr_cur;
            while unsafe { buf_ptr.offset_from(start_ptr) } >= loop_size as isize {
                buf_ptr = unsafe { buf_ptr.sub(loop_size) };
                _unroll_one_rnechr_4!(buf_ptr, cc, start_ptr, loop_size, 0);
            }
            buf_ptr_cur = buf_ptr;
        }
    }
    // the remaining data is the max: 3 bytes.
    _memrnechr_remaining_3_bytes_impl(buf_ptr_cur, cc as u16, start_ptr)
}

#[inline(always)]
pub(crate) fn _memrnechr_remaining_15_bytes_impl(
    buf_ptr_cur: *const u8,
    cc: u64,
    start_ptr: *const u8,
) -> Option<usize> {
    let mut buf_ptr_cur = buf_ptr_cur;
    {
        let loop_size = 8;
        if unsafe { buf_ptr_cur.offset_from(start_ptr) } >= loop_size as isize {
            let buf_ptr = unsafe { buf_ptr_cur.sub(loop_size) };
            _unroll_one_rnechr_8!(buf_ptr, cc, start_ptr, loop_size, 0);
            //
            buf_ptr_cur = buf_ptr;
        }
    }
    // the remaining data is the max: 7 bytes.
    _memrnechr_remaining_7_bytes_impl(buf_ptr_cur, cc as u32, start_ptr)
}

#[inline(always)]
pub(crate) fn _memrnechr_remaining_7_bytes_impl(
    buf_ptr_cur: *const u8,
    cc: u32,
    start_ptr: *const u8,
) -> Option<usize> {
    let mut buf_ptr_cur = buf_ptr_cur;
    {
        let loop_size = 4;
        if unsafe { buf_ptr_cur.offset_from(start_ptr) } >= loop_size as isize {
            let buf_ptr = unsafe { buf_ptr_cur.sub(loop_size) };
            _unroll_one_rnechr_4!(buf_ptr, cc, start_ptr, loop_size, 0);
            //
            buf_ptr_cur = buf_ptr;
        }
    }
    // the remaining data is the max: 3 bytes.
    _memrnechr_remaining_3_bytes_impl(buf_ptr_cur, cc as u16, start_ptr)
}

#[inline(always)]
pub(crate) fn _memrnechr_remaining_3_bytes_impl(
    buf_ptr_cur: *const u8,
    cc: u16,
    start_ptr: *const u8,
) -> Option<usize> {
    let mut buf_ptr_cur = buf_ptr_cur;
    {
        let loop_size = 2;
        if unsafe { buf_ptr_cur.offset_from(start_ptr) } >= loop_size as isize {
            let buf_ptr = unsafe { buf_ptr_cur.sub(loop_size) };
            _unroll_one_rnechr_2!(buf_ptr, cc, start_ptr, loop_size, 0);
            //
            buf_ptr_cur = buf_ptr;
        }
    }
    {
        let loop_size = 1;
        let cc: u8 = cc as u8;
        if unsafe { buf_ptr_cur.offset_from(start_ptr) } >= loop_size as isize {
            let buf_ptr = unsafe { buf_ptr_cur.sub(loop_size) };
            _unroll_one_rnechr_1!(buf_ptr, cc, start_ptr, loop_size, 0);
        }
    }
    //
    None
}

#[cfg(target_pointer_width = "16")]
#[inline(always)]
fn _start_rnechr_16(buf: &[u8], c: u8) -> Option<usize> {
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
                _unroll_one_rnechr_2!(buf_ptr, cc, start_ptr, loop_size, 7);
                _unroll_one_rnechr_2!(buf_ptr, cc, start_ptr, loop_size, 6);
                _unroll_one_rnechr_2!(buf_ptr, cc, start_ptr, loop_size, 5);
                _unroll_one_rnechr_2!(buf_ptr, cc, start_ptr, loop_size, 4);
                _unroll_one_rnechr_2!(buf_ptr, cc, start_ptr, loop_size, 3);
                _unroll_one_rnechr_2!(buf_ptr, cc, start_ptr, loop_size, 2);
                _unroll_one_rnechr_2!(buf_ptr, cc, start_ptr, loop_size, 1);
                _unroll_one_rnechr_2!(buf_ptr, cc, start_ptr, loop_size, 0);
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
                _unroll_one_rnechr_2!(buf_ptr, cc, start_ptr, loop_size, 0);
                //
                buf_ptr = unsafe { buf_ptr.sub(loop_size) };
            }
            buf_ptr_cur = unsafe { buf_ptr.add(loop_size) };
        }
    }
    // the remaining data is the max: 1 bytes.
    {
        let loop_size = 1;
        let cc: u8 = c;
        let buf_ptr = unsafe { buf_ptr_cur.sub(loop_size) };
        if buf_ptr >= start_ptr {
            _unroll_one_rnechr_1!(buf_ptr, cc, start_ptr, loop_size, 0);
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
fn _rnechr_c16(buf_ptr: *const u8, c16: u128, start_ptr: *const u8) -> Option<usize> {
    let v0 = unsafe {
        let slice = std::slice::from_raw_parts(buf_ptr, 16);
        crate::utils::_read_a_native_endian_u128(slice)
    };
    let v = v0 ^ c16;
    let bits = v.wrapping_sub(0x0101_0101_0101_0101_0101_0101_0101_0101_u128)
        & !v
        & 0x8080_8080_8080_8080_8080_8080_8080_8080_u128;
    if bits != 0x8080_8080_8080_8080_8080_8080_8080_8080_u128 {
        let bits = propagate_a_high_bit(bits);
        Some(
            plus_offset_from(buf_ptr, start_ptr) + 16
                - 1
                - ((bits as u128).leading_ones() / 8) as usize,
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
fn _rnechr_c8(buf_ptr: *const u8, c8: u64, start_ptr: *const u8) -> Option<usize> {
    let v0 = unsafe {
        let slice = std::slice::from_raw_parts(buf_ptr, 8);
        crate::utils::_read_a_native_endian_u64(slice)
    };
    let v = v0 ^ c8;
    let bits = v.wrapping_sub(0x0101_0101_0101_0101_u64) & !v & 0x8080_8080_8080_8080_u64;
    if bits != 0x8080_8080_8080_8080_u64 {
        let bits = propagate_a_high_bit(bits);
        Some(
            plus_offset_from(buf_ptr, start_ptr) + 8
                - 1
                - ((bits as u64).leading_ones() / 8) as usize,
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
fn _rnechr_c4(buf_ptr: *const u8, c4: u32, start_ptr: *const u8) -> Option<usize> {
    let v0 = unsafe {
        let slice = std::slice::from_raw_parts(buf_ptr, 4);
        crate::utils::_read_a_native_endian_u32(slice)
    };
    let v = v0 ^ c4;
    let bits = v.wrapping_sub(0x0101_0101_u32) & !v & 0x8080_8080_u32;
    if bits != 0x8080_8080_u32 {
        let bits = propagate_a_high_bit(bits);
        Some(
            plus_offset_from(buf_ptr, start_ptr) + 4
                - 1
                - ((bits as u32).leading_ones() / 8) as usize,
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
fn _rnechr_c2(buf_ptr: *const u8, c2: u16, start_ptr: *const u8) -> Option<usize> {
    let v0 = unsafe {
        let slice = std::slice::from_raw_parts(buf_ptr, 2);
        crate::utils::_read_a_native_endian_u16(slice)
    };
    let v = v0 ^ c2;
    let bits = v.wrapping_sub(0x0101_u16) & !v & 0x8080_u16;
    if bits != 0x8080_u16 {
        let bits = propagate_a_high_bit(bits);
        Some(
            plus_offset_from(buf_ptr, start_ptr) + 2
                - 1
                - ((bits as u16).leading_ones() / 8) as usize,
        )
    } else {
        None
    }
}

#[inline(always)]
fn _rnechr_c1(buf_ptr: *const u8, c1: u8, start_ptr: *const u8) -> Option<usize> {
    let aa_ptr = buf_ptr as *const u8;
    let aac = unsafe { *aa_ptr };
    if aac != c1 {
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
pub fn _memrnechr_impl(buf: &[u8], c: u8) -> Option<usize> {
    for i in 0..buf.len() {
        let j = buf.len()-i-1;
        if buf[j] == c {
            return Some(j);
        }
    }
    None
}

pub fn _memrnechr_impl(buf: &[u8], c: u8) -> Option<usize> {
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
