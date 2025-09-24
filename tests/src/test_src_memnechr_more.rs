/**/

#[test]
fn test_memnechr_empty_buffer() {
    let buf = [];
    assert_eq!(test_memnechr(&buf, b'a'), None);
}

#[test]
fn test_memnechr_single_element_found() {
    let buf = [b'b'];
    assert_eq!(test_memnechr(&buf, b'a'), Some(0));
}

#[test]
fn test_memnechr_single_element_not_found() {
    let buf = [b'a'];
    assert_eq!(test_memnechr(&buf, b'a'), None);
}

#[test]
fn test_memnechr_needle_at_start() {
    let buf = [b'b', b'a', b'a'];
    assert_eq!(test_memnechr(&buf, b'a'), Some(0));
}

#[test]
fn test_memnechr_needle_at_middle() {
    let buf = [b'a', b'b', b'a'];
    assert_eq!(test_memnechr(&buf, b'a'), Some(1));
}

#[test]
fn test_memnechr_needle_at_end() {
    let buf = [b'a', b'a', b'b'];
    assert_eq!(test_memnechr(&buf, b'a'), Some(2));
}

#[test]
fn test_memnechr_needle_not_found() {
    let buf = [b'a', b'a', b'a'];
    assert_eq!(test_memnechr(&buf, b'a'), None);
}

#[test]
fn test_memnechr_multiple_occurrences() {
    let buf = [b'a', b'b', b'a', b'c', b'a'];
    assert_eq!(test_memnechr(&buf, b'a'), Some(1));
}

#[test]
fn test_memnechr_long_buffer() {
    let buf = [b'a'; 1024];
    let mut expected_idx = None;
    let mut long_buf = buf.to_vec();
    for i in 0..1024 {
        if i == 512 {
            long_buf[i] = b'x';
            expected_idx = Some(i);
        }
    }
    assert_eq!(test_memnechr(&long_buf, b'a'), expected_idx);
}

#[test]
fn test_memnechr_long_buffer_not_found() {
    let buf = [b'a'; 1024];
    assert_eq!(test_memnechr(&buf, b'a'), None);
}
