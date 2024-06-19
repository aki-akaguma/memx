use crate::utils::*;
use core::cmp::Ordering;

#[inline(never)]
pub fn _memcmp_impl(a: &[u8], b: &[u8]) -> Ordering {
    #[cfg(all(
        feature = "test",
        any(feature = "test_pointer_width_64", feature = "test_pointer_width_32")
    ))]
    {
        #[cfg(feature = "test_pointer_width_64")]
        let r = _start_cmp_64(a, b);
        #[cfg(feature = "test_pointer_width_32")]
        let r = _start_cmp_32(a, b);
        //
        r
    }
    #[cfg(not(all(
        feature = "test",
        any(feature = "test_pointer_width_64", feature = "test_pointer_width_32")
    )))]
    {
        #[cfg(target_pointer_width = "64")]
        let r = _start_cmp_64(a, b);
        #[cfg(target_pointer_width = "32")]
        let r = _start_cmp_32(a, b);
        //
        r
    }
}

macro_rules! _unroll_one_cmp_to_aligned_x1 {
    ($a_ptr_2:expr, $b_ptr_2:expr, $a_ptr_end:expr, $label:tt) => {{
        if $a_ptr_2 >= $a_ptr_end {
            break $label;
        }
        let r = _cmp_b1_aa_x1($a_ptr_2, $b_ptr_2);
        if !r.is_eq() {
            return (None, Some(r));
        }
        $a_ptr_2 = unsafe { $a_ptr_2.add(1) };
        $b_ptr_2 = unsafe { $b_ptr_2.add(1) };
    }};
}

macro_rules! _unroll_one_cmp_to_aligned_x2 {
    ($a_ptr_2:expr, $b_ptr_2:expr, $a_ptr_end:expr, $label:tt) => {{
        _unroll_one_cmp_to_aligned_x1!($a_ptr_2, $b_ptr_2, $a_ptr_end, $label);
        _unroll_one_cmp_to_aligned_x1!($a_ptr_2, $b_ptr_2, $a_ptr_end, $label);
    }};
}

macro_rules! _unroll_one_cmp_to_aligned_x4 {
    ($a_ptr_2:expr, $b_ptr_2:expr, $a_ptr_end:expr, $label:tt) => {{
        _unroll_one_cmp_to_aligned_x2!($a_ptr_2, $b_ptr_2, $a_ptr_end, $label);
        _unroll_one_cmp_to_aligned_x2!($a_ptr_2, $b_ptr_2, $a_ptr_end, $label);
    }};
}

macro_rules! _unroll_one_cmp_to_aligned_x8 {
    ($a_ptr_2:expr, $b_ptr_2:expr, $a_ptr_end:expr, $label:tt) => {{
        _unroll_one_cmp_to_aligned_x4!($a_ptr_2, $b_ptr_2, $a_ptr_end, $label);
        _unroll_one_cmp_to_aligned_x4!($a_ptr_2, $b_ptr_2, $a_ptr_end, $label);
    }};
}

macro_rules! _unroll_one_cmp_to_aligned_x16 {
    ($a_ptr_2:expr, $b_ptr_2:expr, $a_ptr_end:expr, $label:tt) => {{
        _unroll_one_cmp_to_aligned_x8!($a_ptr_2, $b_ptr_2, $a_ptr_end, $label);
        _unroll_one_cmp_to_aligned_x8!($a_ptr_2, $b_ptr_2, $a_ptr_end, $label);
    }};
}

#[inline(always)]
pub(crate) fn _cmp_to_aligned_u256(
    a_ptr: *const u8,
    b_ptr: *const u8,
) -> (Option<(*const u8, *const u8)>, Option<Ordering>) {
    let remaining_align = 0x20_usize - ((a_ptr as usize) & 0x1F_usize);
    let a_ptr_end = unsafe { a_ptr.add(remaining_align) };
    let b_ptr_end = unsafe { b_ptr.add(remaining_align) };
    let mut a_ptr_2 = a_ptr;
    let mut b_ptr_2 = b_ptr;
    'near: loop {
        _unroll_one_cmp_to_aligned_x16!(a_ptr_2, b_ptr_2, a_ptr_end, 'near);
        _unroll_one_cmp_to_aligned_x16!(a_ptr_2, b_ptr_2, a_ptr_end, 'near);
    }
    (Some((a_ptr_end, b_ptr_end)), None)
}

#[inline(always)]
pub(crate) fn _cmp_to_aligned_u128(
    a_ptr: *const u8,
    b_ptr: *const u8,
) -> (Option<(*const u8, *const u8)>, Option<Ordering>) {
    let remaining_align = 0x10_usize - ((a_ptr as usize) & 0x0F_usize);
    let a_ptr_end = unsafe { a_ptr.add(remaining_align) };
    let b_ptr_end = unsafe { b_ptr.add(remaining_align) };
    let mut a_ptr_2 = a_ptr;
    let mut b_ptr_2 = b_ptr;
    'near: loop {
        _unroll_one_cmp_to_aligned_x16!(a_ptr_2, b_ptr_2, a_ptr_end, 'near);
    }
    (Some((a_ptr_end, b_ptr_end)), None)
}

#[inline(always)]
pub(crate) fn _cmp_to_aligned_u64(
    a_ptr: *const u8,
    b_ptr: *const u8,
) -> (Option<(*const u8, *const u8)>, Option<Ordering>) {
    let remaining_align = 0x08_usize - ((a_ptr as usize) & 0x07_usize);
    let a_ptr_end = unsafe { a_ptr.add(remaining_align) };
    let b_ptr_end = unsafe { b_ptr.add(remaining_align) };
    let mut a_ptr_2 = a_ptr;
    let mut b_ptr_2 = b_ptr;
    'near: loop {
        _unroll_one_cmp_to_aligned_x8!(a_ptr_2, b_ptr_2, a_ptr_end, 'near);
    }
    (Some((a_ptr_end, b_ptr_end)), None)
}

#[inline(always)]
fn _cmp_to_aligned_u32(
    a_ptr: *const u8,
    b_ptr: *const u8,
) -> (Option<(*const u8, *const u8)>, Option<Ordering>) {
    let remaining_align = 0x04_usize - ((a_ptr as usize) & 0x03_usize);
    let a_ptr_end = unsafe { a_ptr.add(remaining_align) };
    let b_ptr_end = unsafe { b_ptr.add(remaining_align) };
    let mut a_ptr_2 = a_ptr;
    let mut b_ptr_2 = b_ptr;
    'near: loop {
        _unroll_one_cmp_to_aligned_x4!(a_ptr_2, b_ptr_2, a_ptr_end, 'near);
    }
    (Some((a_ptr_end, b_ptr_end)), None)
}

//#[cfg(any(target_pointer_width = "128", feature = "test_pointer_width_128"))]
#[inline(always)]
pub(crate) fn _start_cmp_128(a: &[u8], b: &[u8]) -> Ordering {
    //
    let a_len = a.len();
    let b_len = b.len();
    let min_len = a_len.min(b_len);
    let mut a_ptr = a.as_ptr();
    let mut b_ptr = b.as_ptr();
    let end_ptr = unsafe { a_ptr.add(min_len) };
    a_ptr.prefetch_read_data();
    b_ptr.prefetch_read_data();
    //
    if min_len >= 16 {
        // to a aligned pointer
        {
            if !a_ptr.is_aligned_u128() {
                let r = _cmp_to_aligned_u128(a_ptr, b_ptr);
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
                let unroll = 8;
                let loop_size = 16;
                while a_ptr.is_not_over(end_ptr, loop_size * unroll) {
                    let r = _cmp_b16_aa_x8(a_ptr, b_ptr);
                    if !r.is_eq() {
                        return r;
                    }
                    a_ptr = unsafe { a_ptr.add(loop_size * unroll) };
                    b_ptr = unsafe { b_ptr.add(loop_size * unroll) };
                }
            }
            {
                let unroll = 1;
                let loop_size = 16;
                while a_ptr.is_not_over(end_ptr, loop_size * unroll) {
                    let r = _cmp_b16_aa_x1(a_ptr, b_ptr);
                    if !r.is_eq() {
                        return r;
                    }
                    a_ptr = unsafe { a_ptr.add(loop_size * unroll) };
                    b_ptr = unsafe { b_ptr.add(loop_size * unroll) };
                }
            }
        } else {
            {
                let unroll = 1;
                let loop_size = 16;
                while a_ptr.is_not_over(end_ptr, loop_size * unroll) {
                    let r = _cmp_b16_au_x1(a_ptr, b_ptr);
                    if !r.is_eq() {
                        return r;
                    }
                    a_ptr = unsafe { a_ptr.add(loop_size * unroll) };
                    b_ptr = unsafe { b_ptr.add(loop_size * unroll) };
                }
            }
        }
    }
    // the remaining data is the max: 15 bytes.
    _memcmp_remaining_15_bytes_impl(a_ptr, b_ptr, a_len, b_len, end_ptr)
}

//#[cfg(any(target_pointer_width = "64", feature = "test_pointer_width_64"))]
#[inline(always)]
pub(crate) fn _start_cmp_64(a: &[u8], b: &[u8]) -> Ordering {
    //
    let a_len = a.len();
    let b_len = b.len();
    let min_len = a_len.min(b_len);
    let mut a_ptr = a.as_ptr();
    let mut b_ptr = b.as_ptr();
    let end_ptr = unsafe { a_ptr.add(min_len) };
    a_ptr.prefetch_read_data();
    b_ptr.prefetch_read_data();
    //
    if min_len >= 8 {
        // to a aligned pointer
        {
            if !a_ptr.is_aligned_u64() {
                let r = _cmp_to_aligned_u64(a_ptr, b_ptr);
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
                let unroll = 8;
                let loop_size = 8;
                while a_ptr.is_not_over(end_ptr, loop_size * unroll) {
                    let r = _cmp_b8_aa_x8(a_ptr, b_ptr);
                    if !r.is_eq() {
                        return r;
                    }
                    a_ptr = unsafe { a_ptr.add(loop_size * unroll) };
                    b_ptr = unsafe { b_ptr.add(loop_size * unroll) };
                }
            }
            {
                let unroll = 1;
                let loop_size = 8;
                while a_ptr.is_not_over(end_ptr, loop_size * unroll) {
                    let r = _cmp_b8_aa_x1(a_ptr, b_ptr);
                    if !r.is_eq() {
                        return r;
                    }
                    a_ptr = unsafe { a_ptr.add(loop_size * unroll) };
                    b_ptr = unsafe { b_ptr.add(loop_size * unroll) };
                }
            }
        } else {
            {
                let unroll = 1;
                let loop_size = 8;
                while a_ptr.is_not_over(end_ptr, loop_size * unroll) {
                    let r = _cmp_b8_au_x1(a_ptr, b_ptr);
                    if !r.is_eq() {
                        return r;
                    }
                    a_ptr = unsafe { a_ptr.add(loop_size * unroll) };
                    b_ptr = unsafe { b_ptr.add(loop_size * unroll) };
                }
            }
        }
    }
    // the remaining data is the max: 7 bytes.
    _memcmp_remaining_7_bytes_impl(a_ptr, b_ptr, a_len, b_len, end_ptr)
}

//#[cfg(any(target_pointer_width = "32", feature = "test_pointer_width_32"))]
#[inline(always)]
fn _start_cmp_32(a: &[u8], b: &[u8]) -> Ordering {
    //
    let a_len = a.len();
    let b_len = b.len();
    let min_len = a_len.min(b_len);
    let mut a_ptr = a.as_ptr();
    let mut b_ptr = b.as_ptr();
    let end_ptr = unsafe { a_ptr.add(min_len) };
    a_ptr.prefetch_read_data();
    b_ptr.prefetch_read_data();
    //
    if min_len >= 4 {
        // to a aligned pointer
        {
            if !a_ptr.is_aligned_u32() {
                let r = _cmp_to_aligned_u32(a_ptr, b_ptr);
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
                    let r = _cmp_b4_aa_x16(a_ptr, b_ptr);
                    if !r.is_eq() {
                        return r;
                    }
                    a_ptr = unsafe { a_ptr.add(loop_size * unroll) };
                    b_ptr = unsafe { b_ptr.add(loop_size * unroll) };
                }
            }
            {
                let unroll = 1;
                let loop_size = 4;
                while a_ptr.is_not_over(end_ptr, loop_size * unroll) {
                    let r = _cmp_b4_aa_x1(a_ptr, b_ptr);
                    if !r.is_eq() {
                        return r;
                    }
                    a_ptr = unsafe { a_ptr.add(loop_size * unroll) };
                    b_ptr = unsafe { b_ptr.add(loop_size * unroll) };
                }
            }
        } else {
            {
                let unroll = 1;
                let loop_size = 4;
                while a_ptr.is_not_over(end_ptr, loop_size * unroll) {
                    let r = _cmp_b4_au_x1(a_ptr, b_ptr);
                    if !r.is_eq() {
                        return r;
                    }
                    a_ptr = unsafe { a_ptr.add(loop_size * unroll) };
                    b_ptr = unsafe { b_ptr.add(loop_size * unroll) };
                }
            }
        }
    }
    // the remaining data is the max: 3 bytes.
    _memcmp_remaining_3_bytes_impl(a_ptr, b_ptr, a_len, b_len, end_ptr)
}

#[inline(always)]
pub(crate) fn _memcmp_remaining_15_bytes_impl(
    a_ptr: *const u8,
    b_ptr: *const u8,
    a_len: usize,
    b_len: usize,
    end_ptr: *const u8,
) -> Ordering {
    let mut a_ptr = a_ptr;
    let mut b_ptr = b_ptr;
    if a_ptr.is_aligned_u64() {
        let loop_size = 8;
        if a_ptr.is_not_over(end_ptr, loop_size) {
            let eend_ptr = unsafe { end_ptr.sub(loop_size) };
            if b_ptr.is_aligned_u64() {
                'near_aa: loop {
                    for _ in 0..2 {
                        if a_ptr >= eend_ptr {
                            break 'near_aa;
                        }
                        let r = _cmp_b8_aa_x1(a_ptr, b_ptr);
                        if !r.is_eq() {
                            return r;
                        }
                        a_ptr = unsafe { a_ptr.add(loop_size) };
                        b_ptr = unsafe { b_ptr.add(loop_size) };
                    }
                }
            } else {
                'near_au: loop {
                    for _ in 0..2 {
                        if a_ptr >= eend_ptr {
                            break 'near_au;
                        }
                        let r = _cmp_b8_au_x1(a_ptr, b_ptr);
                        if !r.is_eq() {
                            return r;
                        }
                        a_ptr = unsafe { a_ptr.add(loop_size) };
                        b_ptr = unsafe { b_ptr.add(loop_size) };
                    }
                }
            }
        }
    }
    // the remaining data is the max: 7 bytes.
    _memcmp_remaining_7_bytes_impl(a_ptr, b_ptr, a_len, b_len, end_ptr)
}

#[inline(always)]
fn _memcmp_remaining_7_bytes_impl(
    a_ptr: *const u8,
    b_ptr: *const u8,
    a_len: usize,
    b_len: usize,
    end_ptr: *const u8,
) -> Ordering {
    let mut a_ptr = a_ptr;
    let mut b_ptr = b_ptr;
    if a_ptr.is_aligned_u32() {
        let loop_size = 4;
        if a_ptr.is_not_over(end_ptr, loop_size) {
            let eend_ptr = unsafe { end_ptr.sub(loop_size) };
            if b_ptr.is_aligned_u32() {
                'near_aa: loop {
                    for _ in 0..2 {
                        if a_ptr >= eend_ptr {
                            break 'near_aa;
                        }
                        let r = _cmp_b4_aa_x1(a_ptr, b_ptr);
                        if !r.is_eq() {
                            return r;
                        }
                        a_ptr = unsafe { a_ptr.add(loop_size) };
                        b_ptr = unsafe { b_ptr.add(loop_size) };
                    }
                }
            } else {
                'near_au: loop {
                    for _ in 0..2 {
                        if a_ptr >= eend_ptr {
                            break 'near_au;
                        }
                        let r = _cmp_b4_au_x1(a_ptr, b_ptr);
                        if !r.is_eq() {
                            return r;
                        }
                        a_ptr = unsafe { a_ptr.add(loop_size) };
                        b_ptr = unsafe { b_ptr.add(loop_size) };
                    }
                }
            }
        }
    }
    // the remaining data is the max: 3 bytes.
    _memcmp_remaining_3_bytes_impl(a_ptr, b_ptr, a_len, b_len, end_ptr)
}

#[inline(always)]
fn _memcmp_remaining_3_bytes_impl(
    a_ptr: *const u8,
    b_ptr: *const u8,
    a_len: usize,
    b_len: usize,
    end_ptr: *const u8,
) -> Ordering {
    let mut a_ptr = a_ptr;
    let mut b_ptr = b_ptr;
    if a_ptr.is_aligned_u16() {
        let loop_size = 2;
        if a_ptr.is_not_over(end_ptr, loop_size) {
            let eend_ptr = unsafe { end_ptr.sub(loop_size) };
            if b_ptr.is_aligned_u16() {
                'near_aa_2: loop {
                    for _ in 0..2 {
                        if a_ptr >= eend_ptr {
                            break 'near_aa_2;
                        }
                        let r = _cmp_b2_aa_x1(a_ptr, b_ptr);
                        if !r.is_eq() {
                            return r;
                        }
                        a_ptr = unsafe { a_ptr.add(loop_size) };
                        b_ptr = unsafe { b_ptr.add(loop_size) };
                    }
                }
            } else {
                'near_au_2: loop {
                    for _ in 0..2 {
                        if a_ptr >= eend_ptr {
                            break 'near_au_2;
                        }
                        let r = _cmp_b2_au_x1(a_ptr, b_ptr);
                        if !r.is_eq() {
                            return r;
                        }
                        a_ptr = unsafe { a_ptr.add(loop_size) };
                        b_ptr = unsafe { b_ptr.add(loop_size) };
                    }
                }
            }
        }
    }
    {
        let loop_size = 1;
        if a_ptr.is_not_over(end_ptr, loop_size) {
            'near_1: loop {
                for _ in 0..8 {
                    if a_ptr >= end_ptr {
                        break 'near_1;
                    }
                    let r = _cmp_b1_aa_x1(a_ptr, b_ptr);
                    if !r.is_eq() {
                        return r;
                    }
                    a_ptr = unsafe { a_ptr.add(loop_size) };
                    b_ptr = unsafe { b_ptr.add(loop_size) };
                }
            }
        }
    }
    //
    a_len.cmp(&b_len)
}

#[inline(always)]
fn _cmp_b16_ret(a_ptr: *const u8, b_ptr: *const u8, ac: u128, bc: u128) -> Ordering {
    #[cfg(target_endian = "big")]
    let (aac, bbc) = (aac.swap_bytes(), bbc.swap_bytes());
    #[cfg(not(target_endian = "big"))]
    let (aac, bbc) = (ac, bc);
    //
    assert!(aac != bbc);
    let bits = aac ^ bbc;
    let pos = (bits.trailing_zeros() / 8) as usize;
    let aa_ptr = unsafe { a_ptr.add(pos) };
    let bb_ptr = unsafe { b_ptr.add(pos) };
    let aac = unsafe { *aa_ptr };
    let bbc = unsafe { *bb_ptr };
    aac.cmp(&bbc)
}

#[inline(always)]
fn _cmp_b16_au_x1(a_ptr: *const u8, b_ptr: *const u8) -> Ordering {
    let ac = unsafe { (a_ptr as *const u128).read() };
    let bc = unsafe { (b_ptr as *const u128).read_unaligned() };
    if ac == bc {
        Ordering::Equal
    } else {
        _cmp_b16_ret(a_ptr, b_ptr, ac, bc)
    }
}

#[inline(always)]
fn _cmp_b16_aa_x1(a_ptr: *const u8, b_ptr: *const u8) -> Ordering {
    let ac = unsafe { (a_ptr as *const u128).read() };
    let bc = unsafe { (b_ptr as *const u128).read() };
    if ac == bc {
        Ordering::Equal
    } else {
        _cmp_b16_ret(a_ptr, b_ptr, ac, bc)
    }
}

#[inline(always)]
fn _cmp_b16_aa_x2(a_ptr: *const u8, b_ptr: *const u8) -> Ordering {
    let r = _cmp_b16_aa_x1(a_ptr, b_ptr);
    if !r.is_eq() {
        return r;
    }
    _cmp_b16_aa_x1(unsafe { a_ptr.add(16) }, unsafe { b_ptr.add(16) })
}

#[inline(always)]
fn _cmp_b16_aa_x4(a_ptr: *const u8, b_ptr: *const u8) -> Ordering {
    let r = _cmp_b16_aa_x2(a_ptr, b_ptr);
    if !r.is_eq() {
        return r;
    }
    _cmp_b16_aa_x2(unsafe { a_ptr.add(16 * 2) }, unsafe { b_ptr.add(16 * 2) })
}

#[inline(always)]
fn _cmp_b16_aa_x8(a_ptr: *const u8, b_ptr: *const u8) -> Ordering {
    let r = _cmp_b16_aa_x4(a_ptr, b_ptr);
    if !r.is_eq() {
        return r;
    }
    _cmp_b16_aa_x4(unsafe { a_ptr.add(16 * 4) }, unsafe { b_ptr.add(16 * 4) })
}

#[inline(always)]
fn _cmp_b16_aa_x16(a_ptr: *const u8, b_ptr: *const u8) -> Ordering {
    let r = _cmp_b16_aa_x8(a_ptr, b_ptr);
    if !r.is_eq() {
        return r;
    }
    _cmp_b16_aa_x8(unsafe { a_ptr.add(16 * 8) }, unsafe { b_ptr.add(16 * 8) })
}

#[inline(always)]
fn _cmp_b8_ret(a_ptr: *const u8, b_ptr: *const u8, ac: u64, bc: u64) -> Ordering {
    #[cfg(target_endian = "big")]
    let (aac, bbc) = (aac.swap_bytes(), bbc.swap_bytes());
    #[cfg(not(target_endian = "big"))]
    let (aac, bbc) = (ac, bc);
    //
    assert!(aac != bbc);
    let bits = aac ^ bbc;
    let pos = (bits.trailing_zeros() / 8) as usize;
    let aa_ptr = unsafe { a_ptr.add(pos) };
    let bb_ptr = unsafe { b_ptr.add(pos) };
    let aac = unsafe { *aa_ptr };
    let bbc = unsafe { *bb_ptr };
    aac.cmp(&bbc)
}

#[inline(always)]
fn _cmp_b8_uu_x1(a_ptr: *const u8, b_ptr: *const u8) -> Ordering {
    let ac = unsafe { (a_ptr as *const u64).read_unaligned() };
    let bc = unsafe { (b_ptr as *const u64).read_unaligned() };
    if ac == bc {
        Ordering::Equal
    } else {
        _cmp_b8_ret(a_ptr, b_ptr, ac, bc)
    }
}

#[inline(always)]
fn _cmp_b8_au_x1(a_ptr: *const u8, b_ptr: *const u8) -> Ordering {
    let ac = unsafe { (a_ptr as *const u64).read() };
    let bc = unsafe { (b_ptr as *const u64).read_unaligned() };
    if ac == bc {
        Ordering::Equal
    } else {
        _cmp_b8_ret(a_ptr, b_ptr, ac, bc)
    }
}

#[inline(always)]
fn _cmp_b8_aa_x1(a_ptr: *const u8, b_ptr: *const u8) -> Ordering {
    let ac = unsafe { (a_ptr as *const u64).read() };
    let bc = unsafe { (b_ptr as *const u64).read() };
    if ac == bc {
        Ordering::Equal
    } else {
        _cmp_b8_ret(a_ptr, b_ptr, ac, bc)
    }
}

#[inline(always)]
fn _cmp_b8_aa_x2(a_ptr: *const u8, b_ptr: *const u8) -> Ordering {
    let r = _cmp_b8_aa_x1(a_ptr, b_ptr);
    if !r.is_eq() {
        return r;
    }
    _cmp_b8_aa_x1(unsafe { a_ptr.add(8) }, unsafe { b_ptr.add(8) })
}

#[inline(always)]
fn _cmp_b8_aa_x4(a_ptr: *const u8, b_ptr: *const u8) -> Ordering {
    let r = _cmp_b8_aa_x2(a_ptr, b_ptr);
    if !r.is_eq() {
        return r;
    }
    _cmp_b8_aa_x2(unsafe { a_ptr.add(8 * 2) }, unsafe { b_ptr.add(8 * 2) })
}

#[inline(always)]
fn _cmp_b8_aa_x8(a_ptr: *const u8, b_ptr: *const u8) -> Ordering {
    let r = _cmp_b8_aa_x4(a_ptr, b_ptr);
    if !r.is_eq() {
        return r;
    }
    _cmp_b8_aa_x4(unsafe { a_ptr.add(8 * 4) }, unsafe { b_ptr.add(8 * 4) })
}

#[inline(always)]
fn _cmp_b8_aa_x16(a_ptr: *const u8, b_ptr: *const u8) -> Ordering {
    let r = _cmp_b8_aa_x8(a_ptr, b_ptr);
    if !r.is_eq() {
        return r;
    }
    _cmp_b8_aa_x8(unsafe { a_ptr.add(8 * 8) }, unsafe { b_ptr.add(8 * 8) })
}

#[inline(always)]
fn _cmp_b4_ret(a_ptr: *const u8, b_ptr: *const u8, ac: u32, bc: u32) -> Ordering {
    #[cfg(target_endian = "big")]
    let (aac, bbc) = (aac.swap_bytes(), bbc.swap_bytes());
    #[cfg(not(target_endian = "big"))]
    let (aac, bbc) = (ac, bc);
    //
    assert!(aac != bbc);
    let bits = aac ^ bbc;
    let pos = (bits.trailing_zeros() / 8) as usize;
    let aa_ptr = unsafe { a_ptr.add(pos) };
    let bb_ptr = unsafe { b_ptr.add(pos) };
    let aac = unsafe { *aa_ptr };
    let bbc = unsafe { *bb_ptr };
    aac.cmp(&bbc)
}

#[inline(always)]
fn _cmp_b4_uu_x1(a_ptr: *const u8, b_ptr: *const u8) -> Ordering {
    let ac = unsafe { (a_ptr as *const u32).read_unaligned() };
    let bc = unsafe { (b_ptr as *const u32).read_unaligned() };
    if ac == bc {
        Ordering::Equal
    } else {
        _cmp_b4_ret(a_ptr, b_ptr, ac, bc)
    }
}

#[inline(always)]
fn _cmp_b4_au_x1(a_ptr: *const u8, b_ptr: *const u8) -> Ordering {
    let ac = unsafe { (a_ptr as *const u32).read() };
    let bc = unsafe { (b_ptr as *const u32).read_unaligned() };
    if ac == bc {
        Ordering::Equal
    } else {
        _cmp_b4_ret(a_ptr, b_ptr, ac, bc)
    }
}

#[inline(always)]
fn _cmp_b4_aa_x1(a_ptr: *const u8, b_ptr: *const u8) -> Ordering {
    let ac = unsafe { (a_ptr as *const u32).read() };
    let bc = unsafe { (b_ptr as *const u32).read() };
    if ac == bc {
        Ordering::Equal
    } else {
        _cmp_b4_ret(a_ptr, b_ptr, ac, bc)
    }
}

#[inline(always)]
fn _cmp_b4_aa_x2(a_ptr: *const u8, b_ptr: *const u8) -> Ordering {
    let r = _cmp_b4_aa_x1(a_ptr, b_ptr);
    if !r.is_eq() {
        return r;
    }
    _cmp_b4_aa_x1(unsafe { a_ptr.add(4) }, unsafe { b_ptr.add(4) })
}

#[inline(always)]
fn _cmp_b4_aa_x4(a_ptr: *const u8, b_ptr: *const u8) -> Ordering {
    let r = _cmp_b4_aa_x2(a_ptr, b_ptr);
    if !r.is_eq() {
        return r;
    }
    _cmp_b4_aa_x2(unsafe { a_ptr.add(4 * 2) }, unsafe { b_ptr.add(4 * 2) })
}

#[inline(always)]
fn _cmp_b4_aa_x8(a_ptr: *const u8, b_ptr: *const u8) -> Ordering {
    let r = _cmp_b4_aa_x4(a_ptr, b_ptr);
    if !r.is_eq() {
        return r;
    }
    _cmp_b4_aa_x4(unsafe { a_ptr.add(4 * 4) }, unsafe { b_ptr.add(4 * 4) })
}

#[inline(always)]
fn _cmp_b4_aa_x16(a_ptr: *const u8, b_ptr: *const u8) -> Ordering {
    let r = _cmp_b4_aa_x8(a_ptr, b_ptr);
    if !r.is_eq() {
        return r;
    }
    _cmp_b4_aa_x8(unsafe { a_ptr.add(4 * 8) }, unsafe { b_ptr.add(4 * 8) })
}

#[inline(always)]
fn _cmp_b2_ret(a_ptr: *const u8, b_ptr: *const u8, ac: u16, bc: u16) -> Ordering {
    #[cfg(target_endian = "big")]
    let (aac, bbc) = (aac.swap_bytes(), bbc.swap_bytes());
    #[cfg(not(target_endian = "big"))]
    let (aac, bbc) = (ac, bc);
    //
    assert!(aac != bbc);
    let bits = aac ^ bbc;
    let pos = (bits.trailing_zeros() / 8) as usize;
    let aa_ptr = unsafe { a_ptr.add(pos) };
    let bb_ptr = unsafe { b_ptr.add(pos) };
    let aac = unsafe { *aa_ptr };
    let bbc = unsafe { *bb_ptr };
    aac.cmp(&bbc)
}

#[inline(always)]
fn _cmp_b2_uu_x1(a_ptr: *const u8, b_ptr: *const u8) -> Ordering {
    let ac = unsafe { (a_ptr as *const u16).read_unaligned() };
    let bc = unsafe { (b_ptr as *const u16).read_unaligned() };
    if ac == bc {
        Ordering::Equal
    } else {
        _cmp_b2_ret(a_ptr, b_ptr, ac, bc)
    }
}

#[inline(always)]
fn _cmp_b2_au_x1(a_ptr: *const u8, b_ptr: *const u8) -> Ordering {
    let ac = unsafe { (a_ptr as *const u16).read() };
    let bc = unsafe { (b_ptr as *const u16).read_unaligned() };
    if ac == bc {
        Ordering::Equal
    } else {
        _cmp_b2_ret(a_ptr, b_ptr, ac, bc)
    }
}

#[inline(always)]
fn _cmp_b2_aa_x1(a_ptr: *const u8, b_ptr: *const u8) -> Ordering {
    //let ac = unsafe { _read_a_native_endian_from_ptr_u16(a_ptr) };
    //let bc = unsafe { _read_a_native_endian_from_ptr_u16(b_ptr) };
    let ac = unsafe { (a_ptr as *const u16).read() };
    let bc = unsafe { (b_ptr as *const u16).read() };
    if ac == bc {
        Ordering::Equal
    } else {
        _cmp_b2_ret(a_ptr, b_ptr, ac, bc)
    }
}

#[inline(always)]
fn _cmp_b2_aa_x2(a_ptr: *const u8, b_ptr: *const u8) -> Ordering {
    let r = _cmp_b2_aa_x1(a_ptr, b_ptr);
    if !r.is_eq() {
        return r;
    }
    _cmp_b2_aa_x1(unsafe { a_ptr.add(2) }, unsafe { b_ptr.add(2) })
}

#[inline(always)]
fn _cmp_b2_aa_x4(a_ptr: *const u8, b_ptr: *const u8) -> Ordering {
    let r = _cmp_b2_aa_x2(a_ptr, b_ptr);
    if !r.is_eq() {
        return r;
    }
    _cmp_b2_aa_x2(unsafe { a_ptr.add(2 * 2) }, unsafe { b_ptr.add(2 * 2) })
}

#[inline(always)]
fn _cmp_b2_aa_x8(a_ptr: *const u8, b_ptr: *const u8) -> Ordering {
    let r = _cmp_b2_aa_x4(a_ptr, b_ptr);
    if !r.is_eq() {
        return r;
    }
    _cmp_b2_aa_x4(unsafe { a_ptr.add(2 * 4) }, unsafe { b_ptr.add(2 * 4) })
}

#[inline(always)]
fn _cmp_b1_aa_x1(a_ptr: *const u8, b_ptr: *const u8) -> Ordering {
    let ac = unsafe { *a_ptr };
    let bc = unsafe { *b_ptr };
    if ac == bc {
        Ordering::Equal
    } else {
        ac.cmp(&bc)
    }
}

#[inline(always)]
fn _cmp_b1_aa_x2(a_ptr: *const u8, b_ptr: *const u8) -> Ordering {
    let r = _cmp_b1_aa_x1(a_ptr, b_ptr);
    if !r.is_eq() {
        return r;
    }
    _cmp_b1_aa_x1(unsafe { a_ptr.add(1) }, unsafe { b_ptr.add(1) })
}

#[inline(always)]
fn _cmp_b1_aa_x4(a_ptr: *const u8, b_ptr: *const u8) -> Ordering {
    let r = _cmp_b1_aa_x2(a_ptr, b_ptr);
    if !r.is_eq() {
        return r;
    }
    _cmp_b1_aa_x2(unsafe { a_ptr.add(2) }, unsafe { b_ptr.add(2) })
}

#[inline(always)]
fn _cmp_b1_aa_x8(a_ptr: *const u8, b_ptr: *const u8) -> Ordering {
    let r = _cmp_b1_aa_x4(a_ptr, b_ptr);
    if !r.is_eq() {
        return r;
    }
    _cmp_b1_aa_x4(unsafe { a_ptr.add(4) }, unsafe { b_ptr.add(4) })
}

/*
 * The simple implement:

#[inline(always)]
pub fn _memcmp_impl(a: &[u8], b: &[u8]) -> Ordering {
    let a_len = a.len();
    let b_len = b.len();
    let min_len = a_len.min(b_len);
    for i in 0..min_len {
        let cmp = a[i].cmp(&b[i]);
        if cmp != Ordering::Equal {
            return cmp;
        }
    }
    a_len.cmp(&b_len)
}
*/

#[cfg(test)]
mod disasm {
    use super::*;
    use core::cmp::Ordering;
    //
    #[test]
    fn do_procs() {
        let a = b"abcdefg".to_vec();
        let b = b"abcdefg".to_vec();
        let a = a.as_slice();
        let b = b.as_slice();
        assert_eq!(do_proc_basic(a, b), Ordering::Equal);
        #[cfg(target_pointer_width = "64")]
        assert_eq!(do_proc_64(a, b), Ordering::Equal);
        #[cfg(target_pointer_width = "32")]
        assert_eq!(do_proc_32(a, b), Ordering::Equal);
    }
    fn do_proc_basic(a: &[u8], b: &[u8]) -> Ordering {
        _memcmp_impl(a, b)
    }
    #[cfg(target_pointer_width = "64")]
    fn do_proc_64(a: &[u8], b: &[u8]) -> Ordering {
        _start_cmp_64(a, b)
    }
    #[cfg(target_pointer_width = "32")]
    fn do_proc_32(a: &[u8], b: &[u8]) -> Ordering {
        _start_cmp_32(a, b)
    }
}
#[cfg(test)]
mod mini {
    #[test]
    fn t01() {
        use super::*;
        use core::cmp::Ordering;
        //
        let buf_16_1 = "0123456789abcdef".as_bytes().to_vec();
        let buf_16_2 = "0123456789abcdeF".as_bytes().to_vec();
        let r = _cmp_b16_aa_x1(buf_16_1.as_ptr(), buf_16_2.as_ptr());
        assert_eq!(r, Ordering::Greater);
        let buf_8_1 = "01234568".as_bytes().to_vec();
        let buf_8_2 = "01234567".as_bytes().to_vec();
        let r = _cmp_b8_aa_x1(buf_8_1.as_ptr(), buf_8_2.as_ptr());
        assert_eq!(r, Ordering::Greater);
        let buf_4_1 = "0124".as_bytes().to_vec();
        let buf_4_2 = "0123".as_bytes().to_vec();
        let r = _cmp_b4_aa_x1(buf_4_1.as_ptr(), buf_4_2.as_ptr());
        assert_eq!(r, Ordering::Greater);
        let buf_2_1 = "02".as_bytes().to_vec();
        let buf_2_2 = "01".as_bytes().to_vec();
        let r = _cmp_b2_aa_x1(buf_2_1.as_ptr(), buf_2_2.as_ptr());
        assert_eq!(r, Ordering::Greater);
    }
}
