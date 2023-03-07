mod test_std_memrmem {
    fn test_memrmem(buf: &[u8], pat_bytes: &[u8]) -> Option<usize> {
        let buf_len = buf.len();
        let pat_len = pat_bytes.len();
        if buf_len < pat_len {
            return None;
        }
        let max_i = buf_len - pat_len;
        for i in 0..=max_i {
            let j = max_i - i;
            if &buf[j..(j + pat_len)] == pat_bytes {
                return Some(j);
            }
        }
        None
    }
    //
    include!("./src/test_src_memrmem.rs");
}
mod test_memx_memrmem {
    fn test_memrmem(buf: &[u8], pat_bytes: &[u8]) -> Option<usize> {
        memx::memrmem(buf, pat_bytes)
    }
    //
    include!("./src/test_src_memrmem.rs");
}
mod test_memx_memrmem_basic {
    fn test_memrmem(buf: &[u8], pat_bytes: &[u8]) -> Option<usize> {
        memx::mem::memrmem_basic(buf, pat_bytes)
    }
    //
    include!("./src/test_src_memrmem.rs");
}
