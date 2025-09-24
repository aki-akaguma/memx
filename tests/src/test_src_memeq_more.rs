/**/

#[test]
fn test_memeq_empty_buffers() {
    let buf1 = [];
    let buf2 = [];
    assert!(test_memeq(&buf1, &buf2));
}

#[test]
fn test_memeq_equal_buffers() {
    let buf1 = [b'a', b'b', b'c'];
    let buf2 = [b'a', b'b', b'c'];
    assert!(test_memeq(&buf1, &buf2));
}

#[test]
fn test_memeq_different_buffers() {
    let buf1 = [b'a', b'b', b'c'];
    let buf2 = [b'a', b'b', b'd'];
    assert!(!test_memeq(&buf1, &buf2));
}

#[test]
fn test_memeq_different_lengths() {
    let buf1 = [b'a', b'b'];
    let buf2 = [b'a', b'b', b'c'];
    assert!(!test_memeq(&buf1, &buf2));
}

#[test]
fn test_memeq_long_equal_buffers() {
    let buf1 = [0; 1024];
    let buf2 = [0; 1024];
    assert!(test_memeq(&buf1, &buf2));
}

#[test]
fn test_memeq_long_different_buffers() {
    let buf1 = [0; 1024];
    let mut buf2 = [0; 1024];
    buf2[512] = 1;
    assert!(!test_memeq(&buf1, &buf2));
}
