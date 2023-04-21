use crate::utils::*;

#[inline(never)]
pub fn _memeq_impl(a: &[u8], b: &[u8]) -> bool {
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
        let r = _start_eq_128(a, b);
        #[cfg(feature = "test_pointer_width_64")]
        let r = _start_eq_64(a, b);
        #[cfg(feature = "test_pointer_width_32")]
        let r = _start_eq_32(a, b);
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
        let r = _start_eq_128(a, b);
        #[cfg(target_pointer_width = "64")]
        let r = _start_eq_64(a, b);
        #[cfg(target_pointer_width = "32")]
        let r = _start_eq_32(a, b);
        //
        r
    }
}

macro_rules! _unroll_one_eq_to_aligned_x1 {
    ($a_ptr_2:expr, $b_ptr_2:expr, $a_ptr_end:expr, $label:tt) => {{
        if $a_ptr_2 >= $a_ptr_end {
            break $label;
        }
        let r = _eq_b1_aa_x1($a_ptr_2, $b_ptr_2);
        if !r {
            return (None, Some(r));
        }
        $a_ptr_2 = unsafe { $a_ptr_2.add(1) };
        $b_ptr_2 = unsafe { $b_ptr_2.add(1) };
    }};
}

macro_rules! _unroll_one_eq_to_aligned_x2 {
    ($a_ptr_2:expr, $b_ptr_2:expr, $a_ptr_end:expr, $label:tt) => {{
        _unroll_one_eq_to_aligned_x1!($a_ptr_2, $b_ptr_2, $a_ptr_end, $label);
        _unroll_one_eq_to_aligned_x1!($a_ptr_2, $b_ptr_2, $a_ptr_end, $label);
    }};
}

macro_rules! _unroll_one_eq_to_aligned_x4 {
    ($a_ptr_2:expr, $b_ptr_2:expr, $a_ptr_end:expr, $label:tt) => {{
        _unroll_one_eq_to_aligned_x2!($a_ptr_2, $b_ptr_2, $a_ptr_end, $label);
        _unroll_one_eq_to_aligned_x2!($a_ptr_2, $b_ptr_2, $a_ptr_end, $label);
    }};
}

macro_rules! _unroll_one_eq_to_aligned_x8 {
    ($a_ptr_2:expr, $b_ptr_2:expr, $a_ptr_end:expr, $label:tt) => {{
        _unroll_one_eq_to_aligned_x4!($a_ptr_2, $b_ptr_2, $a_ptr_end, $label);
        _unroll_one_eq_to_aligned_x4!($a_ptr_2, $b_ptr_2, $a_ptr_end, $label);
    }};
}

macro_rules! _unroll_one_eq_to_aligned_x16 {
    ($a_ptr_2:expr, $b_ptr_2:expr, $a_ptr_end:expr, $label:tt) => {{
        _unroll_one_eq_to_aligned_x8!($a_ptr_2, $b_ptr_2, $a_ptr_end, $label);
        _unroll_one_eq_to_aligned_x8!($a_ptr_2, $b_ptr_2, $a_ptr_end, $label);
    }};
}

#[inline(always)]
pub(crate) fn _eq_to_aligned_u256(
    a_ptr: *const u8,
    b_ptr: *const u8,
) -> (Option<(*const u8, *const u8)>, Option<bool>) {
    let remaining_align = 0x20_usize - ((a_ptr as usize) & 0x1F_usize);
    let a_ptr_end = unsafe { a_ptr.add(remaining_align) };
    let b_ptr_end = unsafe { b_ptr.add(remaining_align) };
    let mut a_ptr_2 = a_ptr;
    let mut b_ptr_2 = b_ptr;
    'near: loop {
        _unroll_one_eq_to_aligned_x16!(a_ptr_2, b_ptr_2, a_ptr_end, 'near);
        _unroll_one_eq_to_aligned_x16!(a_ptr_2, b_ptr_2, a_ptr_end, 'near);
    }
    (Some((a_ptr_end, b_ptr_end)), None)
}

#[inline(always)]
pub(crate) fn _eq_to_aligned_u128(
    a_ptr: *const u8,
    b_ptr: *const u8,
) -> (Option<(*const u8, *const u8)>, Option<bool>) {
    let remaining_align = 0x10_usize - ((a_ptr as usize) & 0x0F_usize);
    let a_ptr_end = unsafe { a_ptr.add(remaining_align) };
    let b_ptr_end = unsafe { b_ptr.add(remaining_align) };
    let mut a_ptr_2 = a_ptr;
    let mut b_ptr_2 = b_ptr;
    'near: loop {
        _unroll_one_eq_to_aligned_x16!(a_ptr_2, b_ptr_2, a_ptr_end, 'near);
    }
    (Some((a_ptr_end, b_ptr_end)), None)
}

#[inline(always)]
fn _eq_to_aligned_u64(
    a_ptr: *const u8,
    b_ptr: *const u8,
) -> (Option<(*const u8, *const u8)>, Option<bool>) {
    let remaining_align = 0x08_usize - ((a_ptr as usize) & 0x07_usize);
    let a_ptr_end = unsafe { a_ptr.add(remaining_align) };
    let b_ptr_end = unsafe { b_ptr.add(remaining_align) };
    let mut a_ptr_2 = a_ptr;
    let mut b_ptr_2 = b_ptr;
    'near: loop {
        _unroll_one_eq_to_aligned_x8!(a_ptr_2, b_ptr_2, a_ptr_end, 'near);
    }
    (Some((a_ptr_end, b_ptr_end)), None)
}

#[inline(always)]
fn _eq_to_aligned_u32(
    a_ptr: *const u8,
    b_ptr: *const u8,
) -> (Option<(*const u8, *const u8)>, Option<bool>) {
    let remaining_align = 0x04_usize - ((a_ptr as usize) & 0x03_usize);
    let a_ptr_end = unsafe { a_ptr.add(remaining_align) };
    let b_ptr_end = unsafe { b_ptr.add(remaining_align) };
    let mut a_ptr_2 = a_ptr;
    let mut b_ptr_2 = b_ptr;
    'near: loop {
        _unroll_one_eq_to_aligned_x4!(a_ptr_2, b_ptr_2, a_ptr_end, 'near);
    }
    (Some((a_ptr_end, b_ptr_end)), None)
}

//#[cfg(any(target_pointer_width = "128", feature = "test_pointer_width_128"))]
#[inline(always)]
pub(crate) fn _start_eq_128(a: &[u8], b: &[u8]) -> bool {
    //
    let a_len = a.len();
    let b_len = b.len();
    if a_len != b_len {
        return false;
    }
    let mut a_ptr = a.as_ptr();
    let mut b_ptr = b.as_ptr();
    let end_ptr = unsafe { a_ptr.add(a_len) };
    a_ptr.prefetch_read_data();
    b_ptr.prefetch_read_data();
    //
    if a_len >= 16 {
        // to a aligned pointer
        {
            if !a_ptr.is_aligned_u128() {
                let r = _eq_to_aligned_u128(a_ptr, b_ptr);
                if let Some((ap, bp)) = r.0 {
                    a_ptr = ap;
                    b_ptr = bp;
                } else if let Some(v) = r.1 {
                    return v;
                }
            }
        }
        // the loop
        if b_ptr.is_aligned_u128() {
            {
                let unroll = 16;
                let loop_size = 16;
                while a_ptr.is_not_over(end_ptr, loop_size * unroll) {
                    let r = _eq_b16_aa_x16(a_ptr, b_ptr);
                    if !r {
                        return r;
                    }
                    a_ptr = unsafe { a_ptr.add(loop_size * unroll) };
                    b_ptr = unsafe { b_ptr.add(loop_size * unroll) };
                }
            }
            {
                let unroll = 8;
                let loop_size = 16;
                while a_ptr.is_not_over(end_ptr, loop_size * unroll) {
                    let r = _eq_b16_aa_x8(a_ptr, b_ptr);
                    if !r {
                        return r;
                    }
                    a_ptr = unsafe { a_ptr.add(loop_size * unroll) };
                    b_ptr = unsafe { b_ptr.add(loop_size * unroll) };
                }
            }
            /*
            {
                let unroll = 4;
                let loop_size = 16;
                while a_ptr.is_not_over(end_ptr, loop_size * unroll) {
                    let r = _eq_b16_aa_x4(a_ptr, b_ptr);
                    if !r {
                        return r;
                    }
                    a_ptr = unsafe { a_ptr.add(loop_size * unroll) };
                    b_ptr = unsafe { b_ptr.add(loop_size * unroll) };
                }
            }
            {
                let unroll = 2;
                let loop_size = 16;
                while a_ptr.is_not_over(end_ptr, loop_size * unroll) {
                    let r = _eq_b16_aa_x2(a_ptr, b_ptr);
                    if !r {
                        return r;
                    }
                    a_ptr = unsafe { a_ptr.add(loop_size * unroll) };
                    b_ptr = unsafe { b_ptr.add(loop_size * unroll) };
                }
            }
            */
            {
                let unroll = 1;
                let loop_size = 16;
                while a_ptr.is_not_over(end_ptr, loop_size * unroll) {
                    let r = _eq_b16_aa_x1(a_ptr, b_ptr);
                    if !r {
                        return r;
                    }
                    a_ptr = unsafe { a_ptr.add(loop_size * unroll) };
                    b_ptr = unsafe { b_ptr.add(loop_size * unroll) };
                }
            }
        }
    }
    // the remaining data is the max: 15 bytes.
    _memeq_remaining_15_bytes_impl(a_ptr, b_ptr, end_ptr)
}

//#[cfg(any(target_pointer_width = "64", feature = "test_pointer_width_64"))]
#[inline(always)]
fn _start_eq_64(a: &[u8], b: &[u8]) -> bool {
    //
    let a_len = a.len();
    let b_len = b.len();
    if a_len != b_len {
        return false;
    }
    let mut a_ptr = a.as_ptr();
    let mut b_ptr = b.as_ptr();
    let end_ptr = unsafe { a_ptr.add(a_len) };
    a_ptr.prefetch_read_data();
    b_ptr.prefetch_read_data();
    //
    if a_len >= 8 {
        // to a aligned pointer
        {
            if !a_ptr.is_aligned_u64() {
                let r = _eq_to_aligned_u64(a_ptr, b_ptr);
                if let Some((ap, bp)) = r.0 {
                    a_ptr = ap;
                    b_ptr = bp;
                } else if let Some(v) = r.1 {
                    return v;
                }
            }
        }
        // the loop
        if b_ptr.is_aligned_u64() {
            {
                let unroll = 16;
                let loop_size = 8;
                while a_ptr.is_not_over(end_ptr, loop_size * unroll) {
                    let r = _eq_b8_aa_x16(a_ptr, b_ptr);
                    if !r {
                        return r;
                    }
                    a_ptr = unsafe { a_ptr.add(loop_size * unroll) };
                    b_ptr = unsafe { b_ptr.add(loop_size * unroll) };
                }
            }
            {
                let unroll = 8;
                let loop_size = 8;
                while a_ptr.is_not_over(end_ptr, loop_size * unroll) {
                    let r = _eq_b8_aa_x8(a_ptr, b_ptr);
                    if !r {
                        return r;
                    }
                    a_ptr = unsafe { a_ptr.add(loop_size * unroll) };
                    b_ptr = unsafe { b_ptr.add(loop_size * unroll) };
                }
            }
            /*
            {
                let unroll = 4;
                let loop_size = 8;
                while a_ptr.is_not_over(end_ptr, loop_size * unroll) {
                    let r = _eq_b8_aa_x4(a_ptr, b_ptr);
                    if !r {
                        return r;
                    }
                    a_ptr = unsafe { a_ptr.add(loop_size * unroll) };
                    b_ptr = unsafe { b_ptr.add(loop_size * unroll) };
                }
            }
            {
                let unroll = 2;
                let loop_size = 8;
                while a_ptr.is_not_over(end_ptr, loop_size * unroll) {
                    let r = _eq_b8_aa_x2(a_ptr, b_ptr);
                    if !r {
                        return r;
                    }
                    a_ptr = unsafe { a_ptr.add(loop_size * unroll) };
                    b_ptr = unsafe { b_ptr.add(loop_size * unroll) };
                }
            }
            */
            {
                let unroll = 1;
                let loop_size = 8;
                while a_ptr.is_not_over(end_ptr, loop_size * unroll) {
                    let r = _eq_b8_aa_x1(a_ptr, b_ptr);
                    if !r {
                        return r;
                    }
                    a_ptr = unsafe { a_ptr.add(loop_size * unroll) };
                    b_ptr = unsafe { b_ptr.add(loop_size * unroll) };
                }
            }
        }
    }
    // the remaining data is the max: 7 bytes.
    _memeq_remaining_7_bytes_impl(a_ptr, b_ptr, end_ptr)
}

//#[cfg(any(target_pointer_width = "32", feature = "test_pointer_width_32"))]
#[inline(always)]
fn _start_eq_32(a: &[u8], b: &[u8]) -> bool {
    //
    let a_len = a.len();
    let b_len = b.len();
    if a_len != b_len {
        return false;
    }
    let mut a_ptr = a.as_ptr();
    let mut b_ptr = b.as_ptr();
    let end_ptr = unsafe { a_ptr.add(a_len) };
    a_ptr.prefetch_read_data();
    b_ptr.prefetch_read_data();
    //
    if a_len >= 4 {
        // to a aligned pointer
        {
            if !a_ptr.is_aligned_u32() {
                let r = _eq_to_aligned_u32(a_ptr, b_ptr);
                if let Some((ap, bp)) = r.0 {
                    a_ptr = ap;
                    b_ptr = bp;
                } else if let Some(v) = r.1 {
                    return v;
                }
            }
        }
        // the loop
        if b_ptr.is_aligned_u32() {
            {
                let unroll = 16;
                let loop_size = 4;
                while a_ptr.is_not_over(end_ptr, loop_size * unroll) {
                    let r = _eq_b4_aa_x16(a_ptr, b_ptr);
                    if !r {
                        return r;
                    }
                    a_ptr = unsafe { a_ptr.add(loop_size * unroll) };
                    b_ptr = unsafe { b_ptr.add(loop_size * unroll) };
                }
            }
            {
                let unroll = 8;
                let loop_size = 4;
                while a_ptr.is_not_over(end_ptr, loop_size * unroll) {
                    let r = _eq_b4_aa_x8(a_ptr, b_ptr);
                    if !r {
                        return r;
                    }
                    a_ptr = unsafe { a_ptr.add(loop_size * unroll) };
                    b_ptr = unsafe { b_ptr.add(loop_size * unroll) };
                }
            }
            /*
            {
                let unroll = 4;
                let loop_size = 4;
                while a_ptr.is_not_over(end_ptr, loop_size * unroll) {
                    let r = _eq_b4_aa_x4(a_ptr, b_ptr);
                    if !r {
                        return r;
                    }
                    a_ptr = unsafe { a_ptr.add(loop_size * unroll) };
                    b_ptr = unsafe { b_ptr.add(loop_size * unroll) };
                }
            }
            {
                let unroll = 2;
                let loop_size = 4;
                while a_ptr.is_not_over(end_ptr, loop_size * unroll) {
                    let r = _eq_b4_aa_x2(a_ptr, b_ptr);
                    if !r {
                        return r;
                    }
                    a_ptr = unsafe { a_ptr.add(loop_size * unroll) };
                    b_ptr = unsafe { b_ptr.add(loop_size * unroll) };
                }
            }
            */
            {
                let unroll = 1;
                let loop_size = 4;
                while a_ptr.is_not_over(end_ptr, loop_size * unroll) {
                    let r = _eq_b4_aa_x1(a_ptr, b_ptr);
                    if !r {
                        return r;
                    }
                    a_ptr = unsafe { a_ptr.add(loop_size * unroll) };
                    b_ptr = unsafe { b_ptr.add(loop_size * unroll) };
                }
            }
        }
    }
    // the remaining data is the max: 3 bytes.
    _memeq_remaining_3_bytes_impl(a_ptr, b_ptr, end_ptr)
}

#[inline(always)]
pub(crate) fn _memeq_remaining_15_bytes_impl(
    a_ptr: *const u8,
    b_ptr: *const u8,
    end_ptr: *const u8,
) -> bool {
    let mut a_ptr = a_ptr;
    let mut b_ptr = b_ptr;
    if a_ptr.is_aligned_u64() && b_ptr.is_aligned_u64() {
        let loop_size = 8;
        if a_ptr.is_not_over(end_ptr, loop_size) {
            let eend_ptr = unsafe { end_ptr.sub(loop_size) };
            let mut aa_ptr = a_ptr;
            let mut bb_ptr = b_ptr;
            'near: loop {
                for _ in 0..16 {
                    if aa_ptr > eend_ptr {
                        break 'near;
                    }
                    let r = _eq_b8_aa_x1(aa_ptr, bb_ptr);
                    if !r {
                        return r;
                    }
                    aa_ptr = unsafe { aa_ptr.add(loop_size) };
                    bb_ptr = unsafe { bb_ptr.add(loop_size) };
                }
            }
            a_ptr = aa_ptr;
            b_ptr = bb_ptr;
        }
    }
    // the remaining data is the max: 7 bytes.
    _memeq_remaining_7_bytes_impl(a_ptr, b_ptr, end_ptr)
}

#[inline(always)]
fn _memeq_remaining_7_bytes_impl(a_ptr: *const u8, b_ptr: *const u8, end_ptr: *const u8) -> bool {
    let mut a_ptr = a_ptr;
    let mut b_ptr = b_ptr;
    if a_ptr.is_aligned_u32() && b_ptr.is_aligned_u32() {
        let loop_size = 4;
        if a_ptr.is_not_over(end_ptr, loop_size) {
            let eend_ptr = unsafe { end_ptr.sub(loop_size) };
            let mut aa_ptr = a_ptr;
            let mut bb_ptr = b_ptr;
            'near: loop {
                for _ in 0..16 {
                    if aa_ptr > eend_ptr {
                        break 'near;
                    }
                    let r = _eq_b4_aa_x1(aa_ptr, bb_ptr);
                    if !r {
                        return r;
                    }
                    aa_ptr = unsafe { aa_ptr.add(loop_size) };
                    bb_ptr = unsafe { bb_ptr.add(loop_size) };
                }
            }
            a_ptr = aa_ptr;
            b_ptr = bb_ptr;
        }
    }
    // the remaining data is the max: 3 bytes.
    _memeq_remaining_3_bytes_impl(a_ptr, b_ptr, end_ptr)
}

#[inline(always)]
fn _memeq_remaining_3_bytes_impl(a_ptr: *const u8, b_ptr: *const u8, end_ptr: *const u8) -> bool {
    let mut a_ptr = a_ptr;
    let mut b_ptr = b_ptr;
    if a_ptr.is_aligned_u16() && b_ptr.is_aligned_u16() {
        let loop_size = 2;
        if a_ptr.is_not_over(end_ptr, loop_size) {
            let eend_ptr = unsafe { end_ptr.sub(loop_size) };
            let mut aa_ptr = a_ptr;
            let mut bb_ptr = b_ptr;
            'near2: loop {
                for _ in 0..16 {
                    if aa_ptr > eend_ptr {
                        break 'near2;
                    }
                    let r = _eq_b2_aa_x1(aa_ptr, bb_ptr);
                    if !r {
                        return r;
                    }
                    aa_ptr = unsafe { aa_ptr.add(loop_size) };
                    bb_ptr = unsafe { bb_ptr.add(loop_size) };
                }
            }
            a_ptr = aa_ptr;
            b_ptr = bb_ptr;
        }
    }
    {
        let loop_size = 1;
        if a_ptr.is_not_over(end_ptr, loop_size) {
            'near1: loop {
                for _ in 0..32 {
                    if a_ptr >= end_ptr {
                        break 'near1;
                    }
                    let r = _eq_b1_aa_x1(a_ptr, b_ptr);
                    if !r {
                        return r;
                    }
                    a_ptr = unsafe { a_ptr.add(loop_size) };
                    b_ptr = unsafe { b_ptr.add(loop_size) };
                }
            }
        }
    }
    //
    true
}

#[inline(always)]
fn _eq_b16_aa_x1(a_ptr: *const u8, b_ptr: *const u8) -> bool {
    let ac = unsafe { _read_a_native_endian_from_ptr_u128(a_ptr) };
    let bc = unsafe { _read_a_native_endian_from_ptr_u128(b_ptr) };
    ac == bc
}

#[inline(always)]
fn _eq_b16_aa_x2(a_ptr: *const u8, b_ptr: *const u8) -> bool {
    let r = _eq_b16_aa_x1(a_ptr, b_ptr);
    if !r {
        return r;
    }
    _eq_b16_aa_x1(unsafe { a_ptr.add(16) }, unsafe { b_ptr.add(16) })
}

#[inline(always)]
fn _eq_b16_aa_x4(a_ptr: *const u8, b_ptr: *const u8) -> bool {
    let r = _eq_b16_aa_x2(a_ptr, b_ptr);
    if !r {
        return r;
    }
    _eq_b16_aa_x2(unsafe { a_ptr.add(16 * 2) }, unsafe { b_ptr.add(16 * 2) })
}

#[inline(always)]
fn _eq_b16_aa_x8(a_ptr: *const u8, b_ptr: *const u8) -> bool {
    let r = _eq_b16_aa_x4(a_ptr, b_ptr);
    if !r {
        return r;
    }
    _eq_b16_aa_x4(unsafe { a_ptr.add(16 * 4) }, unsafe { b_ptr.add(16 * 4) })
}

#[inline(always)]
fn _eq_b16_aa_x16(a_ptr: *const u8, b_ptr: *const u8) -> bool {
    let r = _eq_b16_aa_x8(a_ptr, b_ptr);
    if !r {
        return r;
    }
    _eq_b16_aa_x8(unsafe { a_ptr.add(16 * 8) }, unsafe { b_ptr.add(16 * 8) })
}

#[inline(always)]
fn _eq_b8_aa_x1(a_ptr: *const u8, b_ptr: *const u8) -> bool {
    let ac = unsafe { _read_a_native_endian_from_ptr_u64(a_ptr) };
    let bc = unsafe { _read_a_native_endian_from_ptr_u64(b_ptr) };
    ac == bc
}

#[inline(always)]
fn _eq_b8_aa_x2(a_ptr: *const u8, b_ptr: *const u8) -> bool {
    let r = _eq_b8_aa_x1(a_ptr, b_ptr);
    if !r {
        return r;
    }
    _eq_b8_aa_x1(unsafe { a_ptr.add(8) }, unsafe { b_ptr.add(8) })
}

#[inline(always)]
fn _eq_b8_aa_x4(a_ptr: *const u8, b_ptr: *const u8) -> bool {
    let r = _eq_b8_aa_x2(a_ptr, b_ptr);
    if !r {
        return r;
    }
    _eq_b8_aa_x2(unsafe { a_ptr.add(8 * 2) }, unsafe { b_ptr.add(8 * 2) })
}

#[inline(always)]
fn _eq_b8_aa_x8(a_ptr: *const u8, b_ptr: *const u8) -> bool {
    let r = _eq_b8_aa_x4(a_ptr, b_ptr);
    if !r {
        return r;
    }
    _eq_b8_aa_x4(unsafe { a_ptr.add(8 * 4) }, unsafe { b_ptr.add(8 * 4) })
}

#[inline(always)]
fn _eq_b8_aa_x16(a_ptr: *const u8, b_ptr: *const u8) -> bool {
    let r = _eq_b8_aa_x8(a_ptr, b_ptr);
    if !r {
        return r;
    }
    _eq_b8_aa_x8(unsafe { a_ptr.add(8 * 8) }, unsafe { b_ptr.add(8 * 8) })
}

#[inline(always)]
fn _eq_b4_aa_x1(a_ptr: *const u8, b_ptr: *const u8) -> bool {
    let ac = unsafe { _read_a_native_endian_from_ptr_u32(a_ptr) };
    let bc = unsafe { _read_a_native_endian_from_ptr_u32(b_ptr) };
    ac == bc
}

#[inline(always)]
fn _eq_b4_aa_x2(a_ptr: *const u8, b_ptr: *const u8) -> bool {
    let r = _eq_b4_aa_x1(a_ptr, b_ptr);
    if !r {
        return r;
    }
    _eq_b4_aa_x1(unsafe { a_ptr.add(4) }, unsafe { b_ptr.add(4) })
}

#[inline(always)]
fn _eq_b4_aa_x4(a_ptr: *const u8, b_ptr: *const u8) -> bool {
    let r = _eq_b4_aa_x2(a_ptr, b_ptr);
    if !r {
        return r;
    }
    _eq_b4_aa_x2(unsafe { a_ptr.add(4 * 2) }, unsafe { b_ptr.add(4 * 2) })
}

#[inline(always)]
fn _eq_b4_aa_x8(a_ptr: *const u8, b_ptr: *const u8) -> bool {
    let r = _eq_b4_aa_x4(a_ptr, b_ptr);
    if !r {
        return r;
    }
    _eq_b4_aa_x4(unsafe { a_ptr.add(4 * 4) }, unsafe { b_ptr.add(4 * 4) })
}

#[inline(always)]
fn _eq_b4_aa_x16(a_ptr: *const u8, b_ptr: *const u8) -> bool {
    let r = _eq_b4_aa_x8(a_ptr, b_ptr);
    if !r {
        return r;
    }
    _eq_b4_aa_x8(unsafe { a_ptr.add(4 * 8) }, unsafe { b_ptr.add(4 * 8) })
}

#[inline(always)]
fn _eq_b2_aa_x1(a_ptr: *const u8, b_ptr: *const u8) -> bool {
    let ac = unsafe { _read_a_native_endian_from_ptr_u16(a_ptr) };
    let bc = unsafe { _read_a_native_endian_from_ptr_u16(b_ptr) };
    ac == bc
}

#[inline(always)]
fn _eq_b2_aa_x2(a_ptr: *const u8, b_ptr: *const u8) -> bool {
    let r = _eq_b2_aa_x1(a_ptr, b_ptr);
    if !r {
        return r;
    }
    _eq_b2_aa_x1(unsafe { a_ptr.add(2) }, unsafe { b_ptr.add(2) })
}

#[inline(always)]
fn _eq_b2_aa_x4(a_ptr: *const u8, b_ptr: *const u8) -> bool {
    let r = _eq_b2_aa_x2(a_ptr, b_ptr);
    if !r {
        return r;
    }
    _eq_b2_aa_x2(unsafe { a_ptr.add(2 * 2) }, unsafe { b_ptr.add(2 * 2) })
}

#[inline(always)]
fn _eq_b2_aa_x8(a_ptr: *const u8, b_ptr: *const u8) -> bool {
    let r = _eq_b2_aa_x4(a_ptr, b_ptr);
    if !r {
        return r;
    }
    _eq_b2_aa_x4(unsafe { a_ptr.add(2 * 4) }, unsafe { b_ptr.add(2 * 4) })
}

#[inline(always)]
fn _eq_b2_aa_x16(a_ptr: *const u8, b_ptr: *const u8) -> bool {
    let r = _eq_b2_aa_x8(a_ptr, b_ptr);
    if !r {
        return r;
    }
    _eq_b2_aa_x8(unsafe { a_ptr.add(2 * 8) }, unsafe { b_ptr.add(2 * 8) })
}

#[inline(always)]
fn _eq_b1_aa_x1(a_ptr: *const u8, b_ptr: *const u8) -> bool {
    let ac = unsafe { *a_ptr };
    let bc = unsafe { *b_ptr };
    ac == bc
}

#[inline(always)]
fn _eq_b1_aa_x2(a_ptr: *const u8, b_ptr: *const u8) -> bool {
    let r = _eq_b1_aa_x1(a_ptr, b_ptr);
    if !r {
        return r;
    }
    _eq_b1_aa_x1(unsafe { a_ptr.add(1) }, unsafe { b_ptr.add(1) })
}

#[inline(always)]
fn _eq_b1_aa_x4(a_ptr: *const u8, b_ptr: *const u8) -> bool {
    let r = _eq_b1_aa_x2(a_ptr, b_ptr);
    if !r {
        return r;
    }
    _eq_b1_aa_x2(unsafe { a_ptr.add(2) }, unsafe { b_ptr.add(2) })
}

#[inline(always)]
fn _eq_b1_aa_x8(a_ptr: *const u8, b_ptr: *const u8) -> bool {
    let r = _eq_b1_aa_x4(a_ptr, b_ptr);
    if !r {
        return r;
    }
    _eq_b1_aa_x4(unsafe { a_ptr.add(4) }, unsafe { b_ptr.add(4) })
}

/*
 * The simple implement:

#[inline(always)]
pub fn _memeq_impl(a: &[u8], b: &[u8]) -> bool {
    let a_len = a.len();
    let b_len = b.len();
    if a_len != b_len {
        return false;
    }
    for i in 0..a_len {
        if a[i] != b[i] {
            return false;
        }
    }
    true
}
*/

#[cfg(test)]
mod disasm {
    use super::*;
    //
    #[test]
    fn do_procs() {
        let a = b"abcdefg".to_vec();
        let b = b"abcdefg".to_vec();
        let a = a.as_slice();
        let b = b.as_slice();
        assert!(do_proc_basic(a, b));
        #[cfg(target_pointer_width = "128")]
        assert!(do_proc_128(a, b));
        #[cfg(target_pointer_width = "64")]
        assert!(do_proc_64(a, b));
        #[cfg(target_pointer_width = "32")]
        assert!(do_proc_32(a, b));
    }
    fn do_proc_basic(a: &[u8], b: &[u8]) -> bool {
        _memeq_impl(a, b)
    }
    #[cfg(target_pointer_width = "128")]
    fn do_proc_128(a: &[u8], b: &[u8]) -> bool {
        _start_eq_128(a, b)
    }
    #[cfg(target_pointer_width = "64")]
    fn do_proc_64(a: &[u8], b: &[u8]) -> bool {
        _start_eq_64(a, b)
    }
    #[cfg(target_pointer_width = "32")]
    fn do_proc_32(a: &[u8], b: &[u8]) -> bool {
        _start_eq_32(a, b)
    }
}
