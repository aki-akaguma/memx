mod impl_std;
mod test_std_memchr_tpl {
    fn test_memchr_tpl(buf: &[u8], by1: u8, by2: u8, by3: u8) -> Option<usize> {
        super::impl_std::_std_memchr_tpl(buf, by1, by2, by3)
    }
    //
    include!("./src/test_src_memchr_tpl.rs");
}
mod test_memx_memchr_tpl {
    fn test_memchr_tpl(buf: &[u8], by1: u8, by2: u8, by3: u8) -> Option<usize> {
        memx::memchr_tpl(buf, by1, by2, by3)
    }
    //
    include!("./src/test_src_memchr_tpl.rs");
}
mod test_memx_memchr_tpl_basic {
    fn test_memchr_tpl(buf: &[u8], by1: u8, by2: u8, by3: u8) -> Option<usize> {
        memx::mem::memchr_tpl_basic(buf, by1, by2, by3)
    }
    //
    include!("./src/test_src_memchr_tpl.rs");
}
