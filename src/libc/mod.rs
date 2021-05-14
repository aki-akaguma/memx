mod libc_chr;
pub(crate) use libc_chr::_memchr_impl;

mod libc_cmp;
pub(crate) use libc_cmp::_memcmp_impl;

//mod mem_cpy;
//pub(crate) use mem_cpy::mem_cpy_impl;

mod libc_eq;
pub(crate) use libc_eq::_memeq_impl;

//mod mem_mem;
//pub(crate) use mem_mem::mem_mem_impl;

mod libc_set;
pub(crate) use libc_set::_memset_impl;
