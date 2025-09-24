/**/

#[test]
fn test_memchr_empty_buffer() {
    let buf = [];
    assert_eq!(test_memchr(&buf, b'a'), None);
}

#[test]
fn test_memchr_single_element_found() {
    let buf = [b'a'];
    assert_eq!(test_memchr(&buf, b'a'), Some(0));
}

#[test]
fn test_memchr_single_element_not_found() {
    let buf = [b'b'];
    assert_eq!(test_memchr(&buf, b'a'), None);
}

#[test]
fn test_memchr_needle_at_start() {
    let buf = [b'a', b'b', b'c'];
    assert_eq!(test_memchr(&buf, b'a'), Some(0));
}

#[test]
fn test_memchr_needle_at_middle() {
    let buf = [b'a', b'b', b'c'];
    assert_eq!(test_memchr(&buf, b'b'), Some(1));
}

#[test]
fn test_memchr_needle_at_end() {
    let buf = [b'a', b'b', b'c'];
    assert_eq!(test_memchr(&buf, b'c'), Some(2));
}

#[test]
fn test_memchr_needle_not_found() {
    let buf = [b'a', b'b', b'c'];
    assert_eq!(test_memchr(&buf, b'd'), None);
}

#[test]
fn test_memchr_multiple_occurrences() {
    let buf = [b'a', b'b', b'a', b'c', b'a'];
    assert_eq!(test_memchr(&buf, b'a'), Some(0));
}

#[test]
fn test_memchr_long_buffer() {
    let buf = [0; 1024];
    let mut expected_idx = None;
    let mut long_buf = buf.to_vec();
    for (i, item) in long_buf.iter_mut().enumerate().take(1024) {
        if i == 512 {
            *item = b'x';
            expected_idx = Some(i);
        }
    }
    assert_eq!(test_memchr(&long_buf, b'x'), expected_idx);
}

#[test]
fn test_memchr_long_buffer_not_found() {
    let buf = [0; 1024];
    assert_eq!(test_memchr(&buf, b'x'), None);
}
