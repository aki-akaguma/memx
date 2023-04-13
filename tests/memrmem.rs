mod impl_std;
mod test_std_memrmem {
    fn test_memrmem(buf: &[u8], pat_bytes: &[u8]) -> Option<usize> {
        super::impl_std::_std_memrmem(buf, pat_bytes)
    }
    //
    const _RT_AC: bool = false;
    include!("./src/test_src_memrmem.rs");
}
mod test_memx_memrmem {
    fn test_memrmem(buf: &[u8], pat_bytes: &[u8]) -> Option<usize> {
        memx::memrmem(buf, pat_bytes)
    }
    //
    const _RT_AC: bool = true;
    include!("./src/test_src_memrmem.rs");
}
mod test_memx_memrmem_basic {
    fn test_memrmem(buf: &[u8], pat_bytes: &[u8]) -> Option<usize> {
        memx::mem::memrmem_basic(buf, pat_bytes)
    }
    //
    const _RT_AC: bool = true;
    include!("./src/test_src_memrmem.rs");
}
