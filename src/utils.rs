use core::convert::TryInto;

#[inline(always)]
pub(crate) fn _read_a_native_endian_u128(input: &[u8]) -> u128 {
    let (int_bytes, _rest) = input.split_at(core::mem::size_of::<u128>());
    u128::from_ne_bytes(int_bytes.try_into().unwrap())
}

#[inline(always)]
pub(crate) fn _read_a_native_endian_u64(input: &[u8]) -> u64 {
    let (int_bytes, _rest) = input.split_at(core::mem::size_of::<u64>());
    u64::from_ne_bytes(int_bytes.try_into().unwrap())
}

#[inline(always)]
pub(crate) fn _read_a_native_endian_u32(input: &[u8]) -> u32 {
    let (int_bytes, _rest) = input.split_at(core::mem::size_of::<u32>());
    u32::from_ne_bytes(int_bytes.try_into().unwrap())
}

#[inline(always)]
pub(crate) fn _read_a_native_endian_u16(input: &[u8]) -> u16 {
    let (int_bytes, _rest) = input.split_at(core::mem::size_of::<u16>());
    u16::from_ne_bytes(int_bytes.try_into().unwrap())
}
