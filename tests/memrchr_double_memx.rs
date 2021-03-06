mod test_memx_memrchr_double {
    fn test_memchr_double(buf: &[u8], byte1: u8, byte2: u8) -> Option<usize> {
        memx::memrchr_double(buf, byte2, byte1)
    }
    //
    #[test]
    fn test01() {
        let buf = vec![b'A', b'G', b'C', b'D', b'E', b'F', b'G', b'H', b'J', b'K'];
        //
        let r = test_memchr_double(&buf, b'G', b'g');
        assert_eq!(r, Some(6));
    }
    #[test]
    fn test02() {
        let buf_g = vec![b'G'];
        let buf_0 = vec![0_u8];
        for x in 0..600 {
            let buf = {
                let mut buf: Vec<u8> = buf_g.clone();
                buf.append(&mut buf_0.repeat(x));
                buf.push(b'G');
                buf.append(&mut buf_0.repeat(1 + x));
                buf
            };
            //
            let r = test_memchr_double(&buf, b'G', b'g');
            assert_eq!(r, Some(1 + x));
        }
    }
}
