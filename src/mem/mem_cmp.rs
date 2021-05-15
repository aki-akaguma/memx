use std::cmp::Ordering;

/*
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

