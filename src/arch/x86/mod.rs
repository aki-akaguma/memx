#![doc(hidden)]

mod x86_chr;
pub(crate) use x86_chr::_memchr_sgl_impl;

mod x86_rchr;
pub(crate) use x86_rchr::_memrchr_sgl_impl;

mod x86_chr_dbl;
pub(crate) use x86_chr_dbl::_memchr_dbl_impl;

mod x86_rchr_dbl;
pub(crate) use x86_rchr_dbl::_memrchr_dbl_impl;

mod x86_chr_tpl;
pub(crate) use x86_chr_tpl::_memchr_tpl_impl;

mod x86_rchr_tpl;
pub(crate) use x86_rchr_tpl::_memrchr_tpl_impl;

mod x86_chr_qpl;
pub(crate) use x86_chr_qpl::_memchr_qpl_impl;

mod x86_rchr_qpl;
pub(crate) use x86_rchr_qpl::_memrchr_qpl_impl;

mod x86_nechr;
pub(crate) use x86_nechr::_memnechr_sgl_impl;

mod x86_rnechr;
pub(crate) use x86_rnechr::_memrnechr_sgl_impl;

mod x86_nechr_dbl;
pub(crate) use x86_nechr_dbl::_memnechr_dbl_impl;

mod x86_rnechr_dbl;
pub(crate) use x86_rnechr_dbl::_memrnechr_dbl_impl;

mod x86_nechr_tpl;
pub(crate) use x86_nechr_tpl::_memnechr_tpl_impl;

mod x86_rnechr_tpl;
pub(crate) use x86_rnechr_tpl::_memrnechr_tpl_impl;

mod x86_nechr_qpl;
pub(crate) use x86_nechr_qpl::_memnechr_qpl_impl;

mod x86_rnechr_qpl;
pub(crate) use x86_rnechr_qpl::_memrnechr_qpl_impl;

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

pub(crate) use x86_chr::_memchr_sgl_avx2;
pub(crate) use x86_chr::_memchr_sgl_sse2;

pub(crate) use x86_rchr::_memrchr_sgl_avx2;
pub(crate) use x86_rchr::_memrchr_sgl_sse2;

pub(crate) use x86_eq::_memeq_avx2;
pub(crate) use x86_eq::_memeq_sse2;

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
            #[allow(static_mut_refs)]
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
                #[allow(static_mut_refs)]
                unsafe {
                    HAS_AVX2_ATOM.store(1, Ordering::Relaxed)
                };
                true
            } else {
                #[allow(static_mut_refs)]
                unsafe {
                    HAS_AVX2_ATOM.store(-1, Ordering::Relaxed)
                };
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

mod needles;
pub(crate) use needles::dbl::{MMB16Dbl, MMB32Dbl};
pub(crate) use needles::qpl::{MMB16Qpl, MMB32Qpl};
pub(crate) use needles::sgl::{MMB16Sgl, MMB32Sgl};
pub(crate) use needles::tpl::{MMB16Tpl, MMB32Tpl};

use crate::utils::{B1Dbl, B1Qpl, B1Sgl, B1Tpl};
use crate::RangeError;
use core::cmp::Ordering;

pub fn memchr_sse2(buf: &[u8], c1: u8) -> Option<usize> {
    let needle = B1Sgl::new(c1);
    unsafe { x86_chr::_memchr_sgl_sse2(buf, needle) }
}
pub fn memchr_avx2(buf: &[u8], c1: u8) -> Option<usize> {
    let needle = B1Sgl::new(c1);
    unsafe { x86_chr::_memchr_sgl_avx2(buf, needle) }
}

pub fn memrchr_sse2(buf: &[u8], c1: u8) -> Option<usize> {
    let needle = B1Sgl::new(c1);
    unsafe { x86_rchr::_memrchr_sgl_sse2(buf, needle) }
}
pub fn memrchr_avx2(buf: &[u8], c1: u8) -> Option<usize> {
    let needle = B1Sgl::new(c1);
    unsafe { x86_rchr::_memrchr_sgl_avx2(buf, needle) }
}

pub fn memchr_dbl_sse2(buf: &[u8], c1: u8, c2: u8) -> Option<usize> {
    let needle = B1Dbl::new(c1, c2);
    unsafe { x86_chr_dbl::_memchr_dbl_sse2(buf, needle) }
}
pub fn memchr_dbl_avx2(buf: &[u8], c1: u8, c2: u8) -> Option<usize> {
    let needle = B1Dbl::new(c1, c2);
    unsafe { x86_chr_dbl::_memchr_dbl_avx2(buf, needle) }
}

pub fn memrchr_dbl_sse2(buf: &[u8], c1: u8, c2: u8) -> Option<usize> {
    let needle = B1Dbl::new(c1, c2);
    unsafe { x86_rchr_dbl::_memrchr_dbl_sse2(buf, needle) }
}
pub fn memrchr_dbl_avx2(buf: &[u8], c1: u8, c2: u8) -> Option<usize> {
    let needle = B1Dbl::new(c1, c2);
    unsafe { x86_rchr_dbl::_memrchr_dbl_avx2(buf, needle) }
}

pub fn memchr_tpl_sse2(buf: &[u8], c1: u8, c2: u8, c3: u8) -> Option<usize> {
    let needle = B1Tpl::new(c1, c2, c3);
    unsafe { x86_chr_tpl::_memchr_tpl_sse2(buf, needle) }
}
pub fn memchr_tpl_avx2(buf: &[u8], c1: u8, c2: u8, c3: u8) -> Option<usize> {
    let needle = B1Tpl::new(c1, c2, c3);
    unsafe { x86_chr_tpl::_memchr_tpl_avx2(buf, needle) }
}

pub fn memrchr_tpl_sse2(buf: &[u8], c1: u8, c2: u8, c3: u8) -> Option<usize> {
    let needle = B1Tpl::new(c1, c2, c3);
    unsafe { x86_rchr_tpl::_memrchr_tpl_sse2(buf, needle) }
}
pub fn memrchr_tpl_avx2(buf: &[u8], c1: u8, c2: u8, c3: u8) -> Option<usize> {
    let needle = B1Tpl::new(c1, c2, c3);
    unsafe { x86_rchr_tpl::_memrchr_tpl_avx2(buf, needle) }
}

pub fn memchr_qpl_sse2(buf: &[u8], c1: u8, c2: u8, c3: u8, c4: u8) -> Option<usize> {
    let needle = B1Qpl::new(c1, c2, c3, c4);
    unsafe { x86_chr_qpl::_memchr_qpl_sse2(buf, needle) }
}
pub fn memchr_qpl_avx2(buf: &[u8], c1: u8, c2: u8, c3: u8, c4: u8) -> Option<usize> {
    let needle = B1Qpl::new(c1, c2, c3, c4);
    unsafe { x86_chr_qpl::_memchr_qpl_avx2(buf, needle) }
}

pub fn memrchr_qpl_sse2(buf: &[u8], c1: u8, c2: u8, c3: u8, c4: u8) -> Option<usize> {
    let needle = B1Qpl::new(c1, c2, c3, c4);
    unsafe { x86_rchr_qpl::_memrchr_qpl_sse2(buf, needle) }
}
pub fn memrchr_qpl_avx2(buf: &[u8], c1: u8, c2: u8, c3: u8, c4: u8) -> Option<usize> {
    let needle = B1Qpl::new(c1, c2, c3, c4);
    unsafe { x86_rchr_qpl::_memrchr_qpl_avx2(buf, needle) }
}

pub fn memnechr_sse2(buf: &[u8], c1: u8) -> Option<usize> {
    let needle = B1Sgl::new(c1);
    unsafe { x86_nechr::_memnechr_sgl_sse2(buf, needle) }
}
pub fn memnechr_avx2(buf: &[u8], c1: u8) -> Option<usize> {
    let needle = B1Sgl::new(c1);
    unsafe { x86_nechr::_memnechr_sgl_avx2(buf, needle) }
}

pub fn memrnechr_sse2(buf: &[u8], c1: u8) -> Option<usize> {
    let needle = B1Sgl::new(c1);
    unsafe { x86_rnechr::_memrnechr_sgl_sse2(buf, needle) }
}
pub fn memrnechr_avx2(buf: &[u8], c1: u8) -> Option<usize> {
    let needle = B1Sgl::new(c1);
    unsafe { x86_rnechr::_memrnechr_sgl_avx2(buf, needle) }
}

pub fn memnechr_dbl_sse2(buf: &[u8], c1: u8, c2: u8) -> Option<usize> {
    let needle = B1Dbl::new(c1, c2);
    unsafe { x86_nechr_dbl::_memnechr_dbl_sse2(buf, needle) }
}
pub fn memnechr_dbl_avx2(buf: &[u8], c1: u8, c2: u8) -> Option<usize> {
    let needle = B1Dbl::new(c1, c2);
    unsafe { x86_nechr_dbl::_memnechr_dbl_avx2(buf, needle) }
}

pub fn memrnechr_dbl_sse2(buf: &[u8], c1: u8, c2: u8) -> Option<usize> {
    let needle = B1Dbl::new(c1, c2);
    unsafe { x86_rnechr_dbl::_memrnechr_dbl_sse2(buf, needle) }
}
pub fn memrnechr_dbl_avx2(buf: &[u8], c1: u8, c2: u8) -> Option<usize> {
    let needle = B1Dbl::new(c1, c2);
    unsafe { x86_rnechr_dbl::_memrnechr_dbl_avx2(buf, needle) }
}

pub fn memnechr_tpl_sse2(buf: &[u8], c1: u8, c2: u8, c3: u8) -> Option<usize> {
    let needle = B1Tpl::new(c1, c2, c3);
    unsafe { x86_nechr_tpl::_memnechr_tpl_sse2(buf, needle) }
}
pub fn memnechr_tpl_avx2(buf: &[u8], c1: u8, c2: u8, c3: u8) -> Option<usize> {
    let needle = B1Tpl::new(c1, c2, c3);
    unsafe { x86_nechr_tpl::_memnechr_tpl_avx2(buf, needle) }
}

pub fn memrnechr_tpl_sse2(buf: &[u8], c1: u8, c2: u8, c3: u8) -> Option<usize> {
    let needle = B1Tpl::new(c1, c2, c3);
    unsafe { x86_rnechr_tpl::_memrnechr_tpl_sse2(buf, needle) }
}
pub fn memrnechr_tpl_avx2(buf: &[u8], c1: u8, c2: u8, c3: u8) -> Option<usize> {
    let needle = B1Tpl::new(c1, c2, c3);
    unsafe { x86_rnechr_tpl::_memrnechr_tpl_avx2(buf, needle) }
}

pub fn memnechr_qpl_sse2(buf: &[u8], c1: u8, c2: u8, c3: u8, c4: u8) -> Option<usize> {
    let needle = B1Qpl::new(c1, c2, c3, c4);
    unsafe { x86_nechr_qpl::_memnechr_qpl_sse2(buf, needle) }
}
pub fn memnechr_qpl_avx2(buf: &[u8], c1: u8, c2: u8, c3: u8, c4: u8) -> Option<usize> {
    let needle = B1Qpl::new(c1, c2, c3, c4);
    unsafe { x86_nechr_qpl::_memnechr_qpl_avx2(buf, needle) }
}

pub fn memrnechr_qpl_sse2(buf: &[u8], c1: u8, c2: u8, c3: u8, c4: u8) -> Option<usize> {
    let needle = B1Qpl::new(c1, c2, c3, c4);
    unsafe { x86_rnechr_qpl::_memrnechr_qpl_sse2(buf, needle) }
}
pub fn memrnechr_qpl_avx2(buf: &[u8], c1: u8, c2: u8, c3: u8, c4: u8) -> Option<usize> {
    let needle = B1Qpl::new(c1, c2, c3, c4);
    unsafe { x86_rnechr_qpl::_memrnechr_qpl_avx2(buf, needle) }
}

pub fn memmem_sse2(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    unsafe { x86_mem::_memmem_sse2(haystack, needle) }
}
pub fn memmem_avx2(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    unsafe { x86_mem::_memmem_avx2(haystack, needle) }
}

pub fn memrmem_sse2(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    unsafe { x86_rmem::_memrmem_sse2(haystack, needle) }
}
pub fn memrmem_avx2(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    unsafe { x86_rmem::_memrmem_avx2(haystack, needle) }
}

pub fn memcmp_sse2(a: &[u8], b: &[u8]) -> Ordering {
    unsafe { x86_cmp::_memcmp_sse2(a, b) }
}
pub fn memcmp_avx2(a: &[u8], b: &[u8]) -> Ordering {
    unsafe { x86_cmp::_memcmp_avx2(a, b) }
}

pub fn memeq_sse2(a: &[u8], b: &[u8]) -> bool {
    unsafe { x86_eq::_memeq_sse2(a, b) }
}
pub fn memeq_avx2(a: &[u8], b: &[u8]) -> bool {
    unsafe { x86_eq::_memeq_avx2(a, b) }
}

pub fn memcpy_sse2(dst: &mut [u8], src: &[u8]) -> Result<(), RangeError> {
    unsafe { x86_cpy::_memcpy_sse2(dst, src) }
}
pub fn memcpy_avx2(dst: &mut [u8], src: &[u8]) -> Result<(), RangeError> {
    unsafe { x86_cpy::_memcpy_avx2(dst, src) }
}

pub fn memset_sse2(buf: &mut [u8], c: u8) {
    unsafe { x86_set::_memset_sse2(buf, c) }
}
pub fn memset_avx2(buf: &mut [u8], c: u8) {
    unsafe { x86_set::_memset_avx2(buf, c) }
}
