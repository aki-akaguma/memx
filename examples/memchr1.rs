fn main() {
    let buf: Vec<u8> = vec![b'a', b'b', b'c', b'd', b'e'];
    let buf_sl = buf.as_slice();
    let _ = memx::memchr(buf_sl, b'd');
}
