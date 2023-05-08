/**/

#[test]
fn test00() {
    let buf = vec![];
    let r = test_memchr_dbl(&buf, b'g', b'Z');
    assert_eq!(r, None);
    //
    let buf = vec![b'A', b'g'];
    let mut vv = vec![b'g', b'Z'];
    for _ in 0..2 {
        vv.rotate_right(1);
        let r = test_memchr_dbl(&buf, vv[0], vv[1]);
        assert_eq!(r, Some(1));
    }
    //
    let buf = vec![b'a', b'B', b'C'];
    let mut vv = vec![b'a', b'Z'];
    for _ in 0..2 {
        vv.rotate_right(1);
        let r = test_memchr_dbl(&buf, vv[0], vv[1]);
        assert_eq!(r, Some(0));
    }
    //
    let buf = vec![b'A', b'b', b'C'];
    let mut vv = vec![b'b', b'Z'];
    for _ in 0..2 {
        vv.rotate_right(1);
        let r = test_memchr_dbl(&buf, vv[0], vv[1]);
        assert_eq!(r, Some(1));
    }
    //
    let buf = vec![b'A', b'B', b'c'];
    let mut vv = vec![b'c', b'Z'];
    for _ in 0..2 {
        vv.rotate_right(1);
        let r = test_memchr_dbl(&buf, vv[0], vv[1]);
        assert_eq!(r, Some(2));
    }
    //
    let buf = vec![b'A', b'B', b'C'];
    let r = test_memchr_dbl(&buf, b'a', b'b');
    assert_eq!(r, None);
}
#[test]
fn test01() {
    #[rustfmt::skip]
    let buf = vec![
        b'A', b'B', b'C', b'D', b'E', b'F', b'G', b'H',
        b'I', b'j', b'k', b'L', b'M', b'N', b'O', b'P',
        b'Q', b'R', b'S', b'T', b'U', b'V', b'W', b'X',
        b'Y', b'Z'
    ];
    //
    let mut vv = vec![b'j', b'K'];
    for _ in 0..2 {
        vv.rotate_right(1);
        let r = test_memchr_dbl(&buf, vv[0], vv[1]);
        assert_eq!(r, Some(9));
    }
    //
    let mut vv = vec![b'j', b'k'];
    for _ in 0..2 {
        vv.rotate_right(1);
        let r = test_memchr_dbl(&buf, vv[0], vv[1]);
        assert_eq!(r, Some(9));
    }
}
#[test]
fn test02() {
    let buf_a = vec![b'x'];
    let buf_0 = vec![0_u8];
    let f = |x: usize| {
        let buf = {
            let mut buf: Vec<u8> = buf_a.clone();
            buf.append(&mut buf_0.repeat(x));
            buf.append(&mut buf_0.repeat(x));
            buf.push(b'G');
            buf.push(b'a');
            buf.append(&mut buf_0.repeat(1 + x));
            buf
        };
        //
        let r = cfg_iif::cfg_iif!(all(not(miri), feature = "test_alignment_check",
        any(target_arch = "x86_64", target_arch = "x86")) {
            x86_alignment_check::ac_call_once(|| { test_memchr_dbl(&buf[x..], b'G', b'g') })
        } else {
            test_memchr_dbl(&buf[x..], b'G', b'g')
        });
        assert_eq!(r, Some(1 + x));
        //
        let r = cfg_iif::cfg_iif!(all(not(miri), feature = "test_alignment_check",
        any(target_arch = "x86_64", target_arch = "x86")) {
            x86_alignment_check::ac_call_once(|| { test_memchr_dbl(&buf[x..], b'g', b'G') })
        } else {
            test_memchr_dbl(&buf[x..], b'g', b'G')
        });
        assert_eq!(r, Some(1 + x));
        //
        let r = cfg_iif::cfg_iif!(all(not(miri), feature = "test_alignment_check",
        any(target_arch = "x86_64", target_arch = "x86")) {
            x86_alignment_check::ac_call_once(|| { test_memchr_dbl(&buf[x..], b'A', b'a') })
        } else {
            test_memchr_dbl(&buf[x..], b'A', b'a')
        });
        assert_eq!(r, Some(2 + x));
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
