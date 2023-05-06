mod impl_std;
mod test_std_memrchr_tpl {
    fn test_memrchr_tpl(buf: &[u8], byte1: u8, byte2: u8, byte3: u8) -> Option<usize> {
        super::impl_std::_std_memrchr_tpl(buf, byte1, byte2, byte3)
    }
    //
    include!("./src/test_src_memrchr_tpl.rs");
}
mod test_memx_memrchr_tpl {
    fn test_memrchr_tpl(buf: &[u8], byte1: u8, byte2: u8, byte3: u8) -> Option<usize> {
        memx::memrchr_tpl(buf, byte1, byte2, byte3)
    }
    //
    include!("./src/test_src_memrchr_tpl.rs");
}
mod test_memx_memrchr_tpl_basic {
    fn test_memrchr_tpl(buf: &[u8], byte1: u8, byte2: u8, byte3: u8) -> Option<usize> {
        memx::mem::memrchr_tpl_basic(buf, byte1, byte2, byte3)
    }
    //
    include!("./src/test_src_memrchr_tpl.rs");
}
