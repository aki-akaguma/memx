mod impl_std;
mod test_std_iter_memrchr_dbl {
    fn test_memrchr_dbl_iter(buf: &[u8], by1: u8, by2: u8) -> super::impl_std::StdMemrchrDblIter {
        super::impl_std::_std_memrchr_dbl_iter(buf, by1, by2)
    }
    //
    include!("./src/test_src_iter_memrchr_dbl.rs");
}
mod test_memx_iter_memrchr_dbl {
    fn test_memrchr_dbl_iter(buf: &[u8], by1: u8, by2: u8) -> memx::iter::MemrchrDblIter {
        memx::iter::memrchr_dbl_iter(buf, by1, by2)
    }
    //
    include!("./src/test_src_iter_memrchr_dbl.rs");
}
