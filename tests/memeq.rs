mod impl_std;
mod test_std_memeq {
    fn test_memeq(buf: &[u8], pat_bytes: &[u8]) -> bool {
        super::impl_std::_std_memeq(buf, pat_bytes)
    }
    //
    const _RT_AC: bool = false;
    include!("./src/test_src_memeq.rs");
}
mod test_memx_memeq {
    fn test_memeq(buf: &[u8], pat_bytes: &[u8]) -> bool {
        memx::memeq(buf, pat_bytes)
    }
    //
    const _RT_AC: bool = true;
    include!("./src/test_src_memeq.rs");
}
mod test_memx_memeq_basic {
    fn test_memeq(buf: &[u8], pat_bytes: &[u8]) -> bool {
        memx::mem::memeq_basic(buf, pat_bytes)
    }
    //
    const _RT_AC: bool = true;
    include!("./src/test_src_memeq.rs");
}
