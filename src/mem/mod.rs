mod mem_chr;
pub(crate) use mem_chr::_memchr_impl;
#[allow(unused_imports)]
pub(crate) use mem_chr::_memchr_remaining_15_bytes_impl;

mod mem_chr_double;
pub(crate) use mem_chr_double::_memchr_double_impl;
#[allow(unused_imports)]
pub(crate) use mem_chr_double::_memchr_double_remaining_15_bytes_impl;

mod mem_cmp;
pub(crate) use mem_cmp::_memcmp_impl;

mod mem_cpy;
pub(crate) use mem_cpy::_memcpy_impl;

mod mem_eq;
pub(crate) use mem_eq::_memeq_impl;

mod mem_mem;
pub(crate) use mem_mem::_memmem_impl;

mod mem_nechr;
pub(crate) use mem_nechr::_memnechr_impl;
#[allow(unused_imports)]
pub(crate) use mem_nechr::_memnechr_remaining_15_bytes_impl;

mod mem_rchr;
pub(crate) use mem_rchr::_memrchr_impl;
#[allow(unused_imports)]
pub(crate) use mem_rchr::_memrchr_remaining_15_bytes_impl;

mod mem_rchr_double;
pub(crate) use mem_rchr_double::_memrchr_double_impl;
#[allow(unused_imports)]
pub(crate) use mem_rchr_double::_memrchr_double_remaining_15_bytes_impl;

mod mem_rnechr;
pub(crate) use mem_rnechr::_memrnechr_impl;
#[allow(unused_imports)]
pub(crate) use mem_rnechr::_memrnechr_remaining_15_bytes_impl;

mod mem_rmem;
pub(crate) use mem_rmem::_memrmem_impl;

mod mem_set;
pub(crate) use mem_set::_memset_impl;
#[allow(unused_imports)]
pub(crate) use mem_set::_memset_remaining_15_bytes_impl;

use super::RangeError;
use core::cmp::Ordering;

pub fn memchr_basic(buf: &[u8], c: u8) -> Option<usize> {
    crate::mem::_memchr_impl(buf, c)
}

pub fn memchr_double_basic(buf: &[u8], c1: u8, c2: u8) -> Option<usize> {
    crate::mem::_memchr_double_impl(buf, c1, c2)
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

pub fn memnechr_basic(buf: &[u8], c: u8) -> Option<usize> {
    crate::mem::_memnechr_impl(buf, c)
}

pub fn memrchr_basic(buf: &[u8], c: u8) -> Option<usize> {
    crate::mem::_memrchr_impl(buf, c)
}

pub fn memrchr_double_basic(buf: &[u8], c1: u8, c2: u8) -> Option<usize> {
    crate::mem::_memrchr_double_impl(buf, c1, c2)
}

pub fn memrnechr_basic(buf: &[u8], c: u8) -> Option<usize> {
    crate::mem::_memrnechr_impl(buf, c)
}

pub fn memrmem_basic(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    crate::mem::_memrmem_impl(haystack, needle)
}

pub fn memset_basic(buf: &mut [u8], c: u8) {
    crate::mem::_memset_impl(buf, c)
}
