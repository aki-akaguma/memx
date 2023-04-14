use crate::utils::*;

#[inline(never)]
pub fn _memchr_impl(buf: &[u8], c1: u8) -> Option<usize> {
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
        let r = _start_chr_128(buf, c1);
        #[cfg(feature = "test_pointer_width_64")]
        let r = _start_chr_64(buf, c1);
        #[cfg(feature = "test_pointer_width_32")]
        let r = _start_chr_32(buf, c1);
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
        let r = _start_chr_128(buf, c1);
        #[cfg(target_pointer_width = "64")]
        let r = _start_chr_64(buf, c1);
        #[cfg(target_pointer_width = "32")]
        let r = _start_chr_32(buf, c1);
        //
        r
    }
}

macro_rules! _unroll_one_chr_to_aligned_x1 {
    ($buf_ptr_2:expr, $buf_ptr_end:expr, $c:expr, $start_ptr:expr) => {{
        if $buf_ptr_2 >= $buf_ptr_end {
            break;
        }
        let r = _chr_c1_aa_x1($buf_ptr_2, $c, $start_ptr);
        if r.is_some() {
            return (None, r);
        }
        $buf_ptr_2 = unsafe { $buf_ptr_2.add(1) };
    }};
}

macro_rules! _unroll_one_chr_to_aligned_x4 {
    ($buf_ptr_2:expr, $buf_ptr_end:expr, $c:expr, $start_ptr:expr) => {{
        _unroll_one_chr_to_aligned_x1!($buf_ptr_2, $buf_ptr_end, $c, $start_ptr);
        _unroll_one_chr_to_aligned_x1!($buf_ptr_2, $buf_ptr_end, $c, $start_ptr);
        _unroll_one_chr_to_aligned_x1!($buf_ptr_2, $buf_ptr_end, $c, $start_ptr);
        _unroll_one_chr_to_aligned_x1!($buf_ptr_2, $buf_ptr_end, $c, $start_ptr);
    }};
}

macro_rules! _unroll_one_chr_to_aligned_x8 {
    ($buf_ptr_2:expr, $buf_ptr_end:expr, $c:expr, $start_ptr:expr) => {{
        _unroll_one_chr_to_aligned_x4!($buf_ptr_2, $buf_ptr_end, $c, $start_ptr);
        _unroll_one_chr_to_aligned_x4!($buf_ptr_2, $buf_ptr_end, $c, $start_ptr);
    }};
}

macro_rules! _unroll_one_chr_to_aligned_x16 {
    ($buf_ptr_2:expr, $buf_ptr_end:expr, $c:expr, $start_ptr:expr) => {{
        _unroll_one_chr_to_aligned_x8!($buf_ptr_2, $buf_ptr_end, $c, $start_ptr);
        _unroll_one_chr_to_aligned_x8!($buf_ptr_2, $buf_ptr_end, $c, $start_ptr);
    }};
}

#[inline(always)]
pub(crate) fn _chr_to_aligned_u256(
    buf_ptr: *const u8,
    c: B1Sgl,
    st_ptr: *const u8,
) -> (Option<*const u8>, Option<usize>) {
    let remaining_align = 0x20_usize - ((buf_ptr as usize) & 0x1F_usize);
    let buf_ptr_end = unsafe { buf_ptr.add(remaining_align) };
    let mut buf_ptr_2 = buf_ptr;
    loop {
        _unroll_one_chr_to_aligned_x16!(buf_ptr_2, buf_ptr_end, c, st_ptr);
        _unroll_one_chr_to_aligned_x16!(buf_ptr_2, buf_ptr_end, c, st_ptr);
    }
    (Some(buf_ptr_end), None)
}

#[inline(always)]
pub(crate) fn _chr_to_aligned_u128(
    buf_ptr: *const u8,
    c: B1Sgl,
    st_ptr: *const u8,
) -> (Option<*const u8>, Option<usize>) {
    let remaining_align = 0x10_usize - ((buf_ptr as usize) & 0x0F_usize);
    let buf_ptr_end = unsafe { buf_ptr.add(remaining_align) };
    let mut buf_ptr_2 = buf_ptr;
    loop {
        _unroll_one_chr_to_aligned_x16!(buf_ptr_2, buf_ptr_end, c, st_ptr);
    }
    (Some(buf_ptr_end), None)
}

#[inline(always)]
fn _chr_to_aligned_u64(
    buf_ptr: *const u8,
    c: B1Sgl,
    st_ptr: *const u8,
) -> (Option<*const u8>, Option<usize>) {
    let remaining_align = 0x08_usize - ((buf_ptr as usize) & 0x07_usize);
    let buf_ptr_end = unsafe { buf_ptr.add(remaining_align) };
    let mut buf_ptr_2 = buf_ptr;
    loop {
        _unroll_one_chr_to_aligned_x8!(buf_ptr_2, buf_ptr_end, c, st_ptr);
    }
    (Some(buf_ptr_end), None)
}

#[inline(always)]
fn _chr_to_aligned_u32(
    buf_ptr: *const u8,
    c: B1Sgl,
    st_ptr: *const u8,
) -> (Option<*const u8>, Option<usize>) {
    let remaining_align = 0x04_usize - ((buf_ptr as usize) & 0x03_usize);
    let buf_ptr_end = unsafe { buf_ptr.add(remaining_align) };
    let mut buf_ptr_2 = buf_ptr;
    loop {
        _unroll_one_chr_to_aligned_x4!(buf_ptr_2, buf_ptr_end, c, st_ptr);
    }
    (Some(buf_ptr_end), None)
}

#[cfg(any(target_pointer_width = "128", feature = "test_pointer_width_128"))]
#[inline(always)]
fn _start_chr_128(buf: &[u8], c_1: u8) -> Option<usize> {
    let buf_len = buf.len();
    let mut buf_ptr = buf.as_ptr();
    let start_ptr = buf_ptr;
    let end_ptr = unsafe { buf_ptr.add(buf_len) };
    let c = B1Sgl::new(c_1);
    let cc = B16Sgl::new(c_1);
    buf_ptr.prefetch_read_data();
    //
    if buf_len >= 16 {
        // to a aligned pointer
        {
            if !buf_ptr.is_aligned_u128() {
                let r = _chr_to_aligned_u128(buf_ptr, c, start_ptr);
                if let Some(p) = r.0 {
                    buf_ptr = p;
                } else if let Some(v) = r.1 {
                    return Some(v);
                }
            }
        }
        // the loop
        {
            let unroll = 8;
            let loop_size = 16;
            while unsafe { end_ptr.offset_from(buf_ptr) } >= (loop_size * unroll) as isize {
                buf_ptr.prefetch_read_data();
                let r = _chr_c16_aa_x8(buf_ptr, cc, start_ptr);
                if r.is_some() {
                    return r;
                }
                buf_ptr = unsafe { buf_ptr.add(loop_size * unroll) };
            }
        }
        {
            let unroll = 4;
            let loop_size = 16;
            while unsafe { end_ptr.offset_from(buf_ptr) } >= (loop_size * unroll) as isize {
                buf_ptr.prefetch_read_data();
                let r = _chr_c16_aa_x4(buf_ptr, cc, start_ptr);
                if r.is_some() {
                    return r;
                }
                buf_ptr = unsafe { buf_ptr.add(loop_size * unroll) };
            }
        }
        {
            let unroll = 2;
            let loop_size = 16;
            while unsafe { end_ptr.offset_from(buf_ptr) } >= (loop_size * unroll) as isize {
                buf_ptr.prefetch_read_data();
                let r = _chr_c16_aa_x2(buf_ptr, cc, start_ptr);
                if r.is_some() {
                    return r;
                }
                buf_ptr = unsafe { buf_ptr.add(loop_size * unroll) };
            }
        }
        {
            let unroll = 1;
            let loop_size = 16;
            while unsafe { end_ptr.offset_from(buf_ptr) } >= (loop_size * unroll) as isize {
                buf_ptr.prefetch_read_data();
                let r = _chr_c16_aa_x1(buf_ptr, cc, start_ptr);
                if r.is_some() {
                    return r;
                }
                buf_ptr = unsafe { buf_ptr.add(loop_size * unroll) };
            }
        }
    }
    // the remaining data is the max: 15 bytes.
    _memchr_remaining_15_bytes_impl(buf_ptr, cc.into(), start_ptr, end_ptr)
}

#[cfg(any(target_pointer_width = "64", feature = "test_pointer_width_64"))]
#[inline(always)]
fn _start_chr_64(buf: &[u8], c_1: u8) -> Option<usize> {
    let buf_len = buf.len();
    let mut buf_ptr = buf.as_ptr();
    let start_ptr = buf_ptr;
    let end_ptr = unsafe { buf_ptr.add(buf_len) };
    let c = B1Sgl::new(c_1);
    let cc = B8Sgl::new(c_1);
    buf_ptr.prefetch_read_data();
    //
    if buf_len >= 8 {
        // to a aligned pointer
        {
            if !buf_ptr.is_aligned_u64() {
                let r = _chr_to_aligned_u64(buf_ptr, c, start_ptr);
                if let Some(p) = r.0 {
                    buf_ptr = p;
                } else if let Some(v) = r.1 {
                    return Some(v);
                }
            }
        }
        // the loop
        {
            let unroll = 8;
            let loop_size = 8;
            while unsafe { end_ptr.offset_from(buf_ptr) } >= (loop_size * unroll) as isize {
                buf_ptr.prefetch_read_data();
                let r = _chr_c8_aa_x8(buf_ptr, cc, start_ptr);
                if r.is_some() {
                    return r;
                }
                buf_ptr = unsafe { buf_ptr.add(loop_size * unroll) };
            }
        }
        {
            let unroll = 4;
            let loop_size = 8;
            while unsafe { end_ptr.offset_from(buf_ptr) } >= (loop_size * unroll) as isize {
                buf_ptr.prefetch_read_data();
                let r = _chr_c8_aa_x4(buf_ptr, cc, start_ptr);
                if r.is_some() {
                    return r;
                }
                buf_ptr = unsafe { buf_ptr.add(loop_size * unroll) };
            }
        }
        {
            let unroll = 2;
            let loop_size = 8;
            while unsafe { end_ptr.offset_from(buf_ptr) } >= (loop_size * unroll) as isize {
                let r = _chr_c8_aa_x2(buf_ptr, cc, start_ptr);
                if r.is_some() {
                    return r;
                }
                buf_ptr = unsafe { buf_ptr.add(loop_size * unroll) };
            }
        }
        {
            let unroll = 1;
            let loop_size = 8;
            while unsafe { end_ptr.offset_from(buf_ptr) } >= (loop_size * unroll) as isize {
                let r = _chr_c8_aa_x1(buf_ptr, cc, start_ptr);
                if r.is_some() {
                    return r;
                }
                buf_ptr = unsafe { buf_ptr.add(loop_size * unroll) };
            }
        }
    }
    // the remaining data is the max: 7 bytes.
    _memchr_remaining_7_bytes_impl(buf_ptr, cc.into(), start_ptr, end_ptr)
}

#[cfg(any(target_pointer_width = "32", feature = "test_pointer_width_32"))]
#[inline(always)]
fn _start_chr_32(buf: &[u8], c_1: u8) -> Option<usize> {
    let buf_len = buf.len();
    let mut buf_ptr = buf.as_ptr();
    let start_ptr = buf_ptr;
    let end_ptr = unsafe { buf_ptr.add(buf_len) };
    let c = B1Sgl::new(c_1);
    let cc = B4Sgl::new(c_1);
    buf_ptr.prefetch_read_data();
    //
    if buf_len >= 4 {
        // to a aligned pointer
        {
            if !buf_ptr.is_aligned_u32() {
                let r = _chr_to_aligned_u32(buf_ptr, c, start_ptr);
                if let Some(p) = r.0 {
                    buf_ptr = p;
                } else if let Some(v) = r.1 {
                    return Some(v);
                }
            }
        }
        // the loop
        {
            let unroll = 8;
            let loop_size = 4;
            while unsafe { end_ptr.offset_from(buf_ptr) } >= (loop_size * unroll) as isize {
                buf_ptr.prefetch_read_data();
                let r = _chr_c4_aa_x8(buf_ptr, cc, start_ptr);
                if r.is_some() {
                    return r;
                }
                buf_ptr = unsafe { buf_ptr.add(loop_size * unroll) };
            }
        }
        {
            let unroll = 4;
            let loop_size = 4;
            while unsafe { end_ptr.offset_from(buf_ptr) } >= (loop_size * unroll) as isize {
                let r = _chr_c4_aa_x4(buf_ptr, cc, start_ptr);
                if r.is_some() {
                    return r;
                }
                buf_ptr = unsafe { buf_ptr.add(loop_size * unroll) };
            }
        }
        {
            let unroll = 2;
            let loop_size = 4;
            while unsafe { end_ptr.offset_from(buf_ptr) } >= (loop_size * unroll) as isize {
                let r = _chr_c4_aa_x2(buf_ptr, cc, start_ptr);
                if r.is_some() {
                    return r;
                }
                buf_ptr = unsafe { buf_ptr.add(loop_size * unroll) };
            }
        }
        {
            let unroll = 1;
            let loop_size = 4;
            while unsafe { end_ptr.offset_from(buf_ptr) } >= (loop_size * unroll) as isize {
                let r = _chr_c4_aa_x1(buf_ptr, cc, start_ptr);
                if r.is_some() {
                    return r;
                }
                buf_ptr = unsafe { buf_ptr.add(loop_size * unroll) };
            }
        }
    }
    // the remaining data is the max: 3 bytes.
    _memchr_remaining_3_bytes_impl(buf_ptr, cc.into(), start_ptr, end_ptr)
}

#[inline(always)]
pub(crate) fn _memchr_remaining_15_bytes_impl(
    buf_ptr: *const u8,
    cc: B8Sgl,
    start_ptr: *const u8,
    end_ptr: *const u8,
) -> Option<usize> {
    let mut buf_ptr = buf_ptr;
    if buf_ptr.is_aligned_u64() {
        let loop_size = 8;
        if unsafe { end_ptr.offset_from(buf_ptr) } >= loop_size as isize {
            let r = _chr_c8_aa_x1(buf_ptr, cc, start_ptr);
            if r.is_some() {
                return r;
            }
            buf_ptr = unsafe { buf_ptr.add(loop_size) };
        }
    }
    // the remaining data is the max: 7 bytes.
    _memchr_remaining_7_bytes_impl(buf_ptr, cc.into(), start_ptr, end_ptr)
}

#[inline(always)]
pub(crate) fn _memchr_remaining_7_bytes_impl(
    buf_ptr: *const u8,
    cc: B4Sgl,
    start_ptr: *const u8,
    end_ptr: *const u8,
) -> Option<usize> {
    let mut buf_ptr = buf_ptr;
    if buf_ptr.is_aligned_u32() {
        let loop_size = 4;
        if unsafe { end_ptr.offset_from(buf_ptr) } >= loop_size as isize {
            let r = _chr_c4_aa_x1(buf_ptr, cc, start_ptr);
            if r.is_some() {
                return r;
            }
            buf_ptr = unsafe { buf_ptr.add(loop_size) };
        }
    }
    // the remaining data is the max: 3 bytes.
    _memchr_remaining_3_bytes_impl(buf_ptr, cc.into(), start_ptr, end_ptr)
}

#[inline(always)]
pub(crate) fn _memchr_remaining_3_bytes_impl(
    buf_ptr: *const u8,
    cc: B2Sgl,
    start_ptr: *const u8,
    end_ptr: *const u8,
) -> Option<usize> {
    let mut buf_ptr = buf_ptr;
    if buf_ptr.is_aligned_u16() {
        let loop_size = 2;
        if unsafe { end_ptr.offset_from(buf_ptr) } >= loop_size as isize {
            let r = _chr_c2_aa_x1(buf_ptr, cc, start_ptr);
            if r.is_some() {
                return r;
            }
            buf_ptr = unsafe { buf_ptr.add(loop_size) };
        }
    }
    {
        let loop_size = 1;
        if unsafe { end_ptr.offset_from(buf_ptr) } >= loop_size as isize {
            while unsafe { end_ptr.offset_from(buf_ptr) } >= loop_size as isize {
                let r = _chr_c1_aa_x1(buf_ptr, cc.into(), start_ptr);
                if r.is_some() {
                    return r;
                }
                buf_ptr = unsafe { buf_ptr.add(loop_size) };
            }
        }
    }
    None
}

#[inline(always)]
fn _chr_c16_aa_x1(buf_ptr: *const u8, c16: B16Sgl, st_ptr: *const u8) -> Option<usize> {
    let v_0 = unsafe { _read_a_little_endian_from_ptr_u128(buf_ptr) } ^ c16.a;
    let bits_0 = PackedU128::new(v_0).may_have_zero_quick();
    //
    if !bits_0.is_zeros() {
        Some(plus_offset_from(buf_ptr, st_ptr) + (bits_0.trailing_zeros() / 8) as usize)
    } else {
        None
    }
}

#[inline(always)]
fn _chr_c16_aa_x2(buf_ptr: *const u8, c16: B16Sgl, st_ptr: *const u8) -> Option<usize> {
    let r = _chr_c16_aa_x1(buf_ptr, c16, st_ptr);
    if r.is_some() {
        return r;
    }
    let r = _chr_c16_aa_x1(unsafe { buf_ptr.add(16) }, c16, st_ptr);
    if r.is_some() {
        return r;
    }
    None
}

#[inline(always)]
fn _chr_c16_aa_x4(buf_ptr: *const u8, c16: B16Sgl, st_ptr: *const u8) -> Option<usize> {
    let r = _chr_c16_aa_x2(buf_ptr, c16, st_ptr);
    if r.is_some() {
        return r;
    }
    let r = _chr_c16_aa_x2(unsafe { buf_ptr.add(16 * 2) }, c16, st_ptr);
    if r.is_some() {
        return r;
    }
    None
}

#[inline(always)]
fn _chr_c16_aa_x8(buf_ptr: *const u8, c16: B16Sgl, st_ptr: *const u8) -> Option<usize> {
    let r = _chr_c16_aa_x4(buf_ptr, c16, st_ptr);
    if r.is_some() {
        return r;
    }
    let r = _chr_c16_aa_x4(unsafe { buf_ptr.add(16 * 4) }, c16, st_ptr);
    if r.is_some() {
        return r;
    }
    None
}

#[inline(always)]
fn _chr_c8_aa_x1(buf_ptr: *const u8, c8: B8Sgl, st_ptr: *const u8) -> Option<usize> {
    let v_0 = unsafe { _read_a_little_endian_from_ptr_u64(buf_ptr) } ^ c8.a;
    let bits_0 = PackedU64::new(v_0).may_have_zero_quick();
    //
    if !bits_0.is_zeros() {
        Some(plus_offset_from(buf_ptr, st_ptr) + (bits_0.trailing_zeros() / 8) as usize)
    } else {
        None
    }
}

#[inline(always)]
fn _chr_c8_aa_x2(buf_ptr: *const u8, c8: B8Sgl, st_ptr: *const u8) -> Option<usize> {
    let r = _chr_c8_aa_x1(buf_ptr, c8, st_ptr);
    if r.is_some() {
        return r;
    }
    let r = _chr_c8_aa_x1(unsafe { buf_ptr.add(8) }, c8, st_ptr);
    if r.is_some() {
        return r;
    }
    None
}

#[inline(always)]
fn _chr_c8_aa_x4(buf_ptr: *const u8, c8: B8Sgl, st_ptr: *const u8) -> Option<usize> {
    let r = _chr_c8_aa_x2(buf_ptr, c8, st_ptr);
    if r.is_some() {
        return r;
    }
    let r = _chr_c8_aa_x2(unsafe { buf_ptr.add(8 * 2) }, c8, st_ptr);
    if r.is_some() {
        return r;
    }
    None
}

#[inline(always)]
fn _chr_c8_aa_x8(buf_ptr: *const u8, c8: B8Sgl, st_ptr: *const u8) -> Option<usize> {
    let r = _chr_c8_aa_x4(buf_ptr, c8, st_ptr);
    if r.is_some() {
        return r;
    }
    let r = _chr_c8_aa_x4(unsafe { buf_ptr.add(8 * 4) }, c8, st_ptr);
    if r.is_some() {
        return r;
    }
    None
}

#[inline(always)]
fn _chr_c4_aa_x1(buf_ptr: *const u8, c4: B4Sgl, st_ptr: *const u8) -> Option<usize> {
    let v_0 = unsafe { _read_a_little_endian_from_ptr_u32(buf_ptr) } ^ c4.a;
    let bits_0 = PackedU32::new(v_0).may_have_zero_quick();
    //
    if !bits_0.is_zeros() {
        Some(plus_offset_from(buf_ptr, st_ptr) + (bits_0.trailing_zeros() / 8) as usize)
    } else {
        None
    }
}

#[inline(always)]
fn _chr_c4_aa_x2(buf_ptr: *const u8, c4: B4Sgl, st_ptr: *const u8) -> Option<usize> {
    let r = _chr_c4_aa_x1(buf_ptr, c4, st_ptr);
    if r.is_some() {
        return r;
    }
    let r = _chr_c4_aa_x1(unsafe { buf_ptr.add(4) }, c4, st_ptr);
    if r.is_some() {
        return r;
    }
    None
}

#[inline(always)]
fn _chr_c4_aa_x4(buf_ptr: *const u8, c4: B4Sgl, st_ptr: *const u8) -> Option<usize> {
    let r = _chr_c4_aa_x2(buf_ptr, c4, st_ptr);
    if r.is_some() {
        return r;
    }
    let r = _chr_c4_aa_x2(unsafe { buf_ptr.add(4 * 2) }, c4, st_ptr);
    if r.is_some() {
        return r;
    }
    None
}

#[inline(always)]
fn _chr_c4_aa_x8(buf_ptr: *const u8, c4: B4Sgl, st_ptr: *const u8) -> Option<usize> {
    let r = _chr_c4_aa_x4(buf_ptr, c4, st_ptr);
    if r.is_some() {
        return r;
    }
    let r = _chr_c4_aa_x4(unsafe { buf_ptr.add(4 * 4) }, c4, st_ptr);
    if r.is_some() {
        return r;
    }
    None
}

#[inline(always)]
fn _chr_c2_aa_x1(buf_ptr: *const u8, c2: B2Sgl, st_ptr: *const u8) -> Option<usize> {
    let v_0 = unsafe { _read_a_little_endian_from_ptr_u16(buf_ptr) } ^ c2.a;
    let bits_0 = PackedU16::new(v_0).may_have_zero_quick();
    //
    if !bits_0.is_zeros() {
        Some(plus_offset_from(buf_ptr, st_ptr) + (bits_0.trailing_zeros() / 8) as usize)
    } else {
        None
    }
}

#[inline(always)]
fn _chr_c1_aa_x1(buf_ptr: *const u8, c1: B1Sgl, st_ptr: *const u8) -> Option<usize> {
    let aa_ptr = buf_ptr as *const u8;
    let aac = unsafe { *aa_ptr };
    if aac == c1.a {
        Some(plus_offset_from(buf_ptr, st_ptr))
    } else {
        None
    }
}

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
*/
