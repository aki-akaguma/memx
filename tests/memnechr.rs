mod impl_std;
mod test_std_memnechr {
    fn test_memnechr(buf: &[u8], byte: u8) -> Option<usize> {
        super::impl_std::_std_memnechr(buf, byte)
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
