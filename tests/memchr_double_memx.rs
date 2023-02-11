mod test_memx_memchr_double {
    fn test_memchr_double(buf: &[u8], byte1: u8, byte2: u8) -> Option<usize> {
        memx::memchr_double(buf, byte2, byte1)
    }
    //
    #[test]
    fn test00() {
        let buf = vec![];
        //
        let r = test_memchr_double(&buf, b'G', b'g');
        assert_eq!(r, None);
    }
    #[test]
    fn test01() {
        let buf = vec![b'A', b'B', b'C', b'D', b'E', b'F', b'G', b'H', b'J', b'K'];
        //
        let r = test_memchr_double(&buf, b'G', b'g');
        assert_eq!(r, Some(6));
    }
    #[test]
    fn test02() {
        let buf_0 = vec![0_u8];
        let f = |x: usize| {
            let buf = {
                let mut buf: Vec<u8> = buf_0.repeat(1 + x);
                buf.push(b'G');
                buf
            };
            //
            let r = test_memchr_double(&buf, b'G', b'g');
            assert_eq!(r, Some(1 + x));
        };
        if cfg!(miri) {
            for x in [0, 299, 599].into_iter() {
                f(x);
            }
        } else {
            for x in 0..600 {
                f(x);
            }
        }
    }
}
