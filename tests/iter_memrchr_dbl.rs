mod test_iter_memrchr {
    fn test_memrchr_dbl_iter(buf: &[u8], byte1: u8, byte2: u8) -> memx::iter::MemrchrDblIter {
        memx::iter::memrchr_dbl_iter(buf, byte1, byte2)
    }
    //
    include!("./src/test_src_iter_memrchr_dbl.rs");
}
