mod test_std_memrchr_double {
    fn test_memrchr_double(buf: &[u8], byte1: u8, byte2: u8) -> Option<usize> {
        buf.iter().rposition(|&x| x == byte1 || x == byte2)
    }
    //
    include!("./src/test_src_memrchr_double.rs");
}
mod test_memx_memrchr_double {
    fn test_memrchr_double(buf: &[u8], byte1: u8, byte2: u8) -> Option<usize> {
        memx::memrchr_double(buf, byte2, byte1)
    }
    //
    include!("./src/test_src_memrchr_double.rs");
}
mod test_memx_memrchr_double_basic {
    fn test_memrchr_double(buf: &[u8], byte1: u8, byte2: u8) -> Option<usize> {
        memx::mem::memrchr_double_basic(buf, byte2, byte1)
    }
    //
    include!("./src/test_src_memrchr_double.rs");
}
