mod test_std_memnechr {
    fn test_memnechr(buf: &[u8], byte: u8) -> Option<usize> {
        buf.iter().position(|&x| x != byte)
    }
    //
    include!("./src/test_src_memnechr.rs");
}
mod test_memx_memnechr {
    fn test_memnechr(buf: &[u8], byte: u8) -> Option<usize> {
        memx::memnechr(buf, byte)
    }
    //
    include!("./src/test_src_memnechr.rs");
}
mod test_memx_memnechr_basic {
    fn test_memnechr(buf: &[u8], byte: u8) -> Option<usize> {
        memx::mem::memnechr_basic(buf, byte)
    }
    //
    include!("./src/test_src_memnechr.rs");
}
