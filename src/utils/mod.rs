#[cfg(feature = "test")]
#[allow(unused_imports)]
use core::convert::TryInto;

pub trait PtrOpsPrefetch {
    fn prefetch_read_data(&self);
}
impl PtrOpsPrefetch for *const u8 {
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
}
impl PtrOpsPrefetch for *mut u8 {
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
}

#[allow(dead_code)]
pub trait PtrOps {
    fn is_aligned_u256(&self) -> bool;
    fn is_aligned_u128(&self) -> bool;
    fn is_aligned_u64(&self) -> bool;
    fn is_aligned_u32(&self) -> bool;
    fn is_aligned_u16(&self) -> bool;
    fn usz_offset_from(&self, origin: *const u8) -> usize;
    fn is_not_over(&self, end_ptr: *const u8, loop_unroll: usize) -> bool;
    fn is_not_under(&self, start_ptr: *const u8, loop_unroll: usize) -> bool;
}
impl PtrOps for *const u8 {
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
    #[inline(always)]
    fn usz_offset_from(&self, origin: *const u8) -> usize {
        assert!((*self as usize) >= (origin as usize));
        (*self as usize) - (origin as usize)
    }
    #[inline(always)]
    fn is_not_over(&self, end_ptr: *const u8, loop_unroll: usize) -> bool {
        let (end_val, overflowing) = (end_ptr as usize).overflowing_sub(loop_unroll);
        !overflowing && (*self as usize) <= end_val
        // (unsafe { end_ptr.offset_from(*self) }) >= loop_unroll as isize
    }
    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    #[inline(always)]
    fn is_not_under(&self, start_ptr: *const u8, loop_unroll: usize) -> bool {
        /*
        let (start_val, overflowing) = loop_unroll.overflowing_add(start_ptr as usize);
        !overflowing && (*self as usize) >= start_val
        */
        (unsafe { self.offset_from(start_ptr) }) >= loop_unroll as isize
    }
}

impl PtrOps for *mut u8 {
    #[inline(always)]
    fn is_aligned_u256(&self) -> bool {
        ((*self as usize) & 0x1F_usize) == 0
    }
    #[inline(always)]
    fn is_aligned_u128(&self) -> bool {
        (*self as *const u8).is_aligned_u128()
    }
    #[inline(always)]
    fn is_aligned_u64(&self) -> bool {
        (*self as *const u8).is_aligned_u64()
    }
    #[inline(always)]
    fn is_aligned_u32(&self) -> bool {
        (*self as *const u8).is_aligned_u32()
    }
    #[inline(always)]
    fn is_aligned_u16(&self) -> bool {
        (*self as *const u8).is_aligned_u16()
    }
    #[inline(always)]
    fn usz_offset_from(&self, origin: *const u8) -> usize {
        (*self as *const u8).usz_offset_from(origin)
    }
    #[inline(always)]
    fn is_not_over(&self, end_ptr: *const u8, loop_unroll: usize) -> bool {
        (*self as *const u8).is_not_over(end_ptr, loop_unroll)
    }
    #[inline(always)]
    fn is_not_under(&self, start_ptr: *const u8, loop_unroll: usize) -> bool {
        (*self as *const u8).is_not_under(start_ptr, loop_unroll)
    }
}

macro_rules! read_native_integer_impl {
    ($($fn_name:ident: $ty:ident,)+) => {$(
        #[inline(always)]
        pub(crate) unsafe fn $fn_name(buf_ptr: *const u8) -> $ty {
            const SIZE_OF: usize = core::mem::size_of::<$ty>();
            let input = core::slice::from_raw_parts(buf_ptr, SIZE_OF);
            let (int_bytes, _rest) = input.split_at(SIZE_OF);
            #[cfg(feature = "test")]
            let r = $ty::from_ne_bytes(int_bytes.try_into().unwrap());
            #[cfg(not(feature = "test"))]
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
            #[cfg(feature = "test")]
            let r = $ty::from_le_bytes(int_bytes.try_into().unwrap());
            #[cfg(not(feature = "test"))]
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
            #[cfg(feature = "test")]
            let r = $ty::from_be_bytes(int_bytes.try_into().unwrap());
            #[cfg(not(feature = "test"))]
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

/// bitwidth operations
#[allow(dead_code)]
pub(crate) trait BitOrt {
    fn is_zeros(&self) -> bool;
    fn is_highs(&self) -> bool;
    fn leading_ones(&self) -> u32;
    fn trailing_ones(&self) -> u32;
    fn leading_zeros(&self) -> u32;
    fn trailing_zeros(&self) -> u32;
}
impl BitOrt for u32 {
    fn is_zeros(&self) -> bool {
        *self == 0
    }
    fn is_highs(&self) -> bool {
        *self == u32::MAX
    }
    fn leading_ones(&self) -> u32 {
        u32::leading_ones(*self)
    }
    fn trailing_ones(&self) -> u32 {
        u32::trailing_ones(*self)
    }
    fn leading_zeros(&self) -> u32 {
        u32::leading_zeros(*self)
    }
    fn trailing_zeros(&self) -> u32 {
        u32::trailing_zeros(*self)
    }
}
impl BitOrt for u16 {
    fn is_zeros(&self) -> bool {
        *self == 0
    }
    fn is_highs(&self) -> bool {
        *self == u16::MAX
    }
    fn leading_ones(&self) -> u32 {
        u16::leading_ones(*self)
    }
    fn trailing_ones(&self) -> u32 {
        u16::trailing_ones(*self)
    }
    fn leading_zeros(&self) -> u32 {
        u16::leading_zeros(*self)
    }
    fn trailing_zeros(&self) -> u32 {
        u16::trailing_zeros(*self)
    }
}

/// a high bit propagations
pub(crate) trait HighBitProp {
    fn propagate_a_high_bit(self) -> Self;
}

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
        }
        impl BitOrt for $packed {
            fn is_zeros(&self) -> bool {
                self.0 == 0
            }
            fn is_highs(&self) -> bool {
                self.0 == Self::HIGHS
            }
            fn leading_ones(&self) -> u32 {
                self.0.leading_ones()
            }
            fn trailing_ones(&self) -> u32 {
                self.0.trailing_ones()
            }
            fn leading_zeros(&self) -> u32 {
                self.0.leading_zeros()
            }
            fn trailing_zeros(&self) -> u32 {
                self.0.trailing_zeros()
            }
        }
        impl HighBitProp for $packed {
            fn propagate_a_high_bit(self) -> $packed {
                Self::new(propagate_a_high_bit(self.0))
            }
        }
        impl core::ops::BitOr for $packed {
            type Output = Self;
            fn bitor(self, rhs: Self) -> Self::Output {
                Self::new(self.0 | rhs.0)
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

mod needles;
pub(crate) use needles::dbl::{B16Dbl, B2Dbl, B4Dbl, B8Dbl};
pub(crate) use needles::qpl::{B16Qpl, B2Qpl, B4Qpl, B8Qpl};
pub(crate) use needles::sgl::{B16Sgl, B2Sgl, B4Sgl, B8Sgl};
pub(crate) use needles::tpl::{B16Tpl, B2Tpl, B4Tpl, B8Tpl};

pub use needles::dbl::B1Dbl;
pub use needles::qpl::B1Qpl;
pub use needles::sgl::B1Sgl;
pub use needles::tpl::B1Tpl;

// ascii stochastics
#[rustfmt::skip]
const _ASCII_STOCHAS: [u8; 128] = [
    // 0x00: NUL,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    // 0x10: DEL,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    // 0x20: SP, 0x21: b'!'
    255, 0, 2, 0, 0, 0, 0, 0, 1, 1, 0, 3, 6, 14, 19, 1,
    // 0x30: b'0'
    3, 4, 3, 2, 2, 1, 1, 1, 1, 1, 2, 0, 0, 1, 0, 0,
    // 0x40: b'@', 0x41: b'A', 0x4f: 'O',
    0, 4, 1, 5, 2, 4, 3, 0, 1, 5, 0, 0, 2, 3, 3, 2,
    // 0x50: b'P', 0x5a: b'Z'
    5, 0, 4, 6, 6, 1, 0, 0, 0, 0, 0, 1, 0, 1, 0, 1,
    // 0x60: b'`', 0x61: b'a', 0x4f: 'o',
    0, 39, 7, 20, 19, 69, 11, 9, 18, 39, 0, 2, 18, 12, 38, 38,
    // 0x70: b'p', 0x7a: b'z'
    12, 1, 34, 35, 50, 13, 5, 5, 2, 7, 0, 0, 2, 0, 0, 0,
];

#[inline(always)]
pub(crate) fn _ascii_stochas(idx: u8) -> u8 {
    //_ASCII_STOCHAS[idx as usize]
    assert!(idx < 128);
    unsafe { *(_ASCII_STOCHAS.as_ptr().add(idx as usize)) }
}
