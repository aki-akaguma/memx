mod impl_std;
mod test_std_memnechr_tpl {
    fn test_memnechr_tpl(buf: &[u8], by1: u8, by2: u8, by3: u8) -> Option<usize> {
        super::impl_std::_std_memnechr_tpl(buf, by1, by2, by3)
    }
    //
    include!("./src/test_src_memnechr_tpl.rs");
}
mod test_memx_memnechr_tpl {
    fn test_memnechr_tpl(buf: &[u8], by1: u8, by2: u8, by3: u8) -> Option<usize> {
        memx::memnechr_tpl(buf, by1, by2, by3)
    }
    //
    include!("./src/test_src_memnechr_tpl.rs");
}
mod test_memx_memnechr_tpl_basic {
    fn test_memnechr_tpl(buf: &[u8], by1: u8, by2: u8, by3: u8) -> Option<usize> {
        memx::mem::memnechr_tpl_basic(buf, by1, by2, by3)
    }
    //
    include!("./src/test_src_memnechr_tpl.rs");
}
