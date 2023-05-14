mod impl_std;
mod test_std_memrnechr_tpl {
    fn test_memrnechr_tpl(buf: &[u8], by1: u8, by2: u8, by3: u8) -> Option<usize> {
        super::impl_std::_std_memrnechr_tpl(buf, by1, by2, by3)
    }
    //
    include!("./src/test_src_memrnechr_tpl.rs");
}
mod test_memx_memrnechr_tpl {
    fn test_memrnechr_tpl(buf: &[u8], by1: u8, by2: u8, by3: u8) -> Option<usize> {
        memx::memrnechr_tpl(buf, by1, by2, by3)
    }
    //
    include!("./src/test_src_memrnechr_tpl.rs");
}
mod test_memx_memrnechr_tpl_basic {
    fn test_memrnechr_tpl(buf: &[u8], by1: u8, by2: u8, by3: u8) -> Option<usize> {
        memx::mem::memrnechr_tpl_basic(buf, by1, by2, by3)
    }
    //
    include!("./src/test_src_memrnechr_tpl.rs");
}
