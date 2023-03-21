#[cfg(test)]
use core::convert::TryInto;

#[inline(always)]
pub(crate) fn _read_a_native_endian_u128(input: &[u8]) -> u128 {
    let (int_bytes, _rest) = input.split_at(core::mem::size_of::<u128>());
    #[cfg(test)]
    let r = u128::from_ne_bytes(int_bytes.try_into().unwrap());
    #[cfg(not(test))]
    let r = u128::from_ne_bytes(*unsafe { &*int_bytes.as_ptr().cast::<[u8; 16]>() });
    //
    r
}

#[inline(always)]
pub(crate) fn _read_a_native_endian_u64(input: &[u8]) -> u64 {
    let (int_bytes, _rest) = input.split_at(core::mem::size_of::<u64>());
    #[cfg(test)]
    let r = u64::from_ne_bytes(int_bytes.try_into().unwrap());
    #[cfg(not(test))]
    let r = u64::from_ne_bytes(*unsafe { &*int_bytes.as_ptr().cast::<[u8; 8]>() });
    //
    r
}

#[inline(always)]
pub(crate) fn _read_a_native_endian_u32(input: &[u8]) -> u32 {
    let (int_bytes, _rest) = input.split_at(core::mem::size_of::<u32>());
    #[cfg(test)]
    let r = u32::from_ne_bytes(int_bytes.try_into().unwrap());
    #[cfg(not(test))]
    let r = u32::from_ne_bytes(*unsafe { &*int_bytes.as_ptr().cast::<[u8; 4]>() });
    //
    r
}

#[inline(always)]
pub(crate) fn _read_a_native_endian_u16(input: &[u8]) -> u16 {
    let (int_bytes, _rest) = input.split_at(core::mem::size_of::<u16>());
    #[cfg(test)]
    let r = u16::from_ne_bytes(int_bytes.try_into().unwrap());
    #[cfg(not(test))]
    let r = u16::from_ne_bytes(*unsafe { &*int_bytes.as_ptr().cast::<[u8; 2]>() });
    //
    r
}

#[inline(always)]
pub(crate) fn _read_a_big_endian_u128(input: &[u8]) -> u128 {
    let (int_bytes, _rest) = input.split_at(core::mem::size_of::<u128>());
    #[cfg(test)]
    let r = u128::from_be_bytes(int_bytes.try_into().unwrap());
    #[cfg(not(test))]
    let r = u128::from_be_bytes(*unsafe { &*int_bytes.as_ptr().cast::<[u8; 16]>() });
    //
    r
}

#[inline(always)]
pub(crate) fn _read_a_big_endian_u64(input: &[u8]) -> u64 {
    let (int_bytes, _rest) = input.split_at(core::mem::size_of::<u64>());
    #[cfg(test)]
    let r = u64::from_be_bytes(int_bytes.try_into().unwrap());
    #[cfg(not(test))]
    let r = u64::from_be_bytes(*unsafe { &*int_bytes.as_ptr().cast::<[u8; 8]>() });
    //
    r
}

#[inline(always)]
pub(crate) fn _read_a_big_endian_u32(input: &[u8]) -> u32 {
    let (int_bytes, _rest) = input.split_at(core::mem::size_of::<u32>());
    #[cfg(test)]
    let r = u32::from_be_bytes(int_bytes.try_into().unwrap());
    #[cfg(not(test))]
    let r = u32::from_be_bytes(*unsafe { &*int_bytes.as_ptr().cast::<[u8; 4]>() });
    //
    r
}

#[inline(always)]
pub(crate) fn _read_a_big_endian_u16(input: &[u8]) -> u16 {
    let (int_bytes, _rest) = input.split_at(core::mem::size_of::<u16>());
    #[cfg(test)]
    let r = u16::from_be_bytes(int_bytes.try_into().unwrap());
    #[cfg(not(test))]
    let r = u16::from_be_bytes(*unsafe { &*int_bytes.as_ptr().cast::<[u8; 2]>() });
    //
    r
}

#[allow(dead_code)]
pub(crate) const HAS_ZERO_ONES_U128: u128 = u128::MAX / (u8::MAX as u128);
#[allow(dead_code)]
pub(crate) const HAS_ZERO_HIGHS_U128: u128 = HAS_ZERO_ONES_U128 * (u8::MAX / 2 + 1) as u128;
#[allow(dead_code)]
pub(crate) const HAS_ZERO_LOWS_U128: u128 = HAS_ZERO_ONES_U128 * (u8::MAX / 2) as u128;

#[allow(dead_code)]
#[inline(always)]
pub(crate) fn has_zero_u128(x: u128) -> u128 {
    x.wrapping_sub(HAS_ZERO_ONES_U128) & !x & HAS_ZERO_HIGHS_U128
}

#[allow(dead_code)]
#[inline(always)]
pub(crate) fn has_zero_byte_u128(x: u128) -> u128 {
    !((((x & HAS_ZERO_LOWS_U128).wrapping_add(HAS_ZERO_LOWS_U128)) | x) | HAS_ZERO_LOWS_U128)
}

#[allow(dead_code)]
pub(crate) const HAS_ZERO_ONES_U64: u64 = u64::MAX / (u8::MAX as u64);
#[allow(dead_code)]
pub(crate) const HAS_ZERO_HIGHS_U64: u64 = HAS_ZERO_ONES_U64 * (u8::MAX / 2 + 1) as u64;
#[allow(dead_code)]
pub(crate) const HAS_ZERO_LOWS_U64: u64 = HAS_ZERO_ONES_U64 * (u8::MAX / 2) as u64;

#[allow(dead_code)]
#[inline(always)]
pub(crate) fn has_zero_u64(x: u64) -> u64 {
    x.wrapping_sub(HAS_ZERO_ONES_U64) & !x & HAS_ZERO_HIGHS_U64
}

#[allow(dead_code)]
#[inline(always)]
pub(crate) fn has_zero_byte_u64(x: u64) -> u64 {
    !((((x & HAS_ZERO_LOWS_U64).wrapping_add(HAS_ZERO_LOWS_U64)) | x) | HAS_ZERO_LOWS_U64)
}

#[allow(dead_code)]
pub(crate) const HAS_ZERO_ONES_U32: u32 = u32::MAX / (u8::MAX as u32);
#[allow(dead_code)]
pub(crate) const HAS_ZERO_HIGHS_U32: u32 = HAS_ZERO_ONES_U32 * (u8::MAX / 2 + 1) as u32;
#[allow(dead_code)]
pub(crate) const HAS_ZERO_LOWS_U32: u32 = HAS_ZERO_ONES_U32 * (u8::MAX / 2) as u32;

#[allow(dead_code)]
#[inline(always)]
pub(crate) fn has_zero_u32(x: u32) -> u32 {
    x.wrapping_sub(HAS_ZERO_ONES_U32) & !x & HAS_ZERO_HIGHS_U32
}

#[allow(dead_code)]
#[inline(always)]
pub(crate) fn has_zero_byte_u32(x: u32) -> u32 {
    !((((x & HAS_ZERO_LOWS_U32).wrapping_add(HAS_ZERO_LOWS_U32)) | x) | HAS_ZERO_LOWS_U32)
}

#[allow(dead_code)]
pub(crate) const HAS_ZERO_ONES_U16: u16 = u16::MAX / (u8::MAX as u16);
#[allow(dead_code)]
pub(crate) const HAS_ZERO_HIGHS_U16: u16 = HAS_ZERO_ONES_U16 * (u8::MAX / 2 + 1) as u16;
#[allow(dead_code)]
pub(crate) const HAS_ZERO_LOWS_U16: u16 = HAS_ZERO_ONES_U16 * (u8::MAX / 2) as u16;

#[allow(dead_code)]
#[inline(always)]
pub(crate) fn has_zero_u16(x: u16) -> u16 {
    x.wrapping_sub(HAS_ZERO_ONES_U16) & !x & HAS_ZERO_HIGHS_U16
}

#[allow(dead_code)]
#[inline(always)]
pub(crate) fn has_zero_byte_u16(x: u16) -> u16 {
    !((((x & HAS_ZERO_LOWS_U16).wrapping_add(HAS_ZERO_LOWS_U16)) | x) | HAS_ZERO_LOWS_U16)
}

/* for rust playground
fn main() {
    const ONES: u32 = u32::MAX/(u8::MAX as u32);
    const HIGHS: u32 = ONES * (u8::MAX/2+1) as u32;
    const LOWS: u32 = ONES * (u8::MAX/2) as u32;
    println!("ONES: 0x{:08x}", ONES);
    println!("HIGHS: 0x{:08x}", HIGHS);
    println!("LOWS: 0x{:08x}", LOWS);
    println!();
    fn has_zero(x: u32) -> u32 {
        x.wrapping_sub(ONES) & !x & HIGHS
    }
    fn has_zero_byte(x: u32) -> u32 {
        !((((x & LOWS).wrapping_add(LOWS)) | x) | LOWS)
    }
    let v0 = 0x43424242_u32;
    let c2 = 0x42424242_u32;
    let v = v0 ^ c2;
    println!("v:             0x{:08x}", v);
    println!("has_zero:      0x{:08x}", has_zero(v));
    println!("has_zero_byte: 0x{:08x}", has_zero_byte(v));
    println!();
    let v0 = 0x42424242_u32;
    let c2 = 0x42424242_u32;
    let v = v0 ^ c2;
    println!("v:             0x{:08x}", v);
    println!("has_zero:      0x{:08x}", has_zero(v));
    println!("has_zero_byte: 0x{:08x}", has_zero_byte(v));
    println!();
}

results>
ONES: 0x01010101
HIGHS: 0x80808080
LOWS: 0x7f7f7f7f

v:             0x01000000
has_zero:      0x80808080
has_zero_byte: 0x00808080

v:             0x00000000
has_zero:      0x80808080
has_zero_byte: 0x80808080
*/
