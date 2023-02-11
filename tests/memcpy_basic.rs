mod test_memx_memcpy_basic {
    fn test_memcpy(dst: &mut [u8], src: &[u8]) {
        let r = memx::mem::memcpy_basic(dst, src);
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test01() {
        let mut dst = vec![b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0'];
        let src = vec![b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b'0'];
        //
        let dst_sl = dst.as_mut_slice();
        let src_sl = src.as_slice();
        //
        test_memcpy(dst_sl, &src_sl[0..3]);
        assert_eq!(&dst[0..3], &src[0..3]);
    }
    #[test]
    fn test02() {
        let dst_0 = vec![0_u8];
        let src_0 = vec![b'1'];
        let f = |x: usize| {
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
