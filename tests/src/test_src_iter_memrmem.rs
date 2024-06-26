/**/

#[test]
fn test01() {
    let buf = vec![
        b'A', b'B', b'g', b'h', b'j', b'F', b'g', b'h', b'j', b'K', b'g',
    ];
    let pat = vec![b'g', b'h', b'j'];
    //
    let mut iter = test_memrmem_iter(&buf, &pat);
    assert_eq!(iter.size_hint(), (0, Some(11)));
    assert_eq!(iter.next(), Some(6));
    assert_eq!(iter.next(), Some(2));
    assert_eq!(iter.next(), None);
    //
    let mut iter = test_memrmem_iter(&buf, &pat);
    assert_eq!(iter.next(), Some(6));
    assert_eq!(iter.next(), Some(2));
    assert_eq!(iter.next_back(), Some(6));
    assert_eq!(iter.next_back(), None);
    assert_eq!(iter.next(), Some(6));
    assert_eq!(iter.next(), Some(2));
    assert_eq!(iter.next(), None);
    assert_eq!(iter.next(), None);
    assert_eq!(iter.next_back(), Some(2));
    assert_eq!(iter.next_back(), Some(6));
    assert_eq!(iter.next_back(), None);
    assert_eq!(iter.next_back(), None);
}
#[test]
fn test02() {
    let buf_0 = [0_u8];
    let pat = vec![b'g', b'h', b'j'];
    let f = |x: usize| {
        let buf = {
            let mut buf: Vec<u8> = buf_0.repeat(1 + x);
            buf.extend_from_slice(&pat);
            buf
        };
        //
        let mut iter = test_memrmem_iter(&buf, &pat);
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
