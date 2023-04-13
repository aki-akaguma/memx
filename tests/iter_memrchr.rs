mod impl_std;
mod test_std_iter_memrchr {
    fn test_memrchr_iter(buf: &[u8], byte: u8) -> super::impl_std::StdMemrchrSglIter {
        super::impl_std::_std_memrchr_iter(buf, byte)
    }
    //
    include!("./src/test_src_iter_memrchr.rs");
}
mod test_memx_iter_memrchr {
    fn test_memrchr_iter(buf: &[u8], byte: u8) -> memx::iter::MemrchrIter {
        memx::iter::memrchr_iter(buf, byte)
    }
    //
    include!("./src/test_src_iter_memrchr.rs");
}
