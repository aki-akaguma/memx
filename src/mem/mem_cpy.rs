use super::super::RangeError;
use crate::utils::_read_a_native_endian_from_ptr_u16;
use crate::utils::_unroll_loop_dual_action;
use crate::utils::_unroll_loop_dual_action_with_prefetch;
use crate::utils::PtrOps;
use crate::utils::PtrOpsPrefetch;

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
    #[cfg(any(target_pointer_width = "64", feature = "test_pointer_width_64"))]
    _start_cpy_64(dst, src);
    #[cfg(all(
        not(any(target_pointer_width = "64", feature = "test_pointer_width_64")),
        any(target_pointer_width = "32", feature = "test_pointer_width_32")
    ))]
    _start_cpy_32(dst, src);
    Ok(())
}

#[inline(always)]
fn _cpy_to_aligned<const ALIGN: usize>(a_ptr: *mut u8, b_ptr: *const u8) -> (*mut u8, *const u8) {
    let remaining_align = ALIGN - ((a_ptr as usize) & (ALIGN - 1));
    let a_ptr_end = unsafe { a_ptr.add(remaining_align) };
    let mut a_ptr_cur = a_ptr;
    let mut b_ptr_cur = b_ptr;
    while a_ptr_cur < a_ptr_end {
        _cpy_b1_aa_x1(a_ptr_cur, b_ptr_cur);
        a_ptr_cur = unsafe { a_ptr_cur.add(1) };
        b_ptr_cur = unsafe { b_ptr_cur.add(1) };
    }
    (a_ptr_end, unsafe { b_ptr.add(remaining_align) })
}

#[inline(always)]
pub(crate) fn _cpy_to_aligned_u256(a_ptr: *mut u8, b_ptr: *const u8) -> (*mut u8, *const u8) {
    _cpy_to_aligned::<32>(a_ptr, b_ptr)
}

#[inline(always)]
pub(crate) fn _cpy_to_aligned_u128(a_ptr: *mut u8, b_ptr: *const u8) -> (*mut u8, *const u8) {
    _cpy_to_aligned::<16>(a_ptr, b_ptr)
}

#[inline(always)]
pub(crate) fn _cpy_to_aligned_u64(a_ptr: *mut u8, b_ptr: *const u8) -> (*mut u8, *const u8) {
    _cpy_to_aligned::<8>(a_ptr, b_ptr)
}

#[inline(always)]
pub(crate) fn _cpy_to_aligned_u32(a_ptr: *mut u8, b_ptr: *const u8) -> (*mut u8, *const u8) {
    _cpy_to_aligned::<4>(a_ptr, b_ptr)
}

//#[cfg(any(target_pointer_width = "128", feature = "test_pointer_width_128"))]
#[inline(always)]
pub(crate) fn _start_cpy_128(dst: &mut [u8], src: &[u8]) {
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
            let (ap, bp) = _unroll_loop_dual_action_with_prefetch::<8, 16, _>(
                a_ptr,
                b_ptr,
                end_ptr,
                |ap, bp| {
                    _cpy_b16_aa_x1(ap, bp);
                },
            );
            a_ptr = ap;
            b_ptr = bp;
            //
            let (ap, bp) = _unroll_loop_dual_action::<1, 16, _>(a_ptr, b_ptr, end_ptr, |ap, bp| {
                _cpy_b16_aa_x1(ap, bp);
            });
            a_ptr = ap;
            b_ptr = bp;
        } else {
            let (ap, bp) = _unroll_loop_dual_action::<1, 16, _>(a_ptr, b_ptr, end_ptr, |ap, bp| {
                _cpy_b16_au_x1(ap, bp);
            });
            a_ptr = ap;
            b_ptr = bp;
        }
    }
    // the remaining data is the max: 15 bytes.
    _memcpy_remaining_15_bytes_impl(a_ptr, b_ptr, end_ptr)
}

#[cfg(any(target_pointer_width = "64", feature = "test_pointer_width_64"))]
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
            let (ap, bp) = _unroll_loop_dual_action_with_prefetch::<8, 8, _>(
                a_ptr,
                b_ptr,
                end_ptr,
                |ap, bp| {
                    _cpy_b8_aa_x1(ap, bp);
                },
            );
            a_ptr = ap;
            b_ptr = bp;
            //
            let (ap, bp) = _unroll_loop_dual_action::<1, 8, _>(a_ptr, b_ptr, end_ptr, |ap, bp| {
                _cpy_b8_aa_x1(ap, bp);
            });
            a_ptr = ap;
            b_ptr = bp;
        } else {
            let (ap, bp) = _unroll_loop_dual_action::<1, 8, _>(a_ptr, b_ptr, end_ptr, |ap, bp| {
                _cpy_b8_au_x1(ap, bp);
            });
            a_ptr = ap;
            b_ptr = bp;
        }
    }
    // the remaining data is the max: 7 bytes.
    _memcpy_remaining_7_bytes_impl(a_ptr, b_ptr, end_ptr)
}

#[cfg(any(target_pointer_width = "32", feature = "test_pointer_width_32"))]
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
            let (ap, bp) = _unroll_loop_dual_action_with_prefetch::<8, 4, _>(
                a_ptr,
                b_ptr,
                end_ptr,
                |ap, bp| {
                    _cpy_b4_aa_x1(ap, bp);
                },
            );
            a_ptr = ap;
            b_ptr = bp;
            //
            let (ap, bp) = _unroll_loop_dual_action::<1, 4, _>(a_ptr, b_ptr, end_ptr, |ap, bp| {
                _cpy_b4_aa_x1(ap, bp);
            });
            a_ptr = ap;
            b_ptr = bp;
        } else {
            let (ap, bp) = _unroll_loop_dual_action::<1, 4, _>(a_ptr, b_ptr, end_ptr, |ap, bp| {
                _cpy_b4_au_x1(ap, bp);
            });
            a_ptr = ap;
            b_ptr = bp;
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
    if a_ptr.is_aligned_u64() {
        let loop_size = 8;
        if a_ptr.is_not_over(end_ptr, loop_size) {
            if b_ptr.is_aligned_u64() {
                _cpy_b8_aa_x1(a_ptr, b_ptr);
            } else {
                _cpy_b8_au_x1(a_ptr, b_ptr);
            }
            a_ptr = unsafe { a_ptr.add(loop_size) };
            b_ptr = unsafe { b_ptr.add(loop_size) };
        }
    }
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
    if a_ptr.is_aligned_u32() {
        let loop_size = 4;
        if a_ptr.is_not_over(end_ptr, loop_size) {
            if b_ptr.is_aligned_u32() {
                _cpy_b4_aa_x1(a_ptr, b_ptr);
            } else {
                _cpy_b4_au_x1(a_ptr, b_ptr);
            }
            a_ptr = unsafe { a_ptr.add(loop_size) };
            b_ptr = unsafe { b_ptr.add(loop_size) };
        }
    }
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
    if a_ptr.is_aligned_u16() {
        let loop_size = 2;
        if a_ptr.is_not_over(end_ptr, loop_size) {
            if b_ptr.is_aligned_u16() {
                _cpy_b2_aa_x1(a_ptr, b_ptr);
            } else {
                _cpy_b2_uu_x1(a_ptr, b_ptr);
            }
            a_ptr = unsafe { a_ptr.add(loop_size) };
            b_ptr = unsafe { b_ptr.add(loop_size) };
        }
    }
    {
        let loop_size = 1;
        while a_ptr < end_ptr {
            _cpy_b1_aa_x1(a_ptr, b_ptr);
            a_ptr = unsafe { a_ptr.add(loop_size) };
            b_ptr = unsafe { b_ptr.add(loop_size) };
        }
    }
}

#[inline(always)]
fn _cpy_b16_uu_x1(a_ptr: *mut u8, b_ptr: *const u8) {
    let bc = unsafe { (b_ptr as *const u128).read_unaligned() };
    unsafe { (a_ptr as *mut u128).write_unaligned(bc) };
}

#[inline(always)]
fn _cpy_b16_au_x1(a_ptr: *mut u8, b_ptr: *const u8) {
    let bc = unsafe { (b_ptr as *const u128).read_unaligned() };
    unsafe { (a_ptr as *mut u128).write(bc) };
}

#[inline(always)]
fn _cpy_b16_aa_x1(a_ptr: *mut u8, b_ptr: *const u8) {
    let bc = unsafe { (b_ptr as *const u128).read() };
    unsafe { (a_ptr as *mut u128).write(bc) };
}

#[inline(always)]
fn _cpy_b8_uu_x1(a_ptr: *mut u8, b_ptr: *const u8) {
    let bc = unsafe { (b_ptr as *const u64).read_unaligned() };
    unsafe { (a_ptr as *mut u64).write_unaligned(bc) };
}

#[inline(always)]
fn _cpy_b8_au_x1(a_ptr: *mut u8, b_ptr: *const u8) {
    let bc = unsafe { (b_ptr as *const u64).read_unaligned() };
    unsafe { (a_ptr as *mut u64).write(bc) };
}

#[inline(always)]
fn _cpy_b8_aa_x1(a_ptr: *mut u8, b_ptr: *const u8) {
    let bc = unsafe { (b_ptr as *const u64).read() };
    unsafe { (a_ptr as *mut u64).write(bc) };
}

#[inline(always)]
fn _cpy_b4_uu_x1(a_ptr: *mut u8, b_ptr: *const u8) {
    let bc = unsafe { (b_ptr as *const u32).read_unaligned() };
    unsafe { (a_ptr as *mut u32).write_unaligned(bc) };
}

#[inline(always)]
fn _cpy_b4_au_x1(a_ptr: *mut u8, b_ptr: *const u8) {
    let bc = unsafe { (b_ptr as *const u32).read_unaligned() };
    unsafe { (a_ptr as *mut u32).write(bc) };
}

#[inline(always)]
fn _cpy_b4_aa_x1(a_ptr: *mut u8, b_ptr: *const u8) {
    let bc = unsafe { (b_ptr as *const u32).read() };
    unsafe { (a_ptr as *mut u32).write(bc) };
}

#[inline(always)]
fn _cpy_b2_uu_x1(a_ptr: *mut u8, b_ptr: *const u8) {
    let bc = unsafe { _read_a_native_endian_from_ptr_u16(b_ptr) };
    let aa_ptr = a_ptr as *mut u16;
    unsafe { aa_ptr.write_unaligned(bc) };
}

#[inline(always)]
fn _cpy_b2_aa_x1(a_ptr: *mut u8, b_ptr: *const u8) {
    let bc = unsafe { _read_a_native_endian_from_ptr_u16(b_ptr) };
    let aa_ptr = a_ptr as *mut u16;
    unsafe { aa_ptr.write(bc) };
}

#[inline(always)]
fn _cpy_b1_aa_x1(a_ptr: *mut u8, b_ptr: *const u8) {
    let bc = unsafe { *b_ptr };
    let aa_ptr = a_ptr;
    unsafe { aa_ptr.write(bc) };
}

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
        #[cfg(target_pointer_width = "64")]
        do_proc_64(a, b);
        #[cfg(target_pointer_width = "32")]
        do_proc_32(a, b);
    }
    fn do_proc_basic(a: &mut [u8], b: &[u8]) -> Result<(), RangeError> {
        _memcpy_impl(a, b)
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
