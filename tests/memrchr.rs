mod impl_std;
mod test_std_memrchr {
    fn test_memrchr(buf: &[u8], by1: u8) -> Option<usize> {
        super::impl_std::_std_memrchr(buf, by1)
    }
    //
    include!("./src/test_src_memrchr.rs");
}
mod test_memx_memrchr {
    fn test_memrchr(buf: &[u8], by1: u8) -> Option<usize> {
        memx::memrchr(buf, by1)
    }
    //
    include!("./src/test_src_memrchr.rs");
}
mod test_memx_memrchr_basic {
    fn test_memrchr(buf: &[u8], by1: u8) -> Option<usize> {
        memx::mem::memrchr_basic(buf, by1)
    }
    //
    include!("./src/test_src_memrchr.rs");
}
