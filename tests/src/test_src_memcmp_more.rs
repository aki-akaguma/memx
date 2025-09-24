/**/

#[test]
fn test_memcmp_empty_buffers() {
    let buf1 = [];
    let buf2 = [];
    assert_eq!(test_memcmp(&buf1, &buf2), Ordering::Equal);
}

#[test]
fn test_memcmp_equal_buffers() {
    let buf1 = [b'a', b'b', b'c'];
    let buf2 = [b'a', b'b', b'c'];
    assert_eq!(test_memcmp(&buf1, &buf2), Ordering::Equal);
}

#[test]
fn test_memcmp_first_buffer_less() {
    let buf1 = [b'a', b'b', b'c'];
    let buf2 = [b'a', b'b', b'd'];
    assert_eq!(test_memcmp(&buf1, &buf2), Ordering::Less);
}

#[test]
fn test_memcmp_first_buffer_greater() {
    let buf1 = [b'a', b'b', b'd'];
    let buf2 = [b'a', b'b', b'c'];
    assert_eq!(test_memcmp(&buf1, &buf2), Ordering::Greater);
}

#[test]
fn test_memcmp_different_lengths_first_shorter() {
    let buf1 = [b'a', b'b'];
    let buf2 = [b'a', b'b', b'c'];
    assert_eq!(test_memcmp(&buf1, &buf2), Ordering::Less);
}

#[test]
fn test_memcmp_different_lengths_first_longer() {
    let buf1 = [b'a', b'b', b'c'];
    let buf2 = [b'a', b'b'];
    assert_eq!(test_memcmp(&buf1, &buf2), Ordering::Greater);
}

#[test]
fn test_memcmp_long_equal_buffers() {
    let buf1 = [0; 1024];
    let buf2 = [0; 1024];
    assert_eq!(test_memcmp(&buf1, &buf2), Ordering::Equal);
}

#[test]
fn test_memcmp_long_different_buffers() {
    let buf1 = [0; 1024];
    let mut buf2 = [0; 1024];
    buf2[512] = 1;
    assert_eq!(test_memcmp(&buf1, &buf2), Ordering::Less);
}
