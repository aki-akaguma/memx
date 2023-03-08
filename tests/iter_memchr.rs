mod test_memx_iter_memchr {
    fn test_memchr_iter(buf: &[u8], byte: u8) -> memx::iter::MemchrIter {
        memx::iter::memchr_iter(buf, byte)
    }
    //
    include!("./src/test_src_iter_memchr.rs");
}
