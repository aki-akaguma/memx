mod impl_std;
mod test_std_memchr {
    fn test_memchr(buf: &[u8], byte: u8) -> Option<usize> {
        super::impl_std::_std_memchr(buf, byte)
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
