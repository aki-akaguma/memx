mod impl_std;
mod test_std_iter_memchr_qpl {
    fn test_memchr_qpl_iter(
        buf: &[u8],
        by1: u8,
        by2: u8,
        by3: u8,
        by4: u8,
    ) -> super::impl_std::StdMemchrQplIter<'_> {
        super::impl_std::_std_memchr_qpl_iter(buf, by1, by2, by3, by4)
    }
    //
    include!("./src/test_src_iter_memchr_qpl.rs");
}
mod test_memx_iter_memchr_qpl {
    fn test_memchr_qpl_iter(
        buf: &[u8],
        by1: u8,
        by2: u8,
        by3: u8,
        by4: u8,
    ) -> memx::iter::MemchrQplIter<'_> {
        memx::iter::memchr_qpl_iter(buf, by1, by2, by3, by4)
    }
    //
    include!("./src/test_src_iter_memchr_qpl.rs");
}
