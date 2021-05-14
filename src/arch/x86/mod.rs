mod x86_chr;
pub(crate) use x86_chr::_memchr_impl;

mod x86_cmp;
pub(crate) use x86_cmp::_memcmp_impl;

//mod mem_cpy;
//pub(crate) use mem_cpy::memcpy_impl;

mod x86_eq;
pub(crate) use x86_eq::_memeq_impl;

//mod mem_mem;
//pub(crate) use mem_mem::memmem_impl;

mod x86_set;
pub(crate) use x86_set::_memset_impl;
