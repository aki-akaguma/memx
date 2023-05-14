/**/

#[test]
fn test00() {
    let buf = vec![];
    let r = test_memnechr_qpl(&buf, b'g', b'Z', b'Y', b'X');
    assert_eq!(r, None);
    //
    let buf = vec![b'A', b'g'];
    let mut vv = vec![b'A', b'Z', b'Y', b'X'];
    for _ in 0..2 {
        vv.rotate_right(1);
        let r = test_memnechr_qpl(&buf, vv[0], vv[1], vv[2], vv[3]);
        assert_eq!(r, Some(1));
    }
    //
    let buf = vec![b'a', b'B', b'C'];
    let mut vv = vec![b'A', b'Z', b'Y', b'X'];
    for _ in 0..2 {
        vv.rotate_right(1);
        let r = test_memnechr_qpl(&buf, vv[0], vv[1], vv[2], vv[3]);
        assert_eq!(r, Some(0));
    }
    //
    let buf = vec![b'A', b'b', b'C'];
    let mut vv = vec![b'A', b'Z', b'Y', b'X'];
    for _ in 0..2 {
        vv.rotate_right(1);
        let r = test_memnechr_qpl(&buf, vv[0], vv[1], vv[2], vv[3]);
        assert_eq!(r, Some(1));
    }
    //
    let buf = vec![b'A', b'B', b'c'];
    let mut vv = vec![b'A', b'B', b'Y', b'X'];
    for _ in 0..2 {
        vv.rotate_right(1);
        let r = test_memnechr_qpl(&buf, vv[0], vv[1], vv[2], vv[3]);
        assert_eq!(r, Some(2));
    }
    //
    let buf = vec![b'A', b'B', b'C', b'D', b'A'];
    let r = test_memnechr_qpl(&buf, b'A', b'B', b'C', b'D');
    assert_eq!(r, None);
}
#[test]
fn test01() {
    #[rustfmt::skip]
    let buf = vec![
        b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
        b' ', b'j', b'k', b'l', b'm', b' ', b' ', b' ',
        b' ', b' ', b' ', b' ', b' ', b' ', b' ', b' ',
        b' ', b' '
    ];
    //
    let mut vv = vec![b'k', b'l', b'm', b' '];
    for _ in 0..2 {
        vv.rotate_right(1);
        let r = test_memnechr_qpl(&buf, vv[0], vv[1], vv[2], vv[3]);
        assert_eq!(r, Some(9));
    }
    //
    let mut vv = vec![b'j', b'l', b'm', b' '];
    for _ in 0..2 {
        vv.rotate_right(1);
        let r = test_memnechr_qpl(&buf, vv[0], vv[1], vv[2], vv[3]);
        assert_eq!(r, Some(10));
    }
}
#[test]
fn test02() {
    let buf_0 = vec![0_u8];
    let f = |x: usize| {
        let buf = {
            let mut buf: Vec<u8> = buf_0.clone();
            buf.append(&mut buf_0.repeat(x));
            buf.append(&mut buf_0.repeat(x));
            buf.push(b'G');
            buf.append(&mut buf_0.repeat(1 + x));
            buf
        };
        //
        let r = cfg_iif::cfg_iif!(all(not(miri), feature = "test_alignment_check",
        any(target_arch = "x86_64", target_arch = "x86")) {
            x86_alignment_check::ac_call_once(|| { test_memnechr_qpl(&buf[x..], 0_u8, b'Z', b'Y', b'X') })
        } else {
            test_memnechr_qpl(&buf[x..], 0_u8, b'Z', b'Y', b'X')
        });
        assert_eq!(r, Some(1 + x));
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
