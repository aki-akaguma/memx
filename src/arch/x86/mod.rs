mod x86_chr;
pub(crate) use x86_chr::_memchr_impl;

mod x86_rchr;
pub(crate) use x86_rchr::_memrchr_impl;

mod x86_chr_dbl;
pub(crate) use x86_chr_dbl::_memchr_dbl_impl;

mod x86_rchr_dbl;
pub(crate) use x86_rchr_dbl::_memrchr_dbl_impl;

mod x86_chr_tpl;
pub(crate) use x86_chr_tpl::_memchr_tpl_impl;

mod x86_rchr_tpl;
pub(crate) use x86_rchr_tpl::_memrchr_tpl_impl;

mod x86_nechr;
pub(crate) use x86_nechr::_memnechr_impl;

mod x86_rnechr;
pub(crate) use x86_rnechr::_memrnechr_impl;

mod x86_mem;
pub(crate) use x86_mem::_memmem_impl;

mod x86_rmem;
pub(crate) use x86_rmem::_memrmem_impl;

mod x86_cmp;
#[allow(unused_imports)]
pub(crate) use x86_cmp::_memcmp_impl;

mod x86_eq;
#[allow(unused_imports)]
pub(crate) use x86_eq::_memeq_impl;

mod x86_cpy;
#[allow(unused_imports)]
pub(crate) use x86_cpy::_memcpy_impl;

mod x86_set;
pub(crate) use x86_set::_memset_impl;

#[allow(unused_imports)]
pub use x86_chr::_memchr_avx2;
#[allow(unused_imports)]
pub use x86_chr::_memchr_sse2;

pub use x86_rchr::_memrchr_avx2;
pub use x86_rchr::_memrchr_sse2;

pub use x86_nechr::_memnechr_avx2;
pub use x86_nechr::_memnechr_sse2;

pub use x86_rnechr::_memrnechr_avx2;
pub use x86_rnechr::_memrnechr_sse2;

pub use x86_chr_dbl::_memchr_dbl_avx2;
pub use x86_chr_dbl::_memchr_dbl_sse2;

pub use x86_rchr_dbl::_memrchr_dbl_avx2;
pub use x86_rchr_dbl::_memrchr_dbl_sse2;

pub use x86_chr_tpl::_memchr_tpl_avx2;
pub use x86_chr_tpl::_memchr_tpl_sse2;

pub use x86_rchr_tpl::_memrchr_tpl_avx2;
pub use x86_rchr_tpl::_memrchr_tpl_sse2;

pub use x86_mem::_memmem_avx2;
pub use x86_mem::_memmem_sse2;

pub use x86_rmem::_memrmem_avx2;
pub use x86_rmem::_memrmem_sse2;

pub use x86_cmp::_memcmp_avx2;
pub use x86_cmp::_memcmp_sse2;

pub use x86_eq::_memeq_avx2;
pub use x86_eq::_memeq_sse2;

pub use x86_cpy::_memcpy_avx2;
pub use x86_cpy::_memcpy_sse2;

pub use x86_set::_memset_avx2;
pub use x86_set::_memset_sse2;

mod cpuid {
    #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
    pub use cpuid_avx2::has_avx2;

    #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
    mod cpuid_avx2 {
        use core::sync::atomic::AtomicI8;
        use core::sync::atomic::Ordering;
        cpufeatures::new!(cpuid_avx2, "avx2");
        static mut HAS_AVX2_ATOM: AtomicI8 = AtomicI8::new(0);
        //
        #[inline(always)]
        pub fn has_avx2() -> bool {
            let val = unsafe { HAS_AVX2_ATOM.load(Ordering::Relaxed) };
            if val != 0 {
                val > 0
            } else {
                setup_avx2()
            }
        }
        #[inline(never)]
        fn setup_avx2() -> bool {
            // TODO: Replace with https://github.com/rust-lang/rfcs/pull/2725
            // after stabilization
            let cpuid_avx2_token: cpuid_avx2::InitToken = cpuid_avx2::init();
            if cpuid_avx2_token.get() {
                unsafe { HAS_AVX2_ATOM.store(1, Ordering::Relaxed) };
                true
            } else {
                unsafe { HAS_AVX2_ATOM.store(-1, Ordering::Relaxed) };
                false
            }
        }
    }

    #[cfg(target_arch = "x86")]
    pub use cpuid_sse2::has_sse2;

    #[cfg(target_arch = "x86")]
    mod cpuid_sse2 {
        use core::sync::atomic::AtomicI8;
        use core::sync::atomic::Ordering;
        cpufeatures::new!(cpuid_sse2, "sse2");
        static mut HAS_SSE2_ATOM: AtomicI8 = AtomicI8::new(0);
        //
        #[inline(always)]
        pub fn has_sse2() -> bool {
            let val = unsafe { HAS_SSE2_ATOM.load(Ordering::Relaxed) };
            if val != 0 {
                val > 0
            } else {
                setup_sse2()
            }
        }
        #[inline(never)]
        fn setup_sse2() -> bool {
            // TODO: Replace with https://github.com/rust-lang/rfcs/pull/2725
            // after stabilization
            let cpuid_sse2_token: cpuid_sse2::InitToken = cpuid_sse2::init();
            if cpuid_sse2_token.get() {
                unsafe { HAS_SSE2_ATOM.store(1, Ordering::Relaxed) };
                true
            } else {
                unsafe { HAS_SSE2_ATOM.store(-1, Ordering::Relaxed) };
                false
            }
        }
    }
}

cpufeatures::new!(cpuid_avx2, "avx2");
cpufeatures::new!(cpuid_sse2, "sse2");

mod multi;
pub(crate) use multi::dbl::{MMB16Dbl, MMB32Dbl};
pub(crate) use multi::sgl::{MMB16Sgl, MMB32Sgl};
pub(crate) use multi::tpl::{MMB16Tpl, MMB32Tpl};
//pub(crate) use multi::qpl::{MMB16Qpl, MMB32Qpl};
