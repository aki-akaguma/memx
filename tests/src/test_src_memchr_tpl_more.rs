/**/

#[test]
fn test_memchr_tpl_empty_buffer() {
    let buf = [];
    assert_eq!(test_memchr_tpl(&buf, b'a', b'b', b'c'), None);
}

#[test]
fn test_memchr_tpl_single_element_found_1() {
    let buf = *b"a";
    assert_eq!(test_memchr_tpl(&buf, b'a', b'b', b'c'), Some(0));
}

#[test]
fn test_memchr_tpl_single_element_found_2() {
    let buf = *b"b";
    assert_eq!(test_memchr_tpl(&buf, b'a', b'b', b'c'), Some(0));
}

#[test]
fn test_memchr_tpl_single_element_found_3() {
    let buf = *b"c";
    assert_eq!(test_memchr_tpl(&buf, b'a', b'b', b'c'), Some(0));
}

#[test]
fn test_memchr_tpl_single_element_not_found() {
    let buf = *b"d";
    assert_eq!(test_memchr_tpl(&buf, b'a', b'b', b'c'), None);
}

#[test]
fn test_memchr_tpl_needle_at_start() {
    let buf = *b"ade";
    assert_eq!(test_memchr_tpl(&buf, b'a', b'b', b'c'), Some(0));
}

#[test]
fn test_memchr_tpl_needle_at_middle() {
    let buf = *b"dbe";
    assert_eq!(test_memchr_tpl(&buf, b'a', b'b', b'c'), Some(1));
}

#[test]
fn test_memchr_tpl_needle_at_end() {
    let buf = *b"dec";
    assert_eq!(test_memchr_tpl(&buf, b'a', b'b', b'c'), Some(2));
}

#[test]
fn test_memchr_tpl_needle_not_found() {
    let buf = *b"def";
    assert_eq!(test_memchr_tpl(&buf, b'a', b'b', b'c'), None);
}

#[test]
fn test_memchr_tpl_multiple_occurrences() {
    let buf = *b"adbec";
    assert_eq!(test_memchr_tpl(&buf, b'a', b'b', b'c'), Some(0));
}

#[test]
fn test_memchr_tpl_long_buffer() {
    let buf = [0; 1024];
    let mut expected_idx = None;
    let mut long_buf = buf.to_vec();
    for (i, item) in long_buf.iter_mut().enumerate().take(1024) {
        if i == 512 {
            *item = b'x';
            expected_idx = Some(i);
        }
    }
    assert_eq!(test_memchr_tpl(&long_buf, b'x', b'y', b'z'), expected_idx);
}

#[test]
fn test_memchr_tpl_long_buffer_not_found() {
    let buf = [0; 1024];
    assert_eq!(test_memchr_tpl(&buf, b'x', b'y', b'z'), None);
}
