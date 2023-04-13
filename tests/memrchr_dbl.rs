mod impl_std;
mod test_std_memrchr_dbl {
    fn test_memrchr_dbl(buf: &[u8], byte1: u8, byte2: u8) -> Option<usize> {
        super::impl_std::_std_memrchr_dbl(buf, byte2, byte1)
    }
    //
    include!("./src/test_src_memrchr_dbl.rs");
}
mod test_memx_memrchr_dbl {
    fn test_memrchr_dbl(buf: &[u8], byte1: u8, byte2: u8) -> Option<usize> {
        memx::memrchr_dbl(buf, byte2, byte1)
    }
    //
    include!("./src/test_src_memrchr_dbl.rs");
}
mod test_memx_memrchr_dbl_basic {
    fn test_memrchr_dbl(buf: &[u8], byte1: u8, byte2: u8) -> Option<usize> {
        memx::mem::memrchr_dbl_basic(buf, byte2, byte1)
    }
    //
    include!("./src/test_src_memrchr_dbl.rs");
}
