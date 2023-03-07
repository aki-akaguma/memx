mod test_std_memeq {
    fn test_memeq(buf: &[u8], pat_bytes: &[u8]) -> bool {
        buf == pat_bytes
    }
    //
    include!("./src/test_src_memeq.rs");
}
mod test_memx_memeq {
    fn test_memeq(buf: &[u8], pat_bytes: &[u8]) -> bool {
        memx::memeq(buf, pat_bytes)
    }
    //
    include!("./src/test_src_memeq.rs");
}
mod test_memx_memeq_basic {
    fn test_memeq(buf: &[u8], pat_bytes: &[u8]) -> bool {
        memx::mem::memeq_basic(buf, pat_bytes)
    }
    //
    include!("./src/test_src_memeq.rs");
}
