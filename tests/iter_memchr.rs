mod impl_std;
mod test_std_iter_memchr {
    fn test_memchr_iter(buf: &[u8], byte: u8) -> super::impl_std::StdMemchrSglIter {
        super::impl_std::_std_memchr_iter(buf, byte)
    }
    //
    include!("./src/test_src_iter_memchr.rs");
}
mod test_memx_iter_memchr {
    fn test_memchr_iter(buf: &[u8], byte: u8) -> memx::iter::MemchrIter {
        memx::iter::memchr_iter(buf, byte)
    }
    //
    include!("./src/test_src_iter_memchr.rs");
}
