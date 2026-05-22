use crate::utils::*;
use core::cmp::Ordering;

#[inline(never)]
pub fn _memcmp_impl(a: &[u8], b: &[u8]) -> Ordering {
    #[cfg(any(target_pointer_width = "64", feature = "test_pointer_width_64"))]
    let r = _start_cmp_64(a, b);
    #[cfg(all(
        not(any(target_pointer_width = "64", feature = "test_pointer_width_64")),
        any(target_pointer_width = "32", feature = "test_pointer_width_32")
    ))]
    let r = _start_cmp_32(a, b);
    //
    r
}

#[inline(always)]
fn _cmp_to_aligned<const ALIGN: usize>(
    a_ptr: *const u8,
    b_ptr: *const u8,
) -> (Option<(*const u8, *const u8)>, Option<Ordering>) {
    let remaining_align = ALIGN - ((a_ptr as usize) & (ALIGN - 1));
    let a_ptr_end = unsafe { a_ptr.add(remaining_align) };
    let mut a_ptr_cur = a_ptr;
    let mut b_ptr_cur = b_ptr;
    while a_ptr_cur < a_ptr_end {
        let r = _cmp_b1_aa_x1(a_ptr_cur, b_ptr_cur);
        if !r.is_eq() {
            return (None, Some(r));
        }
        a_ptr_cur = unsafe { a_ptr_cur.add(1) };
        b_ptr_cur = unsafe { b_ptr_cur.add(1) };
    }
    (
        Some((a_ptr_end, unsafe { b_ptr.add(remaining_align) })),
        None,
    )
}

#[inline(always)]
pub(crate) fn _cmp_to_aligned_u256(
    a_ptr: *const u8,
    b_ptr: *const u8,
) -> (Option<(*const u8, *const u8)>, Option<Ordering>) {
    _cmp_to_aligned::<32>(a_ptr, b_ptr)
}

#[inline(always)]
pub(crate) fn _cmp_to_aligned_u128(
    a_ptr: *const u8,
    b_ptr: *const u8,
) -> (Option<(*const u8, *const u8)>, Option<Ordering>) {
    _cmp_to_aligned::<16>(a_ptr, b_ptr)
}

#[inline(always)]
pub(crate) fn _cmp_to_aligned_u64(
    a_ptr: *const u8,
    b_ptr: *const u8,
) -> (Option<(*const u8, *const u8)>, Option<Ordering>) {
    _cmp_to_aligned::<8>(a_ptr, b_ptr)
}

#[inline(always)]
fn _cmp_to_aligned_u32(
    a_ptr: *const u8,
    b_ptr: *const u8,
) -> (Option<(*const u8, *const u8)>, Option<Ordering>) {
    _cmp_to_aligned::<4>(a_ptr, b_ptr)
}

//#[cfg(any(target_pointer_width = "128", feature = "test_pointer_width_128"))]
#[inline(always)]
pub(crate) fn _start_cmp_128(a: &[u8], b: &[u8]) -> Ordering {
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
            let (r, ap, bp) = _unroll_loop_dual_with_prefetch::<8, 16, Ordering, _>(a_ptr, b_ptr, end_ptr, |ap, bp| {
                let r = _cmp_b16_aa_x1(ap, bp);
                if !r.is_eq() { Some(r) } else { None }
            });
            if let Some(v) = r {
                return v;
            }
            a_ptr = ap;
            b_ptr = bp;
            //
            let (r, ap, bp) = _unroll_loop_dual::<1, 16, Ordering, _>(a_ptr, b_ptr, end_ptr, |ap, bp| {
                let r = _cmp_b16_aa_x1(ap, bp);
                if !r.is_eq() { Some(r) } else { None }
            });
            if let Some(v) = r {
                return v;
            }
            a_ptr = ap;
            b_ptr = bp;
        } else {
            let (r, ap, bp) = _unroll_loop_dual::<1, 16, Ordering, _>(a_ptr, b_ptr, end_ptr, |ap, bp| {
                let r = _cmp_b16_au_x1(ap, bp);
                if !r.is_eq() { Some(r) } else { None }
            });
            if let Some(v) = r {
                return v;
            }
            a_ptr = ap;
            b_ptr = bp;
        }
    }
    // the remaining data is the max: 15 bytes.
    _memcmp_remaining_15_bytes_impl(a_ptr, b_ptr, a_len, b_len, end_ptr)
}

#[cfg(any(target_pointer_width = "64", feature = "test_pointer_width_64"))]
#[inline(always)]
pub(crate) fn _start_cmp_64(a: &[u8], b: &[u8]) -> Ordering {
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
            let (r, ap, bp) = _unroll_loop_dual_with_prefetch::<8, 8, Ordering, _>(a_ptr, b_ptr, end_ptr, |ap, bp| {
                let r = _cmp_b8_aa_x1(ap, bp);
                if !r.is_eq() { Some(r) } else { None }
            });
            if let Some(v) = r {
                return v;
            }
            a_ptr = ap;
            b_ptr = bp;
            //
            let (r, ap, bp) = _unroll_loop_dual::<1, 8, Ordering, _>(a_ptr, b_ptr, end_ptr, |ap, bp| {
                let r = _cmp_b8_aa_x1(ap, bp);
                if !r.is_eq() { Some(r) } else { None }
            });
            if let Some(v) = r {
                return v;
            }
            a_ptr = ap;
            b_ptr = bp;
        } else {
            let (r, ap, bp) = _unroll_loop_dual::<1, 8, Ordering, _>(a_ptr, b_ptr, end_ptr, |ap, bp| {
                let r = _cmp_b8_au_x1(ap, bp);
                if !r.is_eq() { Some(r) } else { None }
            });
            if let Some(v) = r {
                return v;
            }
            a_ptr = ap;
            b_ptr = bp;
        }
    }
    // the remaining data is the max: 7 bytes.
    _memcmp_remaining_7_bytes_impl(a_ptr, b_ptr, a_len, b_len, end_ptr)
}

#[cfg(any(target_pointer_width = "32", feature = "test_pointer_width_32"))]
#[inline(always)]
fn _start_cmp_32(a: &[u8], b: &[u8]) -> Ordering {
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
            let (r, ap, bp) = _unroll_loop_dual_with_prefetch::<8, 4, Ordering, _>(a_ptr, b_ptr, end_ptr, |ap, bp| {
                let r = _cmp_b4_aa_x1(ap, bp);
                if !r.is_eq() { Some(r) } else { None }
            });
            if let Some(v) = r {
                return v;
            }
            a_ptr = ap;
            b_ptr = bp;
            //
            let (r, ap, bp) = _unroll_loop_dual::<1, 4, Ordering, _>(a_ptr, b_ptr, end_ptr, |ap, bp| {
                let r = _cmp_b4_aa_x1(ap, bp);
                if !r.is_eq() { Some(r) } else { None }
            });
            if let Some(v) = r {
                return v;
            }
            a_ptr = ap;
            b_ptr = bp;
        } else {
            let (r, ap, bp) = _unroll_loop_dual::<1, 4, Ordering, _>(a_ptr, b_ptr, end_ptr, |ap, bp| {
                let r = _cmp_b4_au_x1(ap, bp);
                if !r.is_eq() { Some(r) } else { None }
            });
            if let Some(v) = r {
                return v;
            }
            a_ptr = ap;
            b_ptr = bp;
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
            if b_ptr.is_aligned_u64() {
                let r = _cmp_b8_aa_x1(a_ptr, b_ptr);
                if !r.is_eq() {
                    return r;
                }
            } else {
                let r = _cmp_b8_au_x1(a_ptr, b_ptr);
                if !r.is_eq() {
                    return r;
                }
            }
            a_ptr = unsafe { a_ptr.add(loop_size) };
            b_ptr = unsafe { b_ptr.add(loop_size) };
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
            if b_ptr.is_aligned_u32() {
                let r = _cmp_b4_aa_x1(a_ptr, b_ptr);
                if !r.is_eq() {
                    return r;
                }
            } else {
                let r = _cmp_b4_au_x1(a_ptr, b_ptr);
                if !r.is_eq() {
                    return r;
                }
            }
            a_ptr = unsafe { a_ptr.add(loop_size) };
            b_ptr = unsafe { b_ptr.add(loop_size) };
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
            if b_ptr.is_aligned_u16() {
                let r = _cmp_b2_aa_x1(a_ptr, b_ptr);
                if !r.is_eq() {
                    return r;
                }
            } else {
                let r = _cmp_b2_au_x1(a_ptr, b_ptr);
                if !r.is_eq() {
                    return r;
                }
            }
            a_ptr = unsafe { a_ptr.add(loop_size) };
            b_ptr = unsafe { b_ptr.add(loop_size) };
        }
    }
    {
        let loop_size = 1;
        while a_ptr < end_ptr {
            let r = _cmp_b1_aa_x1(a_ptr, b_ptr);
            if !r.is_eq() {
                return r;
            }
            a_ptr = unsafe { a_ptr.add(loop_size) };
            b_ptr = unsafe { b_ptr.add(loop_size) };
        }
    }
    //
    a_len.cmp(&b_len)
}

#[inline(always)]
fn _cmp_b16_ret(a_ptr: *const u8, b_ptr: *const u8, ac: u128, bc: u128) -> Ordering {
    #[cfg(target_endian = "big")]
    let (aac, bbc) = (ac.swap_bytes(), bc.swap_bytes());
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
    let (r, _, _) = _unroll_loop_dual::<2, 16, Ordering, _>(a_ptr, b_ptr, unsafe { a_ptr.add(16 * 2) }, |ap, bp| {
        let r = _cmp_b16_aa_x1(ap, bp);
        if !r.is_eq() { Some(r) } else { None }
    });
    r.unwrap_or(Ordering::Equal)
}

#[inline(always)]
fn _cmp_b16_aa_x4(a_ptr: *const u8, b_ptr: *const u8) -> Ordering {
    let (r, _, _) = _unroll_loop_dual::<4, 16, Ordering, _>(a_ptr, b_ptr, unsafe { a_ptr.add(16 * 4) }, |ap, bp| {
        let r = _cmp_b16_aa_x1(ap, bp);
        if !r.is_eq() { Some(r) } else { None }
    });
    r.unwrap_or(Ordering::Equal)
}

#[inline(always)]
fn _cmp_b16_aa_x8(a_ptr: *const u8, b_ptr: *const u8) -> Ordering {
    let (r, _, _) = _unroll_loop_dual::<8, 16, Ordering, _>(a_ptr, b_ptr, unsafe { a_ptr.add(16 * 8) }, |ap, bp| {
        let r = _cmp_b16_aa_x1(ap, bp);
        if !r.is_eq() { Some(r) } else { None }
    });
    r.unwrap_or(Ordering::Equal)
}

#[inline(always)]
fn _cmp_b8_ret(a_ptr: *const u8, b_ptr: *const u8, ac: u64, bc: u64) -> Ordering {
    #[cfg(target_endian = "big")]
    let (aac, bbc) = (ac.swap_bytes(), bc.swap_bytes());
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
    let (r, _, _) = _unroll_loop_dual::<2, 8, Ordering, _>(a_ptr, b_ptr, unsafe { a_ptr.add(8 * 2) }, |ap, bp| {
        let r = _cmp_b8_aa_x1(ap, bp);
        if !r.is_eq() { Some(r) } else { None }
    });
    r.unwrap_or(Ordering::Equal)
}

#[inline(always)]
fn _cmp_b8_aa_x4(a_ptr: *const u8, b_ptr: *const u8) -> Ordering {
    let (r, _, _) = _unroll_loop_dual::<4, 8, Ordering, _>(a_ptr, b_ptr, unsafe { a_ptr.add(8 * 4) }, |ap, bp| {
        let r = _cmp_b8_aa_x1(ap, bp);
        if !r.is_eq() { Some(r) } else { None }
    });
    r.unwrap_or(Ordering::Equal)
}

#[inline(always)]
fn _cmp_b8_aa_x8(a_ptr: *const u8, b_ptr: *const u8) -> Ordering {
    let (r, _, _) = _unroll_loop_dual::<8, 8, Ordering, _>(a_ptr, b_ptr, unsafe { a_ptr.add(8 * 8) }, |ap, bp| {
        let r = _cmp_b8_aa_x1(ap, bp);
        if !r.is_eq() { Some(r) } else { None }
    });
    r.unwrap_or(Ordering::Equal)
}

#[inline(always)]
fn _cmp_b4_ret(a_ptr: *const u8, b_ptr: *const u8, ac: u32, bc: u32) -> Ordering {
    #[cfg(target_endian = "big")]
    let (aac, bbc) = (ac.swap_bytes(), bc.swap_bytes());
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
    let (r, _, _) = _unroll_loop_dual::<2, 4, Ordering, _>(a_ptr, b_ptr, unsafe { a_ptr.add(4 * 2) }, |ap, bp| {
        let r = _cmp_b4_aa_x1(ap, bp);
        if !r.is_eq() { Some(r) } else { None }
    });
    r.unwrap_or(Ordering::Equal)
}

#[inline(always)]
fn _cmp_b4_aa_x4(a_ptr: *const u8, b_ptr: *const u8) -> Ordering {
    let (r, _, _) = _unroll_loop_dual::<4, 4, Ordering, _>(a_ptr, b_ptr, unsafe { a_ptr.add(4 * 4) }, |ap, bp| {
        let r = _cmp_b4_aa_x1(ap, bp);
        if !r.is_eq() { Some(r) } else { None }
    });
    r.unwrap_or(Ordering::Equal)
}

#[inline(always)]
fn _cmp_b4_aa_x8(a_ptr: *const u8, b_ptr: *const u8) -> Ordering {
    let (r, _, _) = _unroll_loop_dual::<8, 4, Ordering, _>(a_ptr, b_ptr, unsafe { a_ptr.add(4 * 8) }, |ap, bp| {
        let r = _cmp_b4_aa_x1(ap, bp);
        if !r.is_eq() { Some(r) } else { None }
    });
    r.unwrap_or(Ordering::Equal)
}

#[inline(always)]
fn _cmp_b4_aa_x16(a_ptr: *const u8, b_ptr: *const u8) -> Ordering {
    let (r, _, _) = _unroll_loop_dual::<16, 4, Ordering, _>(a_ptr, b_ptr, unsafe { a_ptr.add(4 * 16) }, |ap, bp| {
        let r = _cmp_b4_aa_x1(ap, bp);
        if !r.is_eq() { Some(r) } else { None }
    });
    r.unwrap_or(Ordering::Equal)
}

#[inline(always)]
fn _cmp_b2_ret(a_ptr: *const u8, b_ptr: *const u8, ac: u16, bc: u16) -> Ordering {
    #[cfg(target_endian = "big")]
    let (aac, bbc) = (ac.swap_bytes(), bc.swap_bytes());
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
    let ac = unsafe { (a_ptr as *const u16).read() };
    let bc = unsafe { (b_ptr as *const u16).read() };
    if ac == bc {
        Ordering::Equal
    } else {
        _cmp_b2_ret(a_ptr, b_ptr, ac, bc)
    }
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
