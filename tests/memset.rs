mod test_std_memset {
    #[rustversion::since(1.50)]
    fn test_memset(dst: &mut [u8], byte: u8) {
        dst.fill(byte);
    }
    #[rustversion::before(1.50)]
    fn test_memset(dst: &mut [u8], byte: u8) {
        for i in 0..dst.len() {
            dst[i] = byte;
        }
    }
    //
    include!("./src/test_src_memset.rs");
}
mod test_memx_memset {
    fn test_memset(dst: &mut [u8], byte: u8) {
        memx::memset(dst, byte);
    }
    //
    include!("./src/test_src_memset.rs");
}
mod test_memx_memset_basic {
    fn test_memset(dst: &mut [u8], byte: u8) {
        memx::mem::memset_basic(dst, byte);
    }
    //
    include!("./src/test_src_memset.rs");
}
