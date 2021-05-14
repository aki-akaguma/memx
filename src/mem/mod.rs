mod mem_chr;
pub(crate) use mem_chr::_memchr_impl;

mod mem_cmp;
pub(crate) use mem_cmp::_memcmp_impl;

//mod mem_cpy;
//pub(crate) use mem_cpy::_memcpy_impl;

mod mem_eq;
pub(crate) use mem_eq::_memeq_impl;

//mod mem_mem;
//pub(crate) use mem_mem::_memmem_impl;

mod mem_set;
pub(crate) use mem_set::_memset_impl;
