use crate::utils::_unroll_loop_action;
use crate::utils::B16Sgl;
use crate::utils::B1Sgl;
use crate::utils::B2Sgl;
use crate::utils::B4Sgl;
use crate::utils::B8Sgl;
use crate::utils::PtrOps;

#[inline(never)]
pub fn _memset_impl(buf: &mut [u8], c1: u8) {
    #[cfg(any(target_pointer_width = "64", feature = "test_pointer_width_64"))]
    _start_set_64(buf, c1);
    #[cfg(all(
        not(any(target_pointer_width = "64", feature = "test_pointer_width_64")),
        any(target_pointer_width = "32", feature = "test_pointer_width_32")
    ))]
    _start_set_32(buf, c1);
}

#[inline(always)]
fn _set_to_aligned<const ALIGN: usize>(buf_ptr: *mut u8, c: B1Sgl) -> *mut u8 {
    let remaining_align = ALIGN - ((buf_ptr as usize) & (ALIGN - 1));
    let buf_ptr_end = unsafe { buf_ptr.add(remaining_align) };
    let mut cur_ptr = buf_ptr;
    while cur_ptr < buf_ptr_end {
        _set_c1_aa_x1(cur_ptr, c);
        cur_ptr = unsafe { cur_ptr.add(1) };
    }
    buf_ptr_end
}

#[inline(always)]
pub(crate) fn _set_to_aligned_u256(buf_ptr: *mut u8, c: B1Sgl) -> *mut u8 {
    _set_to_aligned::<32>(buf_ptr, c)
}

#[inline(always)]
pub(crate) fn _set_to_aligned_u128(buf_ptr: *mut u8, c: B1Sgl) -> *mut u8 {
    _set_to_aligned::<16>(buf_ptr, c)
}

#[inline(always)]
fn _set_to_aligned_u64(buf_ptr: *mut u8, c: B1Sgl) -> *mut u8 {
    _set_to_aligned::<8>(buf_ptr, c)
}

#[inline(always)]
fn _set_to_aligned_u32(buf_ptr: *mut u8, c: B1Sgl) -> *mut u8 {
    _set_to_aligned::<4>(buf_ptr, c)
}

#[inline(always)]
fn _set_to_aligned_u16(buf_ptr: *mut u8, c: B1Sgl) -> *mut u8 {
    _set_to_aligned::<2>(buf_ptr, c)
}

//#[cfg(any(target_pointer_width = "128", feature = "test_pointer_width_128"))]
#[inline(always)]
pub(crate) fn _start_set_128(buf: &mut [u8], c_1: u8) {
    let buf_len = buf.len();
    let mut buf_ptr = buf.as_mut_ptr();
    let end_ptr = unsafe { buf_ptr.add(buf_len) };
    let cc = B16Sgl::new(c_1);
    //
    if buf_len >= 16 {
        // to a aligned pointer
        {
            if !buf_ptr.is_aligned_u128() {
                let c = B1Sgl::new(c_1);
                buf_ptr = _set_to_aligned_u128(buf_ptr, c);
            }
        }
        // the loop
        {
            buf_ptr = _unroll_loop_action::<8, 16, _>(buf_ptr, end_ptr, |p| {
                _set_c16_aa_x1(p, cc);
            });
        }
        {
            buf_ptr = _unroll_loop_action::<1, 16, _>(buf_ptr, end_ptr, |p| {
                _set_c16_aa_x1(p, cc);
            });
        }
    }
    // the remaining data is the max: 15 bytes.
    _memset_remaining_15_bytes_impl(buf_ptr, cc.into(), end_ptr)
}

#[cfg(any(target_pointer_width = "64", feature = "test_pointer_width_64"))]
#[inline(always)]
fn _start_set_64(buf: &mut [u8], c_1: u8) {
    let buf_len = buf.len();
    let mut buf_ptr = buf.as_mut_ptr();
    let end_ptr = unsafe { buf_ptr.add(buf_len) };
    let cc = B8Sgl::new(c_1);
    //
    if buf_len >= 8 {
        // to a aligned pointer
        {
            if !buf_ptr.is_aligned_u64() {
                let c = B1Sgl::new(c_1);
                buf_ptr = _set_to_aligned_u64(buf_ptr, c);
            }
        }
        // the loop
        {
            buf_ptr = _unroll_loop_action::<16, 8, _>(buf_ptr, end_ptr, |p| {
                _set_c8_aa_x1(p, cc);
            });
        }
        {
            buf_ptr = _unroll_loop_action::<8, 8, _>(buf_ptr, end_ptr, |p| {
                _set_c8_aa_x1(p, cc);
            });
        }
        {
            buf_ptr = _unroll_loop_action::<1, 8, _>(buf_ptr, end_ptr, |p| {
                _set_c8_aa_x1(p, cc);
            });
        }
    }
    // the remaining data is the max: 7 bytes.
    _memset_remaining_7_bytes_impl(buf_ptr, cc.into(), end_ptr)
}

#[cfg(any(target_pointer_width = "32", feature = "test_pointer_width_32"))]
#[inline(always)]
fn _start_set_32(buf: &mut [u8], c_1: u8) {
    let buf_len = buf.len();
    let mut buf_ptr = buf.as_mut_ptr();
    let end_ptr = unsafe { buf_ptr.add(buf_len) };
    let cc = B4Sgl::new(c_1);
    //
    if buf_len >= 4 {
        // to a aligned pointer
        {
            if !buf_ptr.is_aligned_u32() {
                let c = B1Sgl::new(c_1);
                buf_ptr = _set_to_aligned_u32(buf_ptr, c);
            }
        }
        // the loop
        {
            buf_ptr = _unroll_loop_action::<8, 4, _>(buf_ptr, end_ptr, |p| {
                _set_c4_aa_x1(p, cc);
            });
        }
        {
            buf_ptr = _unroll_loop_action::<1, 4, _>(buf_ptr, end_ptr, |p| {
                _set_c4_aa_x1(p, cc);
            });
        }
    }
    // the remaining data is the max: 3 bytes.
    _memset_remaining_3_bytes_impl(buf_ptr, cc.into(), end_ptr)
}

#[inline(always)]
pub(crate) fn _memset_remaining_15_bytes_impl(buf_ptr: *mut u8, cc: B8Sgl, end_ptr: *const u8) {
    let mut buf_ptr = buf_ptr;
    {
        let loop_size = 8;
        if !buf_ptr.is_aligned_u64() && buf_ptr.is_not_over(end_ptr, loop_size) {
            buf_ptr = _set_to_aligned_u64(buf_ptr, cc.into());
        }
    }
    _memset_remaining_15_bytes_impl_aligned(buf_ptr, cc, end_ptr);
}

#[inline(always)]
pub(crate) fn _memset_remaining_7_bytes_impl(buf_ptr: *mut u8, cc: B4Sgl, end_ptr: *const u8) {
    let mut buf_ptr = buf_ptr;
    {
        let loop_size = 4;
        if !buf_ptr.is_aligned_u32() && buf_ptr.is_not_over(end_ptr, loop_size) {
            buf_ptr = _set_to_aligned_u32(buf_ptr, cc.into());
        }
    }
    _memset_remaining_7_bytes_impl_aligned(buf_ptr, cc, end_ptr);
}

#[inline(always)]
pub(crate) fn _memset_remaining_3_bytes_impl(buf_ptr: *mut u8, cc: B2Sgl, end_ptr: *const u8) {
    let mut buf_ptr = buf_ptr;
    {
        let loop_size = 2;
        if !buf_ptr.is_aligned_u16() && buf_ptr.is_not_over(end_ptr, loop_size) {
            buf_ptr = _set_to_aligned_u16(buf_ptr, cc.into());
        }
    }
    _memset_remaining_3_bytes_impl_aligned(buf_ptr, cc, end_ptr);
}

#[inline(always)]
fn _memset_remaining_15_bytes_impl_aligned(buf_ptr: *mut u8, cc: B8Sgl, end_ptr: *const u8) {
    let mut buf_ptr = buf_ptr;
    {
        let loop_size = 8;
        if buf_ptr.is_not_over(end_ptr, loop_size) {
            _set_c8_aa_x1(buf_ptr, cc);
            buf_ptr = unsafe { buf_ptr.add(loop_size) };
        }
    }
    // the remaining data is the max: 7 bytes.
    #[cfg(not(feature = "test_alignment_check"))]
    _memset_remaining_7_bytes_impl_aligned(buf_ptr, cc.into(), end_ptr);
    #[cfg(feature = "test_alignment_check")]
    _memset_remaining_7_bytes_impl(buf_ptr, cc.into(), end_ptr);
}

#[inline(always)]
fn _memset_remaining_7_bytes_impl_aligned(buf_ptr: *mut u8, cc: B4Sgl, end_ptr: *const u8) {
    let mut buf_ptr = buf_ptr;
    {
        let loop_size = 4;
        if buf_ptr.is_not_over(end_ptr, loop_size) {
            _set_c4_aa_x1(buf_ptr, cc);
            buf_ptr = unsafe { buf_ptr.add(loop_size) };
        }
    }
    // the remaining data is the max: 3 bytes.
    #[cfg(not(feature = "test_alignment_check"))]
    _memset_remaining_3_bytes_impl_unaligned(buf_ptr, cc.into(), end_ptr);
    #[cfg(feature = "test_alignment_check")]
    _memset_remaining_3_bytes_impl(buf_ptr, cc.into(), end_ptr);
}

#[inline(always)]
fn _memset_remaining_3_bytes_impl_unaligned(buf_ptr: *mut u8, cc: B2Sgl, end_ptr: *const u8) {
    let mut buf_ptr = buf_ptr;
    {
        let loop_size = 2;
        if buf_ptr.is_not_over(end_ptr, loop_size) {
            _set_c2_uu_x1(buf_ptr, cc);
            buf_ptr = unsafe { buf_ptr.add(loop_size) };
        }
    }
    // the remaining data is the max: 1 bytes.
    {
        let loop_size = 1;
        if buf_ptr.is_not_over(end_ptr, loop_size) {
            _set_c1_aa_x1(buf_ptr, cc.into());
        }
    }
}

#[inline(always)]
fn _memset_remaining_3_bytes_impl_aligned(buf_ptr: *mut u8, cc: B2Sgl, end_ptr: *const u8) {
    let mut buf_ptr = buf_ptr;
    {
        let loop_size = 2;
        if buf_ptr.is_not_over(end_ptr, loop_size) {
            _set_c2_aa_x1(buf_ptr, cc);
            buf_ptr = unsafe { buf_ptr.add(loop_size) };
        }
    }
    // the remaining data is the max: 1 bytes.
    {
        let loop_size = 1;
        if buf_ptr.is_not_over(end_ptr, loop_size) {
            _set_c1_aa_x1(buf_ptr, cc.into());
        }
    }
}

#[inline(always)]
fn _set_c16_uu_x1(buf_ptr: *mut u8, c16: B16Sgl) {
    let aa_ptr = buf_ptr as *mut u128;
    unsafe { aa_ptr.write_unaligned(c16.v1) };
}

#[inline(always)]
fn _set_c16_aa_x1(buf_ptr: *mut u8, c16: B16Sgl) {
    let aa_ptr = buf_ptr as *mut u128;
    unsafe { aa_ptr.write(c16.v1) };
}

#[inline(always)]
fn _set_c16_aa_x2(buf_ptr: *mut u8, c16: B16Sgl) {
    _unroll_loop_action::<2, 16, _>(buf_ptr, unsafe { buf_ptr.add(16 * 2) }, |p| {
        _set_c16_aa_x1(p, c16);
    });
}

#[inline(always)]
fn _set_c16_aa_x4(buf_ptr: *mut u8, c16: B16Sgl) {
    _unroll_loop_action::<4, 16, _>(buf_ptr, unsafe { buf_ptr.add(16 * 4) }, |p| {
        _set_c16_aa_x1(p, c16);
    });
}

#[inline(always)]
fn _set_c16_aa_x8(buf_ptr: *mut u8, c16: B16Sgl) {
    _unroll_loop_action::<8, 16, _>(buf_ptr, unsafe { buf_ptr.add(16 * 8) }, |p| {
        _set_c16_aa_x1(p, c16);
    });
}

#[inline(always)]
fn _set_c8_uu_x1(buf_ptr: *mut u8, c8: B8Sgl) {
    let aa_ptr = buf_ptr as *mut u64;
    unsafe { aa_ptr.write_unaligned(c8.v1) };
}

#[inline(always)]
fn _set_c8_aa_x1(buf_ptr: *mut u8, c8: B8Sgl) {
    let aa_ptr = buf_ptr as *mut u64;
    unsafe { aa_ptr.write(c8.v1) };
}

#[inline(always)]
fn _set_c8_aa_x2(buf_ptr: *mut u8, c8: B8Sgl) {
    _unroll_loop_action::<2, 8, _>(buf_ptr, unsafe { buf_ptr.add(8 * 2) }, |p| {
        _set_c8_aa_x1(p, c8);
    });
}

#[inline(always)]
fn _set_c8_aa_x4(buf_ptr: *mut u8, c8: B8Sgl) {
    _unroll_loop_action::<4, 8, _>(buf_ptr, unsafe { buf_ptr.add(8 * 4) }, |p| {
        _set_c8_aa_x1(p, c8);
    });
}

#[inline(always)]
fn _set_c8_aa_x8(buf_ptr: *mut u8, c8: B8Sgl) {
    _unroll_loop_action::<8, 8, _>(buf_ptr, unsafe { buf_ptr.add(8 * 8) }, |p| {
        _set_c8_aa_x1(p, c8);
    });
}

#[inline(always)]
fn _set_c8_aa_x16(buf_ptr: *mut u8, c8: B8Sgl) {
    _unroll_loop_action::<16, 8, _>(buf_ptr, unsafe { buf_ptr.add(8 * 16) }, |p| {
        _set_c8_aa_x1(p, c8);
    });
}

#[inline(always)]
fn _set_c4_aa_x1(buf_ptr: *mut u8, c4: B4Sgl) {
    let aa_ptr = buf_ptr as *mut u32;
    unsafe { aa_ptr.write(c4.v1) };
}

#[inline(always)]
fn _set_c4_aa_x2(buf_ptr: *mut u8, c4: B4Sgl) {
    _unroll_loop_action::<2, 4, _>(buf_ptr, unsafe { buf_ptr.add(4 * 2) }, |p| {
        _set_c4_aa_x1(p, c4);
    });
}

#[inline(always)]
fn _set_c4_aa_x4(buf_ptr: *mut u8, c4: B4Sgl) {
    _unroll_loop_action::<4, 4, _>(buf_ptr, unsafe { buf_ptr.add(4 * 4) }, |p| {
        _set_c4_aa_x1(p, c4);
    });
}

#[inline(always)]
fn _set_c4_aa_x8(buf_ptr: *mut u8, c4: B4Sgl) {
    _unroll_loop_action::<8, 4, _>(buf_ptr, unsafe { buf_ptr.add(4 * 8) }, |p| {
        _set_c4_aa_x1(p, c4);
    });
}

#[inline(always)]
fn _set_c2_uu_x1(buf_ptr: *mut u8, c2: B2Sgl) {
    let aa_ptr = buf_ptr as *mut u16;
    unsafe { aa_ptr.write_unaligned(c2.v1) };
}

#[inline(always)]
fn _set_c2_aa_x1(buf_ptr: *mut u8, c2: B2Sgl) {
    let aa_ptr = buf_ptr as *mut u16;
    unsafe { aa_ptr.write(c2.v1) };
}

#[inline(always)]
fn _set_c1_aa_x1(buf_ptr: *mut u8, c1: B1Sgl) {
    let aa_ptr = buf_ptr;
    unsafe { aa_ptr.write(c1.v1) };
}

#[cfg(test)]
mod disasm {
    use super::*;
    //
    #[test]
    fn do_procs() {
        let mut a = b"abcdefg".to_vec();
        let a = a.as_mut_slice();
        let c = b'A';
        do_proc_basic(a, c);
    }
    #[inline(never)]
    fn do_proc_basic(a: &mut [u8], c: u8) {
        _memset_impl(a, c)
    }
}
