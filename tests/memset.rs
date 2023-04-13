mod impl_std;
mod test_std_memset {
    fn test_memset(dst: &mut [u8], byte: u8) {
        super::impl_std::_std_memset(dst, byte)
    }
    //
    const _RT_AC: bool = true;
    include!("./src/test_src_memset.rs");
}
mod test_memx_memset {
    fn test_memset(dst: &mut [u8], byte: u8) {
        memx::memset(dst, byte);
    }
    //
    const _RT_AC: bool = true;
    include!("./src/test_src_memset.rs");
}
mod test_memx_memset_basic {
    fn test_memset(dst: &mut [u8], byte: u8) {
        memx::mem::memset_basic(dst, byte);
    }
    //
    const _RT_AC: bool = true;
    include!("./src/test_src_memset.rs");
}
