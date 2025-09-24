/**/

#[test]
fn test_memrchr_qpl_empty_buffer() {
    let buf = [];
    assert_eq!(test_memrchr_qpl(&buf, b'a', b'b', b'c', b'd'), None);
}

#[test]
fn test_memrchr_qpl_single_element_found_1() {
    let buf = [b'a'];
    assert_eq!(test_memrchr_qpl(&buf, b'a', b'b', b'c', b'd'), Some(0));
}

#[test]
fn test_memrchr_qpl_single_element_found_2() {
    let buf = [b'b'];
    assert_eq!(test_memrchr_qpl(&buf, b'a', b'b', b'c', b'd'), Some(0));
}

#[test]
fn test_memrchr_qpl_single_element_found_3() {
    let buf = [b'c'];
    assert_eq!(test_memrchr_qpl(&buf, b'a', b'b', b'c', b'd'), Some(0));
}

#[test]
fn test_memrchr_qpl_single_element_found_4() {
    let buf = [b'd'];
    assert_eq!(test_memrchr_qpl(&buf, b'a', b'b', b'c', b'd'), Some(0));
}

#[test]
fn test_memrchr_qpl_single_element_not_found() {
    let buf = [b'e'];
    assert_eq!(test_memrchr_qpl(&buf, b'a', b'b', b'c', b'd'), None);
}

#[test]
fn test_memrchr_qpl_needle_at_start() {
    let buf = [b'a', b'e', b'f'];
    assert_eq!(test_memrchr_qpl(&buf, b'a', b'b', b'c', b'd'), Some(0));
}

#[test]
fn test_memrchr_qpl_needle_at_middle() {
    let buf = [b'e', b'b', b'f'];
    assert_eq!(test_memrchr_qpl(&buf, b'a', b'b', b'c', b'd'), Some(1));
}

#[test]
fn test_memrchr_qpl_needle_at_end() {
    let buf = [b'e', b'f', b'c'];
    assert_eq!(test_memrchr_qpl(&buf, b'a', b'b', b'c', b'd'), Some(2));
}

#[test]
fn test_memrchr_qpl_needle_not_found() {
    let buf = [b'e', b'f', b'g'];
    assert_eq!(test_memrchr_qpl(&buf, b'a', b'b', b'c', b'd'), None);
}

#[test]
fn test_memrchr_qpl_multiple_occurrences() {
    let buf = [b'a', b'e', b'b', b'f', b'c', b'g', b'd'];
    assert_eq!(test_memrchr_qpl(&buf, b'a', b'b', b'c', b'd'), Some(6));
}

#[test]
fn test_memrchr_qpl_long_buffer() {
    let buf = [0; 1024];
    let mut expected_idx = None;
    let mut long_buf = buf.to_vec();
    for i in 0..1024 {
        if i == 512 {
            long_buf[i] = b'x';
            expected_idx = Some(i);
        }
    }
    assert_eq!(
        test_memrchr_qpl(&long_buf, b'x', b'y', b'z', b'w'),
        expected_idx
    );
}

#[test]
fn test_memrchr_qpl_long_buffer_not_found() {
    let buf = [0; 1024];
    assert_eq!(test_memrchr_qpl(&buf, b'x', b'y', b'z', b'w'), None);
}
