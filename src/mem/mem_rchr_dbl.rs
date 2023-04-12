use crate::utils::*;

#[inline(never)]
pub fn _memrchr_dbl_impl(buf: &[u8], c1: u8, c2: u8) -> Option<usize> {
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

macro_rules! _unroll_one_rchr_to_aligned_x1 {
    ($buf_ptr_2:expr, $buf_ptr_min:expr, $c:expr, $start_ptr:expr) => {{
        if $buf_ptr_2 <= $buf_ptr_min {
            break;
        }
        $buf_ptr_2 = unsafe { $buf_ptr_2.sub(1) };
        let r = _rchr_dbl_c1_aa_x1($buf_ptr_2, $c, $start_ptr);
        if r.is_some() {
            return (None, r);
        }
    }};
}

macro_rules! _unroll_one_rchr_to_aligned_x4 {
    ($buf_ptr_2:expr, $buf_ptr_min:expr, $c:expr, $start_ptr:expr) => {{
        _unroll_one_rchr_to_aligned_x1!($buf_ptr_2, $buf_ptr_min, $c, $start_ptr);
        _unroll_one_rchr_to_aligned_x1!($buf_ptr_2, $buf_ptr_min, $c, $start_ptr);
        _unroll_one_rchr_to_aligned_x1!($buf_ptr_2, $buf_ptr_min, $c, $start_ptr);
        _unroll_one_rchr_to_aligned_x1!($buf_ptr_2, $buf_ptr_min, $c, $start_ptr);
    }};
}

macro_rules! _unroll_one_rchr_to_aligned_x8 {
    ($buf_ptr_2:expr, $buf_ptr_min:expr, $c:expr, $start_ptr:expr) => {{
        _unroll_one_rchr_to_aligned_x4!($buf_ptr_2, $buf_ptr_min, $c, $start_ptr);
        _unroll_one_rchr_to_aligned_x4!($buf_ptr_2, $buf_ptr_min, $c, $start_ptr);
    }};
}

macro_rules! _unroll_one_rchr_to_aligned_x16 {
    ($buf_ptr_2:expr, $buf_ptr_min:expr, $c:expr, $start_ptr:expr) => {{
        _unroll_one_rchr_to_aligned_x8!($buf_ptr_2, $buf_ptr_min, $c, $start_ptr);
        _unroll_one_rchr_to_aligned_x8!($buf_ptr_2, $buf_ptr_min, $c, $start_ptr);
    }};
}

pub(crate) fn _rchr_dbl_to_aligned_u256(
    buf_ptr_cur: *const u8,
    c: C1Dbl,
    st_ptr: *const u8,
) -> (Option<*const u8>, Option<usize>) {
    let buf_ptr_end = buf_ptr_cur;
    let remaining_align = (buf_ptr_end as usize) & 0x1F_usize;
    let buf_ptr_min = unsafe { buf_ptr_end.sub(remaining_align) };
    let mut buf_ptr_2 = buf_ptr_end;
    loop {
        _unroll_one_rchr_to_aligned_x16!(buf_ptr_2, buf_ptr_min, c, st_ptr);
        _unroll_one_rchr_to_aligned_x16!(buf_ptr_2, buf_ptr_min, c, st_ptr);
    }
    (Some(buf_ptr_min), None)
}

pub(crate) fn _rchr_dbl_to_aligned_u128(
    buf_ptr_cur: *const u8,
    c: C1Dbl,
    st_ptr: *const u8,
) -> (Option<*const u8>, Option<usize>) {
    let buf_ptr_end = buf_ptr_cur;
    let remaining_align = (buf_ptr_end as usize) & 0x0F_usize;
    let buf_ptr_min = unsafe { buf_ptr_end.sub(remaining_align) };
    let mut buf_ptr_2 = buf_ptr_end;
    loop {
        _unroll_one_rchr_to_aligned_x16!(buf_ptr_2, buf_ptr_min, c, st_ptr);
    }
    (Some(buf_ptr_min), None)
}

fn _rchr_dbl_to_aligned_u64(
    buf_ptr_cur: *const u8,
    c: C1Dbl,
    st_ptr: *const u8,
) -> (Option<*const u8>, Option<usize>) {
    let buf_ptr_end = buf_ptr_cur;
    let remaining_align = (buf_ptr_end as usize) & 0x07_usize;
    let buf_ptr_min = unsafe { buf_ptr_end.sub(remaining_align) };
    let mut buf_ptr_2 = buf_ptr_end;
    loop {
        _unroll_one_rchr_to_aligned_x8!(buf_ptr_2, buf_ptr_min, c, st_ptr);
    }
    (Some(buf_ptr_min), None)
}

fn _rchr_dbl_to_aligned_u32(
    buf_ptr_cur: *const u8,
    c: C1Dbl,
    st_ptr: *const u8,
) -> (Option<*const u8>, Option<usize>) {
    let buf_ptr_end = buf_ptr_cur;
    let remaining_align = (buf_ptr_end as usize) & 0x03_usize;
    let buf_ptr_min = unsafe { buf_ptr_end.sub(remaining_align) };
    let mut buf_ptr_2 = buf_ptr_end;
    loop {
        _unroll_one_rchr_to_aligned_x4!(buf_ptr_2, buf_ptr_min, c, st_ptr);
    }
    (Some(buf_ptr_min), None)
}

#[cfg(any(target_pointer_width = "128", feature = "test_pointer_width_128"))]
#[inline(always)]
fn _start_rchr_128(buf: &[u8], c_1: u8, c_2: u8) -> Option<usize> {
    let buf_len = buf.len();
    let start_ptr = buf.as_ptr();
    let mut buf_ptr_cur = unsafe { start_ptr.add(buf_len) };
    let c = C1Dbl::new(c_1, c_2);
    let cc = C16Dbl::new(c_1, c_2);
    //
    if buf_len >= 16 {
        // to a aligned pointer
        {
            let loop_size = 16;
            let buf_ptr = unsafe { buf_ptr_cur.sub(loop_size) };
            buf_ptr.prefetch_read_data();
            if !buf_ptr_cur.is_aligned_u128() {
                let r = _rchr_dbl_to_aligned_u128(buf_ptr_cur, c, start_ptr);
                if let Some(p) = r.0 {
                    buf_ptr_cur = p;
                } else if let Some(v) = r.1 {
                    return Some(v);
                }
            }
        }
        // the loop
        {
            let unroll = 8;
            let loop_size = 16;
            if unsafe { buf_ptr_cur.offset_from(start_ptr) } >= (loop_size * unroll) as isize {
                let mut buf_ptr = buf_ptr_cur;
                while unsafe { buf_ptr.offset_from(start_ptr) } >= (loop_size * unroll) as isize {
                    buf_ptr = unsafe { buf_ptr.sub(loop_size * unroll) };
                    buf_ptr.prefetch_read_data();
                    let r = _rchr_dbl_c16_aa_x8(buf_ptr, cc, start_ptr);
                    if r.is_some() {
                        return r;
                    }
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
                    let r = _rchr_dbl_c16_aa_x4(buf_ptr, cc, start_ptr);
                    if r.is_some() {
                        return r;
                    }
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
                    buf_ptr.prefetch_read_data();
                    let r = _rchr_dbl_c16_aa_x2(buf_ptr, cc, start_ptr);
                    if r.is_some() {
                        return r;
                    }
                }
                buf_ptr_cur = buf_ptr;
            }
        }
        {
            let unroll = 1;
            let loop_size = 16;
            if unsafe { buf_ptr_cur.offset_from(start_ptr) } >= (loop_size * unroll) as isize {
                let mut buf_ptr = buf_ptr_cur;
                while unsafe { buf_ptr.offset_from(start_ptr) } >= (loop_size * unroll) as isize {
                    buf_ptr = unsafe { buf_ptr.sub(loop_size * unroll) };
                    let r = _rchr_dbl_c16_aa_x1(buf_ptr, cc, start_ptr);
                    if r.is_some() {
                        return r;
                    }
                }
                buf_ptr_cur = buf_ptr;
            }
        }
    }
    start_ptr.prefetch_read_data();
    // the remaining data is the max: 15 bytes.
    _memrchr_dbl_remaining_15_bytes_impl(buf_ptr_cur, cc.into(), start_ptr)
}

#[cfg(any(target_pointer_width = "64", feature = "test_pointer_width_64"))]
#[inline(always)]
fn _start_rchr_64(buf: &[u8], c_1: u8, c_2: u8) -> Option<usize> {
    let buf_len = buf.len();
    let start_ptr = buf.as_ptr();
    let mut buf_ptr_cur = unsafe { start_ptr.add(buf_len) };
    let c = C1Dbl::new(c_1, c_2);
    let cc = C8Dbl::new(c_1, c_2);
    //
    if buf_len >= 8 {
        // to a aligned pointer
        {
            let loop_size = 8;
            let buf_ptr = unsafe { buf_ptr_cur.sub(loop_size) };
            buf_ptr.prefetch_read_data();
            if !buf_ptr_cur.is_aligned_u64() {
                let r = _rchr_dbl_to_aligned_u64(buf_ptr_cur, c, start_ptr);
                if let Some(p) = r.0 {
                    buf_ptr_cur = p;
                } else if let Some(v) = r.1 {
                    return Some(v);
                }
            }
        }
        // the loop
        {
            let unroll = 8;
            let loop_size = 8;
            if unsafe { buf_ptr_cur.offset_from(start_ptr) } >= (loop_size * unroll) as isize {
                let mut buf_ptr = buf_ptr_cur;
                while unsafe { buf_ptr.offset_from(start_ptr) } >= (loop_size * unroll) as isize {
                    buf_ptr = unsafe { buf_ptr.sub(loop_size * unroll) };
                    buf_ptr.prefetch_read_data();
                    let r = _rchr_dbl_c8_aa_x8(buf_ptr, cc, start_ptr);
                    if r.is_some() {
                        return r;
                    }
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
                    let r = _rchr_dbl_c8_aa_x4(buf_ptr, cc, start_ptr);
                    if r.is_some() {
                        return r;
                    }
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
                    buf_ptr.prefetch_read_data();
                    let r = _rchr_dbl_c8_aa_x2(buf_ptr, cc, start_ptr);
                    if r.is_some() {
                        return r;
                    }
                }
                buf_ptr_cur = buf_ptr;
            }
        }
        {
            let unroll = 1;
            let loop_size = 8;
            if unsafe { buf_ptr_cur.offset_from(start_ptr) } >= (loop_size * unroll) as isize {
                let mut buf_ptr = buf_ptr_cur;
                while unsafe { buf_ptr.offset_from(start_ptr) } >= (loop_size * unroll) as isize {
                    buf_ptr = unsafe { buf_ptr.sub(loop_size * unroll) };
                    let r = _rchr_dbl_c8_aa_x1(buf_ptr, cc, start_ptr);
                    if r.is_some() {
                        return r;
                    }
                }
                buf_ptr_cur = buf_ptr;
            }
        }
    }
    start_ptr.prefetch_read_data();
    // the remaining data is the max: 7 bytes.
    _memrchr_dbl_remaining_7_bytes_impl(buf_ptr_cur, cc.into(), start_ptr)
}

#[cfg(any(target_pointer_width = "32", feature = "test_pointer_width_32"))]
#[inline(always)]
fn _start_rchr_32(buf: &[u8], c_1: u8, c_2: u8) -> Option<usize> {
    let buf_len = buf.len();
    let start_ptr = buf.as_ptr();
    let mut buf_ptr_cur = unsafe { start_ptr.add(buf_len) };
    let c = C1Dbl::new(c_1, c_2);
    let cc = C4Dbl::new(c_1, c_2);
    //
    if buf_len >= 4 {
        // to a aligned pointer
        {
            let loop_size = 4;
            let buf_ptr = unsafe { buf_ptr_cur.sub(loop_size) };
            buf_ptr.prefetch_read_data();
            if !buf_ptr_cur.is_aligned_u32() {
                let r = _rchr_dbl_to_aligned_u32(buf_ptr_cur, c, start_ptr);
                if let Some(p) = r.0 {
                    buf_ptr_cur = p;
                } else if let Some(v) = r.1 {
                    return Some(v);
                }
            }
        }
        // the loop
        {
            let unroll = 8;
            let loop_size = 4;
            if unsafe { buf_ptr_cur.offset_from(start_ptr) } >= (loop_size * unroll) as isize {
                let mut buf_ptr = buf_ptr_cur;
                while unsafe { buf_ptr.offset_from(start_ptr) } >= (loop_size * unroll) as isize {
                    buf_ptr = unsafe { buf_ptr.sub(loop_size * unroll) };
                    buf_ptr.prefetch_read_data();
                    let r = _rchr_dbl_c4_aa_x8(buf_ptr, cc, start_ptr);
                    if r.is_some() {
                        return r;
                    }
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
                    let r = _rchr_dbl_c4_aa_x4(buf_ptr, cc, start_ptr);
                    if r.is_some() {
                        return r;
                    }
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
                    let r = _rchr_dbl_c4_aa_x2(buf_ptr, cc, start_ptr);
                    if r.is_some() {
                        return r;
                    }
                }
                buf_ptr_cur = buf_ptr;
            }
        }
        {
            let unroll = 1;
            let loop_size = 4;
            if unsafe { buf_ptr_cur.offset_from(start_ptr) } >= (loop_size * unroll) as isize {
                let mut buf_ptr = buf_ptr_cur;
                while unsafe { buf_ptr.offset_from(start_ptr) } >= (loop_size * unroll) as isize {
                    buf_ptr = unsafe { buf_ptr.sub(loop_size * unroll) };
                    let r = _rchr_dbl_c4_aa_x1(buf_ptr, cc, start_ptr);
                    if r.is_some() {
                        return r;
                    }
                }
                buf_ptr_cur = buf_ptr;
            }
        }
    }
    start_ptr.prefetch_read_data();
    // the remaining data is the max: 3 bytes.
    _memrchr_dbl_remaining_3_bytes_impl(buf_ptr_cur, cc.into(), start_ptr)
}

#[inline(always)]
pub(crate) fn _memrchr_dbl_remaining_15_bytes_impl(
    buf_ptr_cur: *const u8,
    cc: C8Dbl,
    start_ptr: *const u8,
) -> Option<usize> {
    let mut buf_ptr_cur = buf_ptr_cur;
    if buf_ptr_cur.is_aligned_u64() {
        let loop_size = 8;
        if unsafe { buf_ptr_cur.offset_from(start_ptr) } >= loop_size as isize {
            let buf_ptr = unsafe { buf_ptr_cur.sub(loop_size) };
            let r = _rchr_dbl_c8_aa_x1(buf_ptr, cc, start_ptr);
            if r.is_some() {
                return r;
            }
            buf_ptr_cur = buf_ptr;
        }
    }
    // the remaining data is the max: 7 bytes.
    _memrchr_dbl_remaining_7_bytes_impl(buf_ptr_cur, cc.into(), start_ptr)
}

#[inline(always)]
pub(crate) fn _memrchr_dbl_remaining_7_bytes_impl(
    buf_ptr_cur: *const u8,
    cc: C4Dbl,
    start_ptr: *const u8,
) -> Option<usize> {
    let mut buf_ptr_cur = buf_ptr_cur;
    if buf_ptr_cur.is_aligned_u32() {
        let loop_size = 4;
        if unsafe { buf_ptr_cur.offset_from(start_ptr) } >= loop_size as isize {
            let buf_ptr = unsafe { buf_ptr_cur.sub(loop_size) };
            let r = _rchr_dbl_c4_aa_x1(buf_ptr, cc, start_ptr);
            if r.is_some() {
                return r;
            }
            buf_ptr_cur = buf_ptr;
        }
    }
    // the remaining data is the max: 3 bytes.
    _memrchr_dbl_remaining_3_bytes_impl(buf_ptr_cur, cc.into(), start_ptr)
}

#[inline(always)]
pub(crate) fn _memrchr_dbl_remaining_3_bytes_impl(
    buf_ptr_cur: *const u8,
    cc: C2Dbl,
    start_ptr: *const u8,
) -> Option<usize> {
    let mut buf_ptr_cur = buf_ptr_cur;
    if buf_ptr_cur.is_aligned_u16() {
        let loop_size = 2;
        if unsafe { buf_ptr_cur.offset_from(start_ptr) } >= loop_size as isize {
            let buf_ptr = unsafe { buf_ptr_cur.sub(loop_size) };
            let r = _rchr_dbl_c2_aa_x1(buf_ptr, cc, start_ptr);
            if r.is_some() {
                return r;
            }
            buf_ptr_cur = buf_ptr;
        }
    }
    {
        let loop_size = 1;
        if unsafe { buf_ptr_cur.offset_from(start_ptr) } >= loop_size as isize {
            let mut buf_ptr = buf_ptr_cur;
            while unsafe { buf_ptr.offset_from(start_ptr) } >= loop_size as isize {
                buf_ptr = unsafe { buf_ptr.sub(loop_size) };
                let r = _rchr_dbl_c1_aa_x1(buf_ptr, cc.into(), start_ptr);
                if r.is_some() {
                    return r;
                }
            }
        }
    }
    //
    None
}

#[inline(always)]
fn _rchr_dbl_c16_aa_x1(buf_ptr: *const u8, c16: C16Dbl, st_ptr: *const u8) -> Option<usize> {
    let v_0 = unsafe { _read_a_big_endian_from_ptr_u128(buf_ptr) };
    let v_0_a = v_0 ^ c16.a;
    let v_0_b = v_0 ^ c16.b;
    //
    let bits_0_a = PackedU128::new(v_0_a).may_have_zero_quick();
    let bits_0_b = PackedU128::new(v_0_b).may_have_zero_quick();
    let base = plus_offset_from(buf_ptr, st_ptr) + 16 - 1;
    //
    if !bits_0_a.is_zeros() {
        if !bits_0_b.is_zeros() {
            let idx1 = (bits_0_a.trailing_zeros() / 8) as usize;
            let idx2 = (bits_0_b.trailing_zeros() / 8) as usize;
            Some(base - idx1.min(idx2))
        } else {
            Some(base - (bits_0_a.trailing_zeros() / 8) as usize)
        }
    } else if !bits_0_b.is_zeros() {
        Some(base - (bits_0_b.trailing_zeros() / 8) as usize)
    } else {
        None
    }
}

#[inline(always)]
fn _rchr_dbl_c16_aa_x2(buf_ptr: *const u8, c16: C16Dbl, st_ptr: *const u8) -> Option<usize> {
    let r = _rchr_dbl_c16_aa_x1(unsafe { buf_ptr.add(16) }, c16, st_ptr);
    if r.is_some() {
        return r;
    }
    let r = _rchr_dbl_c16_aa_x1(buf_ptr, c16, st_ptr);
    if r.is_some() {
        return r;
    }
    None
}

#[inline(always)]
fn _rchr_dbl_c16_aa_x4(buf_ptr: *const u8, c16: C16Dbl, st_ptr: *const u8) -> Option<usize> {
    let r = _rchr_dbl_c16_aa_x2(unsafe { buf_ptr.add(16 * 2) }, c16, st_ptr);
    if r.is_some() {
        return r;
    }
    let r = _rchr_dbl_c16_aa_x2(buf_ptr, c16, st_ptr);
    if r.is_some() {
        return r;
    }
    None
}

#[inline(always)]
fn _rchr_dbl_c16_aa_x8(buf_ptr: *const u8, c16: C16Dbl, st_ptr: *const u8) -> Option<usize> {
    let r = _rchr_dbl_c16_aa_x4(unsafe { buf_ptr.add(16 * 4) }, c16, st_ptr);
    if r.is_some() {
        return r;
    }
    let r = _rchr_dbl_c16_aa_x4(buf_ptr, c16, st_ptr);
    if r.is_some() {
        return r;
    }
    None
}

#[inline(always)]
fn _rchr_dbl_c8_aa_x1(buf_ptr: *const u8, c8: C8Dbl, st_ptr: *const u8) -> Option<usize> {
    let v_0 = unsafe { _read_a_big_endian_from_ptr_u64(buf_ptr) };
    let v_0_a = v_0 ^ c8.a;
    let v_0_b = v_0 ^ c8.b;
    //
    let bits_0_a = PackedU64::new(v_0_a).may_have_zero_quick();
    let bits_0_b = PackedU64::new(v_0_b).may_have_zero_quick();
    let base = plus_offset_from(buf_ptr, st_ptr) + 8 - 1;
    //
    if !bits_0_a.is_zeros() {
        if !bits_0_b.is_zeros() {
            let idx1 = (bits_0_a.trailing_zeros() / 8) as usize;
            let idx2 = (bits_0_b.trailing_zeros() / 8) as usize;
            Some(base - idx1.min(idx2))
        } else {
            Some(base - (bits_0_a.trailing_zeros() / 8) as usize)
        }
    } else if !bits_0_b.is_zeros() {
        Some(base - (bits_0_b.trailing_zeros() / 8) as usize)
    } else {
        None
    }
}

#[inline(always)]
fn _rchr_dbl_c8_aa_x2(buf_ptr: *const u8, c8: C8Dbl, st_ptr: *const u8) -> Option<usize> {
    let r = _rchr_dbl_c8_aa_x1(unsafe { buf_ptr.add(8) }, c8, st_ptr);
    if r.is_some() {
        return r;
    }
    let r = _rchr_dbl_c8_aa_x1(buf_ptr, c8, st_ptr);
    if r.is_some() {
        return r;
    }
    None
}

#[inline(always)]
fn _rchr_dbl_c8_aa_x4(buf_ptr: *const u8, c8: C8Dbl, st_ptr: *const u8) -> Option<usize> {
    let r = _rchr_dbl_c8_aa_x2(unsafe { buf_ptr.add(8 * 2) }, c8, st_ptr);
    if r.is_some() {
        return r;
    }
    let r = _rchr_dbl_c8_aa_x2(buf_ptr, c8, st_ptr);
    if r.is_some() {
        return r;
    }
    None
}

#[inline(always)]
fn _rchr_dbl_c8_aa_x8(buf_ptr: *const u8, c8: C8Dbl, st_ptr: *const u8) -> Option<usize> {
    let r = _rchr_dbl_c8_aa_x4(unsafe { buf_ptr.add(8 * 4) }, c8, st_ptr);
    if r.is_some() {
        return r;
    }
    let r = _rchr_dbl_c8_aa_x4(buf_ptr, c8, st_ptr);
    if r.is_some() {
        return r;
    }
    None
}

#[inline(always)]
fn _rchr_dbl_c4_aa_x1(buf_ptr: *const u8, c4: C4Dbl, st_ptr: *const u8) -> Option<usize> {
    let v_0 = unsafe { _read_a_big_endian_from_ptr_u32(buf_ptr) };
    let v_0_a = v_0 ^ c4.a;
    let v_0_b = v_0 ^ c4.b;
    //
    let bits_0_a = PackedU32::new(v_0_a).may_have_zero_quick();
    let bits_0_b = PackedU32::new(v_0_b).may_have_zero_quick();
    let base = plus_offset_from(buf_ptr, st_ptr) + 4 - 1;
    //
    if !bits_0_a.is_zeros() {
        if !bits_0_b.is_zeros() {
            let idx1 = (bits_0_a.trailing_zeros() / 8) as usize;
            let idx2 = (bits_0_b.trailing_zeros() / 8) as usize;
            Some(base - idx1.min(idx2))
        } else {
            Some(base - (bits_0_a.trailing_zeros() / 8) as usize)
        }
    } else if !bits_0_b.is_zeros() {
        Some(base - (bits_0_b.trailing_zeros() / 8) as usize)
    } else {
        None
    }
}

#[inline(always)]
fn _rchr_dbl_c4_aa_x2(buf_ptr: *const u8, c4: C4Dbl, st_ptr: *const u8) -> Option<usize> {
    let r = _rchr_dbl_c4_aa_x1(unsafe { buf_ptr.add(4) }, c4, st_ptr);
    if r.is_some() {
        return r;
    }
    let r = _rchr_dbl_c4_aa_x1(buf_ptr, c4, st_ptr);
    if r.is_some() {
        return r;
    }
    None
}

#[inline(always)]
fn _rchr_dbl_c4_aa_x4(buf_ptr: *const u8, c4: C4Dbl, st_ptr: *const u8) -> Option<usize> {
    let r = _rchr_dbl_c4_aa_x2(unsafe { buf_ptr.add(4 * 2) }, c4, st_ptr);
    if r.is_some() {
        return r;
    }
    let r = _rchr_dbl_c4_aa_x2(buf_ptr, c4, st_ptr);
    if r.is_some() {
        return r;
    }
    None
}

#[inline(always)]
fn _rchr_dbl_c4_aa_x8(buf_ptr: *const u8, c4: C4Dbl, st_ptr: *const u8) -> Option<usize> {
    let r = _rchr_dbl_c4_aa_x4(unsafe { buf_ptr.add(4 * 4) }, c4, st_ptr);
    if r.is_some() {
        return r;
    }
    let r = _rchr_dbl_c4_aa_x4(buf_ptr, c4, st_ptr);
    if r.is_some() {
        return r;
    }
    None
}

#[inline(always)]
fn _rchr_dbl_c2_aa_x1(buf_ptr: *const u8, c2: C2Dbl, st_ptr: *const u8) -> Option<usize> {
    let v_0 = unsafe { _read_a_big_endian_from_ptr_u16(buf_ptr) };
    let v_0_a = v_0 ^ c2.a;
    let v_0_b = v_0 ^ c2.b;
    //
    let bits_0_a = PackedU16::new(v_0_a).may_have_zero_quick();
    let bits_0_b = PackedU16::new(v_0_b).may_have_zero_quick();
    let base = plus_offset_from(buf_ptr, st_ptr) + 2 - 1;
    //
    if !bits_0_a.is_zeros() {
        if !bits_0_b.is_zeros() {
            let idx1 = (bits_0_a.trailing_zeros() / 8) as usize;
            let idx2 = (bits_0_b.trailing_zeros() / 8) as usize;
            Some(base - idx1.min(idx2))
        } else {
            Some(base - (bits_0_a.trailing_zeros() / 8) as usize)
        }
    } else if !bits_0_b.is_zeros() {
        Some(base - (bits_0_b.trailing_zeros() / 8) as usize)
    } else {
        None
    }
}

#[inline(always)]
fn _rchr_dbl_c1_aa_x1(buf_ptr: *const u8, c1: C1Dbl, st_ptr: *const u8) -> Option<usize> {
    let aa_ptr = buf_ptr as *const u8;
    let aac = unsafe { *aa_ptr };
    if aac == c1.a || aac == c1.b {
        Some(plus_offset_from(buf_ptr, st_ptr))
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
pub fn _memrchr_impl(buf: &[u8], c1: u8, c2: u8) -> Option<usize> {
    for i in 0..buf.len() {
        let j = buf.len()-i-1;
        if buf[j] == c1 || buf[j] == c2 {
            return Some(j);
        }
    }
    None
}
*/
