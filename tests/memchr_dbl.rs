mod impl_std;
mod test_std_memchr_dbl {
    fn test_memchr_dbl(buf: &[u8], by1: u8, by2: u8) -> Option<usize> {
        super::impl_std::_std_memchr_dbl(buf, by1, by2)
    }
    //
    include!("./src/test_src_memchr_dbl.rs");
}
mod test_memx_memchr_dbl {
    fn test_memchr_dbl(buf: &[u8], by1: u8, by2: u8) -> Option<usize> {
        memx::memchr_dbl(buf, by1, by2)
    }
    //
    include!("./src/test_src_memchr_dbl.rs");
}
mod test_memx_memchr_dbl_basic {
    fn test_memchr_dbl(buf: &[u8], by1: u8, by2: u8) -> Option<usize> {
        memx::mem::memchr_dbl_basic(buf, by1, by2)
    }
    //
    include!("./src/test_src_memchr_dbl.rs");
}
