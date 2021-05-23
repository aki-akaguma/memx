mod mem_chr;
pub(crate) use mem_chr::_memchr_impl;
#[allow(unused_imports)]
pub(crate) use mem_chr::_memchr_remaining_15_bytes_impl;

mod mem_cmp;
pub(crate) use mem_cmp::_memcmp_impl;

mod mem_cpy;
pub(crate) use mem_cpy::_memcpy_impl;

mod mem_eq;
pub(crate) use mem_eq::_memeq_impl;

mod mem_mem;
pub(crate) use mem_mem::_memmem_impl;

mod mem_set;
pub(crate) use mem_set::_memset_impl;

use super::RangeError;
use std::cmp::Ordering;

pub fn memchr_basic(buf: &[u8], c: u8) -> Option<usize> {
    crate::mem::_memchr_impl(buf, c)
}

pub fn memcmp_basic(a: &[u8], b: &[u8]) -> Ordering {
    crate::mem::_memcmp_impl(a, b)
}

pub fn memcpy_basic(dst: &mut [u8], src: &[u8]) -> Result<(), RangeError> {
    crate::mem::_memcpy_impl(dst, src)
}

pub fn memeq_basic(a: &[u8], b: &[u8]) -> bool {
    crate::mem::_memeq_impl(a, b)
}

pub fn memmem_basic(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    crate::mem::_memmem_impl(haystack, needle)
}

pub fn memset_basic(buf: &mut [u8], c: u8, n: usize) -> Result<(), RangeError> {
    crate::mem::_memset_impl(buf, c, n)
}
