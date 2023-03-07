mod test_std_memrchr {
    fn test_memrchr(buf: &[u8], byte: u8) -> Option<usize> {
        buf.iter().rposition(|&x| x == byte)
    }
    //
    include!("./src/test_src_memrchr.rs");
}
mod test_memx_memrchr {
    fn test_memrchr(buf: &[u8], byte: u8) -> Option<usize> {
        memx::memrchr(buf, byte)
    }
    //
    include!("./src/test_src_memrchr.rs");
}
mod test_memx_memrchr_basic {
    fn test_memrchr(buf: &[u8], byte: u8) -> Option<usize> {
        memx::mem::memrchr_basic(buf, byte)
    }
    //
    include!("./src/test_src_memrchr.rs");
}
