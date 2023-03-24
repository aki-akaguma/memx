use crate::utils::*;

#[inline(always)]
pub fn _memrchr_double_impl(buf: &[u8], c1: u8, c2: u8) -> Option<usize> {
    if buf.is_empty() {
        return None;
    }
    if buf[buf.len() - 1] == c1 || buf[buf.len() - 1] == c2 {
        return Some(buf.len() - 1);
    }
    #[cfg(all(
        any(feature = "test", tarpaulin),
        any(
            feature = "test_pointer_width_128",
            feature = "test_pointer_width_64",
            feature = "test_pointer_width_32"
        )
    ))]
    {
        #[cfg(feature = "test_pointer_width_128")]
        let r = _start_rchr_128(buf, c1, c2);
        #[cfg(feature = "test_pointer_width_64")]
        let r = _start_rchr_64(buf, c1, c2);
        #[cfg(feature = "test_pointer_width_32")]
        let r = _start_rchr_32(buf, c1, c2);
        //
        r
    }
    #[cfg(not(all(
        any(feature = "test", tarpaulin),
        any(
            feature = "test_pointer_width_128",
            feature = "test_pointer_width_64",
            feature = "test_pointer_width_32"
        )
    )))]
    {
        #[cfg(target_pointer_width = "128")]
        let r = _start_rchr_128(buf, c1, c2);
        #[cfg(target_pointer_width = "64")]
        let r = _start_rchr_64(buf, c1, c2);
        #[cfg(target_pointer_width = "32")]
        let r = _start_rchr_32(buf, c1, c2);
        //
        r
    }
}

macro_rules! _unroll_one_rchr_16 {
    ($a_ptr:expr, $cc1:expr, $cc2:expr, $start_ptr:expr, $loop_size:expr, $idx:expr) => {{
        let aa_ptr = unsafe { $a_ptr.add($loop_size * $idx) };
        let r1 = _rchr_c16(aa_ptr, $cc1, $start_ptr);
        let r2 = _rchr_c16(aa_ptr, $cc2, $start_ptr);
        if !r1.is_none() && r2.is_none() {
            return r1;
        } else if r1.is_none() && !r2.is_none() {
            return r2;
        } else if !r1.is_none() && !r2.is_none() {
            let idx1 = r1.map(|a| a).unwrap();
            let idx2 = r2.map(|a| a).unwrap();
            if idx1 > idx2 {
                return Some(idx1);
            } else {
                return Some(idx2);
            }
        }
    }};
}

macro_rules! _unroll_one_rchr_8 {
    ($a_ptr:expr, $cc1:expr, $cc2:expr, $start_ptr:expr, $loop_size:expr, $idx:expr) => {{
        let aa_ptr = unsafe { $a_ptr.add($loop_size * $idx) };
        let r1 = _rchr_c8(aa_ptr, $cc1, $start_ptr);
        let r2 = _rchr_c8(aa_ptr, $cc2, $start_ptr);
        if !r1.is_none() && r2.is_none() {
            return r1;
        } else if r1.is_none() && !r2.is_none() {
            return r2;
        } else if !r1.is_none() && !r2.is_none() {
            let idx1 = r1.map(|a| a).unwrap();
            let idx2 = r2.map(|a| a).unwrap();
            if idx1 > idx2 {
                return Some(idx1);
            } else {
                return Some(idx2);
            }
        }
    }};
}

macro_rules! _unroll_one_rchr_4 {
    ($a_ptr:expr, $cc1:expr, $cc2:expr, $start_ptr:expr, $loop_size:expr, $idx:expr) => {{
        let aa_ptr = unsafe { $a_ptr.add($loop_size * $idx) };
        let r1 = _rchr_c4(aa_ptr, $cc1, $start_ptr);
        let r2 = _rchr_c4(aa_ptr, $cc2, $start_ptr);
        if !r1.is_none() && r2.is_none() {
            return r1;
        } else if r1.is_none() && !r2.is_none() {
            return r2;
        } else if !r1.is_none() && !r2.is_none() {
            let idx1 = r1.map(|a| a).unwrap();
            let idx2 = r2.map(|a| a).unwrap();
            if idx1 > idx2 {
                return Some(idx1);
            } else {
                return Some(idx2);
            }
        }
    }};
}

macro_rules! _unroll_one_rchr_2 {
    ($a_ptr:expr, $cc1:expr, $cc2:expr, $start_ptr:expr, $loop_size:expr, $idx:expr) => {{
        let aa_ptr = unsafe { $a_ptr.add($loop_size * $idx) };
        let r1 = _rchr_c2(aa_ptr, $cc1, $start_ptr);
        let r2 = _rchr_c2(aa_ptr, $cc2, $start_ptr);
        if !r1.is_none() && r2.is_none() {
            return r1;
        } else if r1.is_none() && !r2.is_none() {
            return r2;
        } else if !r1.is_none() && !r2.is_none() {
            let idx1 = r1.map(|a| a).unwrap();
            let idx2 = r2.map(|a| a).unwrap();
            if idx1 > idx2 {
                return Some(idx1);
            } else {
                return Some(idx2);
            }
        }
    }};
}

macro_rules! _unroll_one_rchr_1 {
    ($a_ptr:expr, $cc1:expr, $cc2:expr, $start_ptr:expr, $loop_size:expr, $idx:expr) => {{
        let aa_ptr = unsafe { $a_ptr.add($loop_size * $idx) };
        let r1 = _rchr_c1(aa_ptr, $cc1, $start_ptr);
        let r2 = _rchr_c1(aa_ptr, $cc2, $start_ptr);
        if !r1.is_none() && r2.is_none() {
            return r1;
        } else if r1.is_none() && !r2.is_none() {
            return r2;
        } else if !r1.is_none() && !r2.is_none() {
            let idx1 = r1.map(|a| a).unwrap();
            let idx2 = r2.map(|a| a).unwrap();
            if idx1 > idx2 {
                return Some(idx1);
            } else {
                return Some(idx2);
            }
        }
    }};
}

#[cfg(any(target_pointer_width = "128", feature = "test_pointer_width_128"))]
#[inline(always)]
fn _start_rchr_128(buf: &[u8], c_1: u8, c_2: u8) -> Option<usize> {
    //
    let buf_len = buf.len();
    let start_ptr = buf.as_ptr();
    let mut buf_ptr_cur = unsafe { start_ptr.add(buf_len) };
    let cc1: u128 = _c16_value(c_1);
    let cc2: u128 = _c16_value(c_2);
    //
    {
        let unroll = 8;
        let loop_size = 16;
        if unsafe { buf_ptr_cur.offset_from(start_ptr) } >= (loop_size * unroll) as isize {
            let mut buf_ptr = buf_ptr_cur;
            while unsafe { buf_ptr.offset_from(start_ptr) } >= (loop_size * unroll) as isize {
                buf_ptr = unsafe { buf_ptr.sub(loop_size * unroll) };
                _unroll_one_rchr_16!(buf_ptr, cc1, cc2, start_ptr, loop_size, 7);
                _unroll_one_rchr_16!(buf_ptr, cc1, cc2, start_ptr, loop_size, 6);
                _unroll_one_rchr_16!(buf_ptr, cc1, cc2, start_ptr, loop_size, 5);
                _unroll_one_rchr_16!(buf_ptr, cc1, cc2, start_ptr, loop_size, 4);
                _unroll_one_rchr_16!(buf_ptr, cc1, cc2, start_ptr, loop_size, 3);
                _unroll_one_rchr_16!(buf_ptr, cc1, cc2, start_ptr, loop_size, 2);
                _unroll_one_rchr_16!(buf_ptr, cc1, cc2, start_ptr, loop_size, 1);
                _unroll_one_rchr_16!(buf_ptr, cc1, cc2, start_ptr, loop_size, 0);
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
                _unroll_one_rchr_16!(buf_ptr, cc1, cc2, start_ptr, loop_size, 0);
            }
            buf_ptr_cur = buf_ptr;
        }
    }
    // the remaining data is the max: 15 bytes.
    _memrchr_double_remaining_15_bytes_impl(buf_ptr_cur, cc1 as u64, cc2 as u64, start_ptr)
}

#[cfg(any(target_pointer_width = "64", feature = "test_pointer_width_64"))]
#[inline(always)]
fn _start_rchr_64(buf: &[u8], c_1: u8, c_2: u8) -> Option<usize> {
    //
    let buf_len = buf.len();
    let start_ptr = buf.as_ptr();
    let mut buf_ptr_cur = unsafe { start_ptr.add(buf_len) };
    let cc1: u64 = _c8_value(c_1);
    let cc2: u64 = _c8_value(c_2);
    //
    {
        let unroll = 8;
        let loop_size = 8;
        if unsafe { buf_ptr_cur.offset_from(start_ptr) } >= (loop_size * unroll) as isize {
            let mut buf_ptr = buf_ptr_cur;
            while unsafe { buf_ptr.offset_from(start_ptr) } >= (loop_size * unroll) as isize {
                buf_ptr = unsafe { buf_ptr.sub(loop_size * unroll) };
                _unroll_one_rchr_8!(buf_ptr, cc1, cc2, start_ptr, loop_size, 7);
                _unroll_one_rchr_8!(buf_ptr, cc1, cc2, start_ptr, loop_size, 6);
                _unroll_one_rchr_8!(buf_ptr, cc1, cc2, start_ptr, loop_size, 5);
                _unroll_one_rchr_8!(buf_ptr, cc1, cc2, start_ptr, loop_size, 4);
                _unroll_one_rchr_8!(buf_ptr, cc1, cc2, start_ptr, loop_size, 3);
                _unroll_one_rchr_8!(buf_ptr, cc1, cc2, start_ptr, loop_size, 2);
                _unroll_one_rchr_8!(buf_ptr, cc1, cc2, start_ptr, loop_size, 1);
                _unroll_one_rchr_8!(buf_ptr, cc1, cc2, start_ptr, loop_size, 0);
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
                _unroll_one_rchr_8!(buf_ptr, cc1, cc2, start_ptr, loop_size, 0);
            }
            buf_ptr_cur = buf_ptr;
        }
    }
    // the remaining data is the max: 7 bytes.
    _memrchr_double_remaining_7_bytes_impl(buf_ptr_cur, cc1 as u32, cc2 as u32, start_ptr)
}

#[cfg(any(target_pointer_width = "32", feature = "test_pointer_width_32"))]
#[inline(always)]
fn _start_rchr_32(buf: &[u8], c_1: u8, c_2: u8) -> Option<usize> {
    //
    let buf_len = buf.len();
    let start_ptr = buf.as_ptr();
    let mut buf_ptr_cur = unsafe { start_ptr.add(buf_len) };
    let cc1: u32 = _c4_value(c_1);
    let cc2: u32 = _c4_value(c_2);
    //
    {
        let unroll = 8;
        let loop_size = 4;
        if unsafe { buf_ptr_cur.offset_from(start_ptr) } >= (loop_size * unroll) as isize {
            let mut buf_ptr = buf_ptr_cur;
            while unsafe { buf_ptr.offset_from(start_ptr) } >= (loop_size * unroll) as isize {
                buf_ptr = unsafe { buf_ptr.sub(loop_size * unroll) };
                _unroll_one_rchr_4!(buf_ptr, cc1, cc2, start_ptr, loop_size, 7);
                _unroll_one_rchr_4!(buf_ptr, cc1, cc2, start_ptr, loop_size, 6);
                _unroll_one_rchr_4!(buf_ptr, cc1, cc2, start_ptr, loop_size, 5);
                _unroll_one_rchr_4!(buf_ptr, cc1, cc2, start_ptr, loop_size, 4);
                _unroll_one_rchr_4!(buf_ptr, cc1, cc2, start_ptr, loop_size, 3);
                _unroll_one_rchr_4!(buf_ptr, cc1, cc2, start_ptr, loop_size, 2);
                _unroll_one_rchr_4!(buf_ptr, cc1, cc2, start_ptr, loop_size, 1);
                _unroll_one_rchr_4!(buf_ptr, cc1, cc2, start_ptr, loop_size, 0);
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
                _unroll_one_rchr_4!(buf_ptr, cc1, cc2, start_ptr, loop_size, 0);
            }
            buf_ptr_cur = buf_ptr;
        }
    }
    // the remaining data is the max: 3 bytes.
    _memrchr_double_remaining_3_bytes_impl(buf_ptr_cur, cc1 as u16, cc2 as u16, start_ptr)
}

#[inline(always)]
pub(crate) fn _memrchr_double_remaining_15_bytes_impl(
    buf_ptr_cur: *const u8,
    cc1: u64,
    cc2: u64,
    start_ptr: *const u8,
) -> Option<usize> {
    let mut buf_ptr_cur = buf_ptr_cur;
    {
        let loop_size = 8;
        if unsafe { buf_ptr_cur.offset_from(start_ptr) } >= loop_size as isize {
            let buf_ptr = unsafe { buf_ptr_cur.sub(loop_size) };
            _unroll_one_rchr_8!(buf_ptr, cc1, cc2, start_ptr, loop_size, 0);
            //
            buf_ptr_cur = buf_ptr;
        }
    }
    // the remaining data is the max: 7 bytes.
    _memrchr_double_remaining_7_bytes_impl(buf_ptr_cur, cc1 as u32, cc2 as u32, start_ptr)
}

#[inline(always)]
pub(crate) fn _memrchr_double_remaining_7_bytes_impl(
    buf_ptr_cur: *const u8,
    cc1: u32,
    cc2: u32,
    start_ptr: *const u8,
) -> Option<usize> {
    let mut buf_ptr_cur = buf_ptr_cur;
    {
        let loop_size = 4;
        if unsafe { buf_ptr_cur.offset_from(start_ptr) } >= loop_size as isize {
            let buf_ptr = unsafe { buf_ptr_cur.sub(loop_size) };
            _unroll_one_rchr_4!(buf_ptr, cc1, cc2, start_ptr, loop_size, 0);
            //
            buf_ptr_cur = buf_ptr;
        }
    }
    // the remaining data is the max: 3 bytes.
    _memrchr_double_remaining_3_bytes_impl(buf_ptr_cur, cc1 as u16, cc2 as u16, start_ptr)
}

#[inline(always)]
pub(crate) fn _memrchr_double_remaining_3_bytes_impl(
    buf_ptr_cur: *const u8,
    cc1: u16,
    cc2: u16,
    start_ptr: *const u8,
) -> Option<usize> {
    let mut buf_ptr_cur = buf_ptr_cur;
    {
        let loop_size = 2;
        if unsafe { buf_ptr_cur.offset_from(start_ptr) } >= loop_size as isize {
            let buf_ptr = unsafe { buf_ptr_cur.sub(loop_size) };
            _unroll_one_rchr_2!(buf_ptr, cc1, cc2, start_ptr, loop_size, 0);
            //
            buf_ptr_cur = buf_ptr;
        }
    }
    {
        let loop_size = 1;
        if unsafe { buf_ptr_cur.offset_from(start_ptr) } >= loop_size as isize {
            let buf_ptr = unsafe { buf_ptr_cur.sub(loop_size) };
            _unroll_one_rchr_1!(buf_ptr, cc1 as u8, cc2 as u8, start_ptr, loop_size, 0);
        }
    }
    //
    None
}

#[inline(always)]
fn _c16_value(c: u8) -> u128 {
    (c as u128) * PackedU128::ONES
}

#[inline(always)]
fn _rchr_c16(buf_ptr: *const u8, c16: u128, start_ptr: *const u8) -> Option<usize> {
    let v0 = unsafe { _read_a_big_endian_from_ptr_u128(buf_ptr) };
    let v = v0 ^ c16;
    //
    let bits = PackedU128::new(v).may_have_zero_quick();
    if !bits.is_zeros() {
        Some(plus_offset_from(buf_ptr, start_ptr) + 16 - 1 - (bits.trailing_zeros() / 8) as usize)
    } else {
        None
    }
}

#[inline(always)]
fn _c8_value(c: u8) -> u64 {
    (c as u64) * PackedU64::ONES
}

#[inline(always)]
fn _rchr_c8(buf_ptr: *const u8, c8: u64, start_ptr: *const u8) -> Option<usize> {
    let v0 = unsafe { _read_a_big_endian_from_ptr_u64(buf_ptr) };
    let v = v0 ^ c8;
    //
    let bits = PackedU64::new(v).may_have_zero_quick();
    if !bits.is_zeros() {
        Some(plus_offset_from(buf_ptr, start_ptr) + 8 - 1 - (bits.trailing_zeros() / 8) as usize)
    } else {
        None
    }
}

#[inline(always)]
fn _c4_value(c: u8) -> u32 {
    (c as u32) * PackedU32::ONES
}

#[inline(always)]
fn _rchr_c4(buf_ptr: *const u8, c4: u32, start_ptr: *const u8) -> Option<usize> {
    let v0 = unsafe { _read_a_big_endian_from_ptr_u32(buf_ptr) };
    let v = v0 ^ c4;
    //
    let bits = PackedU32::new(v).may_have_zero_quick();
    if !bits.is_zeros() {
        Some(plus_offset_from(buf_ptr, start_ptr) + 4 - 1 - (bits.trailing_zeros() / 8) as usize)
    } else {
        None
    }
}

#[inline(always)]
fn _rchr_c2(buf_ptr: *const u8, c2: u16, start_ptr: *const u8) -> Option<usize> {
    let v0 = unsafe { _read_a_big_endian_from_ptr_u16(buf_ptr) };
    let v = v0 ^ c2;
    //
    let bits = PackedU16::new(v).may_have_zero_quick();
    if !bits.is_zeros() {
        Some(plus_offset_from(buf_ptr, start_ptr) + 2 - 1 - (bits.trailing_zeros() / 8) as usize)
    } else {
        None
    }
}

#[inline(always)]
fn _rchr_c1(buf_ptr: *const u8, c1: u8, start_ptr: *const u8) -> Option<usize> {
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
