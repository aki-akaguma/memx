mod impl_std;
mod test_std_iter_memrchr_tpl {
    fn test_memrchr_tpl_iter(
        buf: &[u8],
        byte1: u8,
        byte2: u8,
        byte3: u8,
    ) -> super::impl_std::StdMemrchrTplIter {
        super::impl_std::_std_memrchr_tpl_iter(buf, byte1, byte2, byte3)
    }
    //
    include!("./src/test_src_iter_memrchr_tpl.rs");
}
mod test_memx_iter_memrchr_tpl {
    fn test_memrchr_tpl_iter(buf: &[u8], byte1: u8, byte2: u8, byte3: u8) -> memx::iter::MemrchrTplIter {
        memx::iter::memrchr_tpl_iter(buf, byte1, byte2, byte3)
    }
    //
    include!("./src/test_src_iter_memrchr_tpl.rs");
}
