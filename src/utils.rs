#[cfg(features = "test")]
use core::convert::TryInto;

pub trait PtrOps {
    fn prefetch_read_data(&self);
    fn is_aligned_u256(&self) -> bool;
    fn is_aligned_u128(&self) -> bool;
    fn is_aligned_u64(&self) -> bool;
    fn is_aligned_u32(&self) -> bool;
    fn is_aligned_u16(&self) -> bool;
}
impl PtrOps for *const u8 {
    #[inline(always)]
    fn prefetch_read_data(&self) {
        // TODO: Replace with core::intrinsics::prefetch_read_data
        // after stabilization
        // The cache line size of x86_64 is 64 bytes.
        #[cfg(not(miri))]
        #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
        {
            #[cfg(target_arch = "x86")]
            use core::arch::x86 as mmx;
            #[cfg(target_arch = "x86_64")]
            use core::arch::x86_64 as mmx;
            //
            unsafe { mmx::_mm_prefetch(*self as *const i8, mmx::_MM_HINT_T0) };
        }
    }
    #[inline(always)]
    fn is_aligned_u256(&self) -> bool {
        ((*self as usize) & 0x1F_usize) == 0
    }
    #[inline(always)]
    fn is_aligned_u128(&self) -> bool {
        ((*self as usize) & 0x0F_usize) == 0
    }
    #[inline(always)]
    fn is_aligned_u64(&self) -> bool {
        ((*self as usize) & 0x07_usize) == 0
    }
    #[inline(always)]
    fn is_aligned_u32(&self) -> bool {
        ((*self as usize) & 0x03_usize) == 0
    }
    #[inline(always)]
    fn is_aligned_u16(&self) -> bool {
        ((*self as usize) & 0x01_usize) == 0
    }
}

impl PtrOps for *mut u8 {
    #[inline(always)]
    fn prefetch_read_data(&self) {
        // TODO: Replace with core::intrinsics::prefetch_read_data
        // after stabilization
        // The cache line size of x86_64 is 64 bytes.
        #[cfg(not(miri))]
        #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
        {
            #[cfg(target_arch = "x86")]
            use core::arch::x86 as mmx;
            #[cfg(target_arch = "x86_64")]
            use core::arch::x86_64 as mmx;
            //
            unsafe { mmx::_mm_prefetch(*self as *const i8, mmx::_MM_HINT_T0) };
        }
    }
    #[inline(always)]
    fn is_aligned_u256(&self) -> bool {
        ((*self as usize) & 0x1F_usize) == 0
    }
    #[inline(always)]
    fn is_aligned_u128(&self) -> bool {
        ((*self as usize) & 0x0F_usize) == 0
    }
    #[inline(always)]
    fn is_aligned_u64(&self) -> bool {
        ((*self as usize) & 0x07_usize) == 0
    }
    #[inline(always)]
    fn is_aligned_u32(&self) -> bool {
        ((*self as usize) & 0x03_usize) == 0
    }
    #[inline(always)]
    fn is_aligned_u16(&self) -> bool {
        ((*self as usize) & 0x01_usize) == 0
    }
}

macro_rules! read_native_integer_impl {
    ($($fn_name:ident: $ty:ident,)+) => {$(
        #[inline(always)]
        pub(crate) unsafe fn $fn_name(buf_ptr: *const u8) -> $ty {
            const SIZE_OF: usize = core::mem::size_of::<$ty>();
            let input = core::slice::from_raw_parts(buf_ptr, SIZE_OF);
            let (int_bytes, _rest) = input.split_at(SIZE_OF);
            #[cfg(features = "test")]
            let r = $ty::from_ne_bytes(int_bytes.try_into().unwrap());
            #[cfg(not(features = "test"))]
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
            #[cfg(features = "test")]
            let r = $ty::from_le_bytes(int_bytes.try_into().unwrap());
            #[cfg(not(features = "test"))]
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
            #[cfg(features = "test")]
            let r = $ty::from_be_bytes(int_bytes.try_into().unwrap());
            #[cfg(not(features = "test"))]
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
            // ONES: 0x0101_0101_0101_0101_u64;
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

#[inline(always)]
pub(crate) fn _c16_value(c: u8) -> u128 {
    (c as u128) * PackedU128::ONES
}

#[inline(always)]
pub(crate) fn _c8_value(c: u8) -> u64 {
    (c as u64) * PackedU64::ONES
}

#[inline(always)]
pub(crate) fn _c4_value(c: u8) -> u32 {
    (c as u32) * PackedU32::ONES
}
