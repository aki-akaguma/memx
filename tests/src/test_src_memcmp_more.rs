/**/

#[test]
fn test_memcmp_empty_buffers() {
    let buf1 = [];
    let buf2 = [];
    assert_eq!(test_memcmp(&buf1, &buf2), Ordering::Equal);
}

#[test]
fn test_memcmp_equal_buffers() {
    let buf1 = *b"abc";
    let buf2 = *b"abc";
    assert_eq!(test_memcmp(&buf1, &buf2), Ordering::Equal);
}

#[test]
fn test_memcmp_first_buffer_less() {
    let buf1 = *b"abc";
    let buf2 = *b"abd";
    assert_eq!(test_memcmp(&buf1, &buf2), Ordering::Less);
}

#[test]
fn test_memcmp_first_buffer_greater() {
    let buf1 = *b"abd";
    let buf2 = *b"abc";
    assert_eq!(test_memcmp(&buf1, &buf2), Ordering::Greater);
}

#[test]
fn test_memcmp_different_lengths_first_shorter() {
    let buf1 = *b"ab";
    let buf2 = *b"abc";
    assert_eq!(test_memcmp(&buf1, &buf2), Ordering::Less);
}

#[test]
fn test_memcmp_different_lengths_first_longer() {
    let buf1 = *b"abc";
    let buf2 = *b"ab";
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
