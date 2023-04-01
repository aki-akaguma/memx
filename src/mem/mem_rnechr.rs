use crate::utils::*;

#[inline(always)]
pub fn _memrnechr_impl(buf: &[u8], c: u8) -> Option<usize> {
    if buf.is_empty() {
        return None;
    }
    if buf[buf.len() - 1] != c {
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
        let r = _start_rnechr_128(buf, c);
        #[cfg(feature = "test_pointer_width_64")]
        let r = _start_rnechr_64(buf, c);
        #[cfg(feature = "test_pointer_width_32")]
        let r = _start_rnechr_32(buf, c);
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
        let r = _start_rnechr_128(buf, c);
        #[cfg(target_pointer_width = "64")]
        let r = _start_rnechr_64(buf, c);
        #[cfg(target_pointer_width = "32")]
        let r = _start_rnechr_32(buf, c);
        //
        r
    }
}

macro_rules! _unroll_one_rnechr_to_align {
    ($buf_ptr_2:expr, $buf_ptr_min:expr, $c:expr, $start_ptr:expr) => {{
        if $buf_ptr_2 <= $buf_ptr_min {
            break;
        }
        $buf_ptr_2 = unsafe { $buf_ptr_2.sub(1) };
        if unsafe { *$buf_ptr_2 } != $c {
            return (None, Some(plus_offset_from($buf_ptr_2, $start_ptr)));
        }
    }};
}

macro_rules! _unroll_one_rnechr_to_align_x4 {
    ($buf_ptr_2:expr, $buf_ptr_min:expr, $c:expr, $start_ptr:expr) => {{
        _unroll_one_rnechr_to_align!($buf_ptr_2, $buf_ptr_min, $c, $start_ptr);
        _unroll_one_rnechr_to_align!($buf_ptr_2, $buf_ptr_min, $c, $start_ptr);
        _unroll_one_rnechr_to_align!($buf_ptr_2, $buf_ptr_min, $c, $start_ptr);
        _unroll_one_rnechr_to_align!($buf_ptr_2, $buf_ptr_min, $c, $start_ptr);
    }};
}

pub(crate) fn _rnechr_to_aligned_u256(
    buf_ptr_cur: *const u8,
    c: u8,
    start_ptr: *const u8,
) -> (Option<*const u8>, Option<usize>) {
    let buf_ptr_end = buf_ptr_cur;
    let remaining_align = (buf_ptr_end as usize) & 0x1F_usize;
    let buf_ptr_min = unsafe { buf_ptr_end.sub(remaining_align) };
    let mut buf_ptr_2 = buf_ptr_end;
    loop {
        _unroll_one_rnechr_to_align_x4!(buf_ptr_2, buf_ptr_min, c, start_ptr);
        _unroll_one_rnechr_to_align_x4!(buf_ptr_2, buf_ptr_min, c, start_ptr);
        _unroll_one_rnechr_to_align_x4!(buf_ptr_2, buf_ptr_min, c, start_ptr);
        _unroll_one_rnechr_to_align_x4!(buf_ptr_2, buf_ptr_min, c, start_ptr);
        //
        _unroll_one_rnechr_to_align_x4!(buf_ptr_2, buf_ptr_min, c, start_ptr);
        _unroll_one_rnechr_to_align_x4!(buf_ptr_2, buf_ptr_min, c, start_ptr);
        _unroll_one_rnechr_to_align_x4!(buf_ptr_2, buf_ptr_min, c, start_ptr);
        _unroll_one_rnechr_to_align_x4!(buf_ptr_2, buf_ptr_min, c, start_ptr);
    }
    (Some(buf_ptr_min), None)
}

pub(crate) fn _rnechr_to_aligned_u128(
    buf_ptr_cur: *const u8,
    c: u8,
    start_ptr: *const u8,
) -> (Option<*const u8>, Option<usize>) {
    let buf_ptr_end = buf_ptr_cur;
    let remaining_align = (buf_ptr_end as usize) & 0x0F_usize;
    let buf_ptr_min = unsafe { buf_ptr_end.sub(remaining_align) };
    let mut buf_ptr_2 = buf_ptr_end;
    loop {
        _unroll_one_rnechr_to_align_x4!(buf_ptr_2, buf_ptr_min, c, start_ptr);
        _unroll_one_rnechr_to_align_x4!(buf_ptr_2, buf_ptr_min, c, start_ptr);
        _unroll_one_rnechr_to_align_x4!(buf_ptr_2, buf_ptr_min, c, start_ptr);
        _unroll_one_rnechr_to_align_x4!(buf_ptr_2, buf_ptr_min, c, start_ptr);
    }
    (Some(buf_ptr_min), None)
}

fn _rnechr_to_aligned_u64(
    buf_ptr_cur: *const u8,
    c: u8,
    start_ptr: *const u8,
) -> (Option<*const u8>, Option<usize>) {
    let buf_ptr_end = buf_ptr_cur;
    let remaining_align = (buf_ptr_end as usize) & 0x07_usize;
    let buf_ptr_min = unsafe { buf_ptr_end.sub(remaining_align) };
    let mut buf_ptr_2 = buf_ptr_end;
    loop {
        _unroll_one_rnechr_to_align_x4!(buf_ptr_2, buf_ptr_min, c, start_ptr);
        _unroll_one_rnechr_to_align_x4!(buf_ptr_2, buf_ptr_min, c, start_ptr);
    }
    (Some(buf_ptr_min), None)
}

fn _rnechr_to_aligned_u32(
    buf_ptr_cur: *const u8,
    c: u8,
    start_ptr: *const u8,
) -> (Option<*const u8>, Option<usize>) {
    let buf_ptr_end = buf_ptr_cur;
    let remaining_align = (buf_ptr_end as usize) & 0x03_usize;
    let buf_ptr_min = unsafe { buf_ptr_end.sub(remaining_align) };
    let mut buf_ptr_2 = buf_ptr_end;
    loop {
        _unroll_one_rnechr_to_align_x4!(buf_ptr_2, buf_ptr_min, c, start_ptr);
    }
    (Some(buf_ptr_min), None)
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

#[cfg(any(target_pointer_width = "128", feature = "test_pointer_width_128"))]
#[inline(always)]
fn _start_rnechr_128(buf: &[u8], c: u8) -> Option<usize> {
    //
    let buf_len = buf.len();
    let start_ptr = buf.as_ptr();
    let mut buf_ptr_cur = unsafe { start_ptr.add(buf_len) };
    let cc: u128 = _c16_value(c);
    //
    if buf_len >= 16 {
        {
            let loop_size = 16;
            let buf_ptr = unsafe { buf_ptr_cur.sub(loop_size) };
            buf_ptr.prefetch_read_data();
            if !buf_ptr_cur.is_aligned_u128() {
                let r = _rnechr_to_aligned_u128(buf_ptr_cur, c, start_ptr);
                if let Some(p) = r.0 {
                    buf_ptr_cur = p;
                } else if let Some(v) = r.1 {
                    return Some(v);
                }
            }
        }
        {
            let unroll = 8;
            let loop_size = 16;
            if unsafe { buf_ptr_cur.offset_from(start_ptr) } >= (loop_size * unroll) as isize {
                let mut buf_ptr = buf_ptr_cur;
                while unsafe { buf_ptr.offset_from(start_ptr) } >= (loop_size * unroll) as isize {
                    buf_ptr = unsafe { buf_ptr.sub(loop_size * unroll) };
                    buf_ptr.prefetch_read_data();
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
            let unroll = 4;
            let loop_size = 16;
            if unsafe { buf_ptr_cur.offset_from(start_ptr) } >= (loop_size * unroll) as isize {
                let mut buf_ptr = buf_ptr_cur;
                while unsafe { buf_ptr.offset_from(start_ptr) } >= (loop_size * unroll) as isize {
                    buf_ptr = unsafe { buf_ptr.sub(loop_size * unroll) };
                    buf_ptr.prefetch_read_data();
                    _unroll_one_rnechr_16!(buf_ptr, cc, start_ptr, loop_size, 3);
                    _unroll_one_rnechr_16!(buf_ptr, cc, start_ptr, loop_size, 2);
                    _unroll_one_rnechr_16!(buf_ptr, cc, start_ptr, loop_size, 1);
                    _unroll_one_rnechr_16!(buf_ptr, cc, start_ptr, loop_size, 0);
                }
                buf_ptr_cur = buf_ptr;
            }
        }
        {
            let unroll = 2;
            let loop_size = 16;
            if unsafe { buf_ptr_cur.offset_from(start_ptr) } >= (loop_size * unroll) as isize {
                let mut buf_ptr = buf_ptr_cur;
                while unsafe { buf_ptr.offset_from(start_ptr) } >= (loop_size * unroll) as isize {
                    buf_ptr = unsafe { buf_ptr.sub(loop_size * unroll) };
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
    }
    start_ptr.prefetch_read_data();
    // the remaining data is the max: 15 bytes.
    _memrnechr_remaining_15_bytes_impl(buf_ptr_cur, cc as u64, start_ptr)
}

#[cfg(any(target_pointer_width = "64", feature = "test_pointer_width_64"))]
#[inline(always)]
fn _start_rnechr_64(buf: &[u8], c: u8) -> Option<usize> {
    //
    let buf_len = buf.len();
    let start_ptr = buf.as_ptr();
    let mut buf_ptr_cur = unsafe { start_ptr.add(buf_len) };
    let cc: u64 = _c8_value(c);
    //
    if buf_len >= 8 {
        {
            let loop_size = 8;
            let buf_ptr = unsafe { buf_ptr_cur.sub(loop_size) };
            buf_ptr.prefetch_read_data();
            if !buf_ptr_cur.is_aligned_u64() {
                let r = _rnechr_to_aligned_u64(buf_ptr_cur, c, start_ptr);
                if let Some(p) = r.0 {
                    buf_ptr_cur = p;
                } else if let Some(v) = r.1 {
                    return Some(v);
                }
            }
        }
        {
            let unroll = 8;
            let loop_size = 8;
            if unsafe { buf_ptr_cur.offset_from(start_ptr) } >= (loop_size * unroll) as isize {
                let mut buf_ptr = buf_ptr_cur;
                while unsafe { buf_ptr.offset_from(start_ptr) } >= (loop_size * unroll) as isize {
                    buf_ptr = unsafe { buf_ptr.sub(loop_size * unroll) };
                    buf_ptr.prefetch_read_data();
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
            let unroll = 4;
            let loop_size = 8;
            if unsafe { buf_ptr_cur.offset_from(start_ptr) } >= (loop_size * unroll) as isize {
                let mut buf_ptr = buf_ptr_cur;
                while unsafe { buf_ptr.offset_from(start_ptr) } >= (loop_size * unroll) as isize {
                    buf_ptr = unsafe { buf_ptr.sub(loop_size * unroll) };
                    buf_ptr.prefetch_read_data();
                    _unroll_one_rnechr_8!(buf_ptr, cc, start_ptr, loop_size, 3);
                    _unroll_one_rnechr_8!(buf_ptr, cc, start_ptr, loop_size, 2);
                    _unroll_one_rnechr_8!(buf_ptr, cc, start_ptr, loop_size, 1);
                    _unroll_one_rnechr_8!(buf_ptr, cc, start_ptr, loop_size, 0);
                }
                buf_ptr_cur = buf_ptr;
            }
        }
        {
            let unroll = 2;
            let loop_size = 8;
            if unsafe { buf_ptr_cur.offset_from(start_ptr) } >= (loop_size * unroll) as isize {
                let mut buf_ptr = buf_ptr_cur;
                while unsafe { buf_ptr.offset_from(start_ptr) } >= (loop_size * unroll) as isize {
                    buf_ptr = unsafe { buf_ptr.sub(loop_size * unroll) };
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
    }
    start_ptr.prefetch_read_data();
    // the remaining data is the max: 7 bytes.
    _memrnechr_remaining_7_bytes_impl(buf_ptr_cur, cc as u32, start_ptr)
}

#[cfg(any(target_pointer_width = "32", feature = "test_pointer_width_32"))]
#[inline(always)]
fn _start_rnechr_32(buf: &[u8], c: u8) -> Option<usize> {
    //
    let buf_len = buf.len();
    let start_ptr = buf.as_ptr();
    let mut buf_ptr_cur = unsafe { start_ptr.add(buf_len) };
    let cc: u32 = _c4_value(c);
    //
    if buf_len >= 4 {
        {
            let loop_size = 4;
            let buf_ptr = unsafe { buf_ptr_cur.sub(loop_size) };
            buf_ptr.prefetch_read_data();
            if !buf_ptr_cur.is_aligned_u32() {
                let r = _rnechr_to_aligned_u32(buf_ptr_cur, c, start_ptr);
                if let Some(p) = r.0 {
                    buf_ptr_cur = p;
                } else if let Some(v) = r.1 {
                    return Some(v);
                }
            }
        }
        {
            let unroll = 8;
            let loop_size = 4;
            if unsafe { buf_ptr_cur.offset_from(start_ptr) } >= (loop_size * unroll) as isize {
                let mut buf_ptr = buf_ptr_cur;
                while unsafe { buf_ptr.offset_from(start_ptr) } >= (loop_size * unroll) as isize {
                    buf_ptr = unsafe { buf_ptr.sub(loop_size * unroll) };
                    buf_ptr.prefetch_read_data();
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
            let unroll = 4;
            let loop_size = 4;
            if unsafe { buf_ptr_cur.offset_from(start_ptr) } >= (loop_size * unroll) as isize {
                let mut buf_ptr = buf_ptr_cur;
                while unsafe { buf_ptr.offset_from(start_ptr) } >= (loop_size * unroll) as isize {
                    buf_ptr = unsafe { buf_ptr.sub(loop_size * unroll) };
                    buf_ptr.prefetch_read_data();
                    _unroll_one_rnechr_4!(buf_ptr, cc, start_ptr, loop_size, 3);
                    _unroll_one_rnechr_4!(buf_ptr, cc, start_ptr, loop_size, 2);
                    _unroll_one_rnechr_4!(buf_ptr, cc, start_ptr, loop_size, 1);
                    _unroll_one_rnechr_4!(buf_ptr, cc, start_ptr, loop_size, 0);
                }
                buf_ptr_cur = buf_ptr;
            }
        }
        {
            let unroll = 2;
            let loop_size = 4;
            if unsafe { buf_ptr_cur.offset_from(start_ptr) } >= (loop_size * unroll) as isize {
                let mut buf_ptr = buf_ptr_cur;
                while unsafe { buf_ptr.offset_from(start_ptr) } >= (loop_size * unroll) as isize {
                    buf_ptr = unsafe { buf_ptr.sub(loop_size * unroll) };
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
    if buf_ptr_cur.is_aligned_u64() {
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
    if buf_ptr_cur.is_aligned_u32() {
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
    if buf_ptr_cur.is_aligned_u16() {
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
        if unsafe { buf_ptr_cur.offset_from(start_ptr) } >= loop_size as isize {
            let mut buf_ptr = buf_ptr_cur;
            while unsafe { buf_ptr.offset_from(start_ptr) } >= loop_size as isize {
                buf_ptr = unsafe { buf_ptr.sub(loop_size) };
                _unroll_one_rnechr_1!(buf_ptr, cc as u8, start_ptr, loop_size, 0);
            }
        }
    }
    //
    None
}

#[inline(always)]
fn _rnechr_c16(buf_ptr: *const u8, c16: u128, start_ptr: *const u8) -> Option<usize> {
    let v0 = unsafe { _read_a_little_endian_from_ptr_u128(buf_ptr) };
    let v = v0 ^ c16;
    //
    let bits = PackedU128::new(v).may_have_zero_byte();
    if !bits.is_highs() {
        let bits = bits.propagate_a_high_bit();
        Some(plus_offset_from(buf_ptr, start_ptr) + 16 - 1 - (bits.leading_ones() / 8) as usize)
    } else {
        None
    }
}

#[inline(always)]
fn _rnechr_c8(buf_ptr: *const u8, c8: u64, start_ptr: *const u8) -> Option<usize> {
    let v0 = unsafe { _read_a_little_endian_from_ptr_u64(buf_ptr) };
    let v = v0 ^ c8;
    //
    let bits = PackedU64::new(v).may_have_zero_byte();
    if !bits.is_highs() {
        let bits = bits.propagate_a_high_bit();
        Some(plus_offset_from(buf_ptr, start_ptr) + 8 - 1 - (bits.leading_ones() / 8) as usize)
    } else {
        None
    }
}

#[inline(always)]
fn _rnechr_c4(buf_ptr: *const u8, c4: u32, start_ptr: *const u8) -> Option<usize> {
    let v0 = unsafe { _read_a_little_endian_from_ptr_u32(buf_ptr) };
    let v = v0 ^ c4;
    //
    let bits = PackedU32::new(v).may_have_zero_byte();
    if !bits.is_highs() {
        let bits = bits.propagate_a_high_bit();
        Some(plus_offset_from(buf_ptr, start_ptr) + 4 - 1 - (bits.leading_ones() / 8) as usize)
    } else {
        None
    }
}

#[inline(always)]
fn _rnechr_c2(buf_ptr: *const u8, c2: u16, start_ptr: *const u8) -> Option<usize> {
    let v0 = unsafe { _read_a_little_endian_from_ptr_u16(buf_ptr) };
    let v = v0 ^ c2;
    //
    let bits = PackedU16::new(v).may_have_zero_byte();
    if !bits.is_highs() {
        let bits = bits.propagate_a_high_bit();
        Some(plus_offset_from(buf_ptr, start_ptr) + 2 - 1 - (bits.leading_ones() / 8) as usize)
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
