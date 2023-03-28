use crate::mem as basic;
use crate::RangeError;

#[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
use super::cpuid;

use core::sync::atomic::AtomicPtr;
use core::sync::atomic::Ordering;
type _FuncType = fn(&mut [u8], &[u8]) -> Result<(), RangeError>;

const _FUNC: _FuncType = _fnptr_setup_func;
static _FUNC_PTR_ATOM: AtomicPtr<_FuncType> = AtomicPtr::new(_FUNC as *mut _FuncType);

#[inline(never)]
fn _fnptr_setup_func(dst: &mut [u8], src: &[u8]) -> Result<(), RangeError> {
    #[cfg(target_arch = "x86_64")]
    let func = if cpuid::has_avx2() {
        _memcpy_avx2
    } else {
        _memcpy_sse2
    };
    #[cfg(target_arch = "x86")]
    let func = if cpuid::has_avx2() {
        _memcpy_avx2
    } else if cpuid::has_sse2() {
        _memcpy_sse2
    } else {
        _memcpy_basic
    };
    //
    _FUNC_PTR_ATOM.store(func as *mut _FuncType, Ordering::Relaxed);
    unsafe { func(dst, src) }
}

#[inline(always)]
pub(crate) fn _memcpy_impl(dst: &mut [u8], src: &[u8]) -> Result<(), RangeError> {
    let func_u = _FUNC_PTR_ATOM.load(Ordering::Relaxed);
    #[allow(clippy::crosspointer_transmute)]
    let func: _FuncType = unsafe { core::mem::transmute(func_u) };
    func(dst, src)
}

unsafe fn _memcpy_basic(dst: &mut [u8], src: &[u8]) -> Result<(), RangeError> {
    basic::_memcpy_impl(dst, src)
}

#[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
#[target_feature(enable = "sse2")]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn _memcpy_sse2(dst: &mut [u8], src: &[u8]) -> Result<(), RangeError> {
    basic::_memcpy_impl(dst, src)
}

#[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
#[target_feature(enable = "avx2")]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn _memcpy_avx2(dst: &mut [u8], src: &[u8]) -> Result<(), RangeError> {
    basic::_memcpy_impl(dst, src)
}

#[cfg(test)]
mod disasm {
    use super::*;
    //
    #[test]
    fn do_procs() {
        let mut a = b"       ".to_vec();
        let b = b"abcdefg".to_vec();
        let a = a.as_mut_slice();
        let b = b.as_slice();
        assert_eq!(do_proc_basic(a, b), Ok(()));
        assert_eq!(do_proc_sse2(a, b), Ok(()));
        assert_eq!(do_proc_avx2(a, b), Ok(()));
    }
    #[inline(never)]
    fn do_proc_basic(a: &mut [u8], b: &[u8]) -> Result<(), RangeError> {
        unsafe { _memcpy_basic(a, b) }
    }
    #[inline(never)]
    fn do_proc_sse2(a: &mut [u8], b: &[u8]) -> Result<(), RangeError> {
        unsafe { _memcpy_sse2(a, b) }
    }
    #[inline(never)]
    fn do_proc_avx2(a: &mut [u8], b: &[u8]) -> Result<(), RangeError> {
        unsafe { _memcpy_avx2(a, b) }
    }
}
