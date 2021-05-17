use std::cmp::Ordering;

#[inline(always)]
pub fn _memcmp_impl(a: &[u8], b: &[u8]) -> Ordering {
    //
    let a_len = a.len();
    let b_len = b.len();
    let min_len = a_len.min(b_len);
    let mut a_ptr = a.as_ptr();
    let mut b_ptr = b.as_ptr();
    let end_ptr = unsafe { a_ptr.add(min_len) };
    //
    #[cfg(target_pointer_width = "64")]
    {
        let loop_size = 8;
        while a_ptr <= unsafe { end_ptr.sub(loop_size) } {
            let r = unsafe { _cmp_bytes_8(a_ptr, b_ptr) };
            if r != Ordering::Equal {
                return r;
            }
            a_ptr = unsafe { a_ptr.add(loop_size) };
            b_ptr = unsafe { b_ptr.add(loop_size) };
        }
    }
    //
    let loop_size = 4;
    while a_ptr <= unsafe { end_ptr.sub(loop_size) } {
        let r = unsafe { _cmp_bytes_4(a_ptr, b_ptr) };
        if r != Ordering::Equal {
            return r;
        }
        a_ptr = unsafe { a_ptr.add(loop_size) };
        b_ptr = unsafe { b_ptr.add(loop_size) };
    }
    let loop_size = 2;
    while a_ptr <= unsafe { end_ptr.sub(loop_size) } {
        let r = unsafe { _cmp_bytes_2(a_ptr, b_ptr) };
        if r != Ordering::Equal {
            return r;
        }
        a_ptr = unsafe { a_ptr.add(loop_size) };
        b_ptr = unsafe { b_ptr.add(loop_size) };
    }
    while a_ptr < end_ptr {
        unsafe {
            let r = (*a_ptr).cmp(&(*b_ptr));
            if r != Ordering::Equal {
                return r;
            }
            a_ptr = a_ptr.add(1);
            b_ptr = b_ptr.add(1);
        }
    }
    a_len.cmp(&b_len)
}

macro_rules! one {
    (0; $a1_ptr:expr, $b1_ptr:expr) => {{
        let r = (*$a1_ptr).cmp(&(*$b1_ptr));
        if r != Ordering::Equal {
            return r;
        }
    }};
    (1; $a1_ptr:expr, $b1_ptr:expr) => {{
        $a1_ptr = $a1_ptr.add(1);
        $b1_ptr = $b1_ptr.add(1);
        one!(0; $a1_ptr, $b1_ptr);
    }};
    (2; $a1_ptr:expr, $b1_ptr:expr) => {
        one!(0; $a1_ptr, $b1_ptr);
        one!(1; $a1_ptr, $b1_ptr);
    };
    (4; $a1_ptr:expr, $b1_ptr:expr) => {
        one!(0; $a1_ptr, $b1_ptr);
        one!(1; $a1_ptr, $b1_ptr);
        one!(1; $a1_ptr, $b1_ptr);
        one!(1; $a1_ptr, $b1_ptr);
    };
    (8; $a1_ptr:expr, $b1_ptr:expr) => {
        one!(0; $a1_ptr, $b1_ptr);
        one!(1; $a1_ptr, $b1_ptr);
        one!(1; $a1_ptr, $b1_ptr);
        one!(1; $a1_ptr, $b1_ptr);
        //
        one!(1; $a1_ptr, $b1_ptr);
        one!(1; $a1_ptr, $b1_ptr);
        one!(1; $a1_ptr, $b1_ptr);
        one!(1; $a1_ptr, $b1_ptr);
    };
}

#[inline]
unsafe fn _cmp_bytes_2(a_ptr: *const u8, b_ptr: *const u8) -> Ordering {
    let aa_ptr = a_ptr as *const u16;
    let bb_ptr = b_ptr as *const u16;
    let r = (*aa_ptr).cmp(&(*bb_ptr));
    if r == Ordering::Equal {
        return Ordering::Equal;
    }
    //
    let mut a1_ptr = a_ptr;
    let mut b1_ptr = b_ptr;
    //
    one!(2; a1_ptr, b1_ptr);
    //
    Ordering::Equal
}

#[inline]
unsafe fn _cmp_bytes_4(a_ptr: *const u8, b_ptr: *const u8) -> Ordering {
    let aa_ptr = a_ptr as *const u32;
    let bb_ptr = b_ptr as *const u32;
    let r = (*aa_ptr).cmp(&(*bb_ptr));
    if r == Ordering::Equal {
        return Ordering::Equal;
    }
    //
    let mut a1_ptr = a_ptr;
    let mut b1_ptr = b_ptr;
    //
    one!(4; a1_ptr, b1_ptr);
    //
    Ordering::Equal
}

#[inline]
unsafe fn _cmp_bytes_8(a_ptr: *const u8, b_ptr: *const u8) -> Ordering {
    let aa_ptr = a_ptr as *const u64;
    let bb_ptr = b_ptr as *const u64;
    let r = (*aa_ptr).cmp(&(*bb_ptr));
    if r == Ordering::Equal {
        return Ordering::Equal;
    }
    //
    let mut a1_ptr = a_ptr;
    let mut b1_ptr = b_ptr;
    //
    one!(8; a1_ptr, b1_ptr);
    //
    Ordering::Equal
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

#[inline(always)]
pub fn _memcmp_impl(a: &[u8], b: &[u8]) -> Ordering {
    let a_len = a.len();
    let b_len = b.len();
    let min_len = a_len.min(b_len);
    let mut a_ptr = a.as_ptr();
    let mut b_ptr = b.as_ptr();
    let end_ptr = unsafe { a_ptr.add(min_len) };
    while a_ptr < end_ptr {
        let aa = unsafe { *a_ptr };
        let bb = unsafe { *b_ptr };
        if aa != bb {
            return aa.cmp(&bb);
        }
        a_ptr = unsafe { a_ptr.add(1) };
        b_ptr = unsafe { b_ptr.add(1) };
    }
    a_len.cmp(&b_len)
}
*/
