mod test_std_memmem {
    fn test_memmem(buf: &[u8], pat_bytes: &[u8]) -> Option<usize> {
        if buf.len() < pat_bytes.len() {
            return None;
        }
        (0..=(buf.len() - pat_bytes.len())).find(|&i| &buf[i..(i + pat_bytes.len())] == pat_bytes)
    }
    //
    #[test]
    fn test00() {
        let buf_0 = vec![];
        let pat_1 = vec![];
        let pat_2 = vec![b'A', b'B', b'C', b'D', b'E', b'F', b'G', b'H', b'J', b'0'];
        //
        let r = test_memmem(&buf_0, &pat_1);
        assert_eq!(r, Some(0));
        let r = test_memmem(&buf_0, &pat_2);
        assert_eq!(r, None);
        let r = test_memmem(&pat_2, &pat_1);
        assert_eq!(r, Some(0));
    }
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
