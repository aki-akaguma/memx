/**/

#[test]
fn test_memmem_empty_haystack() {
    let haystack = [];
    let needle = [b'a'];
    assert_eq!(test_memmem(&haystack, &needle), None);
}

#[test]
fn test_memmem_empty_needle() {
    let haystack = [b'a', b'b', b'c'];
    let needle = [];
    assert_eq!(test_memmem(&haystack, &needle), Some(0));
}

#[test]
fn test_memmem_needle_longer_than_haystack() {
    let haystack = [b'a', b'b'];
    let needle = [b'a', b'b', b'c'];
    assert_eq!(test_memmem(&haystack, &needle), None);
}

#[test]
fn test_memmem_needle_at_start() {
    let haystack = [b'a', b'b', b'c', b'd'];
    let needle = [b'a', b'b'];
    assert_eq!(test_memmem(&haystack, &needle), Some(0));
}

#[test]
fn test_memmem_needle_at_middle() {
    let haystack = [b'a', b'b', b'c', b'd'];
    let needle = [b'b', b'c'];
    assert_eq!(test_memmem(&haystack, &needle), Some(1));
}

#[test]
fn test_memmem_needle_at_end() {
    let haystack = [b'a', b'b', b'c', b'd'];
    let needle = [b'c', b'd'];
    assert_eq!(test_memmem(&haystack, &needle), Some(2));
}

#[test]
fn test_memmem_needle_not_found() {
    let haystack = [b'a', b'b', b'c', b'd'];
    let needle = [b'x', b'y'];
    assert_eq!(test_memmem(&haystack, &needle), None);
}

#[test]
fn test_memmem_multiple_occurrences() {
    let haystack = [b'a', b'b', b'a', b'b', b'c'];
    let needle = [b'a', b'b'];
    assert_eq!(test_memmem(&haystack, &needle), Some(0));
}

#[test]
fn test_memmem_long_buffers() {
    let haystack = [0; 1024];
    let needle = [1; 16];
    let mut long_haystack = haystack.to_vec();
    long_haystack[500..516].copy_from_slice(&needle);
    assert_eq!(test_memmem(&long_haystack, &needle), Some(500));
}

#[test]
fn test_memmem_long_buffers_not_found() {
    let haystack = [0; 1024];
    let needle = [1; 16];
    assert_eq!(test_memmem(&haystack, &needle), None);
}
