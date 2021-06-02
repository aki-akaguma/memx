mod test_memx_mem_memrmem {
    fn test_memrmem(buf: &[u8], pat_bytes: &[u8]) -> Option<usize> {
        memx::mem::memrmem_basic(buf, pat_bytes)
    }
    //
    #[test]
    fn test01() {
        let buf = vec![b'A', b'g', b'h', b'j', b'E', b'F', b'g', b'h', b'j', b'K'];
        let pat = vec![b'g', b'h', b'j'];
        //
        let r = test_memrmem(&buf, &pat);
        assert_eq!(r, Some(6));
    }
    #[test]
    fn test02() {
        let buf_0 = vec![0_u8];
        let pat = vec![b'g', b'h', b'j'];
        for x in 0..600 {
            let buf = {
                let mut buf = pat.clone();
                buf.extend_from_slice(&buf_0.repeat(1 + x));
                buf.extend_from_slice(&pat);
                buf
            };
            //
            let r = test_memrmem(&buf, &pat);
            assert_eq!(r, Some(pat.len() + 1 + x));
        }
    }
}
