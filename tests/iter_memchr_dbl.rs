mod test_iter_memchr {
    fn test_memchr_dbl_iter(buf: &[u8], byte1: u8, byte2: u8) -> memx::iter::MemchrDblIter {
        memx::iter::memchr_dbl_iter(buf, byte1, byte2)
    }
    //
    include!("./src/test_src_iter_memchr_dbl.rs");
}
