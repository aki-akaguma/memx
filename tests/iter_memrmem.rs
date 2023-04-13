mod impl_std;
mod test_std_iter_memrmem {
    fn test_memrmem_iter<'a>(buf: &'a [u8], pat: &'a [u8]) -> super::impl_std::StdMemrmemIter<'a> {
        super::impl_std::_std_memrmem_iter(buf, pat)
    }
    //
    include!("./src/test_src_iter_memrmem.rs");
}
mod test_memx_iter_memrmem {
    fn test_memrmem_iter<'a>(buf: &'a [u8], pat: &'a [u8]) -> memx::iter::MemrmemIter<'a> {
        memx::iter::memrmem_iter(buf, pat)
    }
    //
    include!("./src/test_src_iter_memrmem.rs");
}
