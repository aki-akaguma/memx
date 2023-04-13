mod impl_std;
mod test_std_memcmp {
    use std::cmp::Ordering;
    //
    fn test_memcmp(buf: &[u8], pat_bytes: &[u8]) -> Ordering {
        super::impl_std::_std_memcmp(buf, pat_bytes)
    }
    //
    const _RT_AC: bool = false;
    include!("./src/test_src_memcmp.rs");
}
mod test_memx_memcmp {
    use std::cmp::Ordering;
    //
    fn test_memcmp(buf: &[u8], pat_bytes: &[u8]) -> Ordering {
        memx::memcmp(buf, pat_bytes)
    }
    //
    const _RT_AC: bool = true;
    include!("./src/test_src_memcmp.rs");
}
mod test_memx_memcmp_basic {
    use std::cmp::Ordering;
    //
    fn test_memcmp(buf: &[u8], pat_bytes: &[u8]) -> Ordering {
        memx::mem::memcmp_basic(buf, pat_bytes)
    }
    //
    const _RT_AC: bool = true;
    include!("./src/test_src_memcmp.rs");
}
