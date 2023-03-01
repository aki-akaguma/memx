#[inline(always)]
pub fn _memeq_impl(a: &[u8], b: &[u8]) -> bool {
    if a.is_empty() && b.is_empty() {
        return true;
    }
    if a.is_empty() || b.is_empty() {
        return false;
    }
    #[cfg(any(target_arch = "x86_64", target_arch = "x86", target_arch = "aarch64"))]
    {
        _start_eq_128(a, b)
    }
    #[cfg(not(any(target_arch = "x86_64", target_arch = "x86", target_arch = "aarch64")))]
    {
        #[cfg(target_pointer_width = "128")]
        let r = _start_eq_128(a, b);
        #[cfg(target_pointer_width = "64")]
        let r = _start_eq_64(a, b);
        #[cfg(target_pointer_width = "32")]
        let r = _start_eq_32(a, b);
        #[cfg(target_pointer_width = "16")]
        let r = _start_eq_16(a, b);
        //
        r
    }
}

macro_rules! _unroll_one_eq_16 {
    ($a_ptr:expr, $b_ptr:expr, $loop_size:expr, $idx:expr) => {{
        let aa_ptr = unsafe { $a_ptr.add($loop_size * $idx) };
        let bb_ptr = unsafe { $b_ptr.add($loop_size * $idx) };
        //
        let aac = unsafe {
            crate::utils::_read_a_native_endian_u128(core::slice::from_raw_parts(aa_ptr, 16))
        };
        let bbc = unsafe {
            crate::utils::_read_a_native_endian_u128(core::slice::from_raw_parts(bb_ptr, 16))
        };
        if aac != bbc {
            return false;
        }
    }};
}

macro_rules! _unroll_one_eq_8 {
    ($a_ptr:expr, $b_ptr:expr, $loop_size:expr, $idx:expr) => {{
        let aa_ptr = unsafe { $a_ptr.add($loop_size * $idx) };
        let bb_ptr = unsafe { $b_ptr.add($loop_size * $idx) };
        //
        let aac = unsafe {
            crate::utils::_read_a_native_endian_u64(core::slice::from_raw_parts(aa_ptr, 8))
        };
        let bbc = unsafe {
            crate::utils::_read_a_native_endian_u64(core::slice::from_raw_parts(bb_ptr, 8))
        };
        if aac != bbc {
            return false;
        }
    }};
}

macro_rules! _unroll_one_eq_4 {
    ($a_ptr:expr, $b_ptr:expr, $loop_size:expr, $idx:expr) => {{
        let aa_ptr = unsafe { $a_ptr.add($loop_size * $idx) };
        let bb_ptr = unsafe { $b_ptr.add($loop_size * $idx) };
        //
        let aac = unsafe {
            crate::utils::_read_a_native_endian_u32(core::slice::from_raw_parts(aa_ptr, 4))
        };
        let bbc = unsafe {
            crate::utils::_read_a_native_endian_u32(core::slice::from_raw_parts(bb_ptr, 4))
        };
        if aac != bbc {
            return false;
        }
    }};
}

macro_rules! _unroll_one_eq_2 {
    ($a_ptr:expr, $b_ptr:expr, $loop_size:expr, $idx:expr) => {{
        let aa_ptr = unsafe { $a_ptr.add($loop_size * $idx) };
        let bb_ptr = unsafe { $b_ptr.add($loop_size * $idx) };
        //
        let aac = unsafe {
            crate::utils::_read_a_native_endian_u16(core::slice::from_raw_parts(aa_ptr, 2))
        };
        let bbc = unsafe {
            crate::utils::_read_a_native_endian_u16(core::slice::from_raw_parts(bb_ptr, 2))
        };
        if aac != bbc {
            return false;
        }
    }};
}

macro_rules! _unroll_one_eq_1 {
    ($a_ptr:expr, $b_ptr:expr, $loop_size:expr, $idx:expr) => {{
        let aa_ptr = unsafe { $a_ptr.add($loop_size * $idx) };
        let bb_ptr = unsafe { $b_ptr.add($loop_size * $idx) };
        //
        let aac = unsafe { *(aa_ptr as *const u8) };
        let bbc = unsafe { *(bb_ptr as *const u8) };
        if aac != bbc {
            return false;
        }
    }};
}

#[cfg(any(
    target_pointer_width = "128",
    target_arch = "x86_64",
    target_arch = "x86",
    target_arch = "aarch64"
))]
#[inline(always)]
fn _start_eq_128(a: &[u8], b: &[u8]) -> bool {
    //
    let a_len = a.len();
    let b_len = b.len();
    if a_len != b_len {
        return false;
    }
    let mut a_ptr = a.as_ptr();
    let mut b_ptr = b.as_ptr();
    let end_ptr = unsafe { a_ptr.add(a_len) };
    //
    {
        let unroll = 8;
        let loop_size = 16;
        while unsafe { end_ptr.offset_from(a_ptr) } >= (loop_size * unroll) as isize {
            _unroll_one_eq_16!(a_ptr, b_ptr, loop_size, 0);
            _unroll_one_eq_16!(a_ptr, b_ptr, loop_size, 1);
            _unroll_one_eq_16!(a_ptr, b_ptr, loop_size, 2);
            _unroll_one_eq_16!(a_ptr, b_ptr, loop_size, 3);
            _unroll_one_eq_16!(a_ptr, b_ptr, loop_size, 4);
            _unroll_one_eq_16!(a_ptr, b_ptr, loop_size, 5);
            _unroll_one_eq_16!(a_ptr, b_ptr, loop_size, 6);
            _unroll_one_eq_16!(a_ptr, b_ptr, loop_size, 7);
            //
            a_ptr = unsafe { a_ptr.add(loop_size * unroll) };
            b_ptr = unsafe { b_ptr.add(loop_size * unroll) };
        }
    }
    {
        let loop_size = 16;
        while unsafe { end_ptr.offset_from(a_ptr) } >= loop_size as isize {
            _unroll_one_eq_16!(a_ptr, b_ptr, loop_size, 0);
            a_ptr = unsafe { a_ptr.add(loop_size) };
            b_ptr = unsafe { b_ptr.add(loop_size) };
        }
    }
    // the remaining data is the max: 15 bytes.
    _memeq_remaining_15_bytes_impl(a_ptr, b_ptr, end_ptr)
}

#[cfg(target_pointer_width = "64")]
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
    //
    {
        let unroll = 8;
        let loop_size = 8;
        while unsafe { end_ptr.offset_from(a_ptr) } >= (loop_size * unroll) as isize {
            _unroll_one_eq_8!(a_ptr, b_ptr, loop_size, 0);
            _unroll_one_eq_8!(a_ptr, b_ptr, loop_size, 1);
            _unroll_one_eq_8!(a_ptr, b_ptr, loop_size, 2);
            _unroll_one_eq_8!(a_ptr, b_ptr, loop_size, 3);
            _unroll_one_eq_8!(a_ptr, b_ptr, loop_size, 4);
            _unroll_one_eq_8!(a_ptr, b_ptr, loop_size, 5);
            _unroll_one_eq_8!(a_ptr, b_ptr, loop_size, 6);
            _unroll_one_eq_8!(a_ptr, b_ptr, loop_size, 7);
            //
            a_ptr = unsafe { a_ptr.add(loop_size * unroll) };
            b_ptr = unsafe { b_ptr.add(loop_size * unroll) };
        }
    }
    {
        let loop_size = 8;
        while unsafe { end_ptr.offset_from(a_ptr) } >= loop_size as isize {
            _unroll_one_eq_8!(a_ptr, b_ptr, loop_size, 0);
            a_ptr = unsafe { a_ptr.add(loop_size) };
            b_ptr = unsafe { b_ptr.add(loop_size) };
        }
    }
    // the remaining data is the max: 7 bytes.
    _memeq_remaining_7_bytes_impl(a_ptr, b_ptr, end_ptr)
}

#[cfg(target_pointer_width = "32")]
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
    //
    {
        let unroll = 8;
        let loop_size = 4;
        while unsafe { end_ptr.offset_from(a_ptr) } >= (loop_size * unroll) as isize {
            _unroll_one_eq_4!(a_ptr, b_ptr, loop_size, 0);
            _unroll_one_eq_4!(a_ptr, b_ptr, loop_size, 1);
            _unroll_one_eq_4!(a_ptr, b_ptr, loop_size, 2);
            _unroll_one_eq_4!(a_ptr, b_ptr, loop_size, 3);
            _unroll_one_eq_4!(a_ptr, b_ptr, loop_size, 4);
            _unroll_one_eq_4!(a_ptr, b_ptr, loop_size, 5);
            _unroll_one_eq_4!(a_ptr, b_ptr, loop_size, 6);
            _unroll_one_eq_4!(a_ptr, b_ptr, loop_size, 7);
            //
            a_ptr = unsafe { a_ptr.add(loop_size * unroll) };
            b_ptr = unsafe { b_ptr.add(loop_size * unroll) };
        }
    }
    {
        let loop_size = 4;
        while unsafe { end_ptr.offset_from(a_ptr) } >= loop_size as isize {
            _unroll_one_eq_4!(a_ptr, b_ptr, loop_size, 0);
            //
            a_ptr = unsafe { a_ptr.add(loop_size) };
            b_ptr = unsafe { b_ptr.add(loop_size) };
        }
    }
    // the remaining data is the max: 3 bytes.
    _memeq_remaining_3_bytes_impl(a_ptr, b_ptr, end_ptr)
}

#[inline(always)]
fn _memeq_remaining_15_bytes_impl(a_ptr: *const u8, b_ptr: *const u8, end_ptr: *const u8) -> bool {
    let mut a_ptr = a_ptr;
    let mut b_ptr = b_ptr;
    {
        let loop_size = 8;
        if unsafe { end_ptr.offset_from(a_ptr) } >= loop_size as isize {
            _unroll_one_eq_8!(a_ptr, b_ptr, loop_size, 0);
            //
            a_ptr = unsafe { a_ptr.add(loop_size) };
            b_ptr = unsafe { b_ptr.add(loop_size) };
        }
    }
    // the remaining data is the max: 7 bytes.
    _memeq_remaining_7_bytes_impl(a_ptr, b_ptr, end_ptr)
}

#[inline(always)]
fn _memeq_remaining_7_bytes_impl(a_ptr: *const u8, b_ptr: *const u8, end_ptr: *const u8) -> bool {
    let mut a_ptr = a_ptr;
    let mut b_ptr = b_ptr;
    {
        let loop_size = 4;
        if unsafe { end_ptr.offset_from(a_ptr) } >= loop_size as isize {
            _unroll_one_eq_4!(a_ptr, b_ptr, loop_size, 0);
            //
            a_ptr = unsafe { a_ptr.add(loop_size) };
            b_ptr = unsafe { b_ptr.add(loop_size) };
        }
    }
    // the remaining data is the max: 3 bytes.
    _memeq_remaining_3_bytes_impl(a_ptr, b_ptr, end_ptr)
}

#[inline(always)]
fn _memeq_remaining_3_bytes_impl(a_ptr: *const u8, b_ptr: *const u8, end_ptr: *const u8) -> bool {
    let mut a_ptr = a_ptr;
    let mut b_ptr = b_ptr;
    {
        let loop_size = 2;
        if unsafe { end_ptr.offset_from(a_ptr) } >= loop_size as isize {
            _unroll_one_eq_2!(a_ptr, b_ptr, loop_size, 0);
            //
            a_ptr = unsafe { a_ptr.add(loop_size) };
            b_ptr = unsafe { b_ptr.add(loop_size) };
        }
    }
    {
        let loop_size = 1;
        if unsafe { end_ptr.offset_from(a_ptr) } >= loop_size as isize {
            _unroll_one_eq_1!(a_ptr, b_ptr, loop_size, 0);
        }
    }
    //
    true
}

#[cfg(target_pointer_width = "16")]
#[inline(always)]
fn _start_eq_16(a: &[u8], b: &[u8]) -> bool {
    //
    let a_len = a.len();
    let b_len = b.len();
    if a_len != b_len {
        return false;
    }
    let mut a_ptr = a.as_ptr();
    let mut b_ptr = b.as_ptr();
    let end_ptr = unsafe { a_ptr.add(a_len) };
    //
    {
        let unroll = 8;
        let loop_size = 2;
        let end_ptr_2_8 = unsafe { end_ptr.sub(loop_size * unroll) };
        while a_ptr <= end_ptr_2_8 {
            _unroll_one_eq_2!(a_ptr, b_ptr, loop_size, 0);
            _unroll_one_eq_2!(a_ptr, b_ptr, loop_size, 1);
            _unroll_one_eq_2!(a_ptr, b_ptr, loop_size, 2);
            _unroll_one_eq_2!(a_ptr, b_ptr, loop_size, 3);
            _unroll_one_eq_2!(a_ptr, b_ptr, loop_size, 4);
            _unroll_one_eq_2!(a_ptr, b_ptr, loop_size, 5);
            _unroll_one_eq_2!(a_ptr, b_ptr, loop_size, 6);
            _unroll_one_eq_2!(a_ptr, b_ptr, loop_size, 7);
            //
            a_ptr = unsafe { a_ptr.add(loop_size * unroll) };
            b_ptr = unsafe { b_ptr.add(loop_size * unroll) };
        }
    }
    {
        let loop_size = 2;
        let end_ptr_2 = unsafe { end_ptr.sub(loop_size) };
        while a_ptr <= end_ptr_2 {
            _unroll_one_eq_2!(a_ptr, b_ptr, loop_size, 0);
            //
            a_ptr = unsafe { a_ptr.add(loop_size) };
            b_ptr = unsafe { b_ptr.add(loop_size) };
        }
    }
    // the remaining data is the max: 1 bytes.
    {
        let loop_size = 1;
        let end_ptr_1 = unsafe { end_ptr.sub(loop_size) };
        if a_ptr <= end_ptr_1 {
            _unroll_one_eq_1!(a_ptr, b_ptr, loop_size, 0);
        }
    }
    //
    true
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

#[inline(always)]
pub fn _memeq_impl(a: &[u8], b: &[u8]) -> bool {
    let a_len = a.len();
    let b_len = b.len();
    if a_len != b_len {
        return false;
    }
    let min_len = a_len;
    let mut a_ptr = a.as_ptr();
    let mut b_ptr = b.as_ptr();
    let end_ptr = unsafe { a_ptr.add(min_len) };
    while a_ptr < end_ptr {
        let aa = unsafe { *a_ptr };
        let bb = unsafe { *b_ptr };
        if aa != bb {
            return false;
        }
        a_ptr = unsafe { a_ptr.add(1) };
        b_ptr = unsafe { b_ptr.add(1) };
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
