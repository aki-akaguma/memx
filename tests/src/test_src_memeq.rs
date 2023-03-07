/**/

#[test]
fn test00() {
    let buf_0 = vec![];
    let pat_1 = vec![];
    let pat_2 = vec![b'A', b'B', b'C', b'D', b'E', b'F', b'G', b'H', b'J', b'0'];
    //
    assert!(test_memeq(&buf_0, &pat_1));
    assert!(!test_memeq(&buf_0, &pat_2));
}
#[test]
fn test01() {
    let buf_0 = vec![b'A', b'B', b'C', b'D', b'E', b'F', b'G', b'H', b'J', b'5'];
    let pat_1 = vec![b'A', b'B', b'C', b'D', b'E', b'F', b'G', b'H', b'J', b'9'];
    let pat_2 = vec![b'A', b'B', b'C', b'D', b'E', b'F', b'G', b'H', b'J', b'0'];
    //
    let r = test_memeq(&buf_0[0..(buf_0.len() - 1)], &pat_1[0..(pat_1.len() - 1)]);
    assert!(r);
    let r = test_memeq(&buf_0, &pat_1);
    assert!(!r);
    let r = test_memeq(&buf_0, &pat_2);
    assert!(!r);
}
#[test]
fn test02() {
    let buf_0 = vec![0_u8];
    let f = |x: usize| {
        let buf_1 = {
            let buf: Vec<u8> = buf_0.repeat(1 + x);
            buf
        };
        //
        let mut buf = buf_1.clone();
        buf.push(b'5');
        let mut pat = buf_1.clone();
        pat.push(b'5');
        let r = test_memeq(&buf, &pat);
        assert!(r);
        //
        let mut buf = buf_1.clone();
        buf.push(b'5');
        let mut pat = buf_1.clone();
        pat.push(b'9');
        let r = test_memeq(&buf, &pat);
        assert!(!r);
        //
        let mut buf = buf_1.clone();
        buf.push(b'5');
        let mut pat = buf_1.clone();
        pat.push(b'0');
        let r = test_memeq(&buf, &pat);
        assert!(!r);
        //
        let buf = buf_1.clone();
        let mut pat = buf_1.clone();
        pat.push(b'0');
        let r = test_memeq(&buf, &pat);
        assert!(!r);
        //
        let mut buf = buf_1.clone();
        buf.push(b'5');
        let pat = buf_1;
        let r = test_memeq(&buf, &pat);
        assert!(!r);
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
