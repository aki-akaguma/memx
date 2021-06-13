mod test_memx_memcmp {
    use std::cmp::Ordering;
    //
    fn test_memcmp(buf: &[u8], pat_bytes: &[u8]) -> Ordering {
        memx::memcmp(buf, pat_bytes)
    }
    //
    #[test]
    fn test00() {
        let buf_0 = vec![];
        let pat_1 = vec![];
        let pat_2 = vec![b'A', b'B', b'C', b'D', b'E', b'F', b'G', b'H', b'J', b'0'];
        //
        let r = test_memcmp(&buf_0, &pat_1);
        assert_eq!(r, Ordering::Equal);
        let r = test_memcmp(&buf_0, &pat_2);
        assert_eq!(r, Ordering::Less);
        let r = test_memcmp(&pat_2, &pat_1);
        assert_eq!(r, Ordering::Greater);
    }
    #[test]
    fn test01() {
        let buf_0 = vec![b'A', b'B', b'C', b'D', b'E', b'F', b'G', b'H', b'J', b'5'];
        let pat_1 = vec![b'A', b'B', b'C', b'D', b'E', b'F', b'G', b'H', b'J', b'9'];
        let pat_2 = vec![b'A', b'B', b'C', b'D', b'E', b'F', b'G', b'H', b'J', b'0'];
        //
        let r = test_memcmp(&buf_0[0..(buf_0.len() - 1)], &pat_1[0..(pat_1.len() - 1)]);
        assert_eq!(r, Ordering::Equal);
        let r = test_memcmp(&buf_0, &pat_1);
        assert_eq!(r, Ordering::Less);
        let r = test_memcmp(&buf_0, &pat_2);
        assert_eq!(r, Ordering::Greater);
    }
    #[test]
    fn test02() {
        let buf_0 = vec![0_u8];
        for x in 0..600 {
            let buf_1 = {
                let buf: Vec<u8> = buf_0.repeat(1 + x);
                buf
            };
            //
            let mut buf = buf_1.clone();
            buf.push(b'5');
            let mut pat = buf_1.clone();
            pat.push(b'5');
            let r = test_memcmp(&buf, &pat);
            assert_eq!(r, Ordering::Equal);
            //
            let mut buf = buf_1.clone();
            buf.push(b'5');
            let mut pat = buf_1.clone();
            pat.push(b'9');
            let r = test_memcmp(&buf, &pat);
            assert_eq!(r, Ordering::Less);
            //
            let mut buf = buf_1.clone();
            buf.push(b'5');
            let mut pat = buf_1.clone();
            pat.push(b'0');
            let r = test_memcmp(&buf, &pat);
            assert_eq!(r, Ordering::Greater);
            //
            let buf = buf_1.clone();
            let mut pat = buf_1.clone();
            pat.push(b'0');
            let r = test_memcmp(&buf, &pat);
            assert_eq!(r, Ordering::Less);
            //
            let mut buf = buf_1.clone();
            buf.push(b'5');
            let pat = buf_1.clone();
            let r = test_memcmp(&buf, &pat);
            assert_eq!(r, Ordering::Greater);
        }
    }
}
