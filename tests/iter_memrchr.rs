mod test_iter_memrchr {
    fn test_memrchr_iter(buf: &[u8], byte: u8) -> memx::iter::MemrchrIter {
        memx::iter::memrchr_iter(buf, byte)
    }
    //
    #[test]
    fn test01() {
        let buf = vec![
            b'A', b'B', b'g', b'D', b'E', b'F', b'g', b'H', b'J', b'K', b'g',
        ];
        //
        let mut iter = test_memrchr_iter(&buf, b'g');
        assert_eq!(iter.next(), Some(10));
        assert_eq!(iter.next(), Some(6));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), None);
        //
        let mut iter = test_memrchr_iter(&buf, b'g');
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
        for x in 0..600 {
            let buf = {
                let mut buf: Vec<u8> = buf_g.clone();
                buf.append(&mut buf_0.repeat(x));
                buf.push(b'g');
                buf.append(&mut buf_0.repeat(1 + x));
                buf
            };
            //
            let mut iter = test_memrchr_iter(&buf, b'g');
            assert_eq!(iter.next(), Some(1 + x));
            assert_eq!(iter.next(), Some(0));
            assert_eq!(iter.next(), None);
            assert_eq!(iter.next_back(), Some(0));
            assert_eq!(iter.next_back(), Some(1 + x));
        }
    }
}
