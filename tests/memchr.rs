mod test_std_memchr {
    fn test_memchr(buf: &[u8], byte: u8) -> Option<usize> {
        buf.iter().position(|&x| x == byte)
    }
    //
    include!("./src/test_src_memchr.rs");
}
mod test_memx_memchr {
    fn test_memchr(buf: &[u8], byte: u8) -> Option<usize> {
        memx::memchr(buf, byte)
    }
    //
    include!("./src/test_src_memchr.rs");
}
mod test_memx_memchr_basic {
    fn test_memchr(buf: &[u8], byte: u8) -> Option<usize> {
        memx::mem::memchr_basic(buf, byte)
    }
    //
    include!("./src/test_src_memchr.rs");
}
