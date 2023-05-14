mod impl_std;
mod test_std_memnechr_dbl {
    fn test_memnechr_dbl(buf: &[u8], by1: u8, by2: u8) -> Option<usize> {
        super::impl_std::_std_memnechr_dbl(buf, by1, by2)
    }
    //
    include!("./src/test_src_memnechr_dbl.rs");
}
mod test_memx_memnechr_dbl {
    fn test_memnechr_dbl(buf: &[u8], by1: u8, by2: u8) -> Option<usize> {
        memx::memnechr_dbl(buf, by1, by2)
    }
    //
    include!("./src/test_src_memnechr_dbl.rs");
}
mod test_memx_memnechr_dbl_basic {
    fn test_memnechr_dbl(buf: &[u8], by1: u8, by2: u8) -> Option<usize> {
        memx::mem::memnechr_dbl_basic(buf, by1, by2)
    }
    //
    include!("./src/test_src_memnechr_dbl.rs");
}
