mod impl_std;
mod test_std_memmem {
    fn test_memmem(buf: &[u8], pat_bytes: &[u8]) -> Option<usize> {
        super::impl_std::_std_memmem(buf, pat_bytes)
    }
    //
    const _RT_AC: bool = false;
    include!("./src/test_src_memmem.rs");
}
mod test_memx_memmem {
    fn test_memmem(buf: &[u8], pat_bytes: &[u8]) -> Option<usize> {
        memx::memmem(buf, pat_bytes)
    }
    //
    const _RT_AC: bool = true;
    include!("./src/test_src_memmem.rs");
}
mod test_memx_memmem_basic {
    fn test_memmem(buf: &[u8], pat_bytes: &[u8]) -> Option<usize> {
        memx::mem::memmem_basic(buf, pat_bytes)
    }
    //
    const _RT_AC: bool = true;
    include!("./src/test_src_memmem.rs");
}
