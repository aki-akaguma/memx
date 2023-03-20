/**/

#[test]
fn test00() {
    let buf = vec![];
    let r = test_memnechr(&buf, b'G');
    assert_eq!(r, None);
    //
    let buf = vec![b'g'];
    let r = test_memnechr(&buf, b'G');
    assert_eq!(r, Some(0));
    //
    let buf = vec![b'a', b'A', b'A'];
    let r = test_memnechr(&buf, b'A');
    assert_eq!(r, Some(0));
    //
    let buf = vec![b'B', b'b', b'B'];
    let r = test_memnechr(&buf, b'B');
    assert_eq!(r, Some(1));
    //
    let buf = vec![b'C', b'C', b'a'];
    let r = test_memnechr(&buf, b'C');
    assert_eq!(r, Some(2));
    //
    let buf = vec![b'A', b'A', b'A'];
    let r = test_memnechr(&buf, b'A');
    assert_eq!(r, None);
}
#[test]
fn test01() {
    let buf = vec![b'A', b'B', b'C', b'D', b'E', b'F', b'G', b'H', b'J', b'K'];
    //
    let r = test_memnechr(&buf, b'G');
    assert_eq!(r, Some(0));
    //
    let r = test_memnechr(&buf, b'A');
    assert_eq!(r, Some(1));
}
#[test]
fn test02() {
    let buf_0 = vec![b' '];
    let f = |x: usize| {
        let buf = {
            let mut buf: Vec<u8> = buf_0.repeat(1 + x);
            buf.push(b'G');
            buf
        };
        //
        let r = test_memnechr(&buf, b' ');
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
