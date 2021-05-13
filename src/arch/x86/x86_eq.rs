
#[inline(always)]
pub fn _memeq_impl(a: &[u8], b: &[u8]) -> bool {
    //
    let a_len = a.len();
    let b_len = b.len();
    if a_len != b_len {
        return true;
    }
    let mut a_ptr = a.as_ptr();
    let mut b_ptr = b.as_ptr();
    let end_ptr = unsafe { a_ptr.add(a_len) };
    //
    let loop_size = 8;
    while a_ptr <= unsafe { end_ptr.sub(loop_size) } {
        if !unsafe { _cmp_bytes_8(a_ptr, b_ptr) } {
            return false;
        }
        a_ptr = unsafe { a_ptr.add(loop_size) };
        b_ptr = unsafe { b_ptr.add(loop_size) };
    }
    let loop_size = 4;
    while a_ptr <= unsafe { end_ptr.sub(loop_size) } {
        if !unsafe { _cmp_bytes_4(a_ptr, b_ptr) } {
            return false;
        }
        a_ptr = unsafe { a_ptr.add(loop_size) };
        b_ptr = unsafe { b_ptr.add(loop_size) };
    }
    let loop_size = 2;
    while a_ptr <= unsafe { end_ptr.sub(loop_size) } {
        if !unsafe { _cmp_bytes_2(a_ptr, b_ptr) } {
            return false;
        }
        a_ptr = unsafe { a_ptr.add(loop_size) };
        b_ptr = unsafe { b_ptr.add(loop_size) };
    }
    while a_ptr < end_ptr {
        unsafe {
            if *a_ptr != *b_ptr {
                return false;
            }
            a_ptr = a_ptr.add(1);
            b_ptr = b_ptr.add(1);
        }
    }
    true
}

#[inline]
unsafe fn _cmp_bytes_2(a_ptr: *const u8, b_ptr: *const u8) -> bool {
    let a_ptr = a_ptr as *const u16;
    let b_ptr = b_ptr as *const u16;
    *a_ptr == *b_ptr
}

#[inline]
unsafe fn _cmp_bytes_4(a_ptr: *const u8, b_ptr: *const u8) -> bool {
    let a_ptr = a_ptr as *const u32;
    let b_ptr = b_ptr as *const u32;
    *a_ptr == *b_ptr
}

#[inline]
unsafe fn _cmp_bytes_8(a_ptr: *const u8, b_ptr: *const u8) -> bool {
    let a_ptr = a_ptr as *const u64;
    let b_ptr = b_ptr as *const u64;
    *a_ptr == *b_ptr
}
