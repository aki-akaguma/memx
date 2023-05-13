#![doc(hidden)]

use crate::utils::{B1Dbl, B1Qpl, B1Sgl, B1Tpl};

mod mem_chr;
pub(crate) use mem_chr::_memchr_sgl_impl;

#[allow(unused_imports)]
pub(crate) use mem_chr::_chr_sgl_to_aligned_u128;
#[allow(unused_imports)]
pub(crate) use mem_chr::_chr_sgl_to_aligned_u256;
#[allow(unused_imports)]
pub(crate) use mem_chr::_memchr_sgl_remaining_15_bytes_impl;

mod mem_rchr;
pub(crate) use mem_rchr::_memrchr_sgl_impl;

#[allow(unused_imports)]
pub(crate) use mem_rchr::_memrchr_sgl_remaining_15_bytes_impl;
#[allow(unused_imports)]
pub(crate) use mem_rchr::_rchr_sgl_to_aligned_u128;
#[allow(unused_imports)]
pub(crate) use mem_rchr::_rchr_sgl_to_aligned_u256;

mod mem_nechr;
pub(crate) use mem_nechr::_memnechr_sgl_impl;

#[allow(unused_imports)]
pub(crate) use mem_nechr::_memnechr_sgl_remaining_15_bytes_impl;
#[allow(unused_imports)]
pub(crate) use mem_nechr::_nechr_sgl_to_aligned_u128;
#[allow(unused_imports)]
pub(crate) use mem_nechr::_nechr_sgl_to_aligned_u256;

mod mem_rnechr;
pub(crate) use mem_rnechr::_memrnechr_sgl_impl;

#[allow(unused_imports)]
pub(crate) use mem_rnechr::_memrnechr_sgl_remaining_15_bytes_impl;
#[allow(unused_imports)]
pub(crate) use mem_rnechr::_rnechr_sgl_to_aligned_u128;
#[allow(unused_imports)]
pub(crate) use mem_rnechr::_rnechr_sgl_to_aligned_u256;

mod mem_chr_dbl;
pub(crate) use mem_chr_dbl::_memchr_dbl_impl;

#[allow(unused_imports)]
pub(crate) use mem_chr_dbl::_chr_dbl_to_aligned_u128;
#[allow(unused_imports)]
pub(crate) use mem_chr_dbl::_chr_dbl_to_aligned_u256;
#[allow(unused_imports)]
pub(crate) use mem_chr_dbl::_memchr_dbl_remaining_15_bytes_impl;

mod mem_rchr_dbl;
pub(crate) use mem_rchr_dbl::_memrchr_dbl_impl;

#[allow(unused_imports)]
pub(crate) use mem_rchr_dbl::_memrchr_dbl_remaining_15_bytes_impl;
#[allow(unused_imports)]
pub(crate) use mem_rchr_dbl::_rchr_dbl_to_aligned_u128;
#[allow(unused_imports)]
pub(crate) use mem_rchr_dbl::_rchr_dbl_to_aligned_u256;

mod mem_chr_tpl;
pub(crate) use mem_chr_tpl::_memchr_tpl_impl;

#[allow(unused_imports)]
pub(crate) use mem_chr_tpl::_chr_tpl_to_aligned_u128;
#[allow(unused_imports)]
pub(crate) use mem_chr_tpl::_chr_tpl_to_aligned_u256;
#[allow(unused_imports)]
pub(crate) use mem_chr_tpl::_memchr_tpl_remaining_15_bytes_impl;

mod mem_rchr_tpl;
pub(crate) use mem_rchr_tpl::_memrchr_tpl_impl;

#[allow(unused_imports)]
pub(crate) use mem_rchr_tpl::_memrchr_tpl_remaining_15_bytes_impl;
#[allow(unused_imports)]
pub(crate) use mem_rchr_tpl::_rchr_tpl_to_aligned_u128;
#[allow(unused_imports)]
pub(crate) use mem_rchr_tpl::_rchr_tpl_to_aligned_u256;

mod mem_chr_qpl;
pub(crate) use mem_chr_qpl::_memchr_qpl_impl;

#[allow(unused_imports)]
pub(crate) use mem_chr_qpl::_chr_qpl_to_aligned_u128;
#[allow(unused_imports)]
pub(crate) use mem_chr_qpl::_chr_qpl_to_aligned_u256;
#[allow(unused_imports)]
pub(crate) use mem_chr_qpl::_memchr_qpl_remaining_15_bytes_impl;

mod mem_rchr_qpl;
pub(crate) use mem_rchr_qpl::_memrchr_qpl_impl;

#[allow(unused_imports)]
pub(crate) use mem_rchr_qpl::_memrchr_qpl_remaining_15_bytes_impl;
#[allow(unused_imports)]
pub(crate) use mem_rchr_qpl::_rchr_qpl_to_aligned_u128;
#[allow(unused_imports)]
pub(crate) use mem_rchr_qpl::_rchr_qpl_to_aligned_u256;

mod mem_cmp;
pub(crate) use mem_cmp::_memcmp_impl;

#[allow(unused_imports)]
pub(crate) use mem_cmp::_cmp_to_aligned_u128;
#[allow(unused_imports)]
pub(crate) use mem_cmp::_cmp_to_aligned_u256;
#[allow(unused_imports)]
pub(crate) use mem_cmp::_cmp_to_aligned_u64;
#[allow(unused_imports)]
pub(crate) use mem_cmp::_memcmp_remaining_15_bytes_impl;
#[allow(unused_imports)]
pub(crate) use mem_cmp::_start_cmp_128;
#[allow(unused_imports)]
pub(crate) use mem_cmp::_start_cmp_64;

mod mem_eq;
pub(crate) use mem_eq::_memeq_impl;

#[allow(unused_imports)]
pub(crate) use mem_eq::_eq_to_aligned_u128;
#[allow(unused_imports)]
pub(crate) use mem_eq::_eq_to_aligned_u256;
#[allow(unused_imports)]
pub(crate) use mem_eq::_eq_to_aligned_u64;
#[allow(unused_imports)]
pub(crate) use mem_eq::_memeq_remaining_15_bytes_impl;
#[allow(unused_imports)]
pub(crate) use mem_eq::_start_eq_128;
#[allow(unused_imports)]
pub(crate) use mem_eq::_start_eq_64;

mod mem_mem;
pub(crate) use mem_mem::_memmem_impl;

mod mem_rmem;
pub(crate) use mem_rmem::_memrmem_impl;

mod mem_cpy;
pub(crate) use mem_cpy::_memcpy_impl;

#[allow(unused_imports)]
pub(crate) use mem_cpy::_cpy_to_aligned_u128;
#[allow(unused_imports)]
pub(crate) use mem_cpy::_cpy_to_aligned_u256;
#[allow(unused_imports)]
pub(crate) use mem_cpy::_memcpy_remaining_15_bytes_impl;

mod mem_set;
pub(crate) use mem_set::_memset_impl;

#[allow(unused_imports)]
pub(crate) use mem_set::_memset_remaining_15_bytes_impl;
#[allow(unused_imports)]
pub(crate) use mem_set::_set_to_aligned_u128;
#[allow(unused_imports)]
pub(crate) use mem_set::_set_to_aligned_u256;

use super::RangeError;
use core::cmp::Ordering;

pub fn memchr_basic(buf: &[u8], c1: u8) -> Option<usize> {
    let needle = B1Sgl::new(c1);
    crate::mem::_memchr_sgl_impl(buf, needle)
}

pub fn memrchr_basic(buf: &[u8], c1: u8) -> Option<usize> {
    let needle = B1Sgl::new(c1);
    crate::mem::_memrchr_sgl_impl(buf, needle)
}

pub fn memchr_dbl_basic(buf: &[u8], c1: u8, c2: u8) -> Option<usize> {
    let needle = B1Dbl::new(c1, c2);
    crate::mem::_memchr_dbl_impl(buf, needle)
}

pub fn memrchr_dbl_basic(buf: &[u8], c1: u8, c2: u8) -> Option<usize> {
    let needle = B1Dbl::new(c1, c2);
    crate::mem::_memrchr_dbl_impl(buf, needle)
}

pub fn memchr_tpl_basic(buf: &[u8], c1: u8, c2: u8, c3: u8) -> Option<usize> {
    let needle = B1Tpl::new(c1, c2, c3);
    crate::mem::_memchr_tpl_impl(buf, needle)
}

pub fn memrchr_tpl_basic(buf: &[u8], c1: u8, c2: u8, c3: u8) -> Option<usize> {
    let needle = B1Tpl::new(c1, c2, c3);
    crate::mem::_memrchr_tpl_impl(buf, needle)
}

pub fn memchr_qpl_basic(buf: &[u8], c1: u8, c2: u8, c3: u8, c4: u8) -> Option<usize> {
    let needle = B1Qpl::new(c1, c2, c3, c4);
    crate::mem::_memchr_qpl_impl(buf, needle)
}

pub fn memrchr_qpl_basic(buf: &[u8], c1: u8, c2: u8, c3: u8, c4: u8) -> Option<usize> {
    let needle = B1Qpl::new(c1, c2, c3, c4);
    crate::mem::_memrchr_qpl_impl(buf, needle)
}

pub fn memnechr_basic(buf: &[u8], c1: u8) -> Option<usize> {
    let needle = B1Sgl::new(c1);
    crate::mem::_memnechr_sgl_impl(buf, needle)
}

pub fn memrnechr_basic(buf: &[u8], c1: u8) -> Option<usize> {
    let needle = B1Sgl::new(c1);
    crate::mem::_memrnechr_sgl_impl(buf, needle)
}

pub fn memmem_basic(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    crate::mem::_memmem_impl(haystack, needle)
}

pub fn memrmem_basic(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    crate::mem::_memrmem_impl(haystack, needle)
}

pub fn memcmp_basic(a: &[u8], b: &[u8]) -> Ordering {
    crate::mem::_memcmp_impl(a, b)
}

pub fn memeq_basic(a: &[u8], b: &[u8]) -> bool {
    crate::mem::_memeq_impl(a, b)
}

pub fn memcpy_basic(dst: &mut [u8], src: &[u8]) -> Result<(), RangeError> {
    crate::mem::_memcpy_impl(dst, src)
}

pub fn memset_basic(buf: &mut [u8], c: u8) {
    crate::mem::_memset_impl(buf, c)
}
