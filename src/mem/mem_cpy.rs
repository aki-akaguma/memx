use super::super::RangeError;

#[inline(always)]
pub fn _memcpy_impl(dst: &mut [u8], src: &[u8]) -> Result<(), RangeError> {
    #[cfg(target_arch = "arm")]
    let r = {
        // measures to prevent bus errors
        #[cfg(target_pointer_width = "32")]
        let r = _start_cpy_32_no_unroll(dst, src);
        #[cfg(target_pointer_width = "16")]
        let r = _start_cpy_1(dst, src);
        //
        r
    };
    #[cfg(not(target_arch = "arm"))]
    let r = {
        #[cfg(target_pointer_width = "128")]
        let r = _start_cpy_128(dst, src);
        #[cfg(target_pointer_width = "64")]
        let r = _start_cpy_64(dst, src);
        #[cfg(target_pointer_width = "32")]
        let r = _start_cpy_32(dst, src);
        #[cfg(target_pointer_width = "16")]
        let r = _start_cpy_16(dst, src);
        //
        r
    };
    //
    r
}

macro_rules! _unroll_one_cpy_16 {
    ($a_ptr:expr, $b_ptr:expr, $loop_size:expr, $idx:expr) => {{
        let aa_ptr = unsafe { $a_ptr.add($loop_size * $idx) };
        let bb_ptr = unsafe { $b_ptr.add($loop_size * $idx) };
        //
        let aaa_ptr = aa_ptr as *mut u128;
        let bbb_ptr = bb_ptr as *const u128;
        unsafe {
            *aaa_ptr = *bbb_ptr;
        }
    }};
}

macro_rules! _unroll_one_cpy_8 {
    ($a_ptr:expr, $b_ptr:expr, $loop_size:expr, $idx:expr) => {{
        let aa_ptr = unsafe { $a_ptr.add($loop_size * $idx) };
        let bb_ptr = unsafe { $b_ptr.add($loop_size * $idx) };
        //
        let aaa_ptr = aa_ptr as *mut u64;
        let bbb_ptr = bb_ptr as *const u64;
        unsafe {
            *aaa_ptr = *bbb_ptr;
        }
    }};
}

macro_rules! _unroll_one_cpy_4 {
    ($a_ptr:expr, $b_ptr:expr, $loop_size:expr, $idx:expr) => {{
        let aa_ptr = unsafe { $a_ptr.add($loop_size * $idx) };
        let bb_ptr = unsafe { $b_ptr.add($loop_size * $idx) };
        //
        let aaa_ptr = aa_ptr as *mut u32;
        let bbb_ptr = bb_ptr as *const u32;
        unsafe {
            *aaa_ptr = *bbb_ptr;
        }
    }};
}

macro_rules! _unroll_one_cpy_2 {
    ($a_ptr:expr, $b_ptr:expr, $loop_size:expr, $idx:expr) => {{
        let aa_ptr = unsafe { $a_ptr.add($loop_size * $idx) };
        let bb_ptr = unsafe { $b_ptr.add($loop_size * $idx) };
        //
        let aaa_ptr = aa_ptr as *mut u16;
        let bbb_ptr = bb_ptr as *const u16;
        unsafe {
            *aaa_ptr = *bbb_ptr;
        }
    }};
}

macro_rules! _unroll_one_cpy_1 {
    ($a_ptr:expr, $b_ptr:expr, $loop_size:expr, $idx:expr) => {{
        let aa_ptr = unsafe { $a_ptr.add($loop_size * $idx) };
        let bb_ptr = unsafe { $b_ptr.add($loop_size * $idx) };
        //
        let aaa_ptr = aa_ptr as *mut u8;
        let bbb_ptr = bb_ptr as *const u8;
        unsafe {
            *aaa_ptr = *bbb_ptr;
        }
    }};
}

#[cfg(target_pointer_width = "128")]
#[inline(always)]
fn _start_cpy_128(dst: &mut [u8], src: &[u8]) -> Result<(), RangeError> {
    let dst_len = dst.len();
    let src_len = src.len();
    if dst_len < src_len {
        return Err(RangeError);
    }
    let mut a_ptr = dst.as_mut_ptr();
    let mut b_ptr = src.as_ptr();
    let end_ptr = unsafe { b_ptr.add(src_len) };
    //
    {
        let unroll = 8;
        let loop_size = 16;
        let end_ptr_16_8 = unsafe { end_ptr.sub(loop_size * unroll) };
        while b_ptr <= end_ptr_16_8 {
            for i in 0..unroll {
                _unroll_one_cpy_16!(a_ptr, b_ptr, loop_size, i);
            }
            //
            a_ptr = unsafe { a_ptr.add(loop_size * unroll) };
            b_ptr = unsafe { b_ptr.add(loop_size * unroll) };
        }
    }
    {
        let loop_size = 16;
        let end_ptr_16 = unsafe { end_ptr.sub(loop_size) };
        while b_ptr <= end_ptr_16 {
            _unroll_one_cpy_16!(a_ptr, b_ptr, loop_size, 0);
            a_ptr = unsafe { a_ptr.add(loop_size) };
            b_ptr = unsafe { b_ptr.add(loop_size) };
        }
    }
    // the remaining data is the max: 15 bytes.
    _memcpy_remaining_15_bytes_impl(a_ptr, b_ptr, end_ptr)
}

#[inline(always)]
pub(crate) fn _memcpy_remaining_15_bytes_impl(
    dst_ptr: *const u8,
    src_ptr: *const u8,
    end_ptr: *const u8,
) -> Result<(), RangeError> {
    let mut a_ptr = dst_ptr;
    let mut b_ptr = src_ptr;
    {
        let loop_size = 8;
        let end_ptr_8 = unsafe { end_ptr.sub(loop_size) };
        if b_ptr <= end_ptr_8 {
            _unroll_one_cpy_8!(a_ptr, b_ptr, loop_size, 0);
            //
            a_ptr = unsafe { a_ptr.add(loop_size) };
            b_ptr = unsafe { b_ptr.add(loop_size) };
        }
    }
    {
        let loop_size = 4;
        let end_ptr_4 = unsafe { end_ptr.sub(loop_size) };
        if b_ptr <= end_ptr_4 {
            _unroll_one_cpy_4!(a_ptr, b_ptr, loop_size, 0);
            //
            a_ptr = unsafe { a_ptr.add(loop_size) };
            b_ptr = unsafe { b_ptr.add(loop_size) };
        }
    }
    {
        let loop_size = 2;
        let end_ptr_2 = unsafe { end_ptr.sub(loop_size) };
        if b_ptr <= end_ptr_2 {
            _unroll_one_cpy_2!(a_ptr, b_ptr, loop_size, 0);
            //
            a_ptr = unsafe { a_ptr.add(loop_size) };
            b_ptr = unsafe { b_ptr.add(loop_size) };
        }
    }
    {
        let loop_size = 1;
        let end_ptr_1 = unsafe { end_ptr.sub(loop_size) };
        if b_ptr <= end_ptr_1 {
            _unroll_one_cpy_1!(a_ptr, b_ptr, loop_size, 0);
        }
    }
    //
    Ok(())
}

#[cfg(target_pointer_width = "64")]
#[inline(always)]
fn _start_cpy_64(dst: &mut [u8], src: &[u8]) -> Result<(), RangeError> {
    let dst_len = dst.len();
    let src_len = src.len();
    if dst_len < src_len {
        return Err(RangeError);
    }
    let mut a_ptr = dst.as_mut_ptr();
    let mut b_ptr = src.as_ptr();
    let end_ptr = unsafe { b_ptr.add(src_len) };
    //
    {
        let unroll = 8;
        let loop_size = 8;
        let end_ptr_8_8 = unsafe { end_ptr.sub(loop_size * unroll) };
        while b_ptr <= end_ptr_8_8 {
            for i in 0..unroll {
                _unroll_one_cpy_8!(a_ptr, b_ptr, loop_size, i);
            }
            //
            a_ptr = unsafe { a_ptr.add(loop_size * unroll) };
            b_ptr = unsafe { b_ptr.add(loop_size * unroll) };
        }
    }
    {
        let loop_size = 8;
        let end_ptr_8 = unsafe { end_ptr.sub(loop_size) };
        while b_ptr <= end_ptr_8 {
            _unroll_one_cpy_8!(a_ptr, b_ptr, loop_size, 0);
            a_ptr = unsafe { a_ptr.add(loop_size) };
            b_ptr = unsafe { b_ptr.add(loop_size) };
        }
    }
    // the remaining data is the max: 7 bytes.
    _memcpy_remaining_7_bytes_impl(a_ptr, b_ptr, end_ptr)
}

#[inline(always)]
pub(crate) fn _memcpy_remaining_7_bytes_impl(
    dst_ptr: *const u8,
    src_ptr: *const u8,
    end_ptr: *const u8,
) -> Result<(), RangeError> {
    let mut a_ptr = dst_ptr;
    let mut b_ptr = src_ptr;
    {
        let loop_size = 4;
        let end_ptr_4 = unsafe { end_ptr.sub(loop_size) };
        if b_ptr <= end_ptr_4 {
            _unroll_one_cpy_4!(a_ptr, b_ptr, loop_size, 0);
            //
            a_ptr = unsafe { a_ptr.add(loop_size) };
            b_ptr = unsafe { b_ptr.add(loop_size) };
        }
    }
    {
        let loop_size = 2;
        let end_ptr_2 = unsafe { end_ptr.sub(loop_size) };
        if b_ptr <= end_ptr_2 {
            _unroll_one_cpy_2!(a_ptr, b_ptr, loop_size, 0);
            //
            a_ptr = unsafe { a_ptr.add(loop_size) };
            b_ptr = unsafe { b_ptr.add(loop_size) };
        }
    }
    {
        let loop_size = 1;
        let end_ptr_1 = unsafe { end_ptr.sub(loop_size) };
        if b_ptr <= end_ptr_1 {
            _unroll_one_cpy_1!(a_ptr, b_ptr, loop_size, 0);
        }
    }
    //
    Ok(())
}

#[cfg(target_pointer_width = "32")]
#[inline(always)]
fn _start_cpy_32(dst: &mut [u8], src: &[u8]) -> Result<(), RangeError> {
    let dst_len = dst.len();
    let src_len = src.len();
    if dst_len < src_len {
        return Err(RangeError);
    }
    let mut a_ptr = dst.as_mut_ptr();
    let mut b_ptr = src.as_ptr();
    let end_ptr = unsafe { b_ptr.add(src_len) };
    //
    {
        let unroll = 8;
        let loop_size = 4;
        let end_ptr_4_8 = unsafe { end_ptr.sub(loop_size * unroll) };
        while b_ptr <= end_ptr_4_8 {
            for i in 0..unroll {
                _unroll_one_cpy_4!(a_ptr, b_ptr, loop_size, i);
            }
            //
            a_ptr = unsafe { a_ptr.add(loop_size * unroll) };
            b_ptr = unsafe { b_ptr.add(loop_size * unroll) };
        }
    }
    {
        let loop_size = 4;
        let end_ptr_4 = unsafe { end_ptr.sub(loop_size) };
        while b_ptr <= end_ptr_4 {
            _unroll_one_cpy_4!(a_ptr, b_ptr, loop_size, 0);
            a_ptr = unsafe { a_ptr.add(loop_size) };
            b_ptr = unsafe { b_ptr.add(loop_size) };
        }
    }
    // the remaining data is the max: 3 bytes.
    _memcpy_remaining_3_bytes_impl(a_ptr, b_ptr, end_ptr)
}

#[inline(always)]
pub(crate) fn _memcpy_remaining_3_bytes_impl(
    dst_ptr: *const u8,
    src_ptr: *const u8,
    end_ptr: *const u8,
) -> Result<(), RangeError> {
    let mut a_ptr = dst_ptr;
    let mut b_ptr = src_ptr;
    {
        let loop_size = 2;
        let end_ptr_2 = unsafe { end_ptr.sub(loop_size) };
        if b_ptr <= end_ptr_2 {
            _unroll_one_cpy_2!(a_ptr, b_ptr, loop_size, 0);
            //
            a_ptr = unsafe { a_ptr.add(loop_size) };
            b_ptr = unsafe { b_ptr.add(loop_size) };
        }
    }
    {
        let loop_size = 1;
        let end_ptr_1 = unsafe { end_ptr.sub(loop_size) };
        if b_ptr <= end_ptr_1 {
            _unroll_one_cpy_1!(a_ptr, b_ptr, loop_size, 0);
        }
    }
    Ok(())
}

#[cfg(target_pointer_width = "16")]
#[inline(always)]
fn _start_cpy_16(dst: &mut [u8], src: &[u8]) -> Result<(), RangeError> {
    let dst_len = dst.len();
    let src_len = src.len();
    if dst_len < src_len {
        return Err(RangeError);
    }
    let mut a_ptr = dst.as_mut_ptr();
    let mut b_ptr = src.as_ptr();
    let end_ptr = unsafe { b_ptr.add(src_len) };
    //
    {
        let unroll = 8;
        let loop_size = 2;
        let end_ptr_2_8 = unsafe { end_ptr.sub(loop_size * unroll) };
        while b_ptr <= end_ptr_2_8 {
            for i in 0..unroll {
                _unroll_one_cpy_2!(a_ptr, b_ptr, loop_size, i);
            }
            //
            a_ptr = unsafe { a_ptr.add(loop_size * unroll) };
            b_ptr = unsafe { b_ptr.add(loop_size * unroll) };
        }
    }
    {
        let loop_size = 2;
        let end_ptr_2 = unsafe { end_ptr.sub(loop_size) };
        while b_ptr <= end_ptr_2 {
            _unroll_one_cpy_2!(a_ptr, b_ptr, loop_size, 0);
            a_ptr = unsafe { a_ptr.add(loop_size) };
            b_ptr = unsafe { b_ptr.add(loop_size) };
        }
    }
    // the remaining data is the max: 1 bytes.
    {
        let loop_size = 1;
        let end_ptr_1 = unsafe { end_ptr.sub(loop_size) };
        if b_ptr <= end_ptr_1 {
            _unroll_one_cpy_1!(a_ptr, b_ptr, loop_size, 0);
        }
    }
    Ok(())
}

#[inline(always)]
fn _start_cpy_1(dst: &mut [u8], src: &[u8]) -> Result<(), RangeError> {
    let dst_len = dst.len();
    let src_len = src.len();
    if dst_len < src_len {
        return Err(RangeError);
    }
    let mut dst_ptr = dst.as_mut_ptr();
    let mut src_ptr = src.as_ptr();
    let end_ptr = unsafe { dst_ptr.add(src_len) };
    while dst_ptr < end_ptr {
        unsafe {
            *dst_ptr = *src_ptr;
        }
        dst_ptr = unsafe { dst_ptr.add(1) };
        src_ptr = unsafe { src_ptr.add(1) };
    }
    Ok(())
}

#[cfg(target_pointer_width = "32")]
#[inline(always)]
fn _start_cpy_32_no_unroll(dst: &mut [u8], src: &[u8]) -> Result<(), RangeError> {
    let dst_len = dst.len();
    let src_len = src.len();
    if dst_len < src_len {
        return Err(RangeError);
    }
    let mut a_ptr = dst.as_mut_ptr();
    let mut b_ptr = src.as_ptr();
    let end_ptr = unsafe { b_ptr.add(src_len) };
    //
    {
        let loop_size = 4;
        let end_ptr_4 = unsafe { end_ptr.sub(loop_size) };
        while b_ptr <= end_ptr_4 {
            _unroll_one_cpy_4!(a_ptr, b_ptr, loop_size, 0);
            a_ptr = unsafe { a_ptr.add(loop_size) };
            b_ptr = unsafe { b_ptr.add(loop_size) };
        }
    }
    // the remaining data is the max: 3 bytes.
    _memcpy_remaining_3_bytes_impl(a_ptr, b_ptr, end_ptr)
}

/*
#[inline(always)]
pub fn _memcpy_impl(dst: &mut [u8], src: &[u8]) -> Result<(), RangeError> {
    let dst_len = dst.len();
    let src_len = src.len();
    if dst_len < src_len {
        return Err(RangeError);
    }
    let mut dst_ptr = dst.as_mut_ptr();
    let mut src_ptr = src.as_ptr();
    let end_ptr = unsafe { dst_ptr.add(src_len) };
    //
    #[cfg(target_pointer_width = "64")]
    {
        let loop_size = 8;
        while dst_ptr <= unsafe { end_ptr.sub(loop_size) } {
            let dst_ptr_c8 = dst_ptr as *mut u64;
            let src_ptr_c8 = src_ptr as *const u64;
            unsafe {
                *dst_ptr_c8 = *src_ptr_c8;
            }
            dst_ptr = unsafe { dst_ptr.add(loop_size) };
            src_ptr = unsafe { src_ptr.add(loop_size) };
        }
    }
    //
    let loop_size = 4;
    while dst_ptr <= unsafe { end_ptr.sub(loop_size) } {
        let dst_ptr_c4 = dst_ptr as *mut u32;
        let src_ptr_c4 = src_ptr as *const u32;
        unsafe {
            *dst_ptr_c4 = *src_ptr_c4;
        }
        dst_ptr = unsafe { dst_ptr.add(loop_size) };
        src_ptr = unsafe { src_ptr.add(loop_size) };
    }
    /*
    let loop_size = 2;
    while dst_ptr <= unsafe { end_ptr.sub(loop_size) } {
        let dst_ptr_c2 = dst_ptr as *mut u16;
        let src_ptr_c2 = src_ptr as *const u16;
        unsafe {
            *dst_ptr_c2 = *src_ptr_c2;
        }
        dst_ptr = unsafe { dst_ptr.add(loop_size) };
        src_ptr = unsafe { src_ptr.add(loop_size) };
    }
    */
    //
    while dst_ptr < end_ptr {
        unsafe {
            *dst_ptr = *src_ptr;
        }
        dst_ptr = unsafe { dst_ptr.add(1) };
        src_ptr = unsafe { src_ptr.add(1) };
    }
    Ok(())
}
*/

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

#[inline(always)]
pub fn _memcpy_impl(dst: &mut [u8], src: &[u8]) -> Result<(), RangeError> {
    let dst_len = dst.len();
    let src_len = src.len();
    if dst_len < src_len {
        return Err(RangeError);
    }
    let mut dst_ptr = dst.as_mut_ptr();
    let mut src_ptr = src.as_ptr();
    let end_ptr = unsafe { dst_ptr.add(src_len) };
    while dst_ptr < end_ptr {
        unsafe {
            *dst_ptr = *src_ptr;
        }
        dst_ptr = unsafe { dst_ptr.add(1) };
        src_ptr = unsafe { src_ptr.add(1) };
    }
    Ok(())
}
*/
