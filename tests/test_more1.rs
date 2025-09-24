mod impl_std;

// Test cases for memchr
mod test_memx_memchr_more {
    fn test_memchr(buf: &[u8], by1: u8) -> Option<usize> {
        memx::memchr(buf, by1)
    }
    include!("./src/test_src_memchr_more.rs");
}

mod test_std_memchr_more {
    fn test_memchr(buf: &[u8], by1: u8) -> Option<usize> {
        super::impl_std::_std_memchr(buf, by1)
    }
    include!("./src/test_src_memchr_more.rs");
}

// Test cases for memrchr
mod test_memx_memrchr_more {
    fn test_memrchr(buf: &[u8], by1: u8) -> Option<usize> {
        memx::memrchr(buf, by1)
    }
    include!("./src/test_src_memrchr_more.rs");
}

mod test_std_memrchr_more {
    fn test_memrchr(buf: &[u8], by1: u8) -> Option<usize> {
        super::impl_std::_std_memrchr(buf, by1)
    }
    include!("./src/test_src_memrchr_more.rs");
}

// Test cases for memchr_dbl
mod test_memx_memchr_dbl_more {
    fn test_memchr_dbl(buf: &[u8], by1: u8, by2: u8) -> Option<usize> {
        memx::memchr_dbl(buf, by1, by2)
    }
    include!("./src/test_src_memchr_dbl_more.rs");
}

mod test_std_memchr_dbl_more {
    fn test_memchr_dbl(buf: &[u8], by1: u8, by2: u8) -> Option<usize> {
        super::impl_std::_std_memchr_dbl(buf, by1, by2)
    }
    include!("./src/test_src_memchr_dbl_more.rs");
}

// Test cases for memrchr_dbl
mod test_memx_memrchr_dbl_more {
    fn test_memrchr_dbl(buf: &[u8], by1: u8, by2: u8) -> Option<usize> {
        memx::memrchr_dbl(buf, by1, by2)
    }
    include!("./src/test_src_memrchr_dbl_more.rs");
}

mod test_std_memrchr_dbl_more {
    fn test_memrchr_dbl(buf: &[u8], by1: u8, by2: u8) -> Option<usize> {
        super::impl_std::_std_memrchr_dbl(buf, by1, by2)
    }
    include!("./src/test_src_memrchr_dbl_more.rs");
}

// Test cases for memchr_tpl
mod test_memx_memchr_tpl_more {
    fn test_memchr_tpl(buf: &[u8], by1: u8, by2: u8, by3: u8) -> Option<usize> {
        memx::memchr_tpl(buf, by1, by2, by3)
    }
    include!("./src/test_src_memchr_tpl_more.rs");
}

mod test_std_memchr_tpl_more {
    fn test_memchr_tpl(buf: &[u8], by1: u8, by2: u8, by3: u8) -> Option<usize> {
        super::impl_std::_std_memchr_tpl(buf, by1, by2, by3)
    }
    include!("./src/test_src_memchr_tpl_more.rs");
}

// Test cases for memrchr_tpl
mod test_memx_memrchr_tpl_more {
    fn test_memrchr_tpl(buf: &[u8], by1: u8, by2: u8, by3: u8) -> Option<usize> {
        memx::memrchr_tpl(buf, by1, by2, by3)
    }
    include!("./src/test_src_memrchr_tpl_more.rs");
}

mod test_std_memrchr_tpl_more {
    fn test_memrchr_tpl(buf: &[u8], by1: u8, by2: u8, by3: u8) -> Option<usize> {
        super::impl_std::_std_memrchr_tpl(buf, by1, by2, by3)
    }
    include!("./src/test_src_memrchr_tpl_more.rs");
}

// Test cases for memchr_qpl
mod test_memx_memchr_qpl_more {
    fn test_memchr_qpl(buf: &[u8], by1: u8, by2: u8, by3: u8, by4: u8) -> Option<usize> {
        memx::memchr_qpl(buf, by1, by2, by3, by4)
    }
    include!("./src/test_src_memchr_qpl_more.rs");
}

mod test_std_memchr_qpl_more {
    fn test_memchr_qpl(buf: &[u8], by1: u8, by2: u8, by3: u8, by4: u8) -> Option<usize> {
        super::impl_std::_std_memchr_qpl(buf, by1, by2, by3, by4)
    }
    include!("./src/test_src_memchr_qpl_more.rs");
}

// Test cases for memrchr_qpl
mod test_memx_memrchr_qpl_more {
    fn test_memrchr_qpl(buf: &[u8], by1: u8, by2: u8, by3: u8, by4: u8) -> Option<usize> {
        memx::memrchr_qpl(buf, by1, by2, by3, by4)
    }
    include!("./src/test_src_memrchr_qpl_more.rs");
}

mod test_std_memrchr_qpl_more {
    fn test_memrchr_qpl(buf: &[u8], by1: u8, by2: u8, by3: u8, by4: u8) -> Option<usize> {
        super::impl_std::_std_memrchr_qpl(buf, by1, by2, by3, by4)
    }
    include!("./src/test_src_memrchr_qpl_more.rs");
}

// Test cases for memnechr
mod test_memx_memnechr_more {
    fn test_memnechr(buf: &[u8], by1: u8) -> Option<usize> {
        memx::memnechr(buf, by1)
    }
    include!("./src/test_src_memnechr_more.rs");
}

mod test_std_memnechr_more {
    fn test_memnechr(buf: &[u8], by1: u8) -> Option<usize> {
        super::impl_std::_std_memnechr(buf, by1)
    }
    include!("./src/test_src_memnechr_more.rs");
}

// Test cases for memrnechr
mod test_memx_memrnechr_more {
    fn test_memrnechr(buf: &[u8], by1: u8) -> Option<usize> {
        memx::memrnechr(buf, by1)
    }
    include!("./src/test_src_memrnechr_more.rs");
}

mod test_std_memrnechr_more {
    fn test_memrnechr(buf: &[u8], by1: u8) -> Option<usize> {
        super::impl_std::_std_memrnechr(buf, by1)
    }
    include!("./src/test_src_memrnechr_more.rs");
}

// Test cases for memnechr_dbl
mod test_memx_memnechr_dbl_more {
    fn test_memnechr_dbl(buf: &[u8], by1: u8, by2: u8) -> Option<usize> {
        memx::memnechr_dbl(buf, by1, by2)
    }
    include!("./src/test_src_memnechr_dbl_more.rs");
}

mod test_std_memnechr_dbl_more {
    fn test_memnechr_dbl(buf: &[u8], by1: u8, by2: u8) -> Option<usize> {
        super::impl_std::_std_memnechr_dbl(buf, by1, by2)
    }
    include!("./src/test_src_memnechr_dbl_more.rs");
}

// Test cases for memrnechr_dbl
mod test_memx_memrnechr_dbl_more {
    fn test_memrnechr_dbl(buf: &[u8], by1: u8, by2: u8) -> Option<usize> {
        memx::memrnechr_dbl(buf, by1, by2)
    }
    include!("./src/test_src_memrnechr_dbl_more.rs");
}

mod test_std_memrnechr_dbl_more {
    fn test_memrnechr_dbl(buf: &[u8], by1: u8, by2: u8) -> Option<usize> {
        super::impl_std::_std_memrnechr_dbl(buf, by1, by2)
    }
    include!("./src/test_src_memrnechr_dbl_more.rs");
}

// Test cases for memnechr_tpl
mod test_memx_memnechr_tpl_more {
    fn test_memnechr_tpl(buf: &[u8], by1: u8, by2: u8, by3: u8) -> Option<usize> {
        memx::memnechr_tpl(buf, by1, by2, by3)
    }
    include!("./src/test_src_memnechr_tpl_more.rs");
}

mod test_std_memnechr_tpl_more {
    fn test_memnechr_tpl(buf: &[u8], by1: u8, by2: u8, by3: u8) -> Option<usize> {
        super::impl_std::_std_memnechr_tpl(buf, by1, by2, by3)
    }
    include!("./src/test_src_memnechr_tpl_more.rs");
}

// Test cases for memrnechr_tpl
mod test_memx_memrnechr_tpl_more {
    fn test_memrnechr_tpl(buf: &[u8], by1: u8, by2: u8, by3: u8) -> Option<usize> {
        memx::memrnechr_tpl(buf, by1, by2, by3)
    }
    include!("./src/test_src_memrnechr_tpl_more.rs");
}

mod test_std_memrnechr_tpl_more {
    fn test_memrnechr_tpl(buf: &[u8], by1: u8, by2: u8, by3: u8) -> Option<usize> {
        super::impl_std::_std_memrnechr_tpl(buf, by1, by2, by3)
    }
    include!("./src/test_src_memrnechr_tpl_more.rs");
}

// Test cases for memnechr_qpl
mod test_memx_memnechr_qpl_more {
    fn test_memnechr_qpl(buf: &[u8], by1: u8, by2: u8, by3: u8, by4: u8) -> Option<usize> {
        memx::memnechr_qpl(buf, by1, by2, by3, by4)
    }
    include!("./src/test_src_memnechr_qpl_more.rs");
}

mod test_std_memnechr_qpl_more {
    fn test_memnechr_qpl(buf: &[u8], by1: u8, by2: u8, by3: u8, by4: u8) -> Option<usize> {
        super::impl_std::_std_memnechr_qpl(buf, by1, by2, by3, by4)
    }
    include!("./src/test_src_memnechr_qpl_more.rs");
}

// Test cases for memrnechr_qpl
mod test_memx_memrnechr_qpl_more {
    fn test_memrnechr_qpl(buf: &[u8], by1: u8, by2: u8, by3: u8, by4: u8) -> Option<usize> {
        memx::memrnechr_qpl(buf, by1, by2, by3, by4)
    }
    include!("./src/test_src_memrnechr_qpl_more.rs");
}

mod test_std_memrnechr_qpl_more {
    fn test_memrnechr_qpl(buf: &[u8], by1: u8, by2: u8, by3: u8, by4: u8) -> Option<usize> {
        super::impl_std::_std_memrnechr_qpl(buf, by1, by2, by3, by4)
    }
    include!("./src/test_src_memrnechr_qpl_more.rs");
}

// Test cases for memmem
mod test_memx_memmem_more {
    fn test_memmem(buf: &[u8], pat_bytes: &[u8]) -> Option<usize> {
        memx::memmem(buf, pat_bytes)
    }
    const _RT_AC: bool = true;
    include!("./src/test_src_memmem_more.rs");
}

mod test_std_memmem_more {
    fn test_memmem(buf: &[u8], pat_bytes: &[u8]) -> Option<usize> {
        super::impl_std::_std_memmem(buf, pat_bytes)
    }
    const _RT_AC: bool = false;
    include!("./src/test_src_memmem_more.rs");
}

// Test cases for memrmem
mod test_memx_memrmem_more {
    fn test_memrmem(buf: &[u8], pat_bytes: &[u8]) -> Option<usize> {
        memx::memrmem(buf, pat_bytes)
    }
    const _RT_AC: bool = true;
    include!("./src/test_src_memrmem_more.rs");
}

mod test_std_memrmem_more {
    fn test_memrmem(buf: &[u8], pat_bytes: &[u8]) -> Option<usize> {
        super::impl_std::_std_memrmem(buf, pat_bytes)
    }
    const _RT_AC: bool = false;
    include!("./src/test_src_memrmem_more.rs");
}

// Test cases for memcmp
mod test_memx_memcmp_more {
    use std::cmp::Ordering;
    fn test_memcmp(buf: &[u8], pat_bytes: &[u8]) -> Ordering {
        memx::memcmp(buf, pat_bytes)
    }
    const _RT_AC: bool = true;
    include!("./src/test_src_memcmp_more.rs");
}

mod test_std_memcmp_more {
    use std::cmp::Ordering;
    fn test_memcmp(buf: &[u8], pat_bytes: &[u8]) -> Ordering {
        super::impl_std::_std_memcmp(buf, pat_bytes)
    }
    const _RT_AC: bool = false;
    include!("./src/test_src_memcmp_more.rs");
}

// Test cases for memeq
mod test_memx_memeq_more {
    fn test_memeq(buf: &[u8], pat_bytes: &[u8]) -> bool {
        memx::memeq(buf, pat_bytes)
    }
    const _RT_AC: bool = true;
    include!("./src/test_src_memeq_more.rs");
}

mod test_std_memeq_more {
    fn test_memeq(buf: &[u8], pat_bytes: &[u8]) -> bool {
        super::impl_std::_std_memeq(buf, pat_bytes)
    }
    const _RT_AC: bool = false;
    include!("./src/test_src_memeq_more.rs");
}

// Test cases for memcpy
mod test_memx_memcpy_more {
    use memx::RangeError;
    fn test_memcpy(dst: &mut [u8], src: &[u8]) -> Result<(), RangeError> {
        memx::memcpy(dst, src)
    }
    const _RT_AC: bool = true;
    include!("./src/test_src_memcpy_more.rs");
}

mod test_std_memcpy_more {
    use memx::RangeError;
    fn test_memcpy(dst: &mut [u8], src: &[u8]) -> Result<(), RangeError> {
        super::impl_std::_std_memcpy(dst, src)
    }
    const _RT_AC: bool = false;
    include!("./src/test_src_memcpy_more.rs");
}

// Test cases for memset
mod test_memx_memset_more {
    fn test_memset(dst: &mut [u8], byte: u8) {
        memx::memset(dst, byte);
    }
    const _RT_AC: bool = true;
    include!("./src/test_src_memset_more.rs");
}

mod test_std_memset_more {
    fn test_memset(dst: &mut [u8], byte: u8) {
        super::impl_std::_std_memset(dst, byte)
    }
    const _RT_AC: bool = false;
    include!("./src/test_src_memset_more.rs");
}
