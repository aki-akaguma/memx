mod impl_std;
mod test_std_memrnechr_qpl {
    fn test_memrnechr_qpl(buf: &[u8], by1: u8, by2: u8, by3: u8, by4: u8) -> Option<usize> {
        super::impl_std::_std_memrnechr_qpl(buf, by1, by2, by3, by4)
    }
    //
    include!("./src/test_src_memrnechr_qpl.rs");
}
mod test_memx_memrnechr_qpl {
    fn test_memrnechr_qpl(buf: &[u8], by1: u8, by2: u8, by3: u8, by4: u8) -> Option<usize> {
        memx::memrnechr_qpl(buf, by1, by2, by3, by4)
    }
    //
    include!("./src/test_src_memrnechr_qpl.rs");
}
mod test_memx_memrnechr_qpl_basic {
    fn test_memrnechr_qpl(buf: &[u8], by1: u8, by2: u8, by3: u8, by4: u8) -> Option<usize> {
        memx::mem::memrnechr_qpl_basic(buf, by1, by2, by3, by4)
    }
    //
    include!("./src/test_src_memrnechr_qpl.rs");
}
