mod impl_std;
mod test_std_iter_memchr_dbl {
    fn test_memchr_dbl_iter(buf: &[u8], by1: u8, by2: u8) -> super::impl_std::StdMemchrDblIter<'_> {
        super::impl_std::_std_memchr_dbl_iter(buf, by1, by2)
    }
    //
    include!("./src/test_src_iter_memchr_dbl.rs");
}
mod test_memx_iter_memchr_dbl {
    fn test_memchr_dbl_iter(buf: &[u8], by1: u8, by2: u8) -> memx::iter::MemchrDblIter<'_> {
        memx::iter::memchr_dbl_iter(buf, by1, by2)
    }
    //
    include!("./src/test_src_iter_memchr_dbl.rs");
}
