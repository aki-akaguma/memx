/**/

#[test]
fn test00() {
    let buf = vec![];
    let r = test_memrchr(&buf, b'G');
    assert_eq!(r, None);
    //
    let buf = vec![b'A', b'G'];
    let r = test_memrchr(&buf, b'G');
    assert_eq!(r, Some(1));
    //
    let buf = vec![b'A', b'B', b'C'];
    let r = test_memrchr(&buf, b'A');
    assert_eq!(r, Some(0));
    //
    let buf = vec![b'A', b'B', b'C'];
    let r = test_memrchr(&buf, b'B');
    assert_eq!(r, Some(1));
    //
    let buf = vec![b'A', b'B', b'C'];
    let r = test_memrchr(&buf, b'C');
    assert_eq!(r, Some(2));
    //
    let buf = vec![b'A', b'B', b'C'];
    let r = test_memrchr(&buf, b'a');
    assert_eq!(r, None);
}
#[test]
fn test01() {
    let buf = vec![b'A', b'G', b'C', b'D', b'E', b'F', b'G', b'H', b'J', b'K'];
    //
    let r = test_memrchr(&buf, b'G');
    assert_eq!(r, Some(6));
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
            x86_alignment_check::ac_call_once(|| { test_memrchr(&buf[x..], b'G') })
        } else {
            test_memrchr(&buf[x..], b'G')
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
