#[cfg(test)]
use core::convert::TryInto;

macro_rules! read_native_integer_impl {
    ($($fn_name:ident: $ty:ident,)+) => {$(
        #[inline(always)]
        pub(crate) unsafe fn $fn_name(buf_ptr: *const u8) -> $ty {
            const SIZE_OF: usize = core::mem::size_of::<$ty>();
            let input = core::slice::from_raw_parts(buf_ptr, SIZE_OF);
            let (int_bytes, _rest) = input.split_at(SIZE_OF);
            #[cfg(test)]
            let r = $ty::from_ne_bytes(int_bytes.try_into().unwrap());
            #[cfg(not(test))]
            let r = $ty::from_ne_bytes(*int_bytes.as_ptr().cast::<[u8; SIZE_OF]>());
            //
            r
        }
    )+}
}
read_native_integer_impl! {
    _read_a_native_endian_from_ptr_u16: u16,
    _read_a_native_endian_from_ptr_u32: u32,
    _read_a_native_endian_from_ptr_u64: u64,
    _read_a_native_endian_from_ptr_u128: u128,
}

macro_rules! read_little_integer_impl {
    ($($fn_name:ident: $ty:ident,)+) => {$(
        #[inline(always)]
        pub(crate) unsafe fn $fn_name(buf_ptr: *const u8) -> $ty {
            const SIZE_OF: usize = core::mem::size_of::<$ty>();
            let input = core::slice::from_raw_parts(buf_ptr, SIZE_OF);
            let (int_bytes, _rest) = input.split_at(SIZE_OF);
            #[cfg(test)]
            let r = $ty::from_le_bytes(int_bytes.try_into().unwrap());
            #[cfg(not(test))]
            let r = $ty::from_le_bytes(*int_bytes.as_ptr().cast::<[u8; SIZE_OF]>());
            //
            r
        }
    )+}
}
read_little_integer_impl! {
    _read_a_little_endian_from_ptr_u16: u16,
    _read_a_little_endian_from_ptr_u32: u32,
    _read_a_little_endian_from_ptr_u64: u64,
    _read_a_little_endian_from_ptr_u128: u128,
}


macro_rules! read_big_integer_impl {
    ($($fn_name:ident: $ty:ident,)+) => {$(
        #[inline(always)]
        pub(crate) unsafe fn $fn_name(buf_ptr: *const u8) -> $ty {
            const SIZE_OF: usize = core::mem::size_of::<$ty>();
            let input = core::slice::from_raw_parts(buf_ptr, SIZE_OF);
            let (int_bytes, _rest) = input.split_at(SIZE_OF);
            #[cfg(test)]
            let r = $ty::from_be_bytes(int_bytes.try_into().unwrap());
            #[cfg(not(test))]
            let r = $ty::from_be_bytes(*int_bytes.as_ptr().cast::<[u8; SIZE_OF]>());
            //
            r
        }
    )+}
}
read_big_integer_impl! {
    _read_a_big_endian_from_ptr_u16: u16,
    _read_a_big_endian_from_ptr_u32: u32,
    _read_a_big_endian_from_ptr_u64: u64,
    _read_a_big_endian_from_ptr_u128: u128,
}

#[inline(always)]
pub(crate) fn plus_offset_from(ptr: *const u8, origin: *const u8) -> usize {
    (ptr as usize) - (origin as usize)
}

#[inline(always)]
pub(crate) fn propagate_a_high_bit<T>(bits: T) -> T
where
    T: core::ops::Div<Output = T> + core::ops::Mul<Output = T> + From<u8>,
{
    /*
    // a hight bit propagation: bits: 0b_1000_0000
    let bits = bits | (bits >> 1); // 0b_1100_0000
    let bits = bits | (bits >> 2); // 0b_1111_0000
    let bits = bits | (bits >> 4); // 0b_1111_1111
    */
    /*
    // a hight bit propagation:
    // ------------------------ bits: 0b_0000_0000_1000_0000
    let bits = bits / 0x80.into(); // 0b_0000_0000_0000_0001
    let bits = bits * 0xFF.into(); // 0b_0000_0000_1111_1111
    */
    (bits / 0x80.into()) * 0xFF.into()
}

/*
 * Refer.
 *   https://mmi.hatenablog.com/entry/2017/07/27/230005
 *   you should have memcpy(), memcmp(), memset() on nostd environments
*/

macro_rules! packed_integers {
    ($($packed:ident: $ty:ident,)+) => {$(
        #[derive(Debug, Default)]
        pub(crate) struct $packed($ty);

        #[allow(dead_code)]
        impl $packed {
            pub const ONES: $ty = $ty::MAX / (u8::MAX as $ty);
            pub const HIGHS: $ty = Self::ONES * (u8::MAX / 2 + 1) as $ty;
            pub const LOWS: $ty = Self::ONES * (u8::MAX / 2) as $ty;
            //
            pub fn new(v: $ty) -> Self {
                Self(v)
            }
            pub fn may_have_zero_quick(&self) -> Self {
                let x = self.0;
                let v = x.wrapping_sub(Self::ONES) & !x & Self::HIGHS;
                Self::new(v)
            }
            pub fn may_have_zero_byte(&self) -> Self {
                let x = self.0;
                let v = !((((x & Self::LOWS).wrapping_add(Self::LOWS)) | x) | Self::LOWS);
                Self::new(v)
            }
            pub fn is_zeros(&self) -> bool {
                self.0 == 0
            }
            pub fn is_highs(&self) -> bool {
                self.0 == Self::HIGHS
            }
            pub fn propagate_a_high_bit(self) -> Self {
                Self::new(propagate_a_high_bit(self.0))
            }
            pub fn leading_ones(&self) -> u32 {
                self.0.leading_ones()
            }
            pub fn trailing_ones(&self) -> u32 {
                self.0.trailing_ones()
            }
            pub fn leading_zeros(&self) -> u32 {
                self.0.leading_zeros()
            }
            pub fn trailing_zeros(&self) -> u32 {
                self.0.trailing_zeros()
            }
        }
    )+}
}

packed_integers! {
    PackedU16: u16,
    PackedU32: u32,
    PackedU64: u64,
    PackedU128: u128,
}

/*
 * bits operation Reference:
 * https://pzemtsov.github.io/2019/09/26/making-a-char-searcher-in-c.html
 * https://graphics.stanford.edu/~seander/bithacks.html#ZeroInWord
*/

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
