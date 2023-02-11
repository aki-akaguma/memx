mod test_memx_memrnechr {
    fn test_memrnechr(buf: &[u8], byte: u8) -> Option<usize> {
        memx::memrnechr(buf, byte)
    }
    //
    #[test]
    fn test00() {
        let buf = vec![];
        //
        let r = test_memrnechr(&buf, b'G');
        assert_eq!(r, None);
    }
    #[test]
    fn test01() {
        let buf = vec![b'A', b'B', b'C', b'D', b'E', b'F', b'G', b'H', b'J', b'K'];
        //
        let r = test_memrnechr(&buf, b'G');
        assert_eq!(r, Some(9));
        //
        let r = test_memrnechr(&buf, b'K');
        assert_eq!(r, Some(8));
    }
    #[test]
    fn test02() {
        let buf_0 = vec![b' '];
        let f = |x: usize| {
            let buf = {
                let mut buf: Vec<u8> = buf_0.repeat(1 + x);
                buf.push(b'G');
                buf
            };
            //
            let r = test_memrnechr(&buf, b' ');
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
