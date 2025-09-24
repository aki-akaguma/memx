/**/

#[test]
fn test_memnechr_qpl_empty_buffer() {
    let buf = [];
    assert_eq!(test_memnechr_qpl(&buf, b'a', b'b', b'c', b'd'), None);
}

#[test]
fn test_memnechr_qpl_single_element_found() {
    let buf = [b'e'];
    assert_eq!(test_memnechr_qpl(&buf, b'a', b'b', b'c', b'd'), Some(0));
}

#[test]
fn test_memnechr_qpl_single_element_not_found_1() {
    let buf = [b'a'];
    assert_eq!(test_memnechr_qpl(&buf, b'a', b'b', b'c', b'd'), None);
}

#[test]
fn test_memnechr_qpl_single_element_not_found_2() {
    let buf = [b'b'];
    assert_eq!(test_memnechr_qpl(&buf, b'a', b'b', b'c', b'd'), None);
}

#[test]
fn test_memnechr_qpl_single_element_not_found_3() {
    let buf = [b'c'];
    assert_eq!(test_memnechr_qpl(&buf, b'a', b'b', b'c', b'd'), None);
}

#[test]
fn test_memnechr_qpl_single_element_not_found_4() {
    let buf = [b'd'];
    assert_eq!(test_memnechr_qpl(&buf, b'a', b'b', b'c', b'd'), None);
}

#[test]
fn test_memnechr_qpl_needle_at_start() {
    let buf = [b'e', b'a', b'b', b'c', b'd'];
    assert_eq!(test_memnechr_qpl(&buf, b'a', b'b', b'c', b'd'), Some(0));
}

#[test]
fn test_memnechr_qpl_needle_at_middle() {
    let buf = [b'a', b'e', b'b', b'c', b'd'];
    assert_eq!(test_memnechr_qpl(&buf, b'a', b'b', b'c', b'd'), Some(1));
}

#[test]
fn test_memnechr_qpl_needle_at_end() {
    let buf = [b'a', b'b', b'c', b'e', b'd'];
    assert_eq!(test_memnechr_qpl(&buf, b'a', b'b', b'c', b'd'), Some(3));
}

#[test]
fn test_memnechr_qpl_needle_not_found() {
    let buf = [b'a', b'b', b'c', b'd', b'a'];
    assert_eq!(test_memnechr_qpl(&buf, b'a', b'b', b'c', b'd'), None);
}

#[test]
fn test_memnechr_qpl_multiple_occurrences() {
    let buf = [b'a', b'e', b'b', b'f', b'c', b'g', b'd', b'h'];
    assert_eq!(test_memnechr_qpl(&buf, b'a', b'b', b'c', b'd'), Some(1));
}

#[test]
fn test_memnechr_qpl_long_buffer() {
    let buf = [b'a'; 1024];
    let mut expected_idx = None;
    let mut long_buf = buf.to_vec();
    for (i, item) in long_buf.iter_mut().enumerate().take(1024) {
        if i == 512 {
            *item = b'x';
            expected_idx = Some(i);
        }
    }
    assert_eq!(
        test_memnechr_qpl(&long_buf, b'a', b'b', b'c', b'd'),
        expected_idx
    );
}

#[test]
fn test_memnechr_qpl_long_buffer_not_found() {
    let buf = [b'a'; 1024];
    assert_eq!(test_memnechr_qpl(&buf, b'a', b'b', b'c', b'd'), None);
}
