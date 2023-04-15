// multiplicative
pub(crate) mod dbl;
pub(crate) mod qpl;
pub(crate) mod sgl;
pub(crate) mod tpl;

#[inline(always)]
fn _b16_value(b: u8) -> u128 {
    (b as u128) * super::PackedU128::ONES
}

#[inline(always)]
fn _b8_value(b: u8) -> u64 {
    (b as u64) * super::PackedU64::ONES
}

#[inline(always)]
fn _b4_value(b: u8) -> u32 {
    (b as u32) * super::PackedU32::ONES
}

#[inline(always)]
fn _b2_value(b: u8) -> u16 {
    (b as u16) * super::PackedU16::ONES
}
