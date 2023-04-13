mod impl_std;
mod test_std_iter_memmem {
    fn test_memmem_iter<'a>(buf: &'a [u8], pat: &'a [u8]) -> super::impl_std::StdMemmemIter<'a> {
        super::impl_std::_std_memmem_iter(buf, pat)
    }
    //
    include!("./src/test_src_iter_memmem.rs");
}
mod test_memx_iter_memmem {
    fn test_memmem_iter<'a>(buf: &'a [u8], pat: &'a [u8]) -> memx::iter::MemmemIter<'a> {
        memx::iter::memmem_iter(buf, pat)
    }
    //
    include!("./src/test_src_iter_memmem.rs");
}
