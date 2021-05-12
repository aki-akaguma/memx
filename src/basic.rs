use std::cmp::Ordering;
use super::RangeError;

#[inline(always)]
pub fn memeq_impl(a: &[u8], b: &[u8]) -> bool {
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

#[inline(always)]
pub fn memcmp_impl(a: &[u8], b: &[u8]) -> Ordering {
    let a_len = a.len();
    let b_len = b.len();
    let min_len = a_len.min(b_len);
    let mut a_ptr = a.as_ptr();
    let mut b_ptr = b.as_ptr();
    let end_ptr = unsafe { a_ptr.add(min_len) };
    while a_ptr < end_ptr {
        let aa = unsafe { *a_ptr };
        let bb = unsafe { *b_ptr };
        let cmp = aa.cmp(&bb);
        if cmp != Ordering::Equal {
            return cmp;
        }
        a_ptr = unsafe { a_ptr.add(1) };
        b_ptr = unsafe { b_ptr.add(1) };
    }
    a_len.cmp(&b_len)
}

#[inline(always)]
pub fn memchr_impl(buf: &[u8], c: u8) -> Option<usize> {
    let buf_len = buf.len();
    let start_ptr = buf.as_ptr();
    let end_ptr = unsafe { start_ptr.add(buf_len) };
    let mut buf_ptr = start_ptr;
    while buf_ptr < end_ptr {
        let aa = unsafe { *buf_ptr };
        if aa == c {
            return Some( unsafe { buf_ptr.offset_from(start_ptr) } as usize);
        }
        buf_ptr = unsafe { buf_ptr.add(1) };
    }
    None
}

#[inline(always)]
pub fn memmem_impl(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    let hay_len = haystack.len();
    let nee_len = needle.len();
    if hay_len < nee_len {
        return None;
    }
    for i in 0..(hay_len - nee_len) {
        if &haystack[i..(i+nee_len)] == needle {
            return Some(i);
        }
    }
    None
}

#[inline(always)]
pub fn memcpy_impl(dst: &mut [u8], src: &[u8], n: usize) -> Result<(), RangeError> {
    if dst.len() < n || src.len() < n {
        return Err(RangeError);
    }
    for i in 0..n {
        dst[i] = src[i];
    }
    Ok(())
}

#[inline(always)]
pub fn memset_impl(buf: &mut [u8], c: u8, n: usize) -> Result<(), RangeError> {
    if buf.len() < n {
        return Err(RangeError);
    }
    for i in 0..n {
        buf[i] = c;
    }
    Ok(())
}
