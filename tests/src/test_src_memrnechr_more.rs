/**/

#[test]
fn test_memrnechr_empty_buffer() {
    let buf = [];
    assert_eq!(test_memrnechr(&buf, b'a'), None);
}

#[test]
fn test_memrnechr_single_element_found() {
    let buf = *b"b";
    assert_eq!(test_memrnechr(&buf, b'a'), Some(0));
}

#[test]
fn test_memrnechr_single_element_not_found() {
    let buf = *b"a";
    assert_eq!(test_memrnechr(&buf, b'a'), None);
}

#[test]
fn test_memrnechr_needle_at_start() {
    let buf = *b"baa";
    assert_eq!(test_memrnechr(&buf, b'a'), Some(0));
}

#[test]
fn test_memrnechr_needle_at_middle() {
    let buf = *b"aba";
    assert_eq!(test_memrnechr(&buf, b'a'), Some(1));
}

#[test]
fn test_memrnechr_needle_at_end() {
    let buf = *b"aab";
    assert_eq!(test_memrnechr(&buf, b'a'), Some(2));
}

#[test]
fn test_memrnechr_needle_not_found() {
    let buf = *b"aaa";
    assert_eq!(test_memrnechr(&buf, b'a'), None);
}

#[test]
fn test_memrnechr_multiple_occurrences() {
    let buf = *b"abaca";
    assert_eq!(test_memrnechr(&buf, b'a'), Some(3));
}

#[test]
fn test_memrnechr_long_buffer() {
    let buf = [b'a'; 1024];
    let mut expected_idx = None;
    let mut long_buf = buf.to_vec();
    for (i, item) in long_buf.iter_mut().enumerate().take(1024) {
        if i == 512 {
            *item = b'x';
            expected_idx = Some(i);
        }
    }
    assert_eq!(test_memrnechr(&long_buf, b'a'), expected_idx);
}

#[test]
fn test_memrnechr_long_buffer_not_found() {
    let buf = [b'a'; 1024];
    assert_eq!(test_memrnechr(&buf, b'a'), None);
}
