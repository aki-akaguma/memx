mod test_std_memchr_double {
    fn test_memchr_double(buf: &[u8], byte1: u8, byte2: u8) -> Option<usize> {
        buf.iter().position(|&x| x == byte1 || x == byte2)
    }
    //
    include!("./src/test_src_memchr_double.rs");
}
mod test_memx_memchr_double {
    fn test_memchr_double(buf: &[u8], byte1: u8, byte2: u8) -> Option<usize> {
        memx::memchr_double(buf, byte2, byte1)
    }
    //
    include!("./src/test_src_memchr_double.rs");
}
mod test_memx_memchr_double_basic {
    fn test_memchr_double(buf: &[u8], byte1: u8, byte2: u8) -> Option<usize> {
        memx::mem::memchr_double_basic(buf, byte2, byte1)
    }
    //
    include!("./src/test_src_memchr_double.rs");
}
