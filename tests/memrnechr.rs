mod test_std_memrnechr {
    fn test_memrnechr(buf: &[u8], byte: u8) -> Option<usize> {
        buf.iter().rposition(|&x| x != byte)
    }
    //
    include!("./src/test_src_memrnechr.rs");
}
mod test_memx_memrnechr {
    fn test_memrnechr(buf: &[u8], byte: u8) -> Option<usize> {
        memx::memrnechr(buf, byte)
    }
    //
    include!("./src/test_src_memrnechr.rs");
}
mod test_memx_memrnechr_basic {
    fn test_memrnechr(buf: &[u8], byte: u8) -> Option<usize> {
        memx::mem::memrnechr_basic(buf, byte)
    }
    //
    include!("./src/test_src_memrnechr.rs");
}
