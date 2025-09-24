/**/

#[test]
fn test_memnechr_dbl_empty_buffer() {
    let buf = [];
    assert_eq!(test_memnechr_dbl(&buf, b'a', b'b'), None);
}

#[test]
fn test_memnechr_dbl_single_element_found() {
    let buf = [b'c'];
    assert_eq!(test_memnechr_dbl(&buf, b'a', b'b'), Some(0));
}

#[test]
fn test_memnechr_dbl_single_element_not_found_1() {
    let buf = [b'a'];
    assert_eq!(test_memnechr_dbl(&buf, b'a', b'b'), None);
}

#[test]
fn test_memnechr_dbl_single_element_not_found_2() {
    let buf = [b'b'];
    assert_eq!(test_memnechr_dbl(&buf, b'a', b'b'), None);
}

#[test]
fn test_memnechr_dbl_needle_at_start() {
    let buf = [b'c', b'a', b'b'];
    assert_eq!(test_memnechr_dbl(&buf, b'a', b'b'), Some(0));
}

#[test]
fn test_memnechr_dbl_needle_at_middle() {
    let buf = [b'a', b'c', b'b'];
    assert_eq!(test_memnechr_dbl(&buf, b'a', b'b'), Some(1));
}

#[test]
fn test_memnechr_dbl_needle_at_end() {
    let buf = [b'a', b'b', b'c'];
    assert_eq!(test_memnechr_dbl(&buf, b'a', b'b'), Some(2));
}

#[test]
fn test_memnechr_dbl_needle_not_found() {
    let buf = [b'a', b'b', b'a'];
    assert_eq!(test_memnechr_dbl(&buf, b'a', b'b'), None);
}

#[test]
fn test_memnechr_dbl_multiple_occurrences() {
    let buf = [b'a', b'c', b'b', b'd', b'a'];
    assert_eq!(test_memnechr_dbl(&buf, b'a', b'b'), Some(1));
}

#[test]
fn test_memnechr_dbl_long_buffer() {
    let buf = [b'a'; 1024];
    let mut expected_idx = None;
    let mut long_buf = buf.to_vec();
    for (i, item) in long_buf.iter_mut().enumerate().take(1024) {
        if i == 512 {
            *item = b'x';
            expected_idx = Some(i);
        }
    }
    assert_eq!(test_memnechr_dbl(&long_buf, b'a', b'b'), expected_idx);
}

#[test]
fn test_memnechr_dbl_long_buffer_not_found() {
    let buf = [b'a'; 1024];
    assert_eq!(test_memnechr_dbl(&buf, b'a', b'b'), None);
}
