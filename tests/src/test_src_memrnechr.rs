/**/

#[test]
fn test00() {
    let buf = vec![];
    let r = test_memrnechr(&buf, b'G');
    assert_eq!(r, None);
    //
    let buf = vec![b'g'];
    let r = test_memrnechr(&buf, b'G');
    assert_eq!(r, Some(0));
    //
    let buf = vec![b'a', b'A', b'A'];
    let r = test_memrnechr(&buf, b'A');
    assert_eq!(r, Some(0));
    //
    let buf = vec![b'B', b'C', b'B'];
    let r = test_memrnechr(&buf, b'B');
    assert_eq!(r, Some(1));
    //
    let buf = vec![b'B', b'B', b'C'];
    let r = test_memrnechr(&buf, b'B');
    assert_eq!(r, Some(2));
    //
    let buf = vec![b'A', b'A', b'A'];
    let r = test_memrnechr(&buf, b'A');
    assert_eq!(r, None);
    //
    let buf = vec![b'C', b'B', b'B'];
    let r = test_memrnechr(&buf, b'B');
    assert_eq!(r, Some(0));
}
#[test]
fn test01() {
    let buf = vec![b'A', b'B', b'C', b'D', b'E', b'F', b'G', b'H', b'J', b'K'];
    //
    let r = test_memrnechr(&buf, b'G');
    assert_eq!(r, Some(9));
    //
    let r = test_memrnechr(&buf, b'K');
    assert_eq!(r, Some(8));
}
#[test]
fn test02() {
    let buf_g = vec![b'G'];
    let buf_0 = vec![0_u8];
    let f = |x: usize| {
        let buf = {
            let mut buf: Vec<u8> = buf_g.clone();
            buf.append(&mut buf_0.repeat(x));
            buf.append(&mut buf_0.repeat(x));
            buf.push(b'G');
            buf.append(&mut buf_0.repeat(1 + x));
            buf
        };
        //
        let r = cfg_iif::cfg_iif!(all(not(miri), feature = "test_alignment_check",
        any(target_arch = "x86_64", target_arch = "x86")) {
            x86_alignment_check::ac_call_once(|| { test_memrnechr(&buf[x..], 0_u8) })
        } else {
            test_memrnechr(&buf[x..], 0_u8)
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
