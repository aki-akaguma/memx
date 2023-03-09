/**/

#[test]
fn test00() {
    let buf = vec![];
    let r = test_memrnechr(&buf, b'G');
    assert_eq!(r, None);
    //
    let buf = vec![b'A', b'G'];
    let r = test_memrnechr(&buf, b'g');
    assert_eq!(r, Some(1));
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
            buf.push(b'G');
            buf.append(&mut buf_0.repeat(1 + x));
            buf
        };
        //
        let r = test_memrnechr(&buf, 0_u8);
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