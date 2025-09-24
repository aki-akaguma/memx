/**/

#[test]
fn test_memset_empty_buffer() {
    let mut buf = [];
    test_memset(&mut buf, b'a');
    assert_eq!(buf, []);
}

#[test]
fn test_memset_single_element() {
    let mut buf = [0];
    test_memset(&mut buf, b'a');
    assert_eq!(buf, [b'a']);
}

#[test]
fn test_memset_multiple_elements() {
    let mut buf = [0, 0, 0];
    test_memset(&mut buf, b'a');
    assert_eq!(buf, [b'a', b'a', b'a']);
}

#[test]
fn test_memset_long_buffer() {
    let mut buf = [0; 1024];
    test_memset(&mut buf, b'x');
    assert_eq!(buf, [b'x'; 1024]);
}

#[test]
fn test_memset_different_byte() {
    let mut buf = [0, 0, 0];
    test_memset(&mut buf, b'z');
    assert_eq!(buf, [b'z', b'z', b'z']);
}
