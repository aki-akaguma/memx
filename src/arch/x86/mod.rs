cpufeatures::new!(cpuid_avx2, "avx2");
cpufeatures::new!(cpuid_sse2, "sse2");

mod x86_chr;

#[cfg(all(
    any(target_arch = "x86_64", target_arch = "x86"),
    any(target_feature = "sse2", target_feature = "avx2")
))]
pub(crate) use x86_chr::_memchr_impl;

mod x86_chr_double;

#[cfg(all(
    any(target_arch = "x86_64", target_arch = "x86"),
    any(target_feature = "sse2", target_feature = "avx2")
))]
pub(crate) use x86_chr_double::_memchr_double_impl;

mod x86_cmp;

#[cfg(all(
    any(target_arch = "x86_64", target_arch = "x86"),
    any(target_feature = "sse2", target_feature = "avx2")
))]
#[allow(unused_imports)]
pub(crate) use x86_cmp::_memcmp_impl;

mod x86_cpy;

#[cfg(all(
    any(target_arch = "x86_64", target_arch = "x86"),
    any(target_feature = "sse2", target_feature = "avx2")
))]
pub(crate) use x86_cpy::_memcpy_impl;

mod x86_eq;
//pub(crate) use x86_eq::_memeq_impl;

mod x86_mem;

#[cfg(all(
    any(target_arch = "x86_64", target_arch = "x86"),
    any(target_feature = "sse2", target_feature = "avx2")
))]
pub(crate) use x86_mem::_memmem_impl;

#[cfg(all(
    any(target_arch = "x86_64", target_arch = "x86"),
    any(target_feature = "sse2", target_feature = "avx2")
))]
pub use x86_mem::_memmem_sse2_avx2;

mod x86_nechr;

#[cfg(all(
    any(target_arch = "x86_64", target_arch = "x86"),
    any(target_feature = "sse2", target_feature = "avx2")
))]
pub(crate) use x86_nechr::_memnechr_impl;

mod x86_rchr;

#[cfg(all(
    any(target_arch = "x86_64", target_arch = "x86"),
    any(target_feature = "sse2", target_feature = "avx2")
))]
pub(crate) use x86_rchr::_memrchr_impl;

mod x86_rchr_double;

#[cfg(all(
    any(target_arch = "x86_64", target_arch = "x86"),
    any(target_feature = "sse2", target_feature = "avx2")
))]
pub(crate) use x86_rchr_double::_memrchr_double_impl;

mod x86_rnechr;

#[cfg(all(
    any(target_arch = "x86_64", target_arch = "x86"),
    any(target_feature = "sse2", target_feature = "avx2")
))]
pub(crate) use x86_rnechr::_memrnechr_impl;

mod x86_rmem;

#[cfg(all(
    any(target_arch = "x86_64", target_arch = "x86"),
    any(target_feature = "sse2", target_feature = "avx2")
))]
pub(crate) use x86_rmem::_memrmem_impl;

#[cfg(all(
    any(target_arch = "x86_64", target_arch = "x86"),
    any(target_feature = "sse2", target_feature = "avx2")
))]
pub use x86_rmem::_memrmem_sse2_avx2;

mod x86_set;

#[cfg(all(
    any(target_arch = "x86_64", target_arch = "x86"),
    any(target_feature = "sse2", target_feature = "avx2")
))]
pub(crate) use x86_set::_memset_impl;

pub use x86_chr::_memchr_avx2;
pub use x86_chr::_memchr_sse2;

pub use x86_cmp::_memcmp_avx2;
pub use x86_cmp::_memcmp_sse2;

pub use x86_cpy::_memcpy_avx2;
pub use x86_cpy::_memcpy_sse2;

//pub use x86_eq::_memeq_avx;
//pub use x86_eq::_memeq_sse2;

//pub use x86_mem::_memmem_avx;
//pub use x86_mem::_memmem_sse2;

pub use x86_nechr::_memnechr_avx2;
pub use x86_nechr::_memnechr_sse2;

pub use x86_rchr::_memrchr_avx2;
pub use x86_rchr::_memrchr_sse2;

pub use x86_rnechr::_memrnechr_avx2;
pub use x86_rnechr::_memrnechr_sse2;

//pub use x86_rmem::_memrmem_avx;
//pub use x86_rmem::_memrmem_sse2;

pub use x86_set::_memset_avx2;
pub use x86_set::_memset_sse2;
