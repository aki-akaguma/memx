use crate::utils::*;

#[inline(always)]
pub fn _memchr_double_impl(buf: &[u8], c1: u8, c2: u8) -> Option<usize> {
    if buf.is_empty() {
        return None;
    }
    if buf[0] == c1 || buf[0] == c2 {
        return Some(0);
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
        let r = _start_chr_128(buf, c1, c2);
        #[cfg(feature = "test_pointer_width_64")]
        let r = _start_chr_64(buf, c1, c2);
        #[cfg(feature = "test_pointer_width_32")]
        let r = _start_chr_32(buf, c1, c2);
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
        let r = _start_chr_128(buf, c1, c2);
        #[cfg(target_pointer_width = "64")]
        let r = _start_chr_64(buf, c1, c2);
        #[cfg(target_pointer_width = "32")]
        let r = _start_chr_32(buf, c1, c2);
        //
        r
    }
}

macro_rules! _unroll_one_chr_to_align {
    ($buf_ptr_2:expr, $buf_ptr_end:expr, $c1:expr, $c2:expr, $start_ptr:expr) => {{
        if $buf_ptr_2 >= $buf_ptr_end {
            break;
        }
        if unsafe { *$buf_ptr_2 } == $c1 || unsafe { *$buf_ptr_2 } == $c2 {
            return (None, Some(plus_offset_from($buf_ptr_2, $start_ptr)));
        }
        $buf_ptr_2 = unsafe { $buf_ptr_2.add(1) };
    }};
}

macro_rules! _unroll_one_chr_to_align_x4 {
    ($buf_ptr_2:expr, $buf_ptr_end:expr, $c1:expr, $c2:expr, $start_ptr:expr) => {{
        _unroll_one_chr_to_align!($buf_ptr_2, $buf_ptr_end, $c1, $c2, $start_ptr);
        _unroll_one_chr_to_align!($buf_ptr_2, $buf_ptr_end, $c1, $c2, $start_ptr);
        _unroll_one_chr_to_align!($buf_ptr_2, $buf_ptr_end, $c1, $c2, $start_ptr);
        _unroll_one_chr_to_align!($buf_ptr_2, $buf_ptr_end, $c1, $c2, $start_ptr);
    }};
}

pub(crate) fn _chr_dbl_to_aligned_u256(
    buf_ptr: *const u8,
    c1: u8,
    c2: u8,
    start_ptr: *const u8,
) -> (Option<*const u8>, Option<usize>) {
    let remaining_align = 0x20_usize - ((buf_ptr as usize) & 0x1F_usize);
    let buf_ptr_end = unsafe { buf_ptr.add(remaining_align) };
    let mut buf_ptr_2 = buf_ptr;
    loop {
        _unroll_one_chr_to_align_x4!(buf_ptr_2, buf_ptr_end, c1, c2, start_ptr);
        _unroll_one_chr_to_align_x4!(buf_ptr_2, buf_ptr_end, c1, c2, start_ptr);
        _unroll_one_chr_to_align_x4!(buf_ptr_2, buf_ptr_end, c1, c2, start_ptr);
        _unroll_one_chr_to_align_x4!(buf_ptr_2, buf_ptr_end, c1, c2, start_ptr);
        //
        _unroll_one_chr_to_align_x4!(buf_ptr_2, buf_ptr_end, c1, c2, start_ptr);
        _unroll_one_chr_to_align_x4!(buf_ptr_2, buf_ptr_end, c1, c2, start_ptr);
        _unroll_one_chr_to_align_x4!(buf_ptr_2, buf_ptr_end, c1, c2, start_ptr);
        _unroll_one_chr_to_align_x4!(buf_ptr_2, buf_ptr_end, c1, c2, start_ptr);
    }
    (Some(buf_ptr_end), None)
}

pub(crate) fn _chr_dbl_to_aligned_u128(
    buf_ptr: *const u8,
    c1: u8,
    c2: u8,
    start_ptr: *const u8,
) -> (Option<*const u8>, Option<usize>) {
    let remaining_align = 0x10_usize - ((buf_ptr as usize) & 0x0F_usize);
    let buf_ptr_end = unsafe { buf_ptr.add(remaining_align) };
    let mut buf_ptr_2 = buf_ptr;
    loop {
        _unroll_one_chr_to_align_x4!(buf_ptr_2, buf_ptr_end, c1, c2, start_ptr);
        _unroll_one_chr_to_align_x4!(buf_ptr_2, buf_ptr_end, c1, c2, start_ptr);
        _unroll_one_chr_to_align_x4!(buf_ptr_2, buf_ptr_end, c1, c2, start_ptr);
        _unroll_one_chr_to_align_x4!(buf_ptr_2, buf_ptr_end, c1, c2, start_ptr);
    }
    (Some(buf_ptr_end), None)
}

fn _chr_dbl_to_aligned_u64(
    buf_ptr: *const u8,
    c1: u8,
    c2: u8,
    start_ptr: *const u8,
) -> (Option<*const u8>, Option<usize>) {
    let remaining_align = 0x08_usize - ((buf_ptr as usize) & 0x07_usize);
    let buf_ptr_end = unsafe { buf_ptr.add(remaining_align) };
    let mut buf_ptr_2 = buf_ptr;
    loop {
        _unroll_one_chr_to_align_x4!(buf_ptr_2, buf_ptr_end, c1, c2, start_ptr);
        _unroll_one_chr_to_align_x4!(buf_ptr_2, buf_ptr_end, c1, c2, start_ptr);
    }
    (Some(buf_ptr_end), None)
}

fn _chr_dbl_to_aligned_u32(
    buf_ptr: *const u8,
    c1: u8,
    c2: u8,
    start_ptr: *const u8,
) -> (Option<*const u8>, Option<usize>) {
    let remaining_align = 0x04_usize - ((buf_ptr as usize) & 0x03_usize);
    let buf_ptr_end = unsafe { buf_ptr.add(remaining_align) };
    let mut buf_ptr_2 = buf_ptr;
    loop {
        _unroll_one_chr_to_align_x4!(buf_ptr_2, buf_ptr_end, c1, c2, start_ptr);
    }
    (Some(buf_ptr_end), None)
}

macro_rules! _unroll_one_chr_16 {
    ($a_ptr:expr, $cc1:expr, $cc2:expr, $start_ptr:expr, $loop_size:expr, $idx:expr) => {{
        let aa_ptr = unsafe { $a_ptr.add($loop_size * $idx) };
        let r = _chr_dbl_c16(aa_ptr, $cc1, $cc2, $start_ptr);
        if !r.is_none() {
            return r;
        }
    }};
}

macro_rules! _unroll_one_chr_8 {
    ($a_ptr:expr, $cc1:expr, $cc2:expr, $start_ptr:expr, $loop_size:expr, $idx:expr) => {{
        let aa_ptr = unsafe { $a_ptr.add($loop_size * $idx) };
        let r = _chr_dbl_c8(aa_ptr, $cc1, $cc2, $start_ptr);
        if !r.is_none() {
            return r;
        }
    }};
}

macro_rules! _unroll_one_chr_4 {
    ($a_ptr:expr, $cc1:expr, $cc2:expr, $start_ptr:expr, $loop_size:expr, $idx:expr) => {{
        let aa_ptr = unsafe { $a_ptr.add($loop_size * $idx) };
        let r = _chr_dbl_c4(aa_ptr, $cc1, $cc2, $start_ptr);
        if !r.is_none() {
            return r;
        }
    }};
}

macro_rules! _unroll_one_chr_2 {
    ($a_ptr:expr, $cc1:expr, $cc2:expr, $start_ptr:expr, $loop_size:expr, $idx:expr) => {{
        let aa_ptr = unsafe { $a_ptr.add($loop_size * $idx) };
        let r = _chr_dbl_c2(aa_ptr, $cc1, $cc2, $start_ptr);
        if !r.is_none() {
            return r;
        }
    }};
}

macro_rules! _unroll_one_chr_1 {
    ($a_ptr:expr, $cc1:expr, $cc2:expr, $start_ptr:expr, $loop_size:expr, $idx:expr) => {{
        let aa_ptr = unsafe { $a_ptr.add($loop_size * $idx) };
        let r = _chr_dbl_c1(aa_ptr, $cc1, $cc2, $start_ptr);
        if !r.is_none() {
            return r;
        }
    }};
}

#[cfg(any(target_pointer_width = "128", feature = "test_pointer_width_128"))]
#[inline(always)]
fn _start_chr_128(buf: &[u8], c_1: u8, c_2: u8) -> Option<usize> {
    //
    let buf_len = buf.len();
    let mut buf_ptr = buf.as_ptr();
    let start_ptr = buf_ptr;
    let end_ptr = unsafe { buf_ptr.add(buf_len) };
    let cc1: u128 = _c16_value(c_1);
    let cc2: u128 = _c16_value(c_2);
    buf_ptr.prefetch_read_data();
    //
    if buf_len >= 16 {
        {
            if !buf_ptr.is_aligned_u128() {
                let r = _chr_dbl_to_aligned_u128(buf_ptr, c_1, c_2, start_ptr);
                if let Some(p) = r.0 {
                    buf_ptr = p;
                } else if let Some(v) = r.1 {
                    return Some(v);
                }
            }
        }
        {
            let unroll = 8;
            let loop_size = 16;
            while unsafe { end_ptr.offset_from(buf_ptr) } >= (loop_size * unroll) as isize {
                buf_ptr.prefetch_read_data();
                _unroll_one_chr_16!(buf_ptr, cc1, cc2, start_ptr, loop_size, 0);
                _unroll_one_chr_16!(buf_ptr, cc1, cc2, start_ptr, loop_size, 1);
                _unroll_one_chr_16!(buf_ptr, cc1, cc2, start_ptr, loop_size, 2);
                _unroll_one_chr_16!(buf_ptr, cc1, cc2, start_ptr, loop_size, 3);
                _unroll_one_chr_16!(buf_ptr, cc1, cc2, start_ptr, loop_size, 4);
                _unroll_one_chr_16!(buf_ptr, cc1, cc2, start_ptr, loop_size, 5);
                _unroll_one_chr_16!(buf_ptr, cc1, cc2, start_ptr, loop_size, 6);
                _unroll_one_chr_16!(buf_ptr, cc1, cc2, start_ptr, loop_size, 7);
                //
                buf_ptr = unsafe { buf_ptr.add(loop_size * unroll) };
            }
        }
        {
            let unroll = 4;
            let loop_size = 16;
            while unsafe { end_ptr.offset_from(buf_ptr) } >= (loop_size * unroll) as isize {
                _unroll_one_chr_16!(buf_ptr, cc1, cc2, start_ptr, loop_size, 0);
                _unroll_one_chr_16!(buf_ptr, cc1, cc2, start_ptr, loop_size, 1);
                _unroll_one_chr_16!(buf_ptr, cc1, cc2, start_ptr, loop_size, 2);
                _unroll_one_chr_16!(buf_ptr, cc1, cc2, start_ptr, loop_size, 3);
                //
                buf_ptr = unsafe { buf_ptr.add(loop_size * unroll) };
            }
        }
        {
            let unroll = 2;
            let loop_size = 16;
            while unsafe { end_ptr.offset_from(buf_ptr) } >= (loop_size * unroll) as isize {
                _unroll_one_chr_16!(buf_ptr, cc1, cc2, start_ptr, loop_size, 0);
                _unroll_one_chr_16!(buf_ptr, cc1, cc2, start_ptr, loop_size, 1);
                //
                buf_ptr = unsafe { buf_ptr.add(loop_size * unroll) };
            }
        }
        {
            let loop_size = 16;
            while unsafe { end_ptr.offset_from(buf_ptr) } >= loop_size as isize {
                _unroll_one_chr_16!(buf_ptr, cc1, cc2, start_ptr, loop_size, 0);
                //
                buf_ptr = unsafe { buf_ptr.add(loop_size) };
            }
        }
    }
    // the remaining data is the max: 15 bytes.
    _memchr_double_remaining_15_bytes_impl(buf_ptr, cc1 as u64, cc2 as u64, start_ptr, end_ptr)
}

#[cfg(any(target_pointer_width = "64", feature = "test_pointer_width_64"))]
#[inline(always)]
fn _start_chr_64(buf: &[u8], c_1: u8, c_2: u8) -> Option<usize> {
    //
    let buf_len = buf.len();
    let mut buf_ptr = buf.as_ptr();
    let start_ptr = buf_ptr;
    let end_ptr = unsafe { buf_ptr.add(buf_len) };
    let cc1: u64 = _c8_value(c_1);
    let cc2: u64 = _c8_value(c_2);
    buf_ptr.prefetch_read_data();
    //
    if buf_len >= 8 {
        {
            if !buf_ptr.is_aligned_u64() {
                let r = _chr_dbl_to_aligned_u64(buf_ptr, c_1, c_2, start_ptr);
                if let Some(p) = r.0 {
                    buf_ptr = p;
                } else if let Some(v) = r.1 {
                    return Some(v);
                }
            }
        }
        {
            let unroll = 8;
            let loop_size = 8;
            while unsafe { end_ptr.offset_from(buf_ptr) } >= (loop_size * unroll) as isize {
                buf_ptr.prefetch_read_data();
                _unroll_one_chr_8!(buf_ptr, cc1, cc2, start_ptr, loop_size, 0);
                _unroll_one_chr_8!(buf_ptr, cc1, cc2, start_ptr, loop_size, 1);
                _unroll_one_chr_8!(buf_ptr, cc1, cc2, start_ptr, loop_size, 2);
                _unroll_one_chr_8!(buf_ptr, cc1, cc2, start_ptr, loop_size, 3);
                _unroll_one_chr_8!(buf_ptr, cc1, cc2, start_ptr, loop_size, 4);
                _unroll_one_chr_8!(buf_ptr, cc1, cc2, start_ptr, loop_size, 5);
                _unroll_one_chr_8!(buf_ptr, cc1, cc2, start_ptr, loop_size, 6);
                _unroll_one_chr_8!(buf_ptr, cc1, cc2, start_ptr, loop_size, 7);
                //
                buf_ptr = unsafe { buf_ptr.add(loop_size * unroll) };
            }
        }
        {
            let unroll = 4;
            let loop_size = 8;
            while unsafe { end_ptr.offset_from(buf_ptr) } >= (loop_size * unroll) as isize {
                buf_ptr.prefetch_read_data();
                _unroll_one_chr_8!(buf_ptr, cc1, cc2, start_ptr, loop_size, 0);
                _unroll_one_chr_8!(buf_ptr, cc1, cc2, start_ptr, loop_size, 1);
                _unroll_one_chr_8!(buf_ptr, cc1, cc2, start_ptr, loop_size, 2);
                _unroll_one_chr_8!(buf_ptr, cc1, cc2, start_ptr, loop_size, 3);
                //
                buf_ptr = unsafe { buf_ptr.add(loop_size * unroll) };
            }
        }
        {
            let unroll = 2;
            let loop_size = 8;
            while unsafe { end_ptr.offset_from(buf_ptr) } >= (loop_size * unroll) as isize {
                _unroll_one_chr_8!(buf_ptr, cc1, cc2, start_ptr, loop_size, 0);
                _unroll_one_chr_8!(buf_ptr, cc1, cc2, start_ptr, loop_size, 1);
                //
                buf_ptr = unsafe { buf_ptr.add(loop_size * unroll) };
            }
        }
        {
            let loop_size = 8;
            while unsafe { end_ptr.offset_from(buf_ptr) } >= loop_size as isize {
                _unroll_one_chr_8!(buf_ptr, cc1, cc2, start_ptr, loop_size, 0);
                //
                buf_ptr = unsafe { buf_ptr.add(loop_size) };
            }
        }
    }
    // the remaining data is the max: 7 bytes.
    _memchr_double_remaining_7_bytes_impl(buf_ptr, cc1 as u32, cc2 as u32, start_ptr, end_ptr)
}

#[cfg(any(target_pointer_width = "32", feature = "test_pointer_width_32"))]
#[inline(always)]
fn _start_chr_32(buf: &[u8], c_1: u8, c_2: u8) -> Option<usize> {
    //
    let buf_len = buf.len();
    let mut buf_ptr = buf.as_ptr();
    let start_ptr = buf_ptr;
    let end_ptr = unsafe { buf_ptr.add(buf_len) };
    let cc1: u32 = _c4_value(c_1);
    let cc2: u32 = _c4_value(c_2);
    buf_ptr.prefetch_read_data();
    //
    if buf_len >= 4 {
        {
            if !buf_ptr.is_aligned_u32() {
                let r = _chr_dbl_to_aligned_u32(buf_ptr, c_1, c_2, start_ptr);
                if let Some(p) = r.0 {
                    buf_ptr = p;
                } else if let Some(v) = r.1 {
                    return Some(v);
                }
            }
        }
        {
            let unroll = 8;
            let loop_size = 4;
            while unsafe { end_ptr.offset_from(buf_ptr) } >= (loop_size * unroll) as isize {
                buf_ptr.prefetch_read_data();
                _unroll_one_chr_4!(buf_ptr, cc1, cc2, start_ptr, loop_size, 0);
                _unroll_one_chr_4!(buf_ptr, cc1, cc2, start_ptr, loop_size, 1);
                _unroll_one_chr_4!(buf_ptr, cc1, cc2, start_ptr, loop_size, 2);
                _unroll_one_chr_4!(buf_ptr, cc1, cc2, start_ptr, loop_size, 3);
                _unroll_one_chr_4!(buf_ptr, cc1, cc2, start_ptr, loop_size, 4);
                _unroll_one_chr_4!(buf_ptr, cc1, cc2, start_ptr, loop_size, 5);
                _unroll_one_chr_4!(buf_ptr, cc1, cc2, start_ptr, loop_size, 6);
                _unroll_one_chr_4!(buf_ptr, cc1, cc2, start_ptr, loop_size, 7);
                //
                buf_ptr = unsafe { buf_ptr.add(loop_size * unroll) };
            }
        }
        {
            let unroll = 4;
            let loop_size = 4;
            while unsafe { end_ptr.offset_from(buf_ptr) } >= (loop_size * unroll) as isize {
                _unroll_one_chr_4!(buf_ptr, cc1, cc2, start_ptr, loop_size, 0);
                _unroll_one_chr_4!(buf_ptr, cc1, cc2, start_ptr, loop_size, 1);
                _unroll_one_chr_4!(buf_ptr, cc1, cc2, start_ptr, loop_size, 2);
                _unroll_one_chr_4!(buf_ptr, cc1, cc2, start_ptr, loop_size, 3);
                //
                buf_ptr = unsafe { buf_ptr.add(loop_size * unroll) };
            }
        }
        {
            let unroll = 2;
            let loop_size = 4;
            while unsafe { end_ptr.offset_from(buf_ptr) } >= (loop_size * unroll) as isize {
                _unroll_one_chr_4!(buf_ptr, cc1, cc2, start_ptr, loop_size, 0);
                _unroll_one_chr_4!(buf_ptr, cc1, cc2, start_ptr, loop_size, 1);
                //
                buf_ptr = unsafe { buf_ptr.add(loop_size * unroll) };
            }
        }
        {
            let loop_size = 4;
            while unsafe { end_ptr.offset_from(buf_ptr) } >= loop_size as isize {
                _unroll_one_chr_4!(buf_ptr, cc1, cc2, start_ptr, loop_size, 0);
                //
                buf_ptr = unsafe { buf_ptr.add(loop_size) };
            }
        }
    }
    // the remaining data is the max: 3 bytes.
    _memchr_double_remaining_3_bytes_impl(buf_ptr, cc1 as u16, cc2 as u16, start_ptr, end_ptr)
}

#[inline(always)]
pub(crate) fn _memchr_double_remaining_15_bytes_impl(
    buf_ptr: *const u8,
    cc1: u64,
    cc2: u64,
    start_ptr: *const u8,
    end_ptr: *const u8,
) -> Option<usize> {
    let mut buf_ptr = buf_ptr;
    if buf_ptr.is_aligned_u64() {
        let loop_size = 8;
        if unsafe { end_ptr.offset_from(buf_ptr) } >= loop_size as isize {
            _unroll_one_chr_8!(buf_ptr, cc1, cc2, start_ptr, loop_size, 0);
            //
            buf_ptr = unsafe { buf_ptr.add(loop_size) };
        }
    }
    // the remaining data is the max: 7 bytes.
    _memchr_double_remaining_7_bytes_impl(buf_ptr, cc1 as u32, cc2 as u32, start_ptr, end_ptr)
}

#[inline(always)]
pub(crate) fn _memchr_double_remaining_7_bytes_impl(
    buf_ptr: *const u8,
    cc1: u32,
    cc2: u32,
    start_ptr: *const u8,
    end_ptr: *const u8,
) -> Option<usize> {
    let mut buf_ptr = buf_ptr;
    if buf_ptr.is_aligned_u32() {
        let loop_size = 4;
        if unsafe { end_ptr.offset_from(buf_ptr) } >= loop_size as isize {
            _unroll_one_chr_4!(buf_ptr, cc1, cc2, start_ptr, loop_size, 0);
            //
            buf_ptr = unsafe { buf_ptr.add(loop_size) };
        }
    }
    // the remaining data is the max: 3 bytes.
    _memchr_double_remaining_3_bytes_impl(buf_ptr, cc1 as u16, cc2 as u16, start_ptr, end_ptr)
}

#[inline(always)]
pub(crate) fn _memchr_double_remaining_3_bytes_impl(
    buf_ptr: *const u8,
    cc1: u16,
    cc2: u16,
    start_ptr: *const u8,
    end_ptr: *const u8,
) -> Option<usize> {
    let mut buf_ptr = buf_ptr;
    if buf_ptr.is_aligned_u16() {
        let loop_size = 2;
        if unsafe { end_ptr.offset_from(buf_ptr) } >= loop_size as isize {
            _unroll_one_chr_2!(buf_ptr, cc1, cc2, start_ptr, loop_size, 0);
            //
            buf_ptr = unsafe { buf_ptr.add(loop_size) };
        }
    }
    {
        let loop_size = 1;
        if unsafe { end_ptr.offset_from(buf_ptr) } >= loop_size as isize {
            while unsafe { end_ptr.offset_from(buf_ptr) } >= loop_size as isize {
                _unroll_one_chr_1!(buf_ptr, cc1 as u8, cc2 as u8, start_ptr, loop_size, 0);
                buf_ptr = unsafe { buf_ptr.add(loop_size) };
            }
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
fn _chr_dbl_c16(
    buf_ptr: *const u8,
    c16_1: u128,
    c16_2: u128,
    start_ptr: *const u8,
) -> Option<usize> {
    let v0 = unsafe { _read_a_little_endian_from_ptr_u128(buf_ptr) };
    let v_1 = v0 ^ c16_1;
    let v_2 = v0 ^ c16_2;
    //
    let bits_1 = PackedU128::new(v_1).may_have_zero_quick();
    let bits_2 = PackedU128::new(v_2).may_have_zero_quick();
    let base = plus_offset_from(buf_ptr, start_ptr);
    //
    if !bits_1.is_zeros() {
        if !bits_2.is_zeros() {
            let idx1 = (bits_1.trailing_zeros() / 8) as usize;
            let idx2 = (bits_2.trailing_zeros() / 8) as usize;
            if idx1 < idx2 {
                Some(base + idx1)
            } else {
                Some(base + idx2)
            }
        } else {
            Some(base + (bits_1.trailing_zeros() / 8) as usize)
        }
    } else if !bits_2.is_zeros() {
        Some(base + (bits_2.trailing_zeros() / 8) as usize)
    } else {
        None
    }
}

#[inline(always)]
fn _c8_value(c: u8) -> u64 {
    (c as u64) * PackedU64::ONES
}

#[inline(always)]
fn _chr_dbl_c8(buf_ptr: *const u8, c8_1: u64, c8_2: u64, start_ptr: *const u8) -> Option<usize> {
    let v0 = unsafe { _read_a_little_endian_from_ptr_u64(buf_ptr) };
    let v_1 = v0 ^ c8_1;
    let v_2 = v0 ^ c8_2;
    //
    let bits_1 = PackedU64::new(v_1).may_have_zero_quick();
    let bits_2 = PackedU64::new(v_2).may_have_zero_quick();
    let base = plus_offset_from(buf_ptr, start_ptr);
    //
    if !bits_1.is_zeros() {
        if !bits_2.is_zeros() {
            let idx1 = (bits_1.trailing_zeros() / 8) as usize;
            let idx2 = (bits_2.trailing_zeros() / 8) as usize;
            if idx1 < idx2 {
                Some(base + idx1)
            } else {
                Some(base + idx2)
            }
        } else {
            Some(base + (bits_1.trailing_zeros() / 8) as usize)
        }
    } else if !bits_2.is_zeros() {
        Some(base + (bits_2.trailing_zeros() / 8) as usize)
    } else {
        None
    }
}

#[inline(always)]
fn _c4_value(c: u8) -> u32 {
    (c as u32) * PackedU32::ONES
}

#[inline(always)]
fn _chr_dbl_c4(buf_ptr: *const u8, c4_1: u32, c4_2: u32, start_ptr: *const u8) -> Option<usize> {
    let v0 = unsafe { _read_a_little_endian_from_ptr_u32(buf_ptr) };
    let v_1 = v0 ^ c4_1;
    let v_2 = v0 ^ c4_2;
    //
    let bits_1 = PackedU32::new(v_1).may_have_zero_quick();
    let bits_2 = PackedU32::new(v_2).may_have_zero_quick();
    let base = plus_offset_from(buf_ptr, start_ptr);
    //
    if !bits_1.is_zeros() {
        if !bits_2.is_zeros() {
            let idx1 = (bits_1.trailing_zeros() / 8) as usize;
            let idx2 = (bits_2.trailing_zeros() / 8) as usize;
            if idx1 < idx2 {
                Some(base + idx1)
            } else {
                Some(base + idx2)
            }
        } else {
            Some(base + (bits_1.trailing_zeros() / 8) as usize)
        }
    } else if !bits_2.is_zeros() {
        Some(base + (bits_2.trailing_zeros() / 8) as usize)
    } else {
        None
    }
}

#[inline(always)]
fn _chr_dbl_c2(buf_ptr: *const u8, c2_1: u16, c2_2: u16, start_ptr: *const u8) -> Option<usize> {
    let v0 = unsafe { _read_a_little_endian_from_ptr_u16(buf_ptr) };
    let v_1 = v0 ^ c2_1;
    let v_2 = v0 ^ c2_2;
    //
    let bits_1 = PackedU16::new(v_1).may_have_zero_quick();
    let bits_2 = PackedU16::new(v_2).may_have_zero_quick();
    let base = plus_offset_from(buf_ptr, start_ptr);
    //
    if !bits_1.is_zeros() {
        if !bits_2.is_zeros() {
            let idx1 = (bits_1.trailing_zeros() / 8) as usize;
            let idx2 = (bits_2.trailing_zeros() / 8) as usize;
            if idx1 < idx2 {
                Some(base + idx1)
            } else {
                Some(base + idx2)
            }
        } else {
            Some(base + (bits_1.trailing_zeros() / 8) as usize)
        }
    } else if !bits_2.is_zeros() {
        Some(base + (bits_2.trailing_zeros() / 8) as usize)
    } else {
        None
    }
}

/*
#[inline(always)]
fn _chr_dbl_c2(buf_ptr: *const u8, c2_1: u16, c2_2: u16, start_ptr: *const u8) -> Option<usize> {
    let aa_ptr = buf_ptr as *const u8;
    let r = _chr_dbl_c1(aa_ptr, c2_1 as u8, c2_2 as u8, start_ptr);
    if !r.is_none() {
        return r;
    }
    let aa_ptr = unsafe { buf_ptr.add(1) } as *const u8;
    let r = _chr_dbl_c1(aa_ptr, c2_1 as u8, c2_2 as u8, start_ptr);
    if !r.is_none() {
        return r;
    }
    None
}
*/
#[inline(always)]
fn _chr_dbl_c1(buf_ptr: *const u8, c1: u8, c2: u8, start_ptr: *const u8) -> Option<usize> {
    let aa_ptr = buf_ptr as *const u8;
    let aac = unsafe { *aa_ptr };
    if aac == c1 || aac == c2 {
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
