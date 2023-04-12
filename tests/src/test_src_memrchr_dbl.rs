/**/

#[test]
fn test00() {
    let buf = vec![];
    let r = test_memrchr_dbl(&buf, b'G', b'g');
    assert_eq!(r, None);
    //
    let buf = vec![b'A', b'g'];
    let r = test_memrchr_dbl(&buf, b'G', b'g');
    assert_eq!(r, Some(1));
    let r = test_memrchr_dbl(&buf, b'g', b'G');
    assert_eq!(r, Some(1));
    //
    let buf = vec![b'A', b'B', b'C'];
    let r = test_memrchr_dbl(&buf, b'A', b'a');
    assert_eq!(r, Some(0));
    let r = test_memrchr_dbl(&buf, b'a', b'A');
    assert_eq!(r, Some(0));
    //
    let buf = vec![b'A', b'B', b'C'];
    let r = test_memrchr_dbl(&buf, b'B', b'b');
    assert_eq!(r, Some(1));
    let r = test_memrchr_dbl(&buf, b'b', b'B');
    assert_eq!(r, Some(1));
    //
    let buf = vec![b'A', b'B', b'C'];
    let r = test_memrchr_dbl(&buf, b'C', b'c');
    assert_eq!(r, Some(2));
    let r = test_memrchr_dbl(&buf, b'c', b'C');
    assert_eq!(r, Some(2));
    //
    let buf = vec![b'A', b'B', b'C'];
    let r = test_memrchr_dbl(&buf, b'a', b'b');
    assert_eq!(r, None);
    let r = test_memrchr_dbl(&buf, b'b', b'a');
    assert_eq!(r, None);
}
#[test]
fn test01() {
    let buf = vec![b'A', b'G', b'C', b'D', b'E', b'F', b'G', b'H', b'J', b'K'];
    //
    let r = test_memrchr_dbl(&buf, b'G', b'g');
    assert_eq!(r, Some(6));
    let r = test_memrchr_dbl(&buf, b'g', b'G');
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
            x86_alignment_check::ac_call_once(|| { test_memrchr_dbl(&buf[x..], b'G', b'g') })
        } else {
            test_memrchr_dbl(&buf[x..], b'G', b'g')
        });
        assert_eq!(r, Some(1 + x));
        //
        let r = cfg_iif::cfg_iif!(all(not(miri), feature = "test_alignment_check",
        any(target_arch = "x86_64", target_arch = "x86")) {
            x86_alignment_check::ac_call_once(|| { test_memrchr_dbl(&buf[x..], b'g', b'G') })
        } else {
            test_memrchr_dbl(&buf[x..], b'g', b'G')
        });
        assert_eq!(r, Some(1 + x));
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
