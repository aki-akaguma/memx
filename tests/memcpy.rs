mod impl_std;
mod test_std_memcpy {
    use memx::RangeError;
    fn test_memcpy(dst: &mut [u8], src: &[u8]) -> Result<(), RangeError> {
        super::impl_std::_std_memcpy(dst, src)
    }
    //
    const _RT_AC: bool = false;
    include!("./src/test_src_memcpy.rs");
}
mod test_memx_memcpy {
    use memx::RangeError;
    fn test_memcpy(dst: &mut [u8], src: &[u8]) -> Result<(), RangeError> {
        memx::memcpy(dst, src)
    }
    //
    const _RT_AC: bool = false;
    include!("./src/test_src_memcpy.rs");
}
mod test_memx_memcpy_basic {
    use memx::RangeError;
    fn test_memcpy(dst: &mut [u8], src: &[u8]) -> Result<(), RangeError> {
        memx::mem::memcpy_basic(dst, src)
    }
    //
    const _RT_AC: bool = false;
    include!("./src/test_src_memcpy.rs");
}
