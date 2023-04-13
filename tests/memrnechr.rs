mod impl_std;
mod test_std_memrnechr {
    fn test_memrnechr(buf: &[u8], byte: u8) -> Option<usize> {
        super::impl_std::_std_memrnechr(buf, byte)
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
