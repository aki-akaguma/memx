mod test_std_memmem {
    fn test_memmem(buf: &[u8], pat_bytes: &[u8]) -> Option<usize> {
        if buf.len() < pat_bytes.len() {
            return None;
        }
        (0..=(buf.len() - pat_bytes.len())).find(|&i| &buf[i..(i + pat_bytes.len())] == pat_bytes)
    }
    //
    include!("./src/test_src_memmem.rs");
}
mod test_memx_memmem {
    fn test_memmem(buf: &[u8], pat_bytes: &[u8]) -> Option<usize> {
        memx::memmem(buf, pat_bytes)
    }
    //
    include!("./src/test_src_memmem.rs");
}
mod test_memx_memmem_basic {
    fn test_memmem(buf: &[u8], pat_bytes: &[u8]) -> Option<usize> {
        memx::mem::memmem_basic(buf, pat_bytes)
    }
    //
    include!("./src/test_src_memmem.rs");
}
