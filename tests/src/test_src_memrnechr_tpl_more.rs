/**/

#[test]
fn test_memrnechr_tpl_empty_buffer() {
    let buf = [];
    assert_eq!(test_memrnechr_tpl(&buf, b'a', b'b', b'c'), None);
}

#[test]
fn test_memrnechr_tpl_single_element_found() {
    let buf = [b'd'];
    assert_eq!(test_memrnechr_tpl(&buf, b'a', b'b', b'c'), Some(0));
}

#[test]
fn test_memrnechr_tpl_single_element_not_found_1() {
    let buf = [b'a'];
    assert_eq!(test_memrnechr_tpl(&buf, b'a', b'b', b'c'), None);
}

#[test]
fn test_memrnechr_tpl_single_element_not_found_2() {
    let buf = [b'b'];
    assert_eq!(test_memrnechr_tpl(&buf, b'a', b'b', b'c'), None);
}

#[test]
fn test_memrnechr_tpl_single_element_not_found_3() {
    let buf = [b'c'];
    assert_eq!(test_memrnechr_tpl(&buf, b'a', b'b', b'c'), None);
}

#[test]
fn test_memrnechr_tpl_needle_at_start() {
    let buf = [b'd', b'a', b'b', b'c'];
    assert_eq!(test_memrnechr_tpl(&buf, b'a', b'b', b'c'), Some(0));
}

#[test]
fn test_memrnechr_tpl_needle_at_middle() {
    let buf = [b'a', b'd', b'b', b'c'];
    assert_eq!(test_memrnechr_tpl(&buf, b'a', b'b', b'c'), Some(1));
}

#[test]
fn test_memrnechr_tpl_needle_at_end() {
    let buf = [b'a', b'b', b'd', b'c'];
    assert_eq!(test_memrnechr_tpl(&buf, b'a', b'b', b'c'), Some(2));
}

#[test]
fn test_memrnechr_tpl_needle_not_found() {
    let buf = [b'a', b'b', b'c', b'a'];
    assert_eq!(test_memrnechr_tpl(&buf, b'a', b'b', b'c'), None);
}

#[test]
fn test_memrnechr_tpl_multiple_occurrences() {
    let buf = [b'a', b'd', b'b', b'e', b'c', b'f'];
    assert_eq!(test_memrnechr_tpl(&buf, b'a', b'b', b'c'), Some(5));
}

#[test]
fn test_memrnechr_tpl_long_buffer() {
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
        test_memrnechr_tpl(&long_buf, b'a', b'b', b'c'),
        expected_idx
    );
}

#[test]
fn test_memrnechr_tpl_long_buffer_not_found() {
    let buf = [b'a'; 1024];
    assert_eq!(test_memrnechr_tpl(&buf, b'a', b'b', b'c'), None);
}
