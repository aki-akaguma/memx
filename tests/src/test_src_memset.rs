/**/

#[test]
fn test00() {
    let mut dst = vec![];
    //
    let dst_sl = dst.as_mut_slice();
    //
    test_memset(dst_sl, b'a');
}
#[test]
fn test01() {
    let mut dst = vec![b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0'];
    let src = vec![b'A', b'A', b'A', b'A', b'A', b'A', b'A', b'A', b'A', b'A'];
    //
    let dst_sl = dst.as_mut_slice();
    //
    test_memset(&mut dst_sl[0..3], src[0]);
    assert_eq!(&dst[0..3], &src[0..3]);
}
#[test]
fn test02() {
    let dst_0 = vec![0_u8];
    let src_0 = vec![b'1'];
    let f = |x: usize| {
        let mut dst: Vec<u8> = dst_0.repeat(x + 1 + x + 16);
        let src: Vec<u8> = src_0.repeat(1 + x);
        //
        let dst_sl = dst.as_mut_slice();
        let src_sl = src.as_slice();
        //
        cfg_iif::cfg_iif!(all(not(miri), feature = "test_alignment_check",
        any(target_arch = "x86_64", target_arch = "x86")) {
            if _RT_AC {
                x86_alignment_check::ac_call_once(|| { test_memset(&mut dst_sl[x..(x + src_sl.len())], src_sl[0]); })
            } else {
                test_memset(&mut dst_sl[x..(x + src_sl.len())], src_sl[0]);
            }
        } else {
            test_memset(&mut dst_sl[x..(x + src_sl.len())], src_sl[0]);
        });
        assert_eq!(&dst[x..(x + src_sl.len())], src_sl);
        assert_eq!(dst[x + src_sl.len() + 4], 0);
        assert_eq!(dst[x + src_sl.len() + 3], 0);
        assert_eq!(dst[x + src_sl.len() + 2], 0);
        assert_eq!(dst[x + src_sl.len() + 1], 0);
        assert_eq!(dst[x + src_sl.len()], 0);
    };
    #[cfg(not(miri))]
    {
        for x in 0..600 {
            f(x);
        }
    }
    #[cfg(miri)]
    {
        for x in [0, 299, 599].into_iter() {
            f(x);
        }
    }
}
