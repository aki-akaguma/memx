use crate::utils::*;

#[inline(never)]
pub fn _memchr_qpl_impl(buf: &[u8], needle: B1Qpl) -> Option<usize> {
    #[cfg(all(
        feature = "test",
        any(feature = "test_pointer_width_64", feature = "test_pointer_width_32")
    ))]
    {
        #[cfg(feature = "test_pointer_width_64")]
        let r = _start_chr_64(buf, needle);
        #[cfg(feature = "test_pointer_width_32")]
        let r = _start_chr_32(buf, needle);
        //
        r
    }
    #[cfg(not(all(
        feature = "test",
        any(feature = "test_pointer_width_64", feature = "test_pointer_width_32")
    )))]
    {
        #[cfg(target_pointer_width = "64")]
        let r = _start_chr_64(buf, needle);
        #[cfg(target_pointer_width = "32")]
        let r = _start_chr_32(buf, needle);
        //
        r
    }
}

macro_rules! _unroll_one_chr_to_aligned_x1 {
    ($buf_ptr_2:expr, $buf_ptr_end:expr, $c:expr, $start_ptr:expr) => {{
        if $buf_ptr_2 >= $buf_ptr_end {
            break;
        }
        let r = _chr_qpl_c1_aa_x1($buf_ptr_2, $c, $start_ptr);
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
pub(crate) fn _chr_qpl_to_aligned_u256(
    buf_ptr: *const u8,
    c: B1Qpl,
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
pub(crate) fn _chr_qpl_to_aligned_u128(
    buf_ptr: *const u8,
    c: B1Qpl,
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
fn _chr_qpl_to_aligned_u64(
    buf_ptr: *const u8,
    c: B1Qpl,
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
fn _chr_qpl_to_aligned_u32(
    buf_ptr: *const u8,
    c: B1Qpl,
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

#[cfg(any(target_pointer_width = "64", feature = "test_pointer_width_64"))]
#[inline(always)]
fn _start_chr_64(buf: &[u8], needle: B1Qpl) -> Option<usize> {
    let buf_len = buf.len();
    let mut buf_ptr = buf.as_ptr();
    let start_ptr = buf_ptr;
    let end_ptr = unsafe { buf_ptr.add(buf_len) };
    let cc: B8Qpl = needle.into();
    buf_ptr.prefetch_read_data();
    //
    if buf_len >= 8 {
        // to a aligned pointer
        {
            if !buf_ptr.is_aligned_u64() {
                /*
                #[cfg(not(feature = "test_alignment_check"))]
                {
                    let r = _chr_qpl_c8_uu_x1(buf_ptr, cc, start_ptr);
                    if r.is_some() {
                        return r;
                    }
                    let remaining_align = 0x08_usize - ((buf_ptr as usize) & 0x07_usize);
                    buf_ptr = unsafe { buf_ptr.add(remaining_align) };
                }
                #[cfg(feature = "test_alignment_check")]
                {
                    let r = _chr_qpl_to_aligned_u64(buf_ptr, needle, start_ptr);
                    if let Some(p) = r.0 {
                        buf_ptr = p;
                    } else if let Some(v) = r.1 {
                        return Some(v);
                    }
                }
                */
                let r = _chr_qpl_to_aligned_u64(buf_ptr, needle, start_ptr);
                if let Some(p) = r.0 {
                    buf_ptr = p;
                } else if let Some(v) = r.1 {
                    return Some(v);
                }
            }
        }
        // the loop
        /*
        {
            let unroll = 8;
            let loop_size = 8;
            while buf_ptr.is_not_over(end_ptr, loop_size * unroll) {
                //buf_ptr.prefetch_read_data();
                let r = _chr_qpl_c8_aa_x8(buf_ptr, cc, start_ptr);
                if r.is_some() {
                    return r;
                }
                buf_ptr = unsafe { buf_ptr.add(loop_size * unroll) };
            }
        }
        */
        {
            let unroll = 4;
            let loop_size = 8;
            while buf_ptr.is_not_over(end_ptr, loop_size * unroll) {
                buf_ptr.prefetch_read_data();
                let r = _chr_qpl_c8_aa_x4(buf_ptr, cc, start_ptr);
                if r.is_some() {
                    return r;
                }
                buf_ptr = unsafe { buf_ptr.add(loop_size * unroll) };
            }
        }
        /*
        {
            let unroll = 2;
            let loop_size = 8;
            while buf_ptr.is_not_over(end_ptr, loop_size * unroll) {
                let r = _chr_qpl_c8_aa_x2(buf_ptr, cc, start_ptr);
                if r.is_some() {
                    return r;
                }
                buf_ptr = unsafe { buf_ptr.add(loop_size * unroll) };
            }
        }
        */
        {
            let unroll = 1;
            let loop_size = 8;
            while buf_ptr.is_not_over(end_ptr, loop_size * unroll) {
                let r = _chr_qpl_c8_aa_x1(buf_ptr, cc, start_ptr);
                if r.is_some() {
                    return r;
                }
                buf_ptr = unsafe { buf_ptr.add(loop_size * unroll) };
            }
        }
    }
    // the remaining data is the max: 7 bytes.
    _memchr_qpl_remaining_7_bytes_impl(buf_ptr, cc.into(), start_ptr, end_ptr)
}

#[cfg(any(target_pointer_width = "32", feature = "test_pointer_width_32"))]
#[inline(always)]
fn _start_chr_32(buf: &[u8], needle: B1Qpl) -> Option<usize> {
    let buf_len = buf.len();
    let mut buf_ptr = buf.as_ptr();
    let start_ptr = buf_ptr;
    let end_ptr = unsafe { buf_ptr.add(buf_len) };
    let cc: B4Qpl = needle.into();
    buf_ptr.prefetch_read_data();
    //
    if buf_len >= 4 {
        // to a aligned pointer
        {
            if !buf_ptr.is_aligned_u32() {
                /*
                #[cfg(not(feature = "test_alignment_check"))]
                {
                    let r = _chr_qpl_c4_uu_x1(buf_ptr, cc, start_ptr);
                    if r.is_some() {
                        return r;
                    }
                    let remaining_align = 0x04_usize - ((buf_ptr as usize) & 0x03_usize);
                    buf_ptr = unsafe { buf_ptr.add(remaining_align) };
                }
                #[cfg(feature = "test_alignment_check")]
                {
                    let r = _chr_qpl_to_aligned_u32(buf_ptr, needle, start_ptr);
                    if let Some(p) = r.0 {
                        buf_ptr = p;
                    } else if let Some(v) = r.1 {
                        return Some(v);
                    }
                }
                */
                let r = _chr_qpl_to_aligned_u32(buf_ptr, needle, start_ptr);
                if let Some(p) = r.0 {
                    buf_ptr = p;
                } else if let Some(v) = r.1 {
                    return Some(v);
                }
            }
        }
        // the loop
        /*
        {
            let unroll = 8;
            let loop_size = 4;
            while buf_ptr.is_not_over(end_ptr, loop_size * unroll) {
                buf_ptr.prefetch_read_data();
                let r = _chr_qpl_c4_aa_x8(buf_ptr, cc, start_ptr);
                if r.is_some() {
                    return r;
                }
                buf_ptr = unsafe { buf_ptr.add(loop_size * unroll) };
            }
        }
        */
        {
            let unroll = 4;
            let loop_size = 4;
            while buf_ptr.is_not_over(end_ptr, loop_size * unroll) {
                let r = _chr_qpl_c4_aa_x4(buf_ptr, cc, start_ptr);
                if r.is_some() {
                    return r;
                }
                buf_ptr = unsafe { buf_ptr.add(loop_size * unroll) };
            }
        }
        /*
        {
            let unroll = 2;
            let loop_size = 4;
            while buf_ptr.is_not_over(end_ptr, loop_size * unroll) {
                let r = _chr_qpl_c4_aa_x2(buf_ptr, cc, start_ptr);
                if r.is_some() {
                    return r;
                }
                buf_ptr = unsafe { buf_ptr.add(loop_size * unroll) };
            }
        }
        */
        {
            let unroll = 1;
            let loop_size = 4;
            while buf_ptr.is_not_over(end_ptr, loop_size * unroll) {
                let r = _chr_qpl_c4_aa_x1(buf_ptr, cc, start_ptr);
                if r.is_some() {
                    return r;
                }
                buf_ptr = unsafe { buf_ptr.add(loop_size * unroll) };
            }
        }
    }
    // the remaining data is the max: 3 bytes.
    _memchr_qpl_remaining_3_bytes_impl(buf_ptr, cc.into(), start_ptr, end_ptr)
}

#[inline(always)]
pub(crate) fn _memchr_qpl_remaining_15_bytes_impl(
    buf_ptr: *const u8,
    cc: B8Qpl,
    start_ptr: *const u8,
    end_ptr: *const u8,
) -> Option<usize> {
    let mut buf_ptr = buf_ptr;
    if buf_ptr.is_aligned_u64() {
        let loop_size = 8;
        if buf_ptr.is_not_over(end_ptr, loop_size) {
            let r = _chr_qpl_c8_aa_x1(buf_ptr, cc, start_ptr);
            if r.is_some() {
                return r;
            }
            buf_ptr = unsafe { buf_ptr.add(loop_size) };
        }
    }
    // the remaining data is the max: 7 bytes.
    _memchr_qpl_remaining_7_bytes_impl(buf_ptr, cc.into(), start_ptr, end_ptr)
}

#[inline(always)]
pub(crate) fn _memchr_qpl_remaining_7_bytes_impl(
    buf_ptr: *const u8,
    cc: B4Qpl,
    start_ptr: *const u8,
    end_ptr: *const u8,
) -> Option<usize> {
    let mut buf_ptr = buf_ptr;
    if buf_ptr.is_aligned_u32() {
        let loop_size = 4;
        if buf_ptr.is_not_over(end_ptr, loop_size) {
            let r = _chr_qpl_c4_aa_x1(buf_ptr, cc, start_ptr);
            if r.is_some() {
                return r;
            }
            buf_ptr = unsafe { buf_ptr.add(loop_size) };
        }
    }
    // the remaining data is the max: 3 bytes.
    _memchr_qpl_remaining_3_bytes_impl(buf_ptr, cc.into(), start_ptr, end_ptr)
}

#[inline(always)]
fn _memchr_qpl_remaining_3_bytes_impl(
    buf_ptr: *const u8,
    cc: B2Qpl,
    start_ptr: *const u8,
    end_ptr: *const u8,
) -> Option<usize> {
    let mut buf_ptr = buf_ptr;
    if buf_ptr.is_aligned_u16() {
        let loop_size = 2;
        if buf_ptr.is_not_over(end_ptr, loop_size) {
            let r = _chr_qpl_c2_aa_x1(buf_ptr, cc, start_ptr);
            if r.is_some() {
                return r;
            }
            buf_ptr = unsafe { buf_ptr.add(loop_size) };
        }
    }
    {
        let loop_size = 1;
        while buf_ptr.is_not_over(end_ptr, loop_size) {
            let r = _chr_qpl_c1_aa_x1(buf_ptr, cc.into(), start_ptr);
            if r.is_some() {
                return r;
            }
            buf_ptr = unsafe { buf_ptr.add(loop_size) };
        }
    }
    None
}

#[inline(always)]
fn _return_chr_qpl<T, PU>(base: T, bits_a: PU, bits_b: PU, bits_c: PU, bits_d: PU) -> Option<usize>
where
    T: core::ops::Add<usize, Output = usize>,
    PU: BitOrt,
{
    if !bits_a.is_zeros() {
        let idx1 = (bits_a.trailing_zeros() / 8) as usize;
        if !bits_b.is_zeros() {
            let idx2 = (bits_b.trailing_zeros() / 8) as usize;
            if !bits_c.is_zeros() {
                let idx3 = (bits_c.trailing_zeros() / 8) as usize;
                if !bits_d.is_zeros() {
                    let idx4 = (bits_d.trailing_zeros() / 8) as usize;
                    Some(base + idx1.min(idx2.min(idx3.min(idx4))))
                } else {
                    Some(base + idx1.min(idx2.min(idx3)))
                }
            } else if !bits_d.is_zeros() {
                let idx4 = (bits_d.trailing_zeros() / 8) as usize;
                Some(base + idx1.min(idx2.min(idx4)))
            } else {
                Some(base + idx1.min(idx2))
            }
        } else if !bits_c.is_zeros() {
            let idx3 = (bits_c.trailing_zeros() / 8) as usize;
            if !bits_d.is_zeros() {
                let idx4 = (bits_d.trailing_zeros() / 8) as usize;
                Some(base + idx1.min(idx3.min(idx4)))
            } else {
                Some(base + idx1.min(idx3))
            }
        } else if !bits_d.is_zeros() {
            let idx4 = (bits_d.trailing_zeros() / 8) as usize;
            Some(base + idx1.min(idx4))
        } else {
            Some(base + idx1)
        }
    } else if !bits_b.is_zeros() {
        let idx2 = (bits_b.trailing_zeros() / 8) as usize;
        if !bits_c.is_zeros() {
            let idx3 = (bits_c.trailing_zeros() / 8) as usize;
            if !bits_d.is_zeros() {
                let idx4 = (bits_d.trailing_zeros() / 8) as usize;
                Some(base + idx2.min(idx3.min(idx4)))
            } else {
                Some(base + idx2.min(idx3))
            }
        } else if !bits_d.is_zeros() {
            let idx4 = (bits_d.trailing_zeros() / 8) as usize;
            Some(base + idx2.min(idx4))
        } else {
            Some(base + idx2)
        }
    } else if !bits_c.is_zeros() {
        let idx3 = (bits_c.trailing_zeros() / 8) as usize;
        if !bits_d.is_zeros() {
            let idx4 = (bits_d.trailing_zeros() / 8) as usize;
            Some(base + idx3.min(idx4))
        } else {
            Some(base + idx3)
        }
    } else if !bits_d.is_zeros() {
        let idx4 = (bits_d.trailing_zeros() / 8) as usize;
        Some(base + idx4)
    } else {
        None
    }
}

#[inline(always)]
fn _chr_qpl_c16_uu_x1(buf_ptr: *const u8, c16: B16Qpl, st_ptr: *const u8) -> Option<usize> {
    _chr_qpl_c16_aa_x1(buf_ptr, c16, st_ptr)
}

#[inline(always)]
fn _chr_qpl_c16_aa_x1(buf_ptr: *const u8, c16: B16Qpl, st_ptr: *const u8) -> Option<usize> {
    let v_0 = unsafe { _read_a_little_endian_from_ptr_u128(buf_ptr) };
    let v_0_a = v_0 ^ c16.v1;
    let v_0_b = v_0 ^ c16.v2;
    let v_0_c = v_0 ^ c16.v3;
    let v_0_d = v_0 ^ c16.v4;
    let bits_0_a = PackedU128::new(v_0_a).may_have_zero_quick();
    let bits_0_b = PackedU128::new(v_0_b).may_have_zero_quick();
    let bits_0_c = PackedU128::new(v_0_c).may_have_zero_quick();
    let bits_0_d = PackedU128::new(v_0_d).may_have_zero_quick();
    let base = buf_ptr.usz_offset_from(st_ptr);
    //
    _return_chr_qpl(base, bits_0_a, bits_0_b, bits_0_c, bits_0_d)
}

#[inline(always)]
fn _chr_qpl_c16_aa_x2(buf_ptr: *const u8, c16: B16Qpl, st_ptr: *const u8) -> Option<usize> {
    let r = _chr_qpl_c16_aa_x1(buf_ptr, c16, st_ptr);
    if r.is_some() {
        return r;
    }
    let r = _chr_qpl_c16_aa_x1(unsafe { buf_ptr.add(16) }, c16, st_ptr);
    if r.is_some() {
        return r;
    }
    None
}

#[inline(always)]
fn _chr_qpl_c16_aa_x4(buf_ptr: *const u8, c16: B16Qpl, st_ptr: *const u8) -> Option<usize> {
    let r = _chr_qpl_c16_aa_x2(buf_ptr, c16, st_ptr);
    if r.is_some() {
        return r;
    }
    let r = _chr_qpl_c16_aa_x2(unsafe { buf_ptr.add(16 * 2) }, c16, st_ptr);
    if r.is_some() {
        return r;
    }
    None
}

#[inline(always)]
fn _chr_qpl_c16_aa_x8(buf_ptr: *const u8, c16: B16Qpl, st_ptr: *const u8) -> Option<usize> {
    let r = _chr_qpl_c16_aa_x4(buf_ptr, c16, st_ptr);
    if r.is_some() {
        return r;
    }
    let r = _chr_qpl_c16_aa_x4(unsafe { buf_ptr.add(16 * 4) }, c16, st_ptr);
    if r.is_some() {
        return r;
    }
    None
}

#[inline(always)]
fn _chr_qpl_c8_uu_x1(buf_ptr: *const u8, c8: B8Qpl, st_ptr: *const u8) -> Option<usize> {
    _chr_qpl_c8_aa_x1(buf_ptr, c8, st_ptr)
}

#[inline(always)]
fn _chr_qpl_c8_aa_x1(buf_ptr: *const u8, c8: B8Qpl, st_ptr: *const u8) -> Option<usize> {
    let v_0 = unsafe { _read_a_little_endian_from_ptr_u64(buf_ptr) };
    let v_0_a = v_0 ^ c8.v1;
    let v_0_b = v_0 ^ c8.v2;
    let v_0_c = v_0 ^ c8.v3;
    let v_0_d = v_0 ^ c8.v4;
    let bits_0_a = PackedU64::new(v_0_a).may_have_zero_quick();
    let bits_0_b = PackedU64::new(v_0_b).may_have_zero_quick();
    let bits_0_c = PackedU64::new(v_0_c).may_have_zero_quick();
    let bits_0_d = PackedU64::new(v_0_d).may_have_zero_quick();
    let base = buf_ptr.usz_offset_from(st_ptr);
    //
    _return_chr_qpl(base, bits_0_a, bits_0_b, bits_0_c, bits_0_d)
}

#[inline(always)]
fn _chr_qpl_c8_aa_x2(buf_ptr: *const u8, c8: B8Qpl, st_ptr: *const u8) -> Option<usize> {
    let r = _chr_qpl_c8_aa_x1(buf_ptr, c8, st_ptr);
    if r.is_some() {
        return r;
    }
    let r = _chr_qpl_c8_aa_x1(unsafe { buf_ptr.add(8) }, c8, st_ptr);
    if r.is_some() {
        return r;
    }
    None
}

#[inline(always)]
fn _chr_qpl_c8_aa_x4(buf_ptr: *const u8, c8: B8Qpl, st_ptr: *const u8) -> Option<usize> {
    let r = _chr_qpl_c8_aa_x2(buf_ptr, c8, st_ptr);
    if r.is_some() {
        return r;
    }
    let r = _chr_qpl_c8_aa_x2(unsafe { buf_ptr.add(8 * 2) }, c8, st_ptr);
    if r.is_some() {
        return r;
    }
    None
}

#[inline(always)]
fn _chr_qpl_c8_aa_x8(buf_ptr: *const u8, c8: B8Qpl, st_ptr: *const u8) -> Option<usize> {
    let r = _chr_qpl_c8_aa_x4(buf_ptr, c8, st_ptr);
    if r.is_some() {
        return r;
    }
    let r = _chr_qpl_c8_aa_x4(unsafe { buf_ptr.add(8 * 4) }, c8, st_ptr);
    if r.is_some() {
        return r;
    }
    None
}

#[inline(always)]
fn _chr_qpl_c4_uu_x1(buf_ptr: *const u8, c4: B4Qpl, st_ptr: *const u8) -> Option<usize> {
    _chr_qpl_c4_aa_x1(buf_ptr, c4, st_ptr)
}

#[inline(always)]
fn _chr_qpl_c4_aa_x1(buf_ptr: *const u8, c4: B4Qpl, st_ptr: *const u8) -> Option<usize> {
    let v_0 = unsafe { _read_a_little_endian_from_ptr_u32(buf_ptr) };
    let v_0_a = v_0 ^ c4.v1;
    let v_0_b = v_0 ^ c4.v2;
    let v_0_c = v_0 ^ c4.v3;
    let v_0_d = v_0 ^ c4.v4;
    let bits_0_a = PackedU32::new(v_0_a).may_have_zero_quick();
    let bits_0_b = PackedU32::new(v_0_b).may_have_zero_quick();
    let bits_0_c = PackedU32::new(v_0_c).may_have_zero_quick();
    let bits_0_d = PackedU32::new(v_0_d).may_have_zero_quick();
    let base = buf_ptr.usz_offset_from(st_ptr);
    //
    _return_chr_qpl(base, bits_0_a, bits_0_b, bits_0_c, bits_0_d)
}

#[inline(always)]
fn _chr_qpl_c4_aa_x2(buf_ptr: *const u8, c4: B4Qpl, st_ptr: *const u8) -> Option<usize> {
    let r = _chr_qpl_c4_aa_x1(buf_ptr, c4, st_ptr);
    if r.is_some() {
        return r;
    }
    let r = _chr_qpl_c4_aa_x1(unsafe { buf_ptr.add(4) }, c4, st_ptr);
    if r.is_some() {
        return r;
    }
    None
}

#[inline(always)]
fn _chr_qpl_c4_aa_x4(buf_ptr: *const u8, c4: B4Qpl, st_ptr: *const u8) -> Option<usize> {
    let r = _chr_qpl_c4_aa_x2(buf_ptr, c4, st_ptr);
    if r.is_some() {
        return r;
    }
    let r = _chr_qpl_c4_aa_x2(unsafe { buf_ptr.add(4 * 2) }, c4, st_ptr);
    if r.is_some() {
        return r;
    }
    None
}

#[inline(always)]
fn _chr_qpl_c4_aa_x8(buf_ptr: *const u8, c4: B4Qpl, st_ptr: *const u8) -> Option<usize> {
    let r = _chr_qpl_c4_aa_x4(buf_ptr, c4, st_ptr);
    if r.is_some() {
        return r;
    }
    let r = _chr_qpl_c4_aa_x4(unsafe { buf_ptr.add(4 * 4) }, c4, st_ptr);
    if r.is_some() {
        return r;
    }
    None
}

#[inline(always)]
fn _chr_qpl_c2_aa_x1(buf_ptr: *const u8, c2: B2Qpl, st_ptr: *const u8) -> Option<usize> {
    let v_0 = unsafe { _read_a_little_endian_from_ptr_u16(buf_ptr) };
    let v_0_a = v_0 ^ c2.v1;
    let v_0_b = v_0 ^ c2.v2;
    let v_0_c = v_0 ^ c2.v3;
    let v_0_d = v_0 ^ c2.v4;
    let bits_0_a = PackedU16::new(v_0_a).may_have_zero_quick();
    let bits_0_b = PackedU16::new(v_0_b).may_have_zero_quick();
    let bits_0_c = PackedU16::new(v_0_c).may_have_zero_quick();
    let bits_0_d = PackedU16::new(v_0_d).may_have_zero_quick();
    let base = buf_ptr.usz_offset_from(st_ptr);
    //
    _return_chr_qpl(base, bits_0_a, bits_0_b, bits_0_c, bits_0_d)
}

#[inline(always)]
fn _chr_qpl_c1_aa_x1(buf_ptr: *const u8, c1: B1Qpl, st_ptr: *const u8) -> Option<usize> {
    let aa_ptr = buf_ptr;
    let aac = unsafe { *aa_ptr };
    if aac == c1.v1 || aac == c1.v2 || aac == c1.v3 || aac == c1.v4 {
        Some(buf_ptr.usz_offset_from(st_ptr))
    } else {
        None
    }
}

/*
 * The simple implement:

#[inline(always)]
pub fn _memchr_qpl_impl(buf: &[u8], needle: B1Qpl) -> Option<usize> {
    for i in 0..buf.len() {
        if buf[i] == needle.v1 || buf[i] == needle.v2 || buf[i] == needle.v3 || buf[i] == needle.v4 {
            return Some(i);
        }
    }
    None
}
*/
/*
 * Reference.
 * https://pzemtsov.github.io/2019/09/26/making-a-char-searcher-in-c.html
 * https://graphics.stanford.edu/~seander/bithacks.html#ZeroInWord
*/
