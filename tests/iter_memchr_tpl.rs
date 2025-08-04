mod impl_std;
mod test_std_iter_memchr_tpl {
    fn test_memchr_tpl_iter(
        buf: &[u8],
        by1: u8,
        by2: u8,
        by3: u8,
    ) -> super::impl_std::StdMemchrTplIter<'_> {
        super::impl_std::_std_memchr_tpl_iter(buf, by1, by2, by3)
    }
    //
    include!("./src/test_src_iter_memchr_tpl.rs");
}
mod test_memx_iter_memchr_tpl {
    fn test_memchr_tpl_iter(
        buf: &[u8],
        by1: u8,
        by2: u8,
        by3: u8,
    ) -> memx::iter::MemchrTplIter<'_> {
        memx::iter::memchr_tpl_iter(buf, by1, by2, by3)
    }
    //
    include!("./src/test_src_iter_memchr_tpl.rs");
}
