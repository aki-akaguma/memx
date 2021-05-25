fn main() {
    let mut dst: Vec<u8> = vec![b'a', b'b', b'c', b'd', b'e'];
    let src: Vec<u8> = vec![b'A', b'B', b'C'];
    let dst_sl = dst.as_mut_slice();
    let src_sl = src.as_slice();
    let _ = memx::memcpy(dst_sl, src_sl);
}
