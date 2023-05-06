mod impl_std;
mod test_std_iter_memchr_tpl {
    fn test_memchr_tpl_iter(buf: &[u8], byte1: u8, byte2: u8, byte3: u8) -> super::impl_std::StdMemchrTplIter {
        super::impl_std::_std_memchr_tpl_iter(buf, byte1, byte2, byte3)
    }
    //
    include!("./src/test_src_iter_memchr_tpl.rs");
}
mod test_memx_iter_memchr_tpl {
    fn test_memchr_tpl_iter(buf: &[u8], byte1: u8, byte2: u8, byte3: u8) -> memx::iter::MemchrTplIter {
        memx::iter::memchr_tpl_iter(buf, byte1, byte2, byte3)
    }
    //
    include!("./src/test_src_iter_memchr_tpl.rs");
}
