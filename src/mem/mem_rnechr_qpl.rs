use crate::utils::*;

#[inline(never)]
pub fn _memrnechr_qpl_impl(buf: &[u8], needle: B1Qpl) -> Option<usize> {
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
        let r = _start_rnechr_128(buf, needle);
        #[cfg(feature = "test_pointer_width_64")]
        let r = _start_rnechr_64(buf, needle);
        #[cfg(feature = "test_pointer_width_32")]
        let r = _start_rnechr_32(buf, needle);
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
        let r = _start_rnechr_128(buf, needle);
        #[cfg(target_pointer_width = "64")]
        let r = _start_rnechr_64(buf, needle);
        #[cfg(target_pointer_width = "32")]
        let r = _start_rnechr_32(buf, needle);
        //
        r
    }
}

macro_rules! _unroll_one_rnechr_to_align_x1 {
    ($buf_ptr_2:expr, $buf_ptr_min:expr, $c:expr, $start_ptr:expr) => {{
        if $buf_ptr_2 <= $buf_ptr_min {
            break;
        }
        $buf_ptr_2 = unsafe { $buf_ptr_2.sub(1) };
        let r = _rnechr_c1_aa_x1($buf_ptr_2, $c, $start_ptr);
        if r.is_some() {
            return (None, r);
        }
    }};
}

macro_rules! _unroll_one_rnechr_to_align_x4 {
    ($buf_ptr_2:expr, $buf_ptr_min:expr, $c:expr, $start_ptr:expr) => {{
        _unroll_one_rnechr_to_align_x1!($buf_ptr_2, $buf_ptr_min, $c, $start_ptr);
        _unroll_one_rnechr_to_align_x1!($buf_ptr_2, $buf_ptr_min, $c, $start_ptr);
        _unroll_one_rnechr_to_align_x1!($buf_ptr_2, $buf_ptr_min, $c, $start_ptr);
        _unroll_one_rnechr_to_align_x1!($buf_ptr_2, $buf_ptr_min, $c, $start_ptr);
    }};
}

macro_rules! _unroll_one_rnechr_to_align_x8 {
    ($buf_ptr_2:expr, $buf_ptr_min:expr, $c:expr, $start_ptr:expr) => {{
        _unroll_one_rnechr_to_align_x4!($buf_ptr_2, $buf_ptr_min, $c, $start_ptr);
        _unroll_one_rnechr_to_align_x4!($buf_ptr_2, $buf_ptr_min, $c, $start_ptr);
    }};
}

macro_rules! _unroll_one_rnechr_to_align_x16 {
    ($buf_ptr_2:expr, $buf_ptr_min:expr, $c:expr, $start_ptr:expr) => {{
        _unroll_one_rnechr_to_align_x8!($buf_ptr_2, $buf_ptr_min, $c, $start_ptr);
        _unroll_one_rnechr_to_align_x8!($buf_ptr_2, $buf_ptr_min, $c, $start_ptr);
    }};
}

#[inline(always)]
pub(crate) fn _rnechr_qpl_to_aligned_u256(
    buf_ptr_cur: *const u8,
    c: B1Qpl,
    start_ptr: *const u8,
) -> (Option<*const u8>, Option<usize>) {
    let buf_ptr_end = buf_ptr_cur;
    let remaining_align = (buf_ptr_end as usize) & 0x1F_usize;
    let buf_ptr_min = unsafe { buf_ptr_end.sub(remaining_align) };
    let mut buf_ptr_2 = buf_ptr_end;
    loop {
        _unroll_one_rnechr_to_align_x16!(buf_ptr_2, buf_ptr_min, c, start_ptr);
        _unroll_one_rnechr_to_align_x16!(buf_ptr_2, buf_ptr_min, c, start_ptr);
    }
    (Some(buf_ptr_min), None)
}

#[inline(always)]
pub(crate) fn _rnechr_qpl_to_aligned_u128(
    buf_ptr_cur: *const u8,
    c: B1Qpl,
    start_ptr: *const u8,
) -> (Option<*const u8>, Option<usize>) {
    let buf_ptr_end = buf_ptr_cur;
    let remaining_align = (buf_ptr_end as usize) & 0x0F_usize;
    let buf_ptr_min = unsafe { buf_ptr_end.sub(remaining_align) };
    let mut buf_ptr_2 = buf_ptr_end;
    loop {
        _unroll_one_rnechr_to_align_x16!(buf_ptr_2, buf_ptr_min, c, start_ptr);
    }
    (Some(buf_ptr_min), None)
}

#[inline(always)]
fn _rnechr_qpl_to_aligned_u64(
    buf_ptr_cur: *const u8,
    c: B1Qpl,
    start_ptr: *const u8,
) -> (Option<*const u8>, Option<usize>) {
    let buf_ptr_end = buf_ptr_cur;
    let remaining_align = (buf_ptr_end as usize) & 0x07_usize;
    let buf_ptr_min = unsafe { buf_ptr_end.sub(remaining_align) };
    let mut buf_ptr_2 = buf_ptr_end;
    loop {
        _unroll_one_rnechr_to_align_x8!(buf_ptr_2, buf_ptr_min, c, start_ptr);
    }
    (Some(buf_ptr_min), None)
}

#[inline(always)]
fn _rnechr_qpl_to_aligned_u32(
    buf_ptr_cur: *const u8,
    c: B1Qpl,
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

#[cfg(any(target_pointer_width = "128", feature = "test_pointer_width_128"))]
#[inline(always)]
fn _start_rnechr_128(buf: &[u8], needle: B1Qpl) -> Option<usize> {
    let buf_len = buf.len();
    let start_ptr = buf.as_ptr();
    let mut buf_ptr = unsafe { start_ptr.add(buf_len) };
    let cc: B16Qpl = needle.into();
    //
    if buf_len >= 16 {
        // to a aligned pointer
        {
            if !buf_ptr.is_aligned_u128() {
                let r = _rnechr_qpl_to_aligned_u128(buf_ptr, needle, start_ptr);
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
            let loop_size = 16;
            while buf_ptr.is_not_under(start_ptr, loop_size * unroll) {
                buf_ptr = unsafe { buf_ptr.sub(loop_size * unroll) };
                buf_ptr.prefetch_read_data();
                let r = _rnechr_c16_aa_x8(buf_ptr, cc, start_ptr);
                if r.is_some() {
                    return r;
                }
            }
        }
        */
        {
            let unroll = 4;
            let loop_size = 16;
            while buf_ptr.is_not_under(start_ptr, loop_size * unroll) {
                buf_ptr = unsafe { buf_ptr.sub(loop_size * unroll) };
                //buf_ptr.prefetch_read_data();
                let r = _rnechr_c16_aa_x4(buf_ptr, cc, start_ptr);
                if r.is_some() {
                    return r;
                }
            }
        }
        /*
        {
            let unroll = 2;
            let loop_size = 16;
            while buf_ptr.is_not_under(start_ptr, loop_size * unroll) {
                buf_ptr = unsafe { buf_ptr.sub(loop_size * unroll) };
                buf_ptr.prefetch_read_data();
                let r = _rnechr_c16_aa_x2(buf_ptr, cc, start_ptr);
                if r.is_some() {
                    return r;
                }
            }
        }
        */
        {
            let unroll = 1;
            let loop_size = 16;
            while buf_ptr.is_not_under(start_ptr, loop_size * unroll) {
                buf_ptr = unsafe { buf_ptr.sub(loop_size * unroll) };
                let r = _rnechr_c16_aa_x1(buf_ptr, cc, start_ptr);
                if r.is_some() {
                    return r;
                }
            }
        }
    }
    // the remaining data is the max: 15 bytes.
    _memrnechr_qpl_remaining_15_bytes_impl(buf_ptr, cc.into(), start_ptr)
}

#[cfg(any(target_pointer_width = "64", feature = "test_pointer_width_64"))]
#[inline(always)]
fn _start_rnechr_64(buf: &[u8], needle: B1Qpl) -> Option<usize> {
    let buf_len = buf.len();
    let start_ptr = buf.as_ptr();
    let mut buf_ptr = unsafe { start_ptr.add(buf_len) };
    let cc: B8Qpl = needle.into();
    //
    if buf_len >= 8 {
        // to a aligned pointer
        {
            if !buf_ptr.is_aligned_u64() {
                let r = _rnechr_qpl_to_aligned_u64(buf_ptr, needle, start_ptr);
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
            while buf_ptr.is_not_under(start_ptr, loop_size * unroll) {
                buf_ptr = unsafe { buf_ptr.sub(loop_size * unroll) };
                let r = _rnechr_c8_aa_x8(buf_ptr, cc, start_ptr);
                if r.is_some() {
                    return r;
                }
            }
        }
        */
        {
            let unroll = 4;
            let loop_size = 8;
            while buf_ptr.is_not_under(start_ptr, loop_size * unroll) {
                buf_ptr = unsafe { buf_ptr.sub(loop_size * unroll) };
                let r = _rnechr_c8_aa_x4(buf_ptr, cc, start_ptr);
                if r.is_some() {
                    return r;
                }
            }
        }
        /*
        {
            let unroll = 2;
            let loop_size = 8;
            while buf_ptr.is_not_under(start_ptr, loop_size * unroll) {
                buf_ptr = unsafe { buf_ptr.sub(loop_size * unroll) };
                let r = _rnechr_c8_aa_x2(buf_ptr, cc, start_ptr);
                if r.is_some() {
                    return r;
                }
            }
        }
        */
        {
            let unroll = 1;
            let loop_size = 8;
            while buf_ptr.is_not_under(start_ptr, loop_size * unroll) {
                buf_ptr = unsafe { buf_ptr.sub(loop_size * unroll) };
                let r = _rnechr_c8_aa_x1(buf_ptr, cc, start_ptr);
                if r.is_some() {
                    return r;
                }
            }
        }
    }
    // the remaining data is the max: 7 bytes.
    _memrnechr_qpl_remaining_7_bytes_impl(buf_ptr, cc.into(), start_ptr)
}

#[cfg(any(target_pointer_width = "32", feature = "test_pointer_width_32"))]
#[inline(always)]
fn _start_rnechr_32(buf: &[u8], needle: B1Qpl) -> Option<usize> {
    let buf_len = buf.len();
    let start_ptr = buf.as_ptr();
    let mut buf_ptr = unsafe { start_ptr.add(buf_len) };
    let cc: B4Qpl = needle.into();
    //
    if buf_len >= 4 {
        // to a aligned pointer
        {
            if !buf_ptr.is_aligned_u32() {
                let r = _rnechr_qpl_to_aligned_u32(buf_ptr, needle, start_ptr);
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
            while buf_ptr.is_not_under(start_ptr, loop_size * unroll) {
                buf_ptr = unsafe { buf_ptr.sub(loop_size * unroll) };
                buf_ptr.prefetch_read_data();
                let r = _rnechr_c4_aa_x8(buf_ptr, cc, start_ptr);
                if r.is_some() {
                    return r;
                }
            }
        }
        */
        {
            let unroll = 4;
            let loop_size = 4;
            while buf_ptr.is_not_under(start_ptr, loop_size * unroll) {
                buf_ptr = unsafe { buf_ptr.sub(loop_size * unroll) };
                let r = _rnechr_c4_aa_x4(buf_ptr, cc, start_ptr);
                if r.is_some() {
                    return r;
                }
            }
        }
        /*
        {
            let unroll = 2;
            let loop_size = 4;
            while buf_ptr.is_not_under(start_ptr, loop_size * unroll) {
                buf_ptr = unsafe { buf_ptr.sub(loop_size * unroll) };
                let r = _rnechr_c4_aa_x2(buf_ptr, cc, start_ptr);
                if r.is_some() {
                    return r;
                }
            }
        }
        */
        {
            let unroll = 1;
            let loop_size = 4;
            while buf_ptr.is_not_under(start_ptr, loop_size * unroll) {
                buf_ptr = unsafe { buf_ptr.sub(loop_size * unroll) };
                let r = _rnechr_c4_aa_x1(buf_ptr, cc, start_ptr);
                if r.is_some() {
                    return r;
                }
            }
        }
    }
    // the remaining data is the max: 3 bytes.
    _memrnechr_qpl_remaining_3_bytes_impl(buf_ptr, cc.into(), start_ptr)
}

#[inline(always)]
pub(crate) fn _memrnechr_qpl_remaining_15_bytes_impl(
    buf_ptr: *const u8,
    cc: B8Qpl,
    start_ptr: *const u8,
) -> Option<usize> {
    let mut buf_ptr = buf_ptr;
    if buf_ptr.is_aligned_u64() {
        let loop_size = 8;
        if buf_ptr.is_not_under(start_ptr, loop_size) {
            buf_ptr = unsafe { buf_ptr.sub(loop_size) };
            let r = _rnechr_c8_aa_x1(buf_ptr, cc, start_ptr);
            if r.is_some() {
                return r;
            }
        }
    }
    // the remaining data is the max: 7 bytes.
    _memrnechr_qpl_remaining_7_bytes_impl(buf_ptr, cc.into(), start_ptr)
}

#[inline(always)]
pub(crate) fn _memrnechr_qpl_remaining_7_bytes_impl(
    buf_ptr: *const u8,
    cc: B4Qpl,
    start_ptr: *const u8,
) -> Option<usize> {
    let mut buf_ptr = buf_ptr;
    if buf_ptr.is_aligned_u32() {
        let loop_size = 4;
        if buf_ptr.is_not_under(start_ptr, loop_size) {
            buf_ptr = unsafe { buf_ptr.sub(loop_size) };
            let r = _rnechr_c4_aa_x1(buf_ptr, cc, start_ptr);
            if r.is_some() {
                return r;
            }
        }
    }
    // the remaining data is the max: 3 bytes.
    _memrnechr_qpl_remaining_3_bytes_impl(buf_ptr, cc.into(), start_ptr)
}

#[inline(always)]
fn _memrnechr_qpl_remaining_3_bytes_impl(
    buf_ptr: *const u8,
    cc: B2Qpl,
    start_ptr: *const u8,
) -> Option<usize> {
    let mut buf_ptr = buf_ptr;
    if buf_ptr.is_aligned_u16() {
        let loop_size = 2;
        if buf_ptr.is_not_under(start_ptr, loop_size) {
            buf_ptr = unsafe { buf_ptr.sub(loop_size) };
            let r = _rnechr_c2_aa_x1(buf_ptr, cc, start_ptr);
            if r.is_some() {
                return r;
            }
        }
    }
    {
        let loop_size = 1;
        while buf_ptr.is_not_under(start_ptr, loop_size) {
            buf_ptr = unsafe { buf_ptr.sub(loop_size) };
            let r = _rnechr_c1_aa_x1(buf_ptr, cc.into(), start_ptr);
            if r.is_some() {
                return r;
            }
        }
    }
    //
    None
}

#[inline(always)]
fn _return_rnechr_qpl<T, PU>(base: T, bits_ab: PU) -> Option<usize>
where
    T: core::ops::Sub<usize, Output = usize>,
    PU: BitOrt + HighBitProp,
{
    if !bits_ab.is_highs() {
        let bits_ab = bits_ab.propagate_a_high_bit();
        let idx1 = (bits_ab.leading_ones() / 8) as usize;
        Some(base - idx1)
    } else {
        None
    }
}

#[inline(always)]
fn _rnechr_c16_uu_x1(buf_ptr: *const u8, c16: B16Qpl, st_ptr: *const u8) -> Option<usize> {
    _rnechr_c16_aa_x1(buf_ptr, c16, st_ptr)
}

#[inline(always)]
fn _rnechr_c16_aa_x1(buf_ptr: *const u8, c16: B16Qpl, st_ptr: *const u8) -> Option<usize> {
    let v_0 = unsafe { _read_a_little_endian_from_ptr_u128(buf_ptr) };
    let v_0_a = v_0 ^ c16.v1;
    let v_0_b = v_0 ^ c16.v2;
    let v_0_c = v_0 ^ c16.v3;
    let v_0_d = v_0 ^ c16.v4;
    let bits_0_a = PackedU128::new(v_0_a).may_have_zero_byte();
    let bits_0_b = PackedU128::new(v_0_b).may_have_zero_byte();
    let bits_0_c = PackedU128::new(v_0_c).may_have_zero_byte();
    let bits_0_d = PackedU128::new(v_0_d).may_have_zero_byte();
    let bits_0_abcd = bits_0_a | bits_0_b | bits_0_c | bits_0_d;
    let base = buf_ptr.usz_offset_from(st_ptr) + 16 - 1;
    //
    _return_rnechr_qpl(base, bits_0_abcd)
}

#[inline(always)]
fn _rnechr_c16_aa_x2(buf_ptr: *const u8, c16: B16Qpl, st_ptr: *const u8) -> Option<usize> {
    let r = _rnechr_c16_aa_x1(unsafe { buf_ptr.add(16) }, c16, st_ptr);
    if r.is_some() {
        return r;
    }
    let r = _rnechr_c16_aa_x1(buf_ptr, c16, st_ptr);
    if r.is_some() {
        return r;
    }
    None
}

#[inline(always)]
fn _rnechr_c16_aa_x4(buf_ptr: *const u8, c16: B16Qpl, st_ptr: *const u8) -> Option<usize> {
    let r = _rnechr_c16_aa_x2(unsafe { buf_ptr.add(16 * 2) }, c16, st_ptr);
    if r.is_some() {
        return r;
    }
    let r = _rnechr_c16_aa_x2(buf_ptr, c16, st_ptr);
    if r.is_some() {
        return r;
    }
    None
}

#[inline(always)]
fn _rnechr_c16_aa_x8(buf_ptr: *const u8, c16: B16Qpl, st_ptr: *const u8) -> Option<usize> {
    let r = _rnechr_c16_aa_x4(unsafe { buf_ptr.add(16 * 4) }, c16, st_ptr);
    if r.is_some() {
        return r;
    }
    let r = _rnechr_c16_aa_x4(buf_ptr, c16, st_ptr);
    if r.is_some() {
        return r;
    }
    None
}

#[inline(always)]
fn _rnechr_c8_uu_x1(buf_ptr: *const u8, c8: B8Qpl, st_ptr: *const u8) -> Option<usize> {
    _rnechr_c8_aa_x1(buf_ptr, c8, st_ptr)
}

#[inline(always)]
fn _rnechr_c8_aa_x1(buf_ptr: *const u8, c8: B8Qpl, st_ptr: *const u8) -> Option<usize> {
    let v_0 = unsafe { _read_a_little_endian_from_ptr_u64(buf_ptr) };
    let v_0_a = v_0 ^ c8.v1;
    let v_0_b = v_0 ^ c8.v2;
    let v_0_c = v_0 ^ c8.v3;
    let v_0_d = v_0 ^ c8.v4;
    let bits_0_a = PackedU64::new(v_0_a).may_have_zero_byte();
    let bits_0_b = PackedU64::new(v_0_b).may_have_zero_byte();
    let bits_0_c = PackedU64::new(v_0_c).may_have_zero_byte();
    let bits_0_d = PackedU64::new(v_0_d).may_have_zero_byte();
    let bits_0_abcd = bits_0_a | bits_0_b | bits_0_c | bits_0_d;
    let base = buf_ptr.usz_offset_from(st_ptr) + 8 - 1;
    //
    _return_rnechr_qpl(base, bits_0_abcd)
}

#[inline(always)]
fn _rnechr_c8_aa_x2(buf_ptr: *const u8, c8: B8Qpl, st_ptr: *const u8) -> Option<usize> {
    let r = _rnechr_c8_aa_x1(unsafe { buf_ptr.add(8) }, c8, st_ptr);
    if r.is_some() {
        return r;
    }
    let r = _rnechr_c8_aa_x1(buf_ptr, c8, st_ptr);
    if r.is_some() {
        return r;
    }
    None
}

#[inline(always)]
fn _rnechr_c8_aa_x4(buf_ptr: *const u8, c8: B8Qpl, st_ptr: *const u8) -> Option<usize> {
    let r = _rnechr_c8_aa_x2(unsafe { buf_ptr.add(8 * 2) }, c8, st_ptr);
    if r.is_some() {
        return r;
    }
    let r = _rnechr_c8_aa_x2(buf_ptr, c8, st_ptr);
    if r.is_some() {
        return r;
    }
    None
}

#[inline(always)]
fn _rnechr_c8_aa_x8(buf_ptr: *const u8, c8: B8Qpl, st_ptr: *const u8) -> Option<usize> {
    let r = _rnechr_c8_aa_x4(unsafe { buf_ptr.add(8 * 4) }, c8, st_ptr);
    if r.is_some() {
        return r;
    }
    let r = _rnechr_c8_aa_x4(buf_ptr, c8, st_ptr);
    if r.is_some() {
        return r;
    }
    None
}

#[inline(always)]
fn _rnechr_c4_uu_x1(buf_ptr: *const u8, c4: B4Qpl, st_ptr: *const u8) -> Option<usize> {
    _rnechr_c4_aa_x1(buf_ptr, c4, st_ptr)
}

#[inline(always)]
fn _rnechr_c4_aa_x1(buf_ptr: *const u8, c4: B4Qpl, st_ptr: *const u8) -> Option<usize> {
    let v_0 = unsafe { _read_a_little_endian_from_ptr_u32(buf_ptr) };
    let v_0_a = v_0 ^ c4.v1;
    let v_0_b = v_0 ^ c4.v2;
    let v_0_c = v_0 ^ c4.v3;
    let v_0_d = v_0 ^ c4.v4;
    let bits_0_a = PackedU32::new(v_0_a).may_have_zero_byte();
    let bits_0_b = PackedU32::new(v_0_b).may_have_zero_byte();
    let bits_0_c = PackedU32::new(v_0_c).may_have_zero_byte();
    let bits_0_d = PackedU32::new(v_0_d).may_have_zero_byte();
    let bits_0_abcd = bits_0_a | bits_0_b | bits_0_c | bits_0_d;
    let base = buf_ptr.usz_offset_from(st_ptr) + 4 - 1;
    //
    _return_rnechr_qpl(base, bits_0_abcd)
}

#[inline(always)]
fn _rnechr_c4_aa_x2(buf_ptr: *const u8, c4: B4Qpl, st_ptr: *const u8) -> Option<usize> {
    let r = _rnechr_c4_aa_x1(unsafe { buf_ptr.add(4) }, c4, st_ptr);
    if r.is_some() {
        return r;
    }
    let r = _rnechr_c4_aa_x1(buf_ptr, c4, st_ptr);
    if r.is_some() {
        return r;
    }
    None
}

#[inline(always)]
fn _rnechr_c4_aa_x4(buf_ptr: *const u8, c4: B4Qpl, st_ptr: *const u8) -> Option<usize> {
    let r = _rnechr_c4_aa_x2(unsafe { buf_ptr.add(4 * 2) }, c4, st_ptr);
    if r.is_some() {
        return r;
    }
    let r = _rnechr_c4_aa_x2(buf_ptr, c4, st_ptr);
    if r.is_some() {
        return r;
    }
    None
}

#[inline(always)]
fn _rnechr_c4_aa_x8(buf_ptr: *const u8, c4: B4Qpl, st_ptr: *const u8) -> Option<usize> {
    let r = _rnechr_c4_aa_x4(unsafe { buf_ptr.add(4 * 4) }, c4, st_ptr);
    if r.is_some() {
        return r;
    }
    let r = _rnechr_c4_aa_x4(buf_ptr, c4, st_ptr);
    if r.is_some() {
        return r;
    }
    None
}

#[inline(always)]
fn _rnechr_c2_aa_x1(buf_ptr: *const u8, c2: B2Qpl, st_ptr: *const u8) -> Option<usize> {
    let v_0 = unsafe { _read_a_little_endian_from_ptr_u16(buf_ptr) };
    let v_0_a = v_0 ^ c2.v1;
    let v_0_b = v_0 ^ c2.v2;
    let v_0_c = v_0 ^ c2.v3;
    let v_0_d = v_0 ^ c2.v4;
    let bits_0_a = PackedU16::new(v_0_a).may_have_zero_byte();
    let bits_0_b = PackedU16::new(v_0_b).may_have_zero_byte();
    let bits_0_c = PackedU16::new(v_0_c).may_have_zero_byte();
    let bits_0_d = PackedU16::new(v_0_d).may_have_zero_byte();
    let bits_0_abcd = bits_0_a | bits_0_b | bits_0_c | bits_0_d;
    let base = buf_ptr.usz_offset_from(st_ptr) + 2 - 1;
    //
    _return_rnechr_qpl(base, bits_0_abcd)
}

#[inline(always)]
fn _rnechr_c1_aa_x1(buf_ptr: *const u8, c1: B1Qpl, st_ptr: *const u8) -> Option<usize> {
    let aa_ptr = buf_ptr as *const u8;
    let aac = unsafe { *aa_ptr };
    if aac != c1.v1 && aac != c1.v2 && aac != c1.v3 && aac != c1.v4 {
        Some(buf_ptr.usz_offset_from(st_ptr))
    } else {
        None
    }
}

/*
 * The simple implement:

#[inline(always)]
pub fn _memrnechr_qpl_impl(buf: &[u8], needle: B1Qpl) -> Option<usize> {
    for i in 0..buf.len() {
        let j = buf.len() - i - 1;
        if buf[j] != needle.v1 && buf[j] != needle.v2 && buf[j] != needle.v3 && buf[i] != needle.v4 {
            return Some(j);
        }
    }
    None
}
*/
