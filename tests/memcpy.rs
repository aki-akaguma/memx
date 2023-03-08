mod test_std_memcpy {
    use memx::RangeError;
    fn test_memcpy(dst: &mut [u8], src: &[u8]) -> Result<(), RangeError> {
        if dst.len() < src.len() {
            return Err(RangeError);
        }
        let _ = &(dst[0..src.len()]).copy_from_slice(src);
        Ok(())
    }
    //
    include!("./src/test_src_memcpy.rs");
}
mod test_memx_memcpy {
    use memx::RangeError;
    fn test_memcpy(dst: &mut [u8], src: &[u8]) -> Result<(), RangeError> {
        memx::memcpy(dst, src)
    }
    //
    include!("./src/test_src_memcpy.rs");
}
mod test_memx_memcpy_basic {
    use memx::RangeError;
    fn test_memcpy(dst: &mut [u8], src: &[u8]) -> Result<(), RangeError> {
        memx::mem::memcpy_basic(dst, src)
    }
    //
    include!("./src/test_src_memcpy.rs");
}
