mod test_memx_mem_memchr {
    fn test_memchr(buf: &[u8], byte: u8) -> Option<usize> {
        memx::mem::memchr_basic(buf, byte)
    }
    //
    #[test]
    fn test01() {
        let buf = vec![b'A', b'B', b'C', b'D', b'E', b'F', b'G', b'H', b'J', b'K'];
        //
        let r = test_memchr(&buf, b'G');
        assert_eq!(r, Some(6));
    }
    #[test]
    fn test02() {
        let buf_0 = vec![0_u8];
        for x in 0..600 {
            let buf = {
                let mut buf: Vec<u8> = buf_0.repeat(1 + x);
                buf.push(b'G');
                buf
            };
            //
            let r = test_memchr(&buf, b'G');
            assert_eq!(r, Some(1 + x));
        }
    }
}
