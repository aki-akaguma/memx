/**/

#[test]
fn test00() {
    let buf = vec![];
    let mut iter = test_memrchr_qpl_iter(&buf, b'Y', b'X', b'G', b'g');
    assert_eq!(iter.size_hint(), (0, Some(0)));
    assert_eq!(iter.next(), None);
    assert_eq!(iter.next_back(), None);
    //
    let buf = vec![b'A', b'g'];
    let mut iter = test_memrchr_qpl_iter(&buf, b'Y', b'X', b'G', b'g');
    assert_eq!(iter.size_hint(), (0, Some(2)));
    assert_eq!(iter.next(), Some(1));
    assert_eq!(iter.next(), None);
    assert_eq!(iter.next_back(), Some(1));
    assert_eq!(iter.next_back(), None);
    let mut iter = test_memrchr_qpl_iter(&buf, b'Y', b'X', b'g', b'G');
    assert_eq!(iter.size_hint(), (0, Some(2)));
    assert_eq!(iter.next(), Some(1));
    assert_eq!(iter.next(), None);
    assert_eq!(iter.next_back(), Some(1));
    assert_eq!(iter.next_back(), None);
    //
    let buf = vec![b'A', b'B', b'C'];
    let mut iter = test_memrchr_qpl_iter(&buf, b'Y', b'X', b'A', b'a');
    assert_eq!(iter.size_hint(), (0, Some(3)));
    assert_eq!(iter.next(), Some(0));
    assert_eq!(iter.next(), None);
    assert_eq!(iter.next_back(), Some(0));
    assert_eq!(iter.next_back(), None);
    let mut iter = test_memrchr_qpl_iter(&buf, b'Y', b'X', b'a', b'A');
    assert_eq!(iter.size_hint(), (0, Some(3)));
    assert_eq!(iter.next(), Some(0));
    assert_eq!(iter.next(), None);
    assert_eq!(iter.next_back(), Some(0));
    assert_eq!(iter.next_back(), None);
    //
    let buf = vec![b'A', b'B', b'C'];
    let mut iter = test_memrchr_qpl_iter(&buf, b'Y', b'X', b'B', b'b');
    assert_eq!(iter.size_hint(), (0, Some(3)));
    assert_eq!(iter.next(), Some(1));
    assert_eq!(iter.next(), None);
    assert_eq!(iter.next_back(), Some(1));
    assert_eq!(iter.next_back(), None);
    let mut iter = test_memrchr_qpl_iter(&buf, b'Y', b'X', b'b', b'B');
    assert_eq!(iter.size_hint(), (0, Some(3)));
    assert_eq!(iter.next(), Some(1));
    assert_eq!(iter.next(), None);
    assert_eq!(iter.next_back(), Some(1));
    assert_eq!(iter.next_back(), None);
    //
    let buf = vec![b'A', b'B', b'C'];
    let mut iter = test_memrchr_qpl_iter(&buf, b'Y', b'X', b'C', b'c');
    assert_eq!(iter.size_hint(), (0, Some(3)));
    assert_eq!(iter.next(), Some(2));
    assert_eq!(iter.next(), None);
    assert_eq!(iter.next_back(), Some(2));
    assert_eq!(iter.next_back(), None);
    let mut iter = test_memrchr_qpl_iter(&buf, b'Y', b'X', b'c', b'C');
    assert_eq!(iter.size_hint(), (0, Some(3)));
    assert_eq!(iter.next(), Some(2));
    assert_eq!(iter.next(), None);
    assert_eq!(iter.next_back(), Some(2));
    assert_eq!(iter.next_back(), None);
    //
    let buf = vec![b'A', b'B', b'C'];
    let mut iter = test_memrchr_qpl_iter(&buf, b'Y', b'X', b'a', b'b');
    assert_eq!(iter.size_hint(), (0, Some(3)));
    assert_eq!(iter.next(), None);
    assert_eq!(iter.next_back(), None);
    let mut iter = test_memrchr_qpl_iter(&buf, b'Y', b'X', b'b', b'a');
    assert_eq!(iter.size_hint(), (0, Some(3)));
    assert_eq!(iter.next(), None);
    assert_eq!(iter.next_back(), None);
}
#[test]
fn test01() {
    let buf = vec![
        b'A', b'B', b'g', b'D', b'E', b'F', b'G', b'H', b'J', b'K', b'g',
    ];
    //
    let mut iter = test_memrchr_qpl_iter(&buf, b'Y', b'X', b'G', b'g');
    assert_eq!(iter.size_hint(), (0, Some(11)));
    assert_eq!(iter.next(), Some(10));
    assert_eq!(iter.next(), Some(6));
    assert_eq!(iter.next(), Some(2));
    assert_eq!(iter.next(), None);
    //
    let mut iter = test_memrchr_qpl_iter(&buf, b'Y', b'X', b'G', b'g');
    assert_eq!(iter.next(), Some(10));
    assert_eq!(iter.next(), Some(6));
    assert_eq!(iter.next_back(), Some(10));
    assert_eq!(iter.next_back(), None);
    assert_eq!(iter.next(), Some(10));
    assert_eq!(iter.next(), Some(6));
    assert_eq!(iter.next(), Some(2));
    assert_eq!(iter.next(), None);
    assert_eq!(iter.next(), None);
    assert_eq!(iter.next_back(), Some(2));
    assert_eq!(iter.next_back(), Some(6));
    assert_eq!(iter.next_back(), Some(10));
    assert_eq!(iter.next_back(), None);
    assert_eq!(iter.next_back(), None);
}
#[test]
fn test02() {
    let buf_g = vec![b'g'];
    let buf_0 = [0_u8];
    let f = |x: usize| {
        let buf = {
            let mut buf: Vec<u8> = buf_g.clone();
            buf.append(&mut buf_0.repeat(x));
            buf.push(b'g');
            buf.append(&mut buf_0.repeat(1 + x));
            buf
        };
        //
        let mut iter = test_memrchr_qpl_iter(&buf, b'Y', b'X', b'G', b'g');
        assert_eq!(iter.next(), Some(1 + x));
        assert_eq!(iter.next(), Some(0));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next_back(), Some(0));
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
