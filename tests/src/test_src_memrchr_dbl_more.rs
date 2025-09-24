/**/

#[test]
fn test_memrchr_dbl_empty_buffer() {
    let buf = [];
    assert_eq!(test_memrchr_dbl(&buf, b'a', b'b'), None);
}

#[test]
fn test_memrchr_dbl_single_element_found_1() {
    let buf = [b'a'];
    assert_eq!(test_memrchr_dbl(&buf, b'a', b'b'), Some(0));
}

#[test]
fn test_memrchr_dbl_single_element_found_2() {
    let buf = [b'b'];
    assert_eq!(test_memrchr_dbl(&buf, b'a', b'b'), Some(0));
}

#[test]
fn test_memrchr_dbl_single_element_not_found() {
    let buf = [b'c'];
    assert_eq!(test_memrchr_dbl(&buf, b'a', b'b'), None);
}

#[test]
fn test_memrchr_dbl_needle_at_start() {
    let buf = [b'a', b'c', b'd'];
    assert_eq!(test_memrchr_dbl(&buf, b'a', b'b'), Some(0));
}

#[test]
fn test_memrchr_dbl_needle_at_middle() {
    let buf = [b'c', b'b', b'd'];
    assert_eq!(test_memrchr_dbl(&buf, b'a', b'b'), Some(1));
}

#[test]
fn test_memrchr_dbl_needle_at_end() {
    let buf = [b'c', b'd', b'b'];
    assert_eq!(test_memrchr_dbl(&buf, b'a', b'b'), Some(2));
}

#[test]
fn test_memrchr_dbl_needle_not_found() {
    let buf = [b'c', b'd', b'e'];
    assert_eq!(test_memrchr_dbl(&buf, b'a', b'b'), None);
}

#[test]
fn test_memrchr_dbl_multiple_occurrences() {
    let buf = [b'a', b'c', b'b', b'd', b'a'];
    assert_eq!(test_memrchr_dbl(&buf, b'a', b'b'), Some(4));
}

#[test]
fn test_memrchr_dbl_long_buffer() {
    let buf = [0; 1024];
    let mut expected_idx = None;
    let mut long_buf = buf.to_vec();
    for i in 0..1024 {
        if i == 512 {
            long_buf[i] = b'x';
            expected_idx = Some(i);
        }
    }
    assert_eq!(test_memrchr_dbl(&long_buf, b'x', b'y'), expected_idx);
}

#[test]
fn test_memrchr_dbl_long_buffer_not_found() {
    let buf = [0; 1024];
    assert_eq!(test_memrchr_dbl(&buf, b'x', b'y'), None);
}
