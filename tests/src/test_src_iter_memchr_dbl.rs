/**/

#[test]
fn test01() {
    let buf = vec![
        b'A', b'B', b'g', b'D', b'E', b'F', b'G', b'H', b'J', b'K', b'g',
    ];
    //
    let mut iter = test_memchr_dbl_iter(&buf, b'G', b'g');
    assert_eq!(iter.size_hint(), (0, Some(11)));
    assert_eq!(iter.next(), Some(2));
    assert_eq!(iter.next(), Some(6));
    assert_eq!(iter.next(), Some(10));
    assert_eq!(iter.next(), None);
    //
    let mut iter = test_memchr_dbl_iter(&buf, b'G', b'g');
    assert_eq!(iter.next(), Some(2));
    assert_eq!(iter.next(), Some(6));
    assert_eq!(iter.next_back(), Some(2));
    assert_eq!(iter.next_back(), None);
    assert_eq!(iter.next(), Some(2));
    assert_eq!(iter.next(), Some(6));
    assert_eq!(iter.next(), Some(10));
    assert_eq!(iter.next(), None);
    assert_eq!(iter.next(), None);
    assert_eq!(iter.next_back(), Some(10));
    assert_eq!(iter.next_back(), Some(6));
    assert_eq!(iter.next_back(), Some(2));
    assert_eq!(iter.next_back(), None);
    assert_eq!(iter.next_back(), None);
}
#[test]
fn test02() {
    let buf_0 = vec![0_u8];
    let f = |x: usize| {
        let buf = {
            let mut buf: Vec<u8> = buf_0.repeat(1 + x);
            buf.push(b'g');
            buf
        };
        //
        let mut iter = test_memchr_dbl_iter(&buf, b'G', b'g');
        assert_eq!(iter.next(), Some(1 + x));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next_back(), Some(1 + x));
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
