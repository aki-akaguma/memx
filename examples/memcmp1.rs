fn main() {
    let a: Vec<u8> = vec![b'a', b'b', b'c', b'd', b'e'];
    let b: Vec<u8> = vec![b'a', b'b', b'c'];
    let a_sl = a.as_slice();
    let b_sl = b.as_slice();
    let _ = memx::memcmp(a_sl, b_sl);
}
