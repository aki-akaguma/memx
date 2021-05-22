mod libc_chr;
pub(crate) use libc_chr::_memchr_impl;

mod libc_cmp;
pub(crate) use libc_cmp::_memcmp_impl;

mod libc_cpy;
pub(crate) use libc_cpy::_memcpy_impl;

mod libc_eq;
pub(crate) use libc_eq::_memeq_impl;

mod libc_mem;
pub(crate) use libc_mem::_memmem_impl;

mod libc_set;
pub(crate) use libc_set::_memset_impl;

use std::cmp::Ordering;
use super::RangeError;

pub fn memchr_libc(buf: &[u8], c: u8) -> Option<usize> {
    crate::libc::_memchr_impl(buf, c)
}

pub fn memcmp_libc(a: &[u8], b: &[u8]) -> Ordering {
    crate::libc::_memcmp_impl(a, b)
}

pub fn memcpy_libc(dst: &mut [u8], src: &[u8]) -> Result<(), RangeError> {
    crate::libc::_memcpy_impl(dst, src)
}

pub fn memeq_libc(a: &[u8], b: &[u8]) -> bool {
    crate::libc::_memeq_impl(a, b)
}

pub fn memmem_libc(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    crate::libc::_memmem_impl(haystack, needle)
}

pub fn memset_libc(buf: &mut [u8], c: u8, n: usize) -> Result<(), RangeError> {
    crate::libc::_memset_impl(buf, c, n)
}
