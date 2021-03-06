mod test_memx_memmem_basic {
    fn test_memmem(buf: &[u8], pat_bytes: &[u8]) -> Option<usize> {
        memx::mem::memmem_basic(buf, pat_bytes)
    }
    //
    #[test]
    fn test01() {
        let buf = vec![b'A', b'B', b'C', b'D', b'E', b'F', b'G', b'H', b'J', b'K'];
        let pat = vec![b'G', b'H', b'J'];
        //
        let r = test_memmem(&buf, &pat);
        assert_eq!(r, Some(6));
    }
    #[test]
    fn test02() {
        let buf_0 = vec![0_u8];
        let pat = vec![b'G', b'H', b'J'];
        for x in 0..600 {
            let buf = {
                let mut buf: Vec<u8> = buf_0.repeat(1 + x);
                buf.extend_from_slice(&pat);
                buf
            };
            //
            let r = test_memmem(&buf, &pat);
            assert_eq!(r, Some(1 + x));
        }
    }
}
