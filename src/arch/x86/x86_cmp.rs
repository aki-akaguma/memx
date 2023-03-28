use crate::mem as basic;
use core::cmp::Ordering;

#[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
use super::cpuid;

use core::sync::atomic::AtomicPtr;
use core::sync::atomic::Ordering as AtomicOrdering;
type _FuncType = fn(&[u8], &[u8]) -> Ordering;

const _FUNC: _FuncType = _fnptr_setup_func;
static _FUNC_PTR_ATOM: AtomicPtr<_FuncType> = AtomicPtr::new(_FUNC as *mut _FuncType);

#[inline(never)]
fn _fnptr_setup_func(a: &[u8], b: &[u8]) -> Ordering {
    #[cfg(target_arch = "x86_64")]
    let func = if cpuid::has_avx2() {
        _memcmp_avx2
    } else {
        _memcmp_sse2
    };
    #[cfg(target_arch = "x86")]
    let func = if cpuid::has_avx2() {
        _memcmp_avx2
    } else if cpuid::has_sse2() {
        _memcmp_sse2
    } else {
        _memcmp_basic
    };
    //
    _FUNC_PTR_ATOM.store(func as *mut _FuncType, AtomicOrdering::Relaxed);
    unsafe { func(a, b) }
}

#[inline(always)]
pub(crate) fn _memcmp_impl(a: &[u8], b: &[u8]) -> Ordering {
    let func_u = _FUNC_PTR_ATOM.load(AtomicOrdering::Relaxed);
    #[allow(clippy::crosspointer_transmute)]
    let func: _FuncType = unsafe { core::mem::transmute(func_u) };
    func(a, b)
}

unsafe fn _memcmp_basic(a: &[u8], b: &[u8]) -> Ordering {
    basic::_memcmp_impl(a, b)
}

#[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
#[target_feature(enable = "sse2")]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn _memcmp_sse2(a: &[u8], b: &[u8]) -> Ordering {
    basic::_memcmp_impl(a, b)
}

#[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
#[target_feature(enable = "avx2")]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn _memcmp_avx2(a: &[u8], b: &[u8]) -> Ordering {
    basic::_memcmp_impl(a, b)
}

#[cfg(test)]
mod disasm {
    use super::*;
    use core::cmp::Ordering;
    //
    #[test]
    fn do_procs() {
        let a = b"abcdefg".to_vec();
        let b = b"abcdefg".to_vec();
        let a = a.as_slice();
        let b = b.as_slice();
        assert_eq!(do_proc_basic(a, b), Ordering::Equal);
        assert_eq!(do_proc_sse2(a, b), Ordering::Equal);
        assert_eq!(do_proc_avx2(a, b), Ordering::Equal);
    }
    fn do_proc_basic(a: &[u8], b: &[u8]) -> Ordering {
        unsafe { _memcmp_basic(a, b) }
    }
    fn do_proc_sse2(a: &[u8], b: &[u8]) -> Ordering {
        unsafe { _memcmp_sse2(a, b) }
    }
    fn do_proc_avx2(a: &[u8], b: &[u8]) -> Ordering {
        unsafe { _memcmp_avx2(a, b) }
    }
}
