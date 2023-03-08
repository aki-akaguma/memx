mod test_iter_memmem {
    fn test_memmem_iter<'a>(buf: &'a [u8], pat: &'a [u8]) -> memx::iter::MemmemIter<'a> {
        memx::iter::memmem_iter(buf, pat)
    }
    //
    include!("./src/test_src_iter_memmem.rs");
}
