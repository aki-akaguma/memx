mod test_std_memchr_dbl {
    fn test_memchr_dbl(buf: &[u8], byte1: u8, byte2: u8) -> Option<usize> {
        buf.iter().position(|&x| x == byte1 || x == byte2)
    }
    //
    include!("./src/test_src_memchr_dbl.rs");
}
mod test_memx_memchr_dbl {
    fn test_memchr_dbl(buf: &[u8], byte1: u8, byte2: u8) -> Option<usize> {
        memx::memchr_dbl(buf, byte2, byte1)
    }
    //
    include!("./src/test_src_memchr_dbl.rs");
}
mod test_memx_memchr_dbl_basic {
    fn test_memchr_dbl(buf: &[u8], byte1: u8, byte2: u8) -> Option<usize> {
        memx::mem::memchr_dbl_basic(buf, byte2, byte1)
    }
    //
    include!("./src/test_src_memchr_dbl.rs");
}
