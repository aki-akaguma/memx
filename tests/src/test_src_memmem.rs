/**/

#[test]
fn test00() {
    let buf_0 = vec![];
    let pat_1 = vec![];
    let pat_2 = vec![b'A', b'B', b'C', b'D', b'E', b'F', b'G', b'H', b'J', b'0'];
    //
    let r = test_memmem(&buf_0, &pat_1);
    assert_eq!(r, Some(0));
    let r = test_memmem(&buf_0, &pat_2);
    assert_eq!(r, None);
    let r = test_memmem(&pat_2, &pat_1);
    assert_eq!(r, Some(0));
    //
    let buf = vec![b'A', b'B', b'C'];
    let r = test_memmem(&buf, "BC".as_bytes());
    assert_eq!(r, Some(1));
}
#[test]
fn test01() {
    let buf = vec![b'A', b'B', b'G', b'C', b'D', b'E', b'F', b'G', b'H', b'I', b'J', b'K'];
    let pat = vec![b'G', b'H', b'I'];
    let r = test_memmem(&buf, &pat);
    assert_eq!(r, Some(7));
    //
    let buf = vec![b'A', b'B', b'g', b'C', b'D', b'E', b'F', b'G', b'H', b'I', b'J', b'K'];
    let pat = vec![b'g', b'h', b'i'];
    let r = test_memmem(&buf, &pat);
    assert_eq!(r, None);
    //
    let buf = vec![b'A', b'B', b'g', b'C', b'D', b'E', b'F', b'G', b'H', b'I', b'J', b'g'];
    let pat = vec![b'g', b'h', b'i'];
    let r = test_memmem(&buf, &pat);
    assert_eq!(r, None);
    //
    let buf = vec![b'A', b'B', b'C', b'D', b'E', b'F', b'\xE3', b'\x81', b'\x82', b'G', b'H', b'I', b'J', b'K'];
    let pat = vec![b'\xE3', b'\x81', b'\x82'];
    let r = test_memmem(&buf, &pat);
    assert_eq!(r, Some(6));
    //
    let buf = vec![b'A', b'B', b'C', b'D', b'E', b'F', b'\xE3', b'\x81', b'\x82', b'G', b'H', b'I', b'J', b'G', b'K'];
    let pat = vec![b'\xE3', b'\x81', b'\x82', b'G'];
    let r = test_memmem(&buf, &pat);
    assert_eq!(r, Some(6));
    //
    let buf = vec![b'A', b'B', b'C', b'\x82', b'D', b'E', b'F', b'G', b'H', b'I', b'J', b'K', b'\x82'];
    let pat = vec![b'i', b'\xE3', b'\x81', b'\x82'];
    let r = test_memmem(&buf, &pat);
    assert_eq!(r, None);
}
#[test]
fn test02() {
    let buf_0 = vec![0_u8];
    let pat = vec![b'G', b'H', b'J'];
    let f = |x: usize| {
        let buf = {
            let mut buf: Vec<u8> = buf_0.repeat(1 + x);
            buf.append(&mut buf_0.repeat(x));
            buf.extend_from_slice(&pat);
            buf
        };
        //
        let r = cfg_iif::cfg_iif!(all(not(miri), feature = "test_alignment_check",
        any(target_arch = "x86_64", target_arch = "x86")) {
            if _RT_AC {
                x86_alignment_check::ac_call_once(|| { test_memmem(&buf[x..], &pat) })
            } else {
                test_memmem(&buf[x..], &pat)
            }
        } else {
            test_memmem(&buf[x..], &pat)
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
