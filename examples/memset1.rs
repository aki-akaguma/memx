fn main() {
    let mut buf: Vec<u8> = vec![b'a', b'b', b'c', b'd', b'e'];
    let sl = buf.as_mut_slice();
    let _ = memx::memset(sl, b'A');
}
