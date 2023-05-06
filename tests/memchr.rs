mod impl_std;
mod test_std_memchr {
    fn test_memchr(buf: &[u8], by1: u8) -> Option<usize> {
        super::impl_std::_std_memchr(buf, by1)
    }
    //
    include!("./src/test_src_memchr.rs");
}
mod test_memx_memchr {
    fn test_memchr(buf: &[u8], by1: u8) -> Option<usize> {
        memx::memchr(buf, by1)
    }
    //
    include!("./src/test_src_memchr.rs");
}
mod test_memx_memchr_basic {
    fn test_memchr(buf: &[u8], by1: u8) -> Option<usize> {
        memx::mem::memchr_basic(buf, by1)
    }
    //
    include!("./src/test_src_memchr.rs");
}
