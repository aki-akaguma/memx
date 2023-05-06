mod impl_std;
mod test_std_memrchr_tpl {
    fn test_memrchr_tpl(buf: &[u8], by1: u8, by2: u8, by3: u8) -> Option<usize> {
        super::impl_std::_std_memrchr_tpl(buf, by1, by2, by3)
    }
    //
    include!("./src/test_src_memrchr_tpl.rs");
}
mod test_memx_memrchr_tpl {
    fn test_memrchr_tpl(buf: &[u8], by1: u8, by2: u8, by3: u8) -> Option<usize> {
        memx::memrchr_tpl(buf, by1, by2, by3)
    }
    //
    include!("./src/test_src_memrchr_tpl.rs");
}
mod test_memx_memrchr_tpl_basic {
    fn test_memrchr_tpl(buf: &[u8], by1: u8, by2: u8, by3: u8) -> Option<usize> {
        memx::mem::memrchr_tpl_basic(buf, by1, by2, by3)
    }
    //
    include!("./src/test_src_memrchr_tpl.rs");
}
