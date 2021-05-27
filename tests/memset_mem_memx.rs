mod test_memx_mem_memset {
    fn test_memset(dst: &mut [u8], byte: u8, n: usize) {
        let r = memx::mem::memset_basic(dst, byte, n);
        assert_eq!(r.is_ok(), true);
    }
    //
    #[test]
    fn test01() {
        let dst = vec![b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0'];
        let src = vec![b'A', b'A', b'A', b'A', b'A', b'A', b'A', b'A', b'A', b'A'];
        //
        let mut dst2 = dst.clone();
        let dst_sl = dst2.as_mut_slice();
        //
        test_memset(dst_sl, src[0], 3);
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
            test_memset(dst_sl, src_sl[0], src_sl.len());
            assert_eq!(&dst[0..src_sl.len()], src_sl);
            assert_eq!(dst[src_sl.len()], 0);
            assert_eq!(dst[src_sl.len() + 1], 0);
            assert_eq!(dst[src_sl.len() + 2], 0);
            assert_eq!(dst[src_sl.len() + 3], 0);
            assert_eq!(dst[src_sl.len() + 4], 0);
        }
    }
}
