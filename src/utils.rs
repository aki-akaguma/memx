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
