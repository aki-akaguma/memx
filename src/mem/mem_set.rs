use crate::utils::*;

#[inline(never)]
pub fn _memset_impl(buf: &mut [u8], c1: u8) {
    if buf.is_empty() {
        return;
    }
    /*
    // `_start_set_1()` is a fastest by compiler optimaization
    _start_set_1(buf, c);
    */
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
        _start_set_128(buf, c1);
        #[cfg(feature = "test_pointer_width_64")]
        _start_set_64(buf, c1);
        #[cfg(feature = "test_pointer_width_32")]
        _start_set_32(buf, c1);
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
        _start_set_128(buf, c1);
        #[cfg(target_pointer_width = "64")]
        _start_set_64(buf, c1);
        #[cfg(target_pointer_width = "32")]
        _start_set_32(buf, c1);
    }
}

#[inline(always)]
fn _start_set_1(buf: &mut [u8], c: u8) {
    let buf_len = buf.len();
    let mut a_ptr = buf.as_mut_ptr();
    let end_ptr = unsafe { a_ptr.add(buf_len) };
    //
    while a_ptr < end_ptr {
        unsafe { *a_ptr = c };
        a_ptr = unsafe { a_ptr.add(1) };
    }
}

macro_rules! _unroll_one_set_to_align {
    ($buf_ptr_2:expr, $buf_ptr_end:expr, $c:expr) => {{
        if $buf_ptr_2 >= $buf_ptr_end {
            break;
        }
        _set_c1($buf_ptr_2, $c);
        $buf_ptr_2 = unsafe { $buf_ptr_2.add(1) };
    }};
}

macro_rules! _unroll_one_set_to_align_x4 {
    ($buf_ptr_2:expr, $buf_ptr_end:expr, $c:expr) => {{
        _unroll_one_set_to_align!($buf_ptr_2, $buf_ptr_end, $c);
        _unroll_one_set_to_align!($buf_ptr_2, $buf_ptr_end, $c);
        _unroll_one_set_to_align!($buf_ptr_2, $buf_ptr_end, $c);
        _unroll_one_set_to_align!($buf_ptr_2, $buf_ptr_end, $c);
    }};
}

pub(crate) fn _set_to_aligned_u256(buf_ptr: *mut u8, c: B1Sgl) -> *mut u8 {
    let remaining_align = 0x20_usize - ((buf_ptr as usize) & 0x1F_usize);
    let buf_ptr_end = unsafe { buf_ptr.add(remaining_align) };
    let mut buf_ptr_2 = buf_ptr;
    loop {
        _unroll_one_set_to_align_x4!(buf_ptr_2, buf_ptr_end, c);
        _unroll_one_set_to_align_x4!(buf_ptr_2, buf_ptr_end, c);
        _unroll_one_set_to_align_x4!(buf_ptr_2, buf_ptr_end, c);
        _unroll_one_set_to_align_x4!(buf_ptr_2, buf_ptr_end, c);
        //
        _unroll_one_set_to_align_x4!(buf_ptr_2, buf_ptr_end, c);
        _unroll_one_set_to_align_x4!(buf_ptr_2, buf_ptr_end, c);
        _unroll_one_set_to_align_x4!(buf_ptr_2, buf_ptr_end, c);
        _unroll_one_set_to_align_x4!(buf_ptr_2, buf_ptr_end, c);
    }
    buf_ptr_end
}

pub(crate) fn _set_to_aligned_u128(buf_ptr: *mut u8, c: B1Sgl) -> *mut u8 {
    let remaining_align = 0x10_usize - ((buf_ptr as usize) & 0x0F_usize);
    let buf_ptr_end = unsafe { buf_ptr.add(remaining_align) };
    let mut buf_ptr_2 = buf_ptr;
    loop {
        _unroll_one_set_to_align_x4!(buf_ptr_2, buf_ptr_end, c);
        _unroll_one_set_to_align_x4!(buf_ptr_2, buf_ptr_end, c);
        _unroll_one_set_to_align_x4!(buf_ptr_2, buf_ptr_end, c);
        _unroll_one_set_to_align_x4!(buf_ptr_2, buf_ptr_end, c);
    }
    buf_ptr_end
}

fn _set_to_aligned_u64(buf_ptr: *mut u8, c: B1Sgl) -> *mut u8 {
    let remaining_align = 0x08_usize - ((buf_ptr as usize) & 0x07_usize);
    let buf_ptr_end = unsafe { buf_ptr.add(remaining_align) };
    let mut buf_ptr_2 = buf_ptr;
    loop {
        _unroll_one_set_to_align_x4!(buf_ptr_2, buf_ptr_end, c);
        _unroll_one_set_to_align_x4!(buf_ptr_2, buf_ptr_end, c);
    }
    buf_ptr_end
}

fn _set_to_aligned_u32(buf_ptr: *mut u8, c: B1Sgl) -> *mut u8 {
    let remaining_align = 0x04_usize - ((buf_ptr as usize) & 0x03_usize);
    let buf_ptr_end = unsafe { buf_ptr.add(remaining_align) };
    let mut buf_ptr_2 = buf_ptr;
    loop {
        _unroll_one_set_to_align_x4!(buf_ptr_2, buf_ptr_end, c);
    }
    buf_ptr_end
}

macro_rules! _unroll_one_set_16 {
    ($a_ptr:expr, $cc:expr, $loop_size:expr, $idx:expr) => {{
        let aa_ptr = unsafe { $a_ptr.add($loop_size * $idx) };
        _set_c16(aa_ptr, $cc);
    }};
}

macro_rules! _unroll_one_set_8 {
    ($a_ptr:expr, $cc:expr, $loop_size:expr, $idx:expr) => {{
        let aa_ptr = unsafe { $a_ptr.add($loop_size * $idx) };
        _set_c8(aa_ptr, $cc);
    }};
}

macro_rules! _unroll_one_set_4 {
    ($a_ptr:expr, $cc:expr, $loop_size:expr, $idx:expr) => {{
        let aa_ptr = unsafe { $a_ptr.add($loop_size * $idx) };
        _set_c4(aa_ptr, $cc);
    }};
}

macro_rules! _unroll_one_set_2 {
    ($a_ptr:expr, $cc:expr, $loop_size:expr, $idx:expr) => {{
        let aa_ptr = unsafe { $a_ptr.add($loop_size * $idx) };
        _set_c2(aa_ptr, $cc);
    }};
}

macro_rules! _unroll_one_set_1 {
    ($a_ptr:expr, $cc:expr, $loop_size:expr, $idx:expr) => {{
        let aa_ptr = unsafe { $a_ptr.add($loop_size * $idx) };
        _set_c1(aa_ptr, $cc)
    }};
}

#[cfg(any(target_pointer_width = "128", feature = "test_pointer_width_128"))]
#[inline(always)]
fn _start_set_128(buf: &mut [u8], c_1: u8) {
    //
    let buf_len = buf.len();
    let mut buf_ptr = buf.as_mut_ptr();
    let end_ptr = unsafe { buf_ptr.add(buf_len) };
    let c = B1Sgl::new(c_1);
    let cc = B16Sgl::new(c_1);
    //
    if buf_len >= 16 {
        {
            if !buf_ptr.is_aligned_u128() {
                buf_ptr = _set_to_aligned_u128(buf_ptr, c);
            }
        }
        {
            let unroll = 8;
            let loop_size = 16;
            if unsafe { end_ptr.offset_from(buf_ptr) } >= (loop_size * unroll) as isize {
                let end_ptr_16_x8 = unsafe { end_ptr.sub(loop_size * unroll) };
                while buf_ptr <= end_ptr_16_x8 {
                    _unroll_one_set_16!(buf_ptr, cc, loop_size, 0);
                    _unroll_one_set_16!(buf_ptr, cc, loop_size, 1);
                    _unroll_one_set_16!(buf_ptr, cc, loop_size, 2);
                    _unroll_one_set_16!(buf_ptr, cc, loop_size, 3);
                    _unroll_one_set_16!(buf_ptr, cc, loop_size, 4);
                    _unroll_one_set_16!(buf_ptr, cc, loop_size, 5);
                    _unroll_one_set_16!(buf_ptr, cc, loop_size, 6);
                    _unroll_one_set_16!(buf_ptr, cc, loop_size, 7);
                    buf_ptr = unsafe { buf_ptr.add(loop_size * unroll) };
                }
            }
        }
        {
            let unroll = 4;
            let loop_size = 16;
            if unsafe { end_ptr.offset_from(buf_ptr) } >= (loop_size * unroll) as isize {
                let end_ptr_16_x4 = unsafe { end_ptr.sub(loop_size * unroll) };
                while buf_ptr <= end_ptr_16_x4 {
                    _unroll_one_set_16!(buf_ptr, cc, loop_size, 0);
                    _unroll_one_set_16!(buf_ptr, cc, loop_size, 1);
                    _unroll_one_set_16!(buf_ptr, cc, loop_size, 2);
                    _unroll_one_set_16!(buf_ptr, cc, loop_size, 3);
                    buf_ptr = unsafe { buf_ptr.add(loop_size * unroll) };
                }
            }
        }
        {
            let unroll = 2;
            let loop_size = 16;
            if unsafe { end_ptr.offset_from(buf_ptr) } >= (loop_size * unroll) as isize {
                let end_ptr_16_x2 = unsafe { end_ptr.sub(loop_size * unroll) };
                while buf_ptr <= end_ptr_16_x2 {
                    _unroll_one_set_16!(buf_ptr, cc, loop_size, 0);
                    _unroll_one_set_16!(buf_ptr, cc, loop_size, 1);
                    buf_ptr = unsafe { buf_ptr.add(loop_size * unroll) };
                }
            }
        }
        {
            let unroll = 1;
            let loop_size = 16;
            if unsafe { end_ptr.offset_from(buf_ptr) } >= (loop_size * unroll) as isize {
                let end_ptr_16_x1 = unsafe { end_ptr.sub(loop_size * unroll) };
                while buf_ptr <= end_ptr_16_x1 {
                    _unroll_one_set_16!(buf_ptr, cc, loop_size, 0);
                    buf_ptr = unsafe { buf_ptr.add(loop_size) };
                }
            }
        }
    }
    // the remaining data is the max: 15 bytes.
    _memset_remaining_15_bytes_impl(buf_ptr, cc.into(), end_ptr)
}

#[cfg(any(target_pointer_width = "64", feature = "test_pointer_width_64"))]
#[inline(always)]
fn _start_set_64(buf: &mut [u8], c_1: u8) {
    //
    let buf_len = buf.len();
    let mut buf_ptr = buf.as_mut_ptr();
    let end_ptr = unsafe { buf_ptr.add(buf_len) };
    let c = B1Sgl::new(c_1);
    let cc = B8Sgl::new(c_1);
    //
    if buf_len >= 8 {
        {
            if !buf_ptr.is_aligned_u64() {
                buf_ptr = _set_to_aligned_u64(buf_ptr, c);
            }
        }
        {
            let unroll = 8;
            let loop_size = 8;
            if unsafe { end_ptr.offset_from(buf_ptr) } >= (loop_size * unroll) as isize {
                let end_ptr_8_x8 = unsafe { end_ptr.sub(loop_size * unroll) };
                while buf_ptr <= end_ptr_8_x8 {
                    _unroll_one_set_8!(buf_ptr, cc, loop_size, 0);
                    _unroll_one_set_8!(buf_ptr, cc, loop_size, 1);
                    _unroll_one_set_8!(buf_ptr, cc, loop_size, 2);
                    _unroll_one_set_8!(buf_ptr, cc, loop_size, 3);
                    _unroll_one_set_8!(buf_ptr, cc, loop_size, 4);
                    _unroll_one_set_8!(buf_ptr, cc, loop_size, 5);
                    _unroll_one_set_8!(buf_ptr, cc, loop_size, 6);
                    _unroll_one_set_8!(buf_ptr, cc, loop_size, 7);
                    buf_ptr = unsafe { buf_ptr.add(loop_size * unroll) };
                }
            }
        }
        {
            let unroll = 4;
            let loop_size = 8;
            if unsafe { end_ptr.offset_from(buf_ptr) } >= (loop_size * unroll) as isize {
                let end_ptr_8_x4 = unsafe { end_ptr.sub(loop_size * unroll) };
                while buf_ptr <= end_ptr_8_x4 {
                    _unroll_one_set_8!(buf_ptr, cc, loop_size, 0);
                    _unroll_one_set_8!(buf_ptr, cc, loop_size, 1);
                    _unroll_one_set_8!(buf_ptr, cc, loop_size, 2);
                    _unroll_one_set_8!(buf_ptr, cc, loop_size, 3);
                    buf_ptr = unsafe { buf_ptr.add(loop_size * unroll) };
                }
            }
        }
        {
            let unroll = 2;
            let loop_size = 8;
            if unsafe { end_ptr.offset_from(buf_ptr) } >= (loop_size * unroll) as isize {
                let end_ptr_8_x2 = unsafe { end_ptr.sub(loop_size * unroll) };
                while buf_ptr <= end_ptr_8_x2 {
                    _unroll_one_set_8!(buf_ptr, cc, loop_size, 0);
                    _unroll_one_set_8!(buf_ptr, cc, loop_size, 1);
                    buf_ptr = unsafe { buf_ptr.add(loop_size * unroll) };
                }
            }
        }
        {
            let unroll = 1;
            let loop_size = 8;
            if unsafe { end_ptr.offset_from(buf_ptr) } >= (loop_size * unroll) as isize {
                let end_ptr_8_x1 = unsafe { end_ptr.sub(loop_size * unroll) };
                while buf_ptr <= end_ptr_8_x1 {
                    _unroll_one_set_8!(buf_ptr, cc, loop_size, 0);
                    buf_ptr = unsafe { buf_ptr.add(loop_size) };
                }
            }
        }
    }
    // the remaining data is the max: 7 bytes.
    _memset_remaining_7_bytes_impl(buf_ptr, cc.into(), end_ptr)
}

#[cfg(any(target_pointer_width = "32", feature = "test_pointer_width_32"))]
#[inline(always)]
fn _start_set_32(buf: &mut [u8], c_1: u8) {
    //
    let buf_len = buf.len();
    let mut buf_ptr = buf.as_mut_ptr();
    let end_ptr = unsafe { buf_ptr.add(buf_len) };
    let c = B1Sgl::new(c_1);
    let cc = B4Sgl::new(c_1);
    //
    if buf_len >= 4 {
        {
            if !buf_ptr.is_aligned_u32() {
                buf_ptr = _set_to_aligned_u32(buf_ptr, c);
            }
        }
        {
            let unroll = 8;
            let loop_size = 4;
            if unsafe { end_ptr.offset_from(buf_ptr) } >= (loop_size * unroll) as isize {
                let end_ptr_4_x8 = unsafe { end_ptr.sub(loop_size * unroll) };
                while buf_ptr <= end_ptr_4_x8 {
                    _unroll_one_set_4!(buf_ptr, cc, loop_size, 0);
                    _unroll_one_set_4!(buf_ptr, cc, loop_size, 1);
                    _unroll_one_set_4!(buf_ptr, cc, loop_size, 2);
                    _unroll_one_set_4!(buf_ptr, cc, loop_size, 3);
                    _unroll_one_set_4!(buf_ptr, cc, loop_size, 4);
                    _unroll_one_set_4!(buf_ptr, cc, loop_size, 5);
                    _unroll_one_set_4!(buf_ptr, cc, loop_size, 6);
                    _unroll_one_set_4!(buf_ptr, cc, loop_size, 7);
                    buf_ptr = unsafe { buf_ptr.add(loop_size * unroll) };
                }
            }
        }
        {
            let unroll = 4;
            let loop_size = 4;
            if unsafe { end_ptr.offset_from(buf_ptr) } >= (loop_size * unroll) as isize {
                let end_ptr_4_x4 = unsafe { end_ptr.sub(loop_size * unroll) };
                while buf_ptr <= end_ptr_4_x4 {
                    _unroll_one_set_4!(buf_ptr, cc, loop_size, 0);
                    _unroll_one_set_4!(buf_ptr, cc, loop_size, 1);
                    _unroll_one_set_4!(buf_ptr, cc, loop_size, 2);
                    _unroll_one_set_4!(buf_ptr, cc, loop_size, 3);
                    buf_ptr = unsafe { buf_ptr.add(loop_size * unroll) };
                }
            }
        }
        {
            let unroll = 2;
            let loop_size = 4;
            if unsafe { end_ptr.offset_from(buf_ptr) } >= (loop_size * unroll) as isize {
                let end_ptr_4_x2 = unsafe { end_ptr.sub(loop_size * unroll) };
                while buf_ptr <= end_ptr_4_x2 {
                    _unroll_one_set_4!(buf_ptr, cc, loop_size, 0);
                    _unroll_one_set_4!(buf_ptr, cc, loop_size, 1);
                    buf_ptr = unsafe { buf_ptr.add(loop_size * unroll) };
                }
            }
        }
        {
            let unroll = 1;
            let loop_size = 4;
            if unsafe { end_ptr.offset_from(buf_ptr) } >= (loop_size * unroll) as isize {
                let end_ptr_4_x1 = unsafe { end_ptr.sub(loop_size * unroll) };
                while buf_ptr <= end_ptr_4_x1 {
                    _unroll_one_set_4!(buf_ptr, cc, loop_size, 0);
                    buf_ptr = unsafe { buf_ptr.add(loop_size) };
                }
            }
        }
    }
    // the remaining data is the max: 3 bytes.
    _memset_remaining_3_bytes_impl(buf_ptr, cc.into(), end_ptr)
}

#[inline(always)]
pub(crate) fn _memset_remaining_15_bytes_impl(buf_ptr: *mut u8, cc: B8Sgl, end_ptr: *const u8) {
    let mut buf_ptr = buf_ptr;
    if buf_ptr.is_aligned_u64() {
        let loop_size = 8;
        if unsafe { end_ptr.offset_from(buf_ptr) } >= loop_size as isize {
            _unroll_one_set_8!(buf_ptr, cc, loop_size, 0);
            //
            buf_ptr = unsafe { buf_ptr.add(loop_size) };
        }
    }
    // the remaining data is the max: 7 bytes.
    _memset_remaining_7_bytes_impl(buf_ptr, cc.into(), end_ptr)
}

#[inline(always)]
pub(crate) fn _memset_remaining_7_bytes_impl(buf_ptr: *mut u8, cc: B4Sgl, end_ptr: *const u8) {
    let mut buf_ptr = buf_ptr;
    if buf_ptr.is_aligned_u32() {
        let loop_size = 4;
        if unsafe { end_ptr.offset_from(buf_ptr) } >= loop_size as isize {
            _unroll_one_set_4!(buf_ptr, cc, loop_size, 0);
            //
            buf_ptr = unsafe { buf_ptr.add(loop_size) };
        }
    }
    // the remaining data is the max: 3 bytes.
    _memset_remaining_3_bytes_impl(buf_ptr, cc.into(), end_ptr)
}

#[inline(always)]
pub(crate) fn _memset_remaining_3_bytes_impl(buf_ptr: *mut u8, cc: B2Sgl, end_ptr: *const u8) {
    let mut buf_ptr = buf_ptr;
    if buf_ptr.is_aligned_u16() {
        let loop_size = 2;
        if unsafe { end_ptr.offset_from(buf_ptr) } >= loop_size as isize {
            _unroll_one_set_2!(buf_ptr, cc, loop_size, 0);
            //
            buf_ptr = unsafe { buf_ptr.add(loop_size) };
        }
    }
    {
        let loop_size = 1;
        if unsafe { end_ptr.offset_from(buf_ptr) } >= loop_size as isize {
            while unsafe { end_ptr.offset_from(buf_ptr) } >= loop_size as isize {
                _unroll_one_set_1!(buf_ptr, cc.into(), loop_size, 0);
                buf_ptr = unsafe { buf_ptr.add(loop_size) };
            }
        }
    }
}

#[inline(always)]
fn _set_c16(buf_ptr: *mut u8, c16: B16Sgl) {
    let aa_ptr = buf_ptr as *mut u128;
    unsafe { aa_ptr.write(c16.a) };
}

#[inline(always)]
fn _set_c8(buf_ptr: *mut u8, c8: B8Sgl) {
    let aa_ptr = buf_ptr as *mut u64;
    unsafe { aa_ptr.write(c8.a) };
}

#[inline(always)]
fn _set_c4(buf_ptr: *mut u8, c4: B4Sgl) {
    let aa_ptr = buf_ptr as *mut u32;
    unsafe { aa_ptr.write(c4.a) };
}

#[inline(always)]
fn _set_c2(buf_ptr: *mut u8, c2: B2Sgl) {
    let aa_ptr = buf_ptr as *mut u16;
    unsafe { aa_ptr.write(c2.a) };
}

#[inline(always)]
fn _set_c1(buf_ptr: *mut u8, c1: B1Sgl) {
    let aa_ptr = buf_ptr as *mut u8;
    unsafe { aa_ptr.write(c1.a) };
}

/*
 * The simple implement:

#[inline(always)]
pub fn _memset_impl(buf: &mut [u8], c: u8, n: usize) -> Result<(), RangeError> {
    if buf.len() < n {
        return Err(RangeError);
    }
    for i in 0..n {
        buf[i] = c;
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
