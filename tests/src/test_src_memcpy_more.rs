/**/

#[test]
fn test_memcpy_empty_src() {
    let mut dst = [0; 10];
    let src = [];
    assert_eq!(test_memcpy(&mut dst, &src), Ok(()));
}

#[test]
fn test_memcpy_exact_fit() {
    let mut dst = [0; 3];
    let src = [1, 2, 3];
    assert_eq!(test_memcpy(&mut dst, &src), Ok(()));
    assert_eq!(dst, [1, 2, 3]);
}

#[test]
fn test_memcpy_dst_larger() {
    let mut dst = [0; 5];
    let src = [1, 2, 3];
    assert_eq!(test_memcpy(&mut dst, &src), Ok(()));
    assert_eq!(dst, [1, 2, 3, 0, 0]);
}

#[test]
fn test_memcpy_dst_smaller() {
    let mut dst = [0; 2];
    let src = [1, 2, 3];
    assert_eq!(test_memcpy(&mut dst, &src), Err(RangeError));
}

#[test]
fn test_memcpy_long_buffers() {
    let mut dst = [0; 1024];
    let src = [1; 1024];
    assert_eq!(test_memcpy(&mut dst, &src), Ok(()));
    assert_eq!(dst, src);
}

#[test]
fn test_memcpy_long_buffers_partial() {
    let mut dst = [0; 1024];
    let src = [1; 512];
    assert_eq!(test_memcpy(&mut dst, &src), Ok(()));
    assert_eq!(dst[0..512], src);
    assert_eq!(dst[512..1024], [0; 512]);
}
