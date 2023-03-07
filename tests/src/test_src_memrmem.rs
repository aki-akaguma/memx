/**/

#[test]
fn test00() {
    let buf_0 = vec![];
    let pat_1 = vec![];
    let pat_2 = vec![b'A', b'B', b'C', b'D', b'E', b'F', b'G', b'H', b'J', b'0'];
    //
    let r = test_memrmem(&buf_0, &pat_1);
    assert_eq!(r, Some(0));
    let r = test_memrmem(&buf_0, &pat_2);
    assert_eq!(r, None);
    let r = test_memrmem(&pat_2, &pat_1);
    assert_eq!(r, Some(10));
}
#[test]
fn test01() {
    let buf = vec![b'A', b'g', b'h', b'j', b'E', b'F', b'g', b'h', b'j', b'K'];
    let pat = vec![b'g', b'h', b'j'];
    //
    let r = test_memrmem(&buf, &pat);
    assert_eq!(r, Some(6));
}
#[test]
fn test02() {
    let buf_0 = vec![0_u8];
    let pat = vec![b'g', b'h', b'j'];
    let f = |x: usize| {
        let buf = {
            let mut buf = pat.clone();
            buf.extend_from_slice(&buf_0.repeat(1 + x));
            buf.extend_from_slice(&pat);
            buf
        };
        //
        let r = test_memrmem(&buf, &pat);
        assert_eq!(r, Some(pat.len() + 1 + x));
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
