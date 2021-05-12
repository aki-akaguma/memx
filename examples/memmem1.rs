fn main() {
    let hay: Vec<u8> = vec![b'a', b'b', b'c', b'd', b'e'];
    let nee: Vec<u8> = vec![b'c', b'd'];
    let hay_sl = hay.as_slice();
    let nee_sl = nee.as_slice();
    let _ = memx::memmem(hay_sl, nee_sl);
}
