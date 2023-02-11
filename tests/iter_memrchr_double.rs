mod test_iter_memrchr {
    fn test_memrchr_double_iter(buf: &[u8], byte1: u8, byte2: u8) -> memx::iter::MemrchrDoubleIter {
        memx::iter::memrchr_double_iter(buf, byte1, byte2)
    }
    //
    #[test]
    fn test01() {
        let buf = vec![
            b'A', b'B', b'g', b'D', b'E', b'F', b'G', b'H', b'J', b'K', b'g',
        ];
        //
        let mut iter = test_memrchr_double_iter(&buf, b'G', b'g');
        assert_eq!(iter.next(), Some(10));
        assert_eq!(iter.next(), Some(6));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), None);
        //
        let mut iter = test_memrchr_double_iter(&buf, b'G', b'g');
        assert_eq!(iter.next(), Some(10));
        assert_eq!(iter.next(), Some(6));
        assert_eq!(iter.next_back(), Some(10));
        assert_eq!(iter.next_back(), None);
        assert_eq!(iter.next(), Some(10));
        assert_eq!(iter.next(), Some(6));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next_back(), Some(2));
        assert_eq!(iter.next_back(), Some(6));
        assert_eq!(iter.next_back(), Some(10));
        assert_eq!(iter.next_back(), None);
        assert_eq!(iter.next_back(), None);
    }
    #[test]
    fn test02() {
        let buf_g = vec![b'g'];
        let buf_0 = vec![0_u8];
        let f = |x: usize| {
            let buf = {
                let mut buf: Vec<u8> = buf_g.clone();
                buf.append(&mut buf_0.repeat(x));
                buf.push(b'g');
                buf.append(&mut buf_0.repeat(1 + x));
                buf
            };
            //
            let mut iter = test_memrchr_double_iter(&buf, b'G', b'g');
            assert_eq!(iter.next(), Some(1 + x));
            assert_eq!(iter.next(), Some(0));
            assert_eq!(iter.next(), None);
            assert_eq!(iter.next_back(), Some(0));
            assert_eq!(iter.next_back(), Some(1 + x));
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
