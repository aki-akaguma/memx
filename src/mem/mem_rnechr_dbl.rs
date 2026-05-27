use crate::utils::_read_a_little_endian_from_ptr_u128;
use crate::utils::_read_a_little_endian_from_ptr_u16;
use crate::utils::_read_a_little_endian_from_ptr_u32;
use crate::utils::_read_a_little_endian_from_ptr_u64;
use crate::utils::_unroll_loop_backward;
use crate::utils::_unroll_loop_backward_with_prefetch;
use crate::utils::B16Dbl;
use crate::utils::B1Dbl;
use crate::utils::B2Dbl;
use crate::utils::B4Dbl;
use crate::utils::B8Dbl;
use crate::utils::BitOrt;
use crate::utils::HighBitProp;
use crate::utils::PackedU128;
use crate::utils::PackedU16;
use crate::utils::PackedU32;
use crate::utils::PackedU64;
use crate::utils::PtrOps;
use crate::utils::PtrOpsPrefetch;

#[inline(never)]
pub fn _memrnechr_dbl_impl(buf: &[u8], needle: B1Dbl) -> Option<usize> {
    #[cfg(any(target_pointer_width = "64", feature = "test_pointer_width_64"))]
    let r = _start_rnechr_dbl_64(buf, needle);
    #[cfg(all(
        not(any(target_pointer_width = "64", feature = "test_pointer_width_64")),
        any(target_pointer_width = "32", feature = "test_pointer_width_32")
    ))]
    let r = _start_rnechr_dbl_32(buf, needle);
    //
    r
}

#[cfg(any(target_pointer_width = "64", feature = "test_pointer_width_64"))]
#[inline(always)]
fn _start_rnechr_dbl_64(buf: &[u8], needle: B1Dbl) -> Option<usize> {
    let buf_len = buf.len();
    let start_ptr = buf.as_ptr();
    let mut buf_ptr = unsafe { start_ptr.add(buf_len) };
    let cc: B8Dbl = needle.into();
    buf_ptr.prefetch_read_data();
    //
    if buf_len >= 8 {
        // to a aligned pointer
        {
            if !buf_ptr.is_aligned_u64() {
                let r = _rnechr_dbl_to_aligned_u64(buf_ptr, needle, start_ptr);
                if let Some(p) = r.0 {
                    buf_ptr = p;
                } else if let Some(v) = r.1 {
                    return Some(v);
                }
            }
        }
        // the loop
        {
            let (r, p) = _unroll_loop_backward_with_prefetch::<4, 8, _>(buf_ptr, start_ptr, |p| {
                _rnechr_dbl_c8_aa_x1(p, cc, start_ptr)
            });
            if r.is_some() {
                return r;
            }
            buf_ptr = p;
        }
        {
            let (r, p) = _unroll_loop_backward::<1, 8, _>(buf_ptr, start_ptr, |p| {
                _rnechr_dbl_c8_aa_x1(p, cc, start_ptr)
            });
            if r.is_some() {
                return r;
            }
            buf_ptr = p;
        }
    }
    // the remaining data is the max: 7 bytes.
    _memrnechr_dbl_remaining_7_bytes_impl(buf_ptr, cc.into(), start_ptr)
}

#[cfg(any(target_pointer_width = "32", feature = "test_pointer_width_32"))]
#[inline(always)]
fn _start_rnechr_dbl_32(buf: &[u8], needle: B1Dbl) -> Option<usize> {
    let buf_len = buf.len();
    let start_ptr = buf.as_ptr();
    let mut buf_ptr = unsafe { start_ptr.add(buf_len) };
    let cc: B4Dbl = needle.into();
    buf_ptr.prefetch_read_data();
    //
    if buf_len >= 4 {
        // to a aligned pointer
        {
            if !buf_ptr.is_aligned_u32() {
                let r = _rnechr_dbl_to_aligned_u32(buf_ptr, needle, start_ptr);
                if let Some(p) = r.0 {
                    buf_ptr = p;
                } else if let Some(v) = r.1 {
                    return Some(v);
                }
            }
        }
        // the loop
        {
            let (r, p) = _unroll_loop_backward_with_prefetch::<4, 4, _>(buf_ptr, start_ptr, |p| {
                _rnechr_dbl_c4_aa_x1(p, cc, start_ptr)
            });
            if r.is_some() {
                return r;
            }
            buf_ptr = p;
        }
        {
            let (r, p) = _unroll_loop_backward::<1, 4, _>(buf_ptr, start_ptr, |p| {
                _rnechr_dbl_c4_aa_x1(p, cc, start_ptr)
            });
            if r.is_some() {
                return r;
            }
            buf_ptr = p;
        }
    }
    // the remaining data is the max: 3 bytes.
    _memrnechr_dbl_remaining_3_bytes_impl(buf_ptr, cc.into(), start_ptr)
}

#[inline(always)]
pub(crate) fn _memrnechr_dbl_remaining_15_bytes_impl(
    buf_ptr: *const u8,
    cc: B8Dbl,
    start_ptr: *const u8,
) -> Option<usize> {
    let mut buf_ptr = buf_ptr;
    if buf_ptr.is_aligned_u64() {
        let loop_size = 8;
        if buf_ptr.is_not_under(start_ptr, loop_size) {
            buf_ptr = unsafe { buf_ptr.sub(loop_size) };
            let r = _rnechr_dbl_c8_aa_x1(buf_ptr, cc, start_ptr);
            if r.is_some() {
                return r;
            }
        }
    }
    // the remaining data is the max: 7 bytes.
    _memrnechr_dbl_remaining_7_bytes_impl(buf_ptr, cc.into(), start_ptr)
}

#[inline(always)]
pub(crate) fn _memrnechr_dbl_remaining_7_bytes_impl(
    buf_ptr: *const u8,
    cc: B4Dbl,
    start_ptr: *const u8,
) -> Option<usize> {
    let mut buf_ptr = buf_ptr;
    if buf_ptr.is_aligned_u32() {
        let loop_size = 4;
        if buf_ptr.is_not_under(start_ptr, loop_size) {
            buf_ptr = unsafe { buf_ptr.sub(loop_size) };
            let r = _rnechr_dbl_c4_aa_x1(buf_ptr, cc, start_ptr);
            if r.is_some() {
                return r;
            }
        }
    }
    // the remaining data is the max: 3 bytes.
    _memrnechr_dbl_remaining_3_bytes_impl(buf_ptr, cc.into(), start_ptr)
}

#[inline(always)]
fn _memrnechr_dbl_remaining_3_bytes_impl(
    buf_ptr: *const u8,
    cc: B2Dbl,
    start_ptr: *const u8,
) -> Option<usize> {
    let mut buf_ptr = buf_ptr;
    if buf_ptr.is_aligned_u16() {
        let loop_size = 2;
        if buf_ptr.is_not_under(start_ptr, loop_size) {
            buf_ptr = unsafe { buf_ptr.sub(loop_size) };
            let r = _rnechr_dbl_c2_aa_x1(buf_ptr, cc, start_ptr);
            if r.is_some() {
                return r;
            }
        }
    }
    {
        let loop_size = 1;
        while buf_ptr.is_not_under(start_ptr, loop_size) {
            buf_ptr = unsafe { buf_ptr.sub(loop_size) };
            let r = _rnechr_dbl_c1_aa_x1(buf_ptr, cc.into(), start_ptr);
            if r.is_some() {
                return r;
            }
        }
    }
    //
    None
}

#[inline(always)]
fn _return_rnechr_dbl<T, PU>(base: T, bits_ab: PU) -> Option<usize>
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
fn _rnechr_dbl_c16_uu_x1(buf_ptr: *const u8, c16: B16Dbl, st_ptr: *const u8) -> Option<usize> {
    _rnechr_dbl_c16_aa_x1(buf_ptr, c16, st_ptr)
}

#[inline(always)]
fn _rnechr_dbl_c16_aa_x1(buf_ptr: *const u8, c16: B16Dbl, st_ptr: *const u8) -> Option<usize> {
    let v_0 = unsafe { _read_a_little_endian_from_ptr_u128(buf_ptr) };
    let v_0_a = v_0 ^ c16.v1;
    let v_0_b = v_0 ^ c16.v2;
    let bits_0_a = PackedU128::new(v_0_a).may_have_zero_byte();
    let bits_0_b = PackedU128::new(v_0_b).may_have_zero_byte();
    let bits_0_ab = bits_0_a | bits_0_b;
    let base = buf_ptr.usz_offset_from(st_ptr) + 16 - 1;
    //
    _return_rnechr_dbl(base, bits_0_ab)
}

#[inline(always)]
fn _rnechr_dbl_c16_aa_x2(buf_ptr: *const u8, c16: B16Dbl, st_ptr: *const u8) -> Option<usize> {
    let (r, _) = _unroll_loop_backward::<2, 16, _>(unsafe { buf_ptr.add(16 * 2) }, buf_ptr, |p| {
        _rnechr_dbl_c16_aa_x1(p, c16, st_ptr)
    });
    r
}

#[inline(always)]
fn _rnechr_dbl_c16_aa_x4(buf_ptr: *const u8, c16: B16Dbl, st_ptr: *const u8) -> Option<usize> {
    let (r, _) = _unroll_loop_backward::<4, 16, _>(unsafe { buf_ptr.add(16 * 4) }, buf_ptr, |p| {
        _rnechr_dbl_c16_aa_x1(p, c16, st_ptr)
    });
    r
}

#[inline(always)]
fn _rnechr_dbl_c16_aa_x8(buf_ptr: *const u8, c16: B16Dbl, st_ptr: *const u8) -> Option<usize> {
    let (r, _) = _unroll_loop_backward::<8, 16, _>(unsafe { buf_ptr.add(16 * 8) }, buf_ptr, |p| {
        _rnechr_dbl_c16_aa_x1(p, c16, st_ptr)
    });
    r
}

#[inline(always)]
fn _rnechr_dbl_c8_uu_x1(buf_ptr: *const u8, c8: B8Dbl, st_ptr: *const u8) -> Option<usize> {
    _rnechr_dbl_c8_aa_x1(buf_ptr, c8, st_ptr)
}

#[inline(always)]
fn _rnechr_dbl_c8_aa_x1(buf_ptr: *const u8, c8: B8Dbl, st_ptr: *const u8) -> Option<usize> {
    let v_0 = unsafe { _read_a_little_endian_from_ptr_u64(buf_ptr) };
    let v_0_a = v_0 ^ c8.v1;
    let v_0_b = v_0 ^ c8.v2;
    let bits_0_a = PackedU64::new(v_0_a).may_have_zero_byte();
    let bits_0_b = PackedU64::new(v_0_b).may_have_zero_byte();
    let bits_0_ab = bits_0_a | bits_0_b;
    let base = buf_ptr.usz_offset_from(st_ptr) + 8 - 1;
    //
    _return_rnechr_dbl(base, bits_0_ab)
}

#[inline(always)]
fn _rnechr_dbl_c8_aa_x2(buf_ptr: *const u8, c8: B8Dbl, st_ptr: *const u8) -> Option<usize> {
    let (r, _) = _unroll_loop_backward::<2, 8, _>(unsafe { buf_ptr.add(8 * 2) }, buf_ptr, |p| {
        _rnechr_dbl_c8_aa_x1(p, c8, st_ptr)
    });
    r
}

#[inline(always)]
fn _rnechr_dbl_c8_aa_x4(buf_ptr: *const u8, c8: B8Dbl, st_ptr: *const u8) -> Option<usize> {
    let (r, _) = _unroll_loop_backward::<4, 8, _>(unsafe { buf_ptr.add(8 * 4) }, buf_ptr, |p| {
        _rnechr_dbl_c8_aa_x1(p, c8, st_ptr)
    });
    r
}

#[inline(always)]
fn _rnechr_dbl_c8_aa_x8(buf_ptr: *const u8, c8: B8Dbl, st_ptr: *const u8) -> Option<usize> {
    let (r, _) = _unroll_loop_backward::<8, 8, _>(unsafe { buf_ptr.add(8 * 8) }, buf_ptr, |p| {
        _rnechr_dbl_c8_aa_x1(p, c8, st_ptr)
    });
    r
}

#[inline(always)]
fn _rnechr_dbl_c4_uu_x1(buf_ptr: *const u8, c4: B4Dbl, st_ptr: *const u8) -> Option<usize> {
    _rnechr_dbl_c4_aa_x1(buf_ptr, c4, st_ptr)
}

#[inline(always)]
fn _rnechr_dbl_c4_aa_x1(buf_ptr: *const u8, c4: B4Dbl, st_ptr: *const u8) -> Option<usize> {
    let v_0 = unsafe { _read_a_little_endian_from_ptr_u32(buf_ptr) };
    let v_0_a = v_0 ^ c4.v1;
    let v_0_b = v_0 ^ c4.v2;
    let bits_0_a = PackedU32::new(v_0_a).may_have_zero_byte();
    let bits_0_b = PackedU32::new(v_0_b).may_have_zero_byte();
    let bits_0_ab = bits_0_a | bits_0_b;
    let base = buf_ptr.usz_offset_from(st_ptr) + 4 - 1;
    //
    _return_rnechr_dbl(base, bits_0_ab)
}

#[inline(always)]
fn _rnechr_dbl_c4_aa_x2(buf_ptr: *const u8, c4: B4Dbl, st_ptr: *const u8) -> Option<usize> {
    let (r, _) = _unroll_loop_backward::<2, 4, _>(unsafe { buf_ptr.add(4 * 2) }, buf_ptr, |p| {
        _rnechr_dbl_c4_aa_x1(p, c4, st_ptr)
    });
    r
}

#[inline(always)]
fn _rnechr_dbl_c4_aa_x4(buf_ptr: *const u8, c4: B4Dbl, st_ptr: *const u8) -> Option<usize> {
    let (r, _) = _unroll_loop_backward::<4, 4, _>(unsafe { buf_ptr.add(4 * 4) }, buf_ptr, |p| {
        _rnechr_dbl_c4_aa_x1(p, c4, st_ptr)
    });
    r
}

#[inline(always)]
fn _rnechr_dbl_c4_aa_x8(buf_ptr: *const u8, c4: B4Dbl, st_ptr: *const u8) -> Option<usize> {
    let (r, _) = _unroll_loop_backward::<8, 4, _>(unsafe { buf_ptr.add(4 * 8) }, buf_ptr, |p| {
        _rnechr_dbl_c4_aa_x1(p, c4, st_ptr)
    });
    r
}

#[inline(always)]
fn _rnechr_dbl_c2_aa_x1(buf_ptr: *const u8, c2: B2Dbl, st_ptr: *const u8) -> Option<usize> {
    let v_0 = unsafe { _read_a_little_endian_from_ptr_u16(buf_ptr) };
    let v_0_a = v_0 ^ c2.v1;
    let v_0_b = v_0 ^ c2.v2;
    let bits_0_a = PackedU16::new(v_0_a).may_have_zero_byte();
    let bits_0_b = PackedU16::new(v_0_b).may_have_zero_byte();
    let bits_0_ab = bits_0_a | bits_0_b;
    let base = buf_ptr.usz_offset_from(st_ptr) + 2 - 1;
    //
    _return_rnechr_dbl(base, bits_0_ab)
}

#[inline(always)]
fn _rnechr_dbl_c1_aa_x1(buf_ptr: *const u8, c1: B1Dbl, st_ptr: *const u8) -> Option<usize> {
    let aa_ptr = buf_ptr;
    let aac = unsafe { *aa_ptr };
    if aac != c1.v1 && aac != c1.v2 {
        Some(buf_ptr.usz_offset_from(st_ptr))
    } else {
        None
    }
}

#[inline(always)]
fn _rnechr_dbl_to_aligned<const ALIGN: usize>(
    buf_ptr_cur: *const u8,
    c: B1Dbl,
    start_ptr: *const u8,
) -> (Option<*const u8>, Option<usize>) {
    let buf_ptr_end = buf_ptr_cur;
    let remaining_align = (buf_ptr_end as usize) & (ALIGN - 1);
    let buf_ptr_min = unsafe { buf_ptr_end.sub(remaining_align) };
    let mut cur_ptr = buf_ptr_end;
    while cur_ptr > buf_ptr_min {
        cur_ptr = unsafe { cur_ptr.sub(1) };
        if let Some(pos) = _rnechr_dbl_c1_aa_x1(cur_ptr, c, start_ptr) {
            return (None, Some(pos));
        }
    }
    (Some(buf_ptr_min), None)
}

#[inline(always)]
pub(crate) fn _rnechr_dbl_to_aligned_u256(
    buf_ptr_cur: *const u8,
    c: B1Dbl,
    start_ptr: *const u8,
) -> (Option<*const u8>, Option<usize>) {
    _rnechr_dbl_to_aligned::<32>(buf_ptr_cur, c, start_ptr)
}

#[inline(always)]
pub(crate) fn _rnechr_dbl_to_aligned_u128(
    buf_ptr_cur: *const u8,
    c: B1Dbl,
    start_ptr: *const u8,
) -> (Option<*const u8>, Option<usize>) {
    _rnechr_dbl_to_aligned::<16>(buf_ptr_cur, c, start_ptr)
}

#[inline(always)]
fn _rnechr_dbl_to_aligned_u64(
    buf_ptr_cur: *const u8,
    c: B1Dbl,
    start_ptr: *const u8,
) -> (Option<*const u8>, Option<usize>) {
    _rnechr_dbl_to_aligned::<8>(buf_ptr_cur, c, start_ptr)
}

#[inline(always)]
fn _rnechr_dbl_to_aligned_u32(
    buf_ptr_cur: *const u8,
    c: B1Dbl,
    start_ptr: *const u8,
) -> (Option<*const u8>, Option<usize>) {
    _rnechr_dbl_to_aligned::<4>(buf_ptr_cur, c, start_ptr)
}
