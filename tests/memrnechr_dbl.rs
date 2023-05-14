mod impl_std;
mod test_std_memrnechr_dbl {
    fn test_memrnechr_dbl(buf: &[u8], by1: u8, by2: u8) -> Option<usize> {
        super::impl_std::_std_memrnechr_dbl(buf, by1, by2)
    }
    //
    include!("./src/test_src_memrnechr_dbl.rs");
}
mod test_memx_memrnechr_dbl {
    fn test_memrnechr_dbl(buf: &[u8], by1: u8, by2: u8) -> Option<usize> {
        memx::memrnechr_dbl(buf, by1, by2)
    }
    //
    include!("./src/test_src_memrnechr_dbl.rs");
}
mod test_memx_memrnechr_dbl_basic {
    fn test_memrnechr_dbl(buf: &[u8], by1: u8, by2: u8) -> Option<usize> {
        memx::mem::memrnechr_dbl_basic(buf, by1, by2)
    }
    //
    include!("./src/test_src_memrnechr_dbl.rs");
}
