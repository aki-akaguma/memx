use crate::utils::*;

#[inline(never)]
pub fn _memrchr_sgl_impl(buf: &[u8], needle: B1Sgl) -> Option<usize> {
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
        let r = _start_rchr_sgl_128(buf, needle);
        #[cfg(feature = "test_pointer_width_64")]
        let r = _start_rchr_sgl_64(buf, needle);
        #[cfg(feature = "test_pointer_width_32")]
        let r = _start_rchr_sgl_32(buf, needle);
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
        let r = _start_rchr_sgl_128(buf, needle);
        #[cfg(target_pointer_width = "64")]
        let r = _start_rchr_sgl_64(buf, needle);
        #[cfg(target_pointer_width = "32")]
        let r = _start_rchr_sgl_32(buf, needle);
        //
        r
    }
}

macro_rules! _unroll_one_rchr_sgl_to_aligned_x1 {
    ($buf_ptr_2:expr, $buf_ptr_min:expr, $c:expr, $start_ptr:expr) => {{
        if $buf_ptr_2 <= $buf_ptr_min {
            break;
        }
        $buf_ptr_2 = unsafe { $buf_ptr_2.sub(1) };
        let r = _rchr_sgl_c1_aa_x1($buf_ptr_2, $c, $start_ptr);
        if r.is_some() {
            return (None, r);
        }
    }};
}

macro_rules! _unroll_one_rchr_sgl_to_aligned_x4 {
    ($buf_ptr_2:expr, $buf_ptr_min:expr, $c:expr, $start_ptr:expr) => {{
        _unroll_one_rchr_sgl_to_aligned_x1!($buf_ptr_2, $buf_ptr_min, $c, $start_ptr);
        _unroll_one_rchr_sgl_to_aligned_x1!($buf_ptr_2, $buf_ptr_min, $c, $start_ptr);
        _unroll_one_rchr_sgl_to_aligned_x1!($buf_ptr_2, $buf_ptr_min, $c, $start_ptr);
        _unroll_one_rchr_sgl_to_aligned_x1!($buf_ptr_2, $buf_ptr_min, $c, $start_ptr);
    }};
}

macro_rules! _unroll_one_rchr_sgl_to_aligned_x8 {
    ($buf_ptr_2:expr, $buf_ptr_min:expr, $c:expr, $start_ptr:expr) => {{
        _unroll_one_rchr_sgl_to_aligned_x4!($buf_ptr_2, $buf_ptr_min, $c, $start_ptr);
        _unroll_one_rchr_sgl_to_aligned_x4!($buf_ptr_2, $buf_ptr_min, $c, $start_ptr);
    }};
}

macro_rules! _unroll_one_rchr_sgl_to_aligned_x16 {
    ($buf_ptr_2:expr, $buf_ptr_min:expr, $c:expr, $start_ptr:expr) => {{
        _unroll_one_rchr_sgl_to_aligned_x8!($buf_ptr_2, $buf_ptr_min, $c, $start_ptr);
        _unroll_one_rchr_sgl_to_aligned_x8!($buf_ptr_2, $buf_ptr_min, $c, $start_ptr);
    }};
}

#[inline(always)]
pub(crate) fn _rchr_sgl_to_aligned_u256(
    buf_ptr_cur: *const u8,
    c: B1Sgl,
    st_ptr: *const u8,
) -> (Option<*const u8>, Option<usize>) {
    let buf_ptr_end = buf_ptr_cur;
    let remaining_align = (buf_ptr_end as usize) & 0x1F_usize;
    let buf_ptr_min = unsafe { buf_ptr_end.sub(remaining_align) };
    let mut buf_ptr_2 = buf_ptr_end;
    loop {
        _unroll_one_rchr_sgl_to_aligned_x16!(buf_ptr_2, buf_ptr_min, c, st_ptr);
        _unroll_one_rchr_sgl_to_aligned_x16!(buf_ptr_2, buf_ptr_min, c, st_ptr);
    }
    (Some(buf_ptr_min), None)
}

#[inline(always)]
pub(crate) fn _rchr_sgl_to_aligned_u128(
    buf_ptr_cur: *const u8,
    c: B1Sgl,
    st_ptr: *const u8,
) -> (Option<*const u8>, Option<usize>) {
    let buf_ptr_end = buf_ptr_cur;
    let remaining_align = (buf_ptr_end as usize) & 0x0F_usize;
    let buf_ptr_min = unsafe { buf_ptr_end.sub(remaining_align) };
    let mut buf_ptr_2 = buf_ptr_end;
    loop {
        _unroll_one_rchr_sgl_to_aligned_x16!(buf_ptr_2, buf_ptr_min, c, st_ptr);
    }
    (Some(buf_ptr_min), None)
}

#[inline(always)]
fn _rchr_sgl_to_aligned_u64(
    buf_ptr_cur: *const u8,
    c: B1Sgl,
    st_ptr: *const u8,
) -> (Option<*const u8>, Option<usize>) {
    let buf_ptr_end = buf_ptr_cur;
    let remaining_align = (buf_ptr_end as usize) & 0x07_usize;
    let buf_ptr_min = unsafe { buf_ptr_end.sub(remaining_align) };
    let mut buf_ptr_2 = buf_ptr_end;
    loop {
        _unroll_one_rchr_sgl_to_aligned_x8!(buf_ptr_2, buf_ptr_min, c, st_ptr);
    }
    (Some(buf_ptr_min), None)
}

#[inline(always)]
fn _rchr_sgl_to_aligned_u32(
    buf_ptr_cur: *const u8,
    c: B1Sgl,
    st_ptr: *const u8,
) -> (Option<*const u8>, Option<usize>) {
    let buf_ptr_end = buf_ptr_cur;
    let remaining_align = (buf_ptr_end as usize) & 0x03_usize;
    let buf_ptr_min = unsafe { buf_ptr_end.sub(remaining_align) };
    let mut buf_ptr_2 = buf_ptr_end;
    loop {
        _unroll_one_rchr_sgl_to_aligned_x4!(buf_ptr_2, buf_ptr_min, c, st_ptr);
    }
    (Some(buf_ptr_min), None)
}

#[cfg(any(target_pointer_width = "128", feature = "test_pointer_width_128"))]
#[inline(always)]
fn _start_rchr_sgl_128(buf: &[u8], needle: B1Sgl) -> Option<usize> {
    let buf_len = buf.len();
    let start_ptr = buf.as_ptr();
    let mut buf_ptr = unsafe { start_ptr.add(buf_len) };
    let cc: B16Sgl = needle.into();
    //
    if buf_len >= 16 {
        // to a aligned pointer
        {
            if !buf_ptr.is_aligned_u128() {
                let r = _rchr_sgl_to_aligned_u128(buf_ptr, needle, start_ptr);
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
                let r = _rchr_sgl_c16_aa_x8(buf_ptr, cc, start_ptr);
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
                let r = _rchr_sgl_c16_aa_x4(buf_ptr, cc, start_ptr);
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
                let r = _rchr_sgl_c16_aa_x2(buf_ptr, cc, start_ptr);
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
                let r = _rchr_sgl_c16_aa_x1(buf_ptr, cc, start_ptr);
                if r.is_some() {
                    return r;
                }
            }
        }
    }
    // the remaining data is the max: 15 bytes.
    _memrchr_remaining_15_bytes_impl(buf_ptr, cc.into(), start_ptr)
}

#[cfg(any(target_pointer_width = "64", feature = "test_pointer_width_64"))]
#[inline(always)]
fn _start_rchr_sgl_64(buf: &[u8], needle: B1Sgl) -> Option<usize> {
    let buf_len = buf.len();
    let start_ptr = buf.as_ptr();
    let mut buf_ptr = unsafe { start_ptr.add(buf_len) };
    let cc: B8Sgl = needle.into();
    //
    if buf_len >= 8 {
        // to a aligned pointer
        {
            if !buf_ptr.is_aligned_u64() {
                let r = _rchr_sgl_to_aligned_u64(buf_ptr, needle, start_ptr);
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
            while buf_ptr.is_not_under(start_ptr, loop_size * unroll) {
                buf_ptr = unsafe { buf_ptr.sub(loop_size * unroll) };
                let r = _rchr_sgl_c8_aa_x8(buf_ptr, cc, start_ptr);
                if r.is_some() {
                    return r;
                }
            }
        }
        /*
        {
            let unroll = 4;
            let loop_size = 8;
            while buf_ptr.is_not_under(start_ptr, loop_size * unroll) {
                buf_ptr = unsafe { buf_ptr.sub(loop_size * unroll) };
                buf_ptr.prefetch_read_data();
                let r = _rchr_sgl_c8_aa_x4(buf_ptr, cc, start_ptr);
                if r.is_some() {
                    return r;
                }
            }
        }
        {
            let unroll = 2;
            let loop_size = 8;
            while buf_ptr.is_not_under(start_ptr, loop_size * unroll) {
                buf_ptr = unsafe { buf_ptr.sub(loop_size * unroll) };
                buf_ptr.prefetch_read_data();
                let r = _rchr_sgl_c8_aa_x2(buf_ptr, cc, start_ptr);
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
                let r = _rchr_sgl_c8_aa_x1(buf_ptr, cc, start_ptr);
                if r.is_some() {
                    return r;
                }
            }
        }
    }
    // the remaining data is the max: 7 bytes.
    _memrchr_remaining_7_bytes_impl(buf_ptr, cc.into(), start_ptr)
}

#[cfg(any(target_pointer_width = "32", feature = "test_pointer_width_32"))]
#[inline(always)]
fn _start_rchr_sgl_32(buf: &[u8], needle: B1Sgl) -> Option<usize> {
    let buf_len = buf.len();
    let start_ptr = buf.as_ptr();
    let mut buf_ptr = unsafe { start_ptr.add(buf_len) };
    let cc: B4Sgl = needle.into();
    //
    if buf_len >= 4 {
        // to a aligned pointer
        {
            if !buf_ptr.is_aligned_u32() {
                let r = _rchr_sgl_to_aligned_u32(buf_ptr, needle, start_ptr);
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
            while buf_ptr.is_not_under(start_ptr, loop_size * unroll) {
                buf_ptr = unsafe { buf_ptr.sub(loop_size * unroll) };
                let r = _rchr_sgl_c4_aa_x8(buf_ptr, cc, start_ptr);
                if r.is_some() {
                    return r;
                }
            }
        }
        /*
        {
            let unroll = 4;
            let loop_size = 4;
            while buf_ptr.is_not_under(start_ptr, loop_size * unroll) {
                buf_ptr = unsafe { buf_ptr.sub(loop_size * unroll) };
                let r = _rchr_sgl_c4_aa_x4(buf_ptr, cc, start_ptr);
                if r.is_some() {
                    return r;
                }
            }
        }
        {
            let unroll = 2;
            let loop_size = 4;
            while buf_ptr.is_not_under(start_ptr, loop_size * unroll) {
                buf_ptr = unsafe { buf_ptr.sub(loop_size * unroll) };
                let r = _rchr_sgl_c4_aa_x2(buf_ptr, cc, start_ptr);
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
                let r = _rchr_sgl_c4_aa_x1(buf_ptr, cc, start_ptr);
                if r.is_some() {
                    return r;
                }
            }
        }
    }
    // the remaining data is the max: 3 bytes.
    _memrchr_remaining_3_bytes_impl(buf_ptr, cc.into(), start_ptr)
}

#[inline(always)]
pub(crate) fn _memrchr_remaining_15_bytes_impl(
    buf_ptr: *const u8,
    cc: B8Sgl,
    start_ptr: *const u8,
) -> Option<usize> {
    let mut buf_ptr = buf_ptr;
    if buf_ptr.is_aligned_u64() {
        let loop_size = 8;
        if buf_ptr.is_not_under(start_ptr, loop_size) {
            buf_ptr = unsafe { buf_ptr.sub(loop_size) };
            let r = _rchr_sgl_c8_aa_x1(buf_ptr, cc, start_ptr);
            if r.is_some() {
                return r;
            }
        }
    }
    // the remaining data is the max: 7 bytes.
    _memrchr_remaining_7_bytes_impl(buf_ptr, cc.into(), start_ptr)
}

#[inline(always)]
pub(crate) fn _memrchr_remaining_7_bytes_impl(
    buf_ptr: *const u8,
    cc: B4Sgl,
    start_ptr: *const u8,
) -> Option<usize> {
    let mut buf_ptr = buf_ptr;
    if buf_ptr.is_aligned_u32() {
        let loop_size = 4;
        if buf_ptr.is_not_under(start_ptr, loop_size) {
            buf_ptr = unsafe { buf_ptr.sub(loop_size) };
            let r = _rchr_sgl_c4_aa_x1(buf_ptr, cc, start_ptr);
            if r.is_some() {
                return r;
            }
        }
    }
    // the remaining data is the max: 3 bytes.
    _memrchr_remaining_3_bytes_impl(buf_ptr, cc.into(), start_ptr)
}

#[inline(always)]
pub(crate) fn _memrchr_remaining_3_bytes_impl(
    buf_ptr: *const u8,
    cc: B2Sgl,
    start_ptr: *const u8,
) -> Option<usize> {
    let mut buf_ptr = buf_ptr;
    if buf_ptr.is_aligned_u16() {
        let loop_size = 2;
        if buf_ptr.is_not_under(start_ptr, loop_size) {
            buf_ptr = unsafe { buf_ptr.sub(loop_size) };
            let r = _rchr_sgl_c2_aa_x1(buf_ptr, cc, start_ptr);
            if r.is_some() {
                return r;
            }
        }
    }
    {
        let loop_size = 1;
        while buf_ptr.is_not_under(start_ptr, loop_size) {
            buf_ptr = unsafe { buf_ptr.sub(loop_size) };
            let r = _rchr_sgl_c1_aa_x1(buf_ptr, cc.into(), start_ptr);
            if r.is_some() {
                return r;
            }
        }
    }
    None
}

#[inline(always)]
fn _return_rchr_sgl<T, PU>(base: T, bits_a: PU) -> Option<usize>
where
    T: core::ops::Sub<usize, Output = usize>,
    PU: BitOrt,
{
    if !bits_a.is_zeros() {
        let idx1 = (bits_a.trailing_zeros() / 8) as usize;
        Some(base - idx1)
    } else {
        None
    }
}

#[inline(always)]
pub(crate) fn _rchr_sgl_c16_uu_x1(
    buf_ptr: *const u8,
    c16: B16Sgl,
    st_ptr: *const u8,
) -> Option<usize> {
    _rchr_sgl_c16_aa_x1(buf_ptr, c16, st_ptr)
}

#[inline(always)]
pub(crate) fn _rchr_sgl_c16_aa_x1(
    buf_ptr: *const u8,
    c16: B16Sgl,
    st_ptr: *const u8,
) -> Option<usize> {
    let v_0 = unsafe { _read_a_big_endian_from_ptr_u128(buf_ptr) };
    let v_0_a = v_0 ^ c16.v1;
    let bits_0_a = PackedU128::new(v_0_a).may_have_zero_quick();
    let base = buf_ptr.usz_offset_from(st_ptr) + 16 - 1;
    //
    _return_rchr_sgl(base, bits_0_a)
}

#[inline(always)]
fn _rchr_sgl_c16_aa_x2(buf_ptr: *const u8, c16: B16Sgl, st_ptr: *const u8) -> Option<usize> {
    let r = _rchr_sgl_c16_aa_x1(unsafe { buf_ptr.add(16) }, c16, st_ptr);
    if r.is_some() {
        return r;
    }
    let r = _rchr_sgl_c16_aa_x1(buf_ptr, c16, st_ptr);
    if r.is_some() {
        return r;
    }
    None
}

#[inline(always)]
fn _rchr_sgl_c16_aa_x4(buf_ptr: *const u8, c16: B16Sgl, st_ptr: *const u8) -> Option<usize> {
    let r = _rchr_sgl_c16_aa_x2(unsafe { buf_ptr.add(16 * 2) }, c16, st_ptr);
    if r.is_some() {
        return r;
    }
    let r = _rchr_sgl_c16_aa_x2(buf_ptr, c16, st_ptr);
    if r.is_some() {
        return r;
    }
    None
}

#[inline(always)]
fn _rchr_sgl_c16_aa_x8(buf_ptr: *const u8, c16: B16Sgl, st_ptr: *const u8) -> Option<usize> {
    let r = _rchr_sgl_c16_aa_x4(unsafe { buf_ptr.add(16 * 4) }, c16, st_ptr);
    if r.is_some() {
        return r;
    }
    let r = _rchr_sgl_c16_aa_x4(buf_ptr, c16, st_ptr);
    if r.is_some() {
        return r;
    }
    None
}

#[inline(always)]
fn _rchr_sgl_c8_uu_x1(buf_ptr: *const u8, c8: B8Sgl, st_ptr: *const u8) -> Option<usize> {
    _rchr_sgl_c8_aa_x1(buf_ptr, c8, st_ptr)
}

#[inline(always)]
fn _rchr_sgl_c8_aa_x1(buf_ptr: *const u8, c8: B8Sgl, st_ptr: *const u8) -> Option<usize> {
    let v_0 = unsafe { _read_a_big_endian_from_ptr_u64(buf_ptr) };
    let v_0_a = v_0 ^ c8.v1;
    let bits_0_a = PackedU64::new(v_0_a).may_have_zero_quick();
    let base = buf_ptr.usz_offset_from(st_ptr) + 8 - 1;
    //
    _return_rchr_sgl(base, bits_0_a)
}

#[inline(always)]
fn _rchr_sgl_c8_aa_x2(buf_ptr: *const u8, c8: B8Sgl, st_ptr: *const u8) -> Option<usize> {
    let r = _rchr_sgl_c8_aa_x1(unsafe { buf_ptr.add(8) }, c8, st_ptr);
    if r.is_some() {
        return r;
    }
    let r = _rchr_sgl_c8_aa_x1(buf_ptr, c8, st_ptr);
    if r.is_some() {
        return r;
    }
    None
}

#[inline(always)]
fn _rchr_sgl_c8_aa_x4(buf_ptr: *const u8, c8: B8Sgl, st_ptr: *const u8) -> Option<usize> {
    let r = _rchr_sgl_c8_aa_x2(unsafe { buf_ptr.add(8 * 2) }, c8, st_ptr);
    if r.is_some() {
        return r;
    }
    let r = _rchr_sgl_c8_aa_x2(buf_ptr, c8, st_ptr);
    if r.is_some() {
        return r;
    }
    None
}

#[inline(always)]
fn _rchr_sgl_c8_aa_x8(buf_ptr: *const u8, c8: B8Sgl, st_ptr: *const u8) -> Option<usize> {
    let r = _rchr_sgl_c8_aa_x4(unsafe { buf_ptr.add(8 * 4) }, c8, st_ptr);
    if r.is_some() {
        return r;
    }
    let r = _rchr_sgl_c8_aa_x4(buf_ptr, c8, st_ptr);
    if r.is_some() {
        return r;
    }
    None
}

#[inline(always)]
fn _rchr_sgl_c4_uu_x1(buf_ptr: *const u8, c4: B4Sgl, st_ptr: *const u8) -> Option<usize> {
    _rchr_sgl_c4_aa_x1(buf_ptr, c4, st_ptr)
}

#[inline(always)]
fn _rchr_sgl_c4_aa_x1(buf_ptr: *const u8, c4: B4Sgl, st_ptr: *const u8) -> Option<usize> {
    let v_0 = unsafe { _read_a_big_endian_from_ptr_u32(buf_ptr) };
    let v_0_a = v_0 ^ c4.v1;
    let bits_0_a = PackedU32::new(v_0_a).may_have_zero_quick();
    let base = buf_ptr.usz_offset_from(st_ptr) + 4 - 1;
    //
    _return_rchr_sgl(base, bits_0_a)
}

#[inline(always)]
fn _rchr_sgl_c4_aa_x2(buf_ptr: *const u8, c4: B4Sgl, st_ptr: *const u8) -> Option<usize> {
    let r = _rchr_sgl_c4_aa_x1(unsafe { buf_ptr.add(4) }, c4, st_ptr);
    if r.is_some() {
        return r;
    }
    let r = _rchr_sgl_c4_aa_x1(buf_ptr, c4, st_ptr);
    if r.is_some() {
        return r;
    }
    None
}

#[inline(always)]
fn _rchr_sgl_c4_aa_x4(buf_ptr: *const u8, c4: B4Sgl, st_ptr: *const u8) -> Option<usize> {
    let r = _rchr_sgl_c4_aa_x2(unsafe { buf_ptr.add(4 * 2) }, c4, st_ptr);
    if r.is_some() {
        return r;
    }
    let r = _rchr_sgl_c4_aa_x2(buf_ptr, c4, st_ptr);
    if r.is_some() {
        return r;
    }
    None
}

#[inline(always)]
fn _rchr_sgl_c4_aa_x8(buf_ptr: *const u8, c4: B4Sgl, st_ptr: *const u8) -> Option<usize> {
    let r = _rchr_sgl_c4_aa_x4(unsafe { buf_ptr.add(4 * 4) }, c4, st_ptr);
    if r.is_some() {
        return r;
    }
    let r = _rchr_sgl_c4_aa_x4(buf_ptr, c4, st_ptr);
    if r.is_some() {
        return r;
    }
    None
}

#[inline(always)]
fn _rchr_sgl_c2_aa_x1(buf_ptr: *const u8, c2: B2Sgl, st_ptr: *const u8) -> Option<usize> {
    let v_0 = unsafe { _read_a_big_endian_from_ptr_u16(buf_ptr) };
    let v_0_a = v_0 ^ c2.v1;
    let bits_0_a = PackedU16::new(v_0_a).may_have_zero_quick();
    let base = buf_ptr.usz_offset_from(st_ptr) + 2 - 1;
    //
    _return_rchr_sgl(base, bits_0_a)
}

#[inline(always)]
fn _rchr_sgl_c1_aa_x1(buf_ptr: *const u8, c1: B1Sgl, st_ptr: *const u8) -> Option<usize> {
    let aa_ptr = buf_ptr as *const u8;
    let aac = unsafe { *aa_ptr };
    if aac == c1.v1 {
        Some(buf_ptr.usz_offset_from(st_ptr))
    } else {
        None
    }
}

/*
 * The simple implement:

#[inline(always)]
pub fn _memrchr_impl(buf: &[u8], c: u8) -> Option<usize> {
    for i in 0..buf.len() {
        let j = buf.len() - i - 1;
        if buf[j] == c {
            return Some(j);
        }
    }
    None
}
*/
