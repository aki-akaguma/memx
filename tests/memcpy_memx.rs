mod test_memx_memcpy {
    fn test_memcpy(dst: &mut [u8], src: &[u8]) {
        let r = memx::memcpy(dst, src);
        assert_eq!(r.is_ok(), true);
    }
    //
    #[test]
    fn test01() {
        let dst = vec![b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0'];
        let src = vec![b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b'0'];
        //
        let mut dst2 = dst.clone();
        let dst_sl = dst2.as_mut_slice();
        let src_sl = src.as_slice();
        //
        test_memcpy(dst_sl, &src_sl[0..3]);
        assert_eq!(&dst2[0..3], &src[0..3]);
    }
    #[test]
    fn test02() {
        let dst_0 = vec![0_u8];
        let src_0 = vec![b'1'];
        for x in 0..600 {
            let mut dst: Vec<u8> = dst_0.repeat(1 + x + 16);
            let src: Vec<u8> = src_0.repeat(1 + x);
            //
            let dst_sl = dst.as_mut_slice();
            let src_sl = src.as_slice();
            //
            test_memcpy(dst_sl, src_sl);
            assert_eq!(&dst[0..src_sl.len()], src_sl);
            assert_eq!(dst[src_sl.len()], 0);
            assert_eq!(dst[src_sl.len() + 1], 0);
            assert_eq!(dst[src_sl.len() + 2], 0);
            assert_eq!(dst[src_sl.len() + 3], 0);
            assert_eq!(dst[src_sl.len() + 4], 0);
        }
    }
}
