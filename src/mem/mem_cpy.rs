use super::super::RangeError;
use crate::utils::*;

#[inline(never)]
pub fn _memcpy_impl(dst: &mut [u8], src: &[u8]) -> Result<(), RangeError> {
    if src.is_empty() {
        return Ok(());
    }
    let dst_len = dst.len();
    let src_len = src.len();
    if dst_len < src_len {
        return Err(RangeError);
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
        _start_cpy_128(dst, src);
        #[cfg(feature = "test_pointer_width_64")]
        _start_cpy_64(dst, src);
        #[cfg(feature = "test_pointer_width_32")]
        _start_cpy_32(dst, src);
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
        _start_cpy_128(dst, src);
        #[cfg(target_pointer_width = "64")]
        _start_cpy_64(dst, src);
        #[cfg(target_pointer_width = "32")]
        _start_cpy_32(dst, src);
    }
    Ok(())
}

macro_rules! _unroll_one_cpy_to_aligned_x1 {
    ($a_ptr_2:expr, $b_ptr_2:expr, $a_ptr_end:expr, $label:tt) => {{
        _cpy_b1_aa_x1($a_ptr_2, $b_ptr_2);
        $a_ptr_2 = unsafe { $a_ptr_2.add(1) };
        $b_ptr_2 = unsafe { $b_ptr_2.add(1) };
        if $a_ptr_2 >= $a_ptr_end {
            break $label;
        }
    }};
}

macro_rules! _unroll_one_cpy_to_aligned_x2 {
    ($a_ptr_2:expr, $b_ptr_2:expr, $a_ptr_end:expr, $label:tt) => {{
        _unroll_one_cpy_to_aligned_x1!($a_ptr_2, $b_ptr_2, $a_ptr_end, $label);
        _unroll_one_cpy_to_aligned_x1!($a_ptr_2, $b_ptr_2, $a_ptr_end, $label);
    }};
}

macro_rules! _unroll_one_cpy_to_aligned_x4 {
    ($a_ptr_2:expr, $b_ptr_2:expr, $a_ptr_end:expr, $label:tt) => {{
        _unroll_one_cpy_to_aligned_x2!($a_ptr_2, $b_ptr_2, $a_ptr_end, $label);
        _unroll_one_cpy_to_aligned_x2!($a_ptr_2, $b_ptr_2, $a_ptr_end, $label);
    }};
}

macro_rules! _unroll_one_cpy_to_aligned_x8 {
    ($a_ptr_2:expr, $b_ptr_2:expr, $a_ptr_end:expr, $label:tt) => {{
        _unroll_one_cpy_to_aligned_x4!($a_ptr_2, $b_ptr_2, $a_ptr_end, $label);
        _unroll_one_cpy_to_aligned_x4!($a_ptr_2, $b_ptr_2, $a_ptr_end, $label);
    }};
}

macro_rules! _unroll_one_cpy_to_aligned_x16 {
    ($a_ptr_2:expr, $b_ptr_2:expr, $a_ptr_end:expr, $label:tt) => {{
        _unroll_one_cpy_to_aligned_x8!($a_ptr_2, $b_ptr_2, $a_ptr_end, $label);
        _unroll_one_cpy_to_aligned_x8!($a_ptr_2, $b_ptr_2, $a_ptr_end, $label);
    }};
}

#[inline(always)]
pub(crate) fn _cpy_to_aligned_u256(a_ptr: *mut u8, b_ptr: *const u8) -> (*mut u8, *const u8) {
    let remaining_align = 0x20_usize - ((a_ptr as usize) & 0x1F_usize);
    let a_ptr_end = unsafe { a_ptr.add(remaining_align) };
    let b_ptr_end = unsafe { b_ptr.add(remaining_align) };
    let mut a_ptr_2 = a_ptr;
    let mut b_ptr_2 = b_ptr;
    'near: loop {
        _unroll_one_cpy_to_aligned_x16!(a_ptr_2, b_ptr_2, a_ptr_end, 'near);
        _unroll_one_cpy_to_aligned_x16!(a_ptr_2, b_ptr_2, a_ptr_end, 'near);
    }
    (a_ptr_end, b_ptr_end)
}

#[inline(always)]
pub(crate) fn _cpy_to_aligned_u128(a_ptr: *mut u8, b_ptr: *const u8) -> (*mut u8, *const u8) {
    let remaining_align = 0x10_usize - ((a_ptr as usize) & 0x0F_usize);
    let a_ptr_end = unsafe { a_ptr.add(remaining_align) };
    let b_ptr_end = unsafe { b_ptr.add(remaining_align) };
    let mut a_ptr_2 = a_ptr;
    let mut b_ptr_2 = b_ptr;
    'near: loop {
        _unroll_one_cpy_to_aligned_x16!(a_ptr_2, b_ptr_2, a_ptr_end, 'near);
    }
    (a_ptr_end, b_ptr_end)
}

#[inline(always)]
pub(crate) fn _cpy_to_aligned_u64(a_ptr: *mut u8, b_ptr: *const u8) -> (*mut u8, *const u8) {
    let remaining_align = 0x08_usize - ((a_ptr as usize) & 0x07_usize);
    let a_ptr_end = unsafe { a_ptr.add(remaining_align) };
    let b_ptr_end = unsafe { b_ptr.add(remaining_align) };
    let mut a_ptr_2 = a_ptr;
    let mut b_ptr_2 = b_ptr;
    'near: loop {
        _unroll_one_cpy_to_aligned_x8!(a_ptr_2, b_ptr_2, a_ptr_end, 'near);
    }
    (a_ptr_end, b_ptr_end)
}

#[inline(always)]
pub(crate) fn _cpy_to_aligned_u32(a_ptr: *mut u8, b_ptr: *const u8) -> (*mut u8, *const u8) {
    let remaining_align = 0x04_usize - ((a_ptr as usize) & 0x03_usize);
    let a_ptr_end = unsafe { a_ptr.add(remaining_align) };
    let b_ptr_end = unsafe { b_ptr.add(remaining_align) };
    let mut a_ptr_2 = a_ptr;
    let mut b_ptr_2 = b_ptr;
    'near: loop {
        _unroll_one_cpy_to_aligned_x4!(a_ptr_2, b_ptr_2, a_ptr_end, 'near);
    }
    (a_ptr_end, b_ptr_end)
}

//#[cfg(any(target_pointer_width = "128", feature = "test_pointer_width_128"))]
#[inline(always)]
fn _start_cpy_128(dst: &mut [u8], src: &[u8]) {
    let src_len = src.len();
    let mut a_ptr = dst.as_mut_ptr();
    let mut b_ptr = src.as_ptr();
    let end_ptr = unsafe { a_ptr.add(src_len) };
    b_ptr.prefetch_read_data();
    //
    if src_len >= 16 {
        // to a aligned pointer
        {
            if !a_ptr.is_aligned_u128() {
                let (ap, bp) = _cpy_to_aligned_u128(a_ptr, b_ptr);
                a_ptr = ap;
                b_ptr = bp;
            }
        }
        // the loop
        if b_ptr.is_aligned_u128() {
            {
                let unroll = 16;
                let loop_size = 16;
                while a_ptr.is_not_over(end_ptr, loop_size * unroll) {
                    _cpy_b16_aa_x16(a_ptr, b_ptr);
                    a_ptr = unsafe { a_ptr.add(loop_size * unroll) };
                    b_ptr = unsafe { b_ptr.add(loop_size * unroll) };
                }
            }
            {
                let unroll = 8;
                let loop_size = 16;
                while a_ptr.is_not_over(end_ptr, loop_size * unroll) {
                    _cpy_b16_aa_x8(a_ptr, b_ptr);
                    a_ptr = unsafe { a_ptr.add(loop_size * unroll) };
                    b_ptr = unsafe { b_ptr.add(loop_size * unroll) };
                }
            }
            /*
            {
                let unroll = 4;
                let loop_size = 16;
                while a_ptr.is_not_over(end_ptr, loop_size * unroll) {
                    _cpy_b16_aa_x4(a_ptr, b_ptr);
                    a_ptr = unsafe { a_ptr.add(loop_size * unroll) };
                    b_ptr = unsafe { b_ptr.add(loop_size * unroll) };
                }
            }
            {
                let unroll = 2;
                let loop_size = 16;
                while a_ptr.is_not_over(end_ptr, loop_size * unroll) {
                    _cpy_b16_aa_x2(a_ptr, b_ptr);
                    a_ptr = unsafe { a_ptr.add(loop_size * unroll) };
                    b_ptr = unsafe { b_ptr.add(loop_size * unroll) };
                }
            }
            */
            {
                let unroll = 1;
                let loop_size = 16;
                while a_ptr.is_not_over(end_ptr, loop_size * unroll) {
                    _cpy_b16_aa_x1(a_ptr, b_ptr);
                    a_ptr = unsafe { a_ptr.add(loop_size * unroll) };
                    b_ptr = unsafe { b_ptr.add(loop_size * unroll) };
                }
            }
        }
    }
    // the remaining data is the max: 15 bytes.
    _memcpy_remaining_15_bytes_impl(a_ptr, b_ptr, end_ptr)
}

//#[cfg(any(target_pointer_width = "64", feature = "test_pointer_width_64"))]
#[inline(always)]
fn _start_cpy_64(dst: &mut [u8], src: &[u8]) {
    let src_len = src.len();
    let mut a_ptr = dst.as_mut_ptr();
    let mut b_ptr = src.as_ptr();
    let end_ptr = unsafe { a_ptr.add(src_len) };
    b_ptr.prefetch_read_data();
    //
    if src_len >= 8 {
        // to a aligned pointer
        {
            if !a_ptr.is_aligned_u64() {
                let (ap, bp) = _cpy_to_aligned_u64(a_ptr, b_ptr);
                a_ptr = ap;
                b_ptr = bp;
            }
        }
        // the loop
        if b_ptr.is_aligned_u64() {
            {
                let unroll = 16;
                let loop_size = 8;
                while a_ptr.is_not_over(end_ptr, loop_size * unroll) {
                    _cpy_b8_aa_x16(a_ptr, b_ptr);
                    a_ptr = unsafe { a_ptr.add(loop_size * unroll) };
                    b_ptr = unsafe { b_ptr.add(loop_size * unroll) };
                }
            }
            {
                let unroll = 8;
                let loop_size = 8;
                while a_ptr.is_not_over(end_ptr, loop_size * unroll) {
                    _cpy_b8_aa_x8(a_ptr, b_ptr);
                    a_ptr = unsafe { a_ptr.add(loop_size * unroll) };
                    b_ptr = unsafe { b_ptr.add(loop_size * unroll) };
                }
            }
            /*
            {
                let unroll = 4;
                let loop_size = 8;
                while a_ptr.is_not_over(end_ptr, loop_size * unroll) {
                    _cpy_b8_aa_x4(a_ptr, b_ptr);
                    a_ptr = unsafe { a_ptr.add(loop_size * unroll) };
                    b_ptr = unsafe { b_ptr.add(loop_size * unroll) };
                }
            }
            {
                let unroll = 2;
                let loop_size = 8;
                while a_ptr.is_not_over(end_ptr, loop_size * unroll) {
                    _cpy_b8_aa_x2(a_ptr, b_ptr);
                    a_ptr = unsafe { a_ptr.add(loop_size * unroll) };
                    b_ptr = unsafe { b_ptr.add(loop_size * unroll) };
                }
            }
            */
            {
                let unroll = 1;
                let loop_size = 8;
                while a_ptr.is_not_over(end_ptr, loop_size * unroll) {
                    _cpy_b8_aa_x1(a_ptr, b_ptr);
                    a_ptr = unsafe { a_ptr.add(loop_size * unroll) };
                    b_ptr = unsafe { b_ptr.add(loop_size * unroll) };
                }
            }
        }
    }
    // the remaining data is the max: 7 bytes.
    _memcpy_remaining_7_bytes_impl(a_ptr, b_ptr, end_ptr)
}

//#[cfg(any(target_pointer_width = "32", feature = "test_pointer_width_32"))]
#[inline(always)]
fn _start_cpy_32(dst: &mut [u8], src: &[u8]) {
    let src_len = src.len();
    let mut a_ptr = dst.as_mut_ptr();
    let mut b_ptr = src.as_ptr();
    let end_ptr = unsafe { a_ptr.add(src_len) };
    b_ptr.prefetch_read_data();
    //
    if src_len >= 4 {
        // to a aligned pointer
        {
            if !a_ptr.is_aligned_u32() {
                let (ap, bp) = _cpy_to_aligned_u32(a_ptr, b_ptr);
                a_ptr = ap;
                b_ptr = bp;
            }
        }
        // the loop
        if b_ptr.is_aligned_u32() {
            {
                let unroll = 16;
                let loop_size = 4;
                while a_ptr.is_not_over(end_ptr, loop_size * unroll) {
                    _cpy_b4_aa_x16(a_ptr, b_ptr);
                    a_ptr = unsafe { a_ptr.add(loop_size * unroll) };
                    b_ptr = unsafe { b_ptr.add(loop_size * unroll) };
                }
            }
            {
                let unroll = 8;
                let loop_size = 4;
                while a_ptr.is_not_over(end_ptr, loop_size * unroll) {
                    _cpy_b4_aa_x8(a_ptr, b_ptr);
                    a_ptr = unsafe { a_ptr.add(loop_size * unroll) };
                    b_ptr = unsafe { b_ptr.add(loop_size * unroll) };
                }
            }
            /*
            {
                let unroll = 4;
                let loop_size = 4;
                while a_ptr.is_not_over(end_ptr, loop_size * unroll) {
                    _cpy_b4_aa_x4(a_ptr, b_ptr);
                    a_ptr = unsafe { a_ptr.add(loop_size * unroll) };
                    b_ptr = unsafe { b_ptr.add(loop_size * unroll) };
                }
            }
            {
                let unroll = 2;
                let loop_size = 4;
                while a_ptr.is_not_over(end_ptr, loop_size * unroll) {
                    _cpy_b4_aa_x2(a_ptr, b_ptr);
                    a_ptr = unsafe { a_ptr.add(loop_size * unroll) };
                    b_ptr = unsafe { b_ptr.add(loop_size * unroll) };
                }
            }
            */
            {
                let unroll = 1;
                let loop_size = 4;
                while a_ptr.is_not_over(end_ptr, loop_size * unroll) {
                    _cpy_b4_aa_x1(a_ptr, b_ptr);
                    a_ptr = unsafe { a_ptr.add(loop_size * unroll) };
                    b_ptr = unsafe { b_ptr.add(loop_size * unroll) };
                }
            }
        }
    }
    // the remaining data is the max: 3 bytes.
    _memcpy_remaining_3_bytes_impl(a_ptr, b_ptr, end_ptr)
}

#[inline(always)]
pub(crate) fn _memcpy_remaining_15_bytes_impl(
    dst_ptr: *mut u8,
    src_ptr: *const u8,
    end_ptr: *mut u8,
) {
    let mut a_ptr = dst_ptr;
    let mut b_ptr = src_ptr;
    if a_ptr.is_aligned_u64() && b_ptr.is_aligned_u64() {
        let loop_size = 8;
        if a_ptr.is_not_over(end_ptr, loop_size) {
            let eend_ptr = unsafe { end_ptr.sub(loop_size) };
            let mut aa_ptr = a_ptr;
            let mut bb_ptr = b_ptr;
            'near: loop {
                for _ in 0..16 {
                    if aa_ptr >= eend_ptr {
                        break 'near;
                    }
                    _cpy_b8_aa_x1(aa_ptr, bb_ptr);
                    aa_ptr = unsafe { aa_ptr.add(loop_size) };
                    bb_ptr = unsafe { bb_ptr.add(loop_size) };
                }
            }
            a_ptr = aa_ptr;
            b_ptr = bb_ptr;
        }
    }
    // the remaining data is the max: 7 bytes.
    _memcpy_remaining_7_bytes_impl(a_ptr, b_ptr, end_ptr)
}

#[inline(always)]
pub(crate) fn _memcpy_remaining_7_bytes_impl(
    dst_ptr: *mut u8,
    src_ptr: *const u8,
    end_ptr: *mut u8,
) {
    let mut a_ptr = dst_ptr;
    let mut b_ptr = src_ptr;
    if a_ptr.is_aligned_u32() && b_ptr.is_aligned_u32() {
        let loop_size = 4;
        if a_ptr.is_not_over(end_ptr, loop_size) {
            let eend_ptr = unsafe { end_ptr.sub(loop_size) };
            let mut aa_ptr = a_ptr;
            let mut bb_ptr = b_ptr;
            'near: loop {
                for _ in 0..64 {
                    if aa_ptr >= eend_ptr {
                        break 'near;
                    }
                    _cpy_b4_aa_x1(aa_ptr, bb_ptr);
                    aa_ptr = unsafe { aa_ptr.add(loop_size) };
                    bb_ptr = unsafe { bb_ptr.add(loop_size) };
                }
            }
            a_ptr = aa_ptr;
            b_ptr = bb_ptr;
        }
    }
    // the remaining data is the max: 3 bytes.
    _memcpy_remaining_3_bytes_impl(a_ptr, b_ptr, end_ptr)
}

#[inline(always)]
pub(crate) fn _memcpy_remaining_3_bytes_impl(
    dst_ptr: *mut u8,
    src_ptr: *const u8,
    end_ptr: *mut u8,
) {
    let mut a_ptr = dst_ptr;
    let mut b_ptr = src_ptr;
    /*
    if a_ptr.is_aligned_u16() && b_ptr.is_aligned_u16() {
        let loop_size = 2;
        if a_ptr.is_not_over(end_ptr, loop_size) {
            let eend_ptr = unsafe { end_ptr.sub(loop_size) };
            let mut aa_ptr = a_ptr;
            let mut bb_ptr = b_ptr;
            'near2: loop {
                for _ in 0..16 {
                    if aa_ptr >= eend_ptr {
                        break 'near2;
                    }
                    _cpy_b2_aa_x1(aa_ptr, bb_ptr);
                    aa_ptr = unsafe { aa_ptr.add(loop_size) };
                    bb_ptr = unsafe { bb_ptr.add(loop_size) };
                }
            }
            a_ptr = aa_ptr;
            b_ptr = bb_ptr;
        }
    }
    */
    {
        let loop_size = 1;
        if a_ptr.is_not_over(end_ptr, loop_size) {
            'near1: loop {
                for _ in 0..32 {
                    if a_ptr >= end_ptr {
                        break 'near1;
                    }
                    _cpy_b1_aa_x1(a_ptr, b_ptr);
                    a_ptr = unsafe { a_ptr.add(loop_size) };
                    b_ptr = unsafe { b_ptr.add(loop_size) };
                }
            }
        }
    }
}

#[inline(always)]
fn _cpy_b16_aa_x1(a_ptr: *const u8, b_ptr: *const u8) {
    let bc = unsafe { _read_a_native_endian_from_ptr_u128(b_ptr) };
    let aa_ptr = a_ptr as *mut u128;
    unsafe { aa_ptr.write(bc) };
}

#[inline(always)]
fn _cpy_b16_aa_x2(a_ptr: *const u8, b_ptr: *const u8) {
    _cpy_b16_aa_x1(a_ptr, b_ptr);
    _cpy_b16_aa_x1(unsafe { a_ptr.add(16) }, unsafe { b_ptr.add(16) })
}

#[inline(always)]
fn _cpy_b16_aa_x4(a_ptr: *const u8, b_ptr: *const u8) {
    _cpy_b16_aa_x2(a_ptr, b_ptr);
    _cpy_b16_aa_x2(unsafe { a_ptr.add(16 * 2) }, unsafe { b_ptr.add(16 * 2) })
}

#[inline(always)]
fn _cpy_b16_aa_x8(a_ptr: *const u8, b_ptr: *const u8) {
    _cpy_b16_aa_x4(a_ptr, b_ptr);
    _cpy_b16_aa_x4(unsafe { a_ptr.add(16 * 4) }, unsafe { b_ptr.add(16 * 4) })
}

#[inline(always)]
fn _cpy_b16_aa_x16(a_ptr: *const u8, b_ptr: *const u8) {
    _cpy_b16_aa_x8(a_ptr, b_ptr);
    _cpy_b16_aa_x8(unsafe { a_ptr.add(16 * 8) }, unsafe { b_ptr.add(16 * 8) })
}

#[inline(always)]
fn _cpy_b8_aa_x1(a_ptr: *const u8, b_ptr: *const u8) {
    let bc = unsafe { _read_a_native_endian_from_ptr_u64(b_ptr) };
    let aa_ptr = a_ptr as *mut u64;
    unsafe { aa_ptr.write(bc) };
}

#[inline(always)]
fn _cpy_b8_aa_x2(a_ptr: *const u8, b_ptr: *const u8) {
    _cpy_b8_aa_x1(a_ptr, b_ptr);
    _cpy_b8_aa_x1(unsafe { a_ptr.add(8) }, unsafe { b_ptr.add(8) })
}

#[inline(always)]
fn _cpy_b8_aa_x4(a_ptr: *const u8, b_ptr: *const u8) {
    _cpy_b8_aa_x2(a_ptr, b_ptr);
    _cpy_b8_aa_x2(unsafe { a_ptr.add(8 * 2) }, unsafe { b_ptr.add(8 * 2) })
}

#[inline(always)]
fn _cpy_b8_aa_x8(a_ptr: *const u8, b_ptr: *const u8) {
    _cpy_b8_aa_x4(a_ptr, b_ptr);
    _cpy_b8_aa_x4(unsafe { a_ptr.add(8 * 4) }, unsafe { b_ptr.add(8 * 4) })
}

#[inline(always)]
fn _cpy_b8_aa_x16(a_ptr: *const u8, b_ptr: *const u8) {
    _cpy_b8_aa_x8(a_ptr, b_ptr);
    _cpy_b8_aa_x8(unsafe { a_ptr.add(8 * 8) }, unsafe { b_ptr.add(8 * 8) })
}

#[inline(always)]
fn _cpy_b4_aa_x1(a_ptr: *const u8, b_ptr: *const u8) {
    let bc = unsafe { _read_a_native_endian_from_ptr_u32(b_ptr) };
    let aa_ptr = a_ptr as *mut u32;
    unsafe { aa_ptr.write(bc) };
}

#[inline(always)]
fn _cpy_b4_aa_x2(a_ptr: *const u8, b_ptr: *const u8) {
    _cpy_b4_aa_x1(a_ptr, b_ptr);
    _cpy_b4_aa_x1(unsafe { a_ptr.add(4) }, unsafe { b_ptr.add(4) })
}

#[inline(always)]
fn _cpy_b4_aa_x4(a_ptr: *const u8, b_ptr: *const u8) {
    _cpy_b4_aa_x2(a_ptr, b_ptr);
    _cpy_b4_aa_x2(unsafe { a_ptr.add(4 * 2) }, unsafe { b_ptr.add(4 * 2) })
}

#[inline(always)]
fn _cpy_b4_aa_x8(a_ptr: *const u8, b_ptr: *const u8) {
    _cpy_b4_aa_x4(a_ptr, b_ptr);
    _cpy_b4_aa_x4(unsafe { a_ptr.add(4 * 4) }, unsafe { b_ptr.add(4 * 4) })
}

#[inline(always)]
fn _cpy_b4_aa_x16(a_ptr: *const u8, b_ptr: *const u8) {
    _cpy_b4_aa_x8(a_ptr, b_ptr);
    _cpy_b4_aa_x8(unsafe { a_ptr.add(4 * 8) }, unsafe { b_ptr.add(4 * 8) })
}

#[inline(always)]
fn _cpy_b2_aa_x1(a_ptr: *const u8, b_ptr: *const u8) {
    let bc = unsafe { _read_a_native_endian_from_ptr_u16(b_ptr) };
    let aa_ptr = a_ptr as *mut u16;
    unsafe { aa_ptr.write(bc) };
}

#[inline(always)]
fn _cpy_b2_aa_x2(a_ptr: *const u8, b_ptr: *const u8) {
    _cpy_b2_aa_x1(a_ptr, b_ptr);
    _cpy_b2_aa_x1(unsafe { a_ptr.add(2) }, unsafe { b_ptr.add(2) })
}

#[inline(always)]
fn _cpy_b2_aa_x4(a_ptr: *const u8, b_ptr: *const u8) {
    _cpy_b2_aa_x2(a_ptr, b_ptr);
    _cpy_b2_aa_x2(unsafe { a_ptr.add(2 * 2) }, unsafe { b_ptr.add(2 * 2) })
}

#[inline(always)]
fn _cpy_b2_aa_x8(a_ptr: *const u8, b_ptr: *const u8) {
    _cpy_b2_aa_x4(a_ptr, b_ptr);
    _cpy_b2_aa_x4(unsafe { a_ptr.add(2 * 4) }, unsafe { b_ptr.add(2 * 4) })
}

#[inline(always)]
fn _cpy_b2_aa_x16(a_ptr: *const u8, b_ptr: *const u8) {
    _cpy_b2_aa_x8(a_ptr, b_ptr);
    _cpy_b2_aa_x8(unsafe { a_ptr.add(2 * 8) }, unsafe { b_ptr.add(2 * 8) })
}

#[inline(always)]
fn _cpy_b1_aa_x1(a_ptr: *const u8, b_ptr: *const u8) {
    let bc = unsafe { *b_ptr };
    let aa_ptr = a_ptr as *mut u8;
    unsafe { aa_ptr.write(bc) };
}

#[inline(always)]
fn _cpy_b1_aa_x2(a_ptr: *const u8, b_ptr: *const u8) {
    _cpy_b1_aa_x1(a_ptr, b_ptr);
    _cpy_b1_aa_x1(unsafe { a_ptr.add(1) }, unsafe { b_ptr.add(1) })
}

#[inline(always)]
fn _cpy_b1_aa_x4(a_ptr: *const u8, b_ptr: *const u8) {
    _cpy_b1_aa_x2(a_ptr, b_ptr);
    _cpy_b1_aa_x2(unsafe { a_ptr.add(2) }, unsafe { b_ptr.add(2) })
}

#[inline(always)]
fn _cpy_b1_aa_x8(a_ptr: *const u8, b_ptr: *const u8) {
    _cpy_b1_aa_x4(a_ptr, b_ptr);
    _cpy_b1_aa_x4(unsafe { a_ptr.add(4) }, unsafe { b_ptr.add(4) })
}

#[inline(always)]
fn _cpy_b1_aa_x16(a_ptr: *const u8, b_ptr: *const u8) {
    _cpy_b1_aa_x8(a_ptr, b_ptr);
    _cpy_b1_aa_x8(unsafe { a_ptr.add(8) }, unsafe { b_ptr.add(8) })
}

/*
 * The simple implement:

#[inline(always)]
pub fn _memcpy_impl(dst: &mut [u8], src: &[u8]) -> Result<(), RangeError> {
    if dst.len() < src.len() {
        return Err(RangeError);
    }
    for i in 0..src.len() {
        dst[i] = src[i];
    }
    Ok(())
}
*/

#[cfg(test)]
mod disasm {
    use super::*;
    //
    #[test]
    fn do_procs() {
        let mut a = b"       ".to_vec();
        let b = b"abcdefg".to_vec();
        let a = a.as_mut_slice();
        let b = b.as_slice();
        assert_eq!(do_proc_basic(a, b), Ok(()));
        #[cfg(target_pointer_width = "128")]
        do_proc_128(a, b);
        #[cfg(target_pointer_width = "64")]
        do_proc_64(a, b);
        #[cfg(target_pointer_width = "32")]
        do_proc_32(a, b);
    }
    fn do_proc_basic(a: &mut [u8], b: &[u8]) -> Result<(), RangeError> {
        _memcpy_impl(a, b)
    }
    #[cfg(target_pointer_width = "128")]
    fn do_proc_128(a: &mut [u8], b: &[u8]) {
        _start_cpy_128(a, b)
    }
    #[cfg(target_pointer_width = "64")]
    fn do_proc_64(a: &mut [u8], b: &[u8]) {
        _start_cpy_64(a, b)
    }
    #[cfg(target_pointer_width = "32")]
    fn do_proc_32(a: &mut [u8], b: &[u8]) {
        _start_cpy_32(a, b)
    }
}
