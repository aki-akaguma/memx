#![allow(dead_code)]
//#![tarpaulin::skip]
#![cfg(not(tarpaulin_include))]

// ref.) https://en.wikipedia.org/wiki/Memory_ordering
pub fn memory_barrier<T>(_arg: T) {
    #[cfg(target_feature = "sse2")]
    {
        #[cfg(target_arch = "x86")]
        use std::arch::x86 as mmx;
        #[cfg(target_arch = "x86_64")]
        use std::arch::x86_64 as mmx;

        unsafe { mmx::_mm_mfence() };
    }
    /*
    #[cfg(target_arch = "arm")]
    {
        unsafe { core::arch::arm::__dmb(_arg) };
    }
    #[cfg(target_arch = "aarch64")]
    {
        unsafe { core::arch::aarch64::__dmb(_arg) };
    }
    */
}

pub fn cache_line_flush(_arg: &[u8]) {
    clf::cache_line_flush_with_slice(_arg);
    /*
    #[cfg(target_feature = "sse2")]
    {
        #[cfg(target_arch = "x86")]
        use std::arch::x86 as mmx;
        #[cfg(target_arch = "x86_64")]
        use std::arch::x86_64 as mmx;
        //
        #[cfg(target_pointer_width = "128")]
        let pw = 128;
        #[cfg(target_pointer_width = "64")]
        let pw = 64;
        #[cfg(target_pointer_width = "32")]
        let pw = 32;
        #[cfg(target_pointer_width = "16")]
        let pw = 16;
        {
            let mut i = 0;
            while i < _arg.len() {
                unsafe { mmx::_mm_clflush(&_arg[i]) };
                i += pw;
            }
        }
        /*
        {
            let mut i = 0;
            while i < _arg.len() {
                unsafe { mmx::_mm_clflushopt(&_arg[i]) };
                i += pw;
            }
            mmx::_mm_sfence();
        }
        */
    }
    */
}
