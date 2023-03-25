/**/

#[test]
fn test00() {
    let buf = vec![];
    let r = test_memchr_double(&buf, b'G', b'g');
    assert_eq!(r, None);
    //
    let buf = vec![b'A', b'g'];
    let r = test_memchr_double(&buf, b'G', b'g');
    assert_eq!(r, Some(1));
    let r = test_memchr_double(&buf, b'g', b'G');
    assert_eq!(r, Some(1));
    //
    let buf = vec![b'A', b'B', b'C'];
    let r = test_memchr_double(&buf, b'A', b'a');
    assert_eq!(r, Some(0));
    let r = test_memchr_double(&buf, b'a', b'A');
    assert_eq!(r, Some(0));
    //
    let buf = vec![b'A', b'B', b'C'];
    let r = test_memchr_double(&buf, b'B', b'b');
    assert_eq!(r, Some(1));
    let r = test_memchr_double(&buf, b'b', b'B');
    assert_eq!(r, Some(1));
    //
    let buf = vec![b'A', b'B', b'C'];
    let r = test_memchr_double(&buf, b'C', b'c');
    assert_eq!(r, Some(2));
    let r = test_memchr_double(&buf, b'c', b'C');
    assert_eq!(r, Some(2));
    //
    let buf = vec![b'A', b'B', b'C'];
    let r = test_memchr_double(&buf, b'a', b'b');
    assert_eq!(r, None);
    let r = test_memchr_double(&buf, b'b', b'a');
    assert_eq!(r, None);
}
#[test]
fn test01() {
    let buf = vec![b'A', b'B', b'C', b'D', b'E', b'F', b'G', b'H', b'J', b'K'];
    //
    let r = test_memchr_double(&buf, b'G', b'g');
    assert_eq!(r, Some(6));
    let r = test_memchr_double(&buf, b'g', b'G');
    assert_eq!(r, Some(6));
}
#[test]
fn test02() {
    let buf_0 = vec![0_u8];
    let f = |x: usize| {
        let buf = {
            let mut buf: Vec<u8> = buf_0.repeat(1 + x);
            buf.push(b'G');
            buf
        };
        //
        let r = test_memchr_double(&buf, b'G', b'g');
        assert_eq!(r, Some(1 + x));
        let r = test_memchr_double(&buf, b'g', b'G');
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
