mod impl_std;
mod test_std_memchr_qpl {
    fn test_memchr_qpl(buf: &[u8], by1: u8, by2: u8, by3: u8, by4: u8) -> Option<usize> {
        super::impl_std::_std_memchr_qpl(buf, by1, by2, by3, by4)
    }
    //
    include!("./src/test_src_memchr_qpl.rs");
}
mod test_memx_memchr_qpl {
    fn test_memchr_qpl(buf: &[u8], by1: u8, by2: u8, by3: u8, by4: u8) -> Option<usize> {
        memx::memchr_qpl(buf, by1, by2, by3, by4)
    }
    //
    include!("./src/test_src_memchr_qpl.rs");
}
mod test_memx_memchr_qpl_basic {
    fn test_memchr_qpl(buf: &[u8], by1: u8, by2: u8, by3: u8, by4: u8) -> Option<usize> {
        memx::mem::memchr_qpl_basic(buf, by1, by2, by3, by4)
    }
    //
    include!("./src/test_src_memchr_qpl.rs");
}
