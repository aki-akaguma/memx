/**/

#[test]
fn test00() {
    let buf = vec![];
    let r = test_memchr_qpl(&buf, b'G', b'g', b'X', b'Y');
    assert_eq!(r, None);
    //
    let buf = vec![b'A', b'g'];
    let r = test_memchr_qpl(&buf, b'Y', b'X', b'G', b'g');
    assert_eq!(r, Some(1));
    let r = test_memchr_qpl(&buf, b'X', b'G', b'g', b'Y');
    assert_eq!(r, Some(1));
    let r = test_memchr_qpl(&buf, b'G', b'g', b'Y', b'X');
    assert_eq!(r, Some(1));
    let r = test_memchr_qpl(&buf, b'g', b'Y', b'X', b'G');
    assert_eq!(r, Some(1));
    //
    let buf = vec![b'A', b'B', b'C'];
    let r = test_memchr_qpl(&buf, b'A', b'a', b'X', b'Y');
    assert_eq!(r, Some(0));
    let r = test_memchr_qpl(&buf, b'Y', b'A', b'a', b'X');
    assert_eq!(r, Some(0));
    let r = test_memchr_qpl(&buf, b'X', b'Y', b'A', b'a');
    assert_eq!(r, Some(0));
    let r = test_memchr_qpl(&buf, b'a', b'X', b'Y', b'A');
    assert_eq!(r, Some(0));
    //
    let buf = vec![b'A', b'B', b'C'];
    let r = test_memchr_qpl(&buf, b'B', b'b', b'X', b'Y');
    assert_eq!(r, Some(1));
    let r = test_memchr_qpl(&buf, b'b', b'B', b'X', b'Y');
    assert_eq!(r, Some(1));
    let r = test_memchr_qpl(&buf, b'Y', b'X', b'b', b'B');
    assert_eq!(r, Some(1));
    //
    let buf = vec![b'A', b'B', b'C'];
    let r = test_memchr_qpl(&buf, b'C', b'c', b'X', b'Y');
    assert_eq!(r, Some(2));
    let r = test_memchr_qpl(&buf, b'c', b'C', b'X', b'Y');
    assert_eq!(r, Some(2));
    let r = test_memchr_qpl(&buf, b'Y', b'X', b'c', b'C');
    assert_eq!(r, Some(2));
    //
    let buf = vec![b'A', b'B', b'C'];
    let r = test_memchr_qpl(&buf, b'a', b'b', b'c', b'd');
    assert_eq!(r, None);
}
#[test]
fn test01() {
    let buf = vec![b'A', b'B', b'C', b'D', b'E', b'F', b'G', b'H', b'J', b'K'];
    //
    let r = test_memchr_qpl(&buf, b'G', b'g', b'X', b'Y');
    assert_eq!(r, Some(6));
    let r = test_memchr_qpl(&buf, b'g', b'G', b'X', b'Y');
    assert_eq!(r, Some(6));
    let r = test_memchr_qpl(&buf, b'Y', b'X', b'g', b'G');
    assert_eq!(r, Some(6));
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
            x86_alignment_check::ac_call_once(|| { test_memchr_qpl(&buf[x..], b'Y', b'X', b'G', b'g') })
        } else {
            test_memchr_qpl(&buf[x..], b'Y', b'X', b'G', b'g')
        });
        assert_eq!(r, Some(1 + x));
        //
        let r = cfg_iif::cfg_iif!(all(not(miri), feature = "test_alignment_check",
        any(target_arch = "x86_64", target_arch = "x86")) {
            x86_alignment_check::ac_call_once(|| { test_memchr_qpl(&buf[x..], b'Y', b'X', b'g', b'G') })
        } else {
            test_memchr_qpl(&buf[x..], b'Y', b'X', b'g', b'G')
        });
        assert_eq!(r, Some(1 + x));
        //
        let r = cfg_iif::cfg_iif!(all(not(miri), feature = "test_alignment_check",
        any(target_arch = "x86_64", target_arch = "x86")) {
            x86_alignment_check::ac_call_once(|| { test_memchr_qpl(&buf[x..], b'Y', b'X', b'A', b'a') })
        } else {
            test_memchr_qpl(&buf[x..], b'Y', b'X', b'A', b'a')
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
