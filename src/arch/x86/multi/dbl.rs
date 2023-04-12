use super::super::{__m128i, __m256i};
use super::super::{_c16_from_c32, _c16_value, _c32_value};

#[derive(Copy, Clone)]
pub(crate) struct MMC16Dbl {
    pub a: __m128i,
    pub b: __m128i,
}
impl MMC16Dbl {
    #[inline(always)]
    pub fn new(c1: u8, c2: u8) -> Self {
        Self {
            a: unsafe { _c16_value(c1) },
            b: unsafe { _c16_value(c2) },
        }
    }
}

#[derive(Copy, Clone)]
pub(crate) struct MMC32Dbl {
    pub a: __m256i,
    pub b: __m256i,
}
impl MMC32Dbl {
    #[inline(always)]
    pub fn new(c1: u8, c2: u8) -> Self {
        Self {
            a: unsafe { _c32_value(c1) },
            b: unsafe { _c32_value(c2) },
        }
    }
}

impl From<MMC32Dbl> for MMC16Dbl {
    fn from(cc: MMC32Dbl) -> Self {
        Self {
            a: unsafe { _c16_from_c32(cc.a) },
            b: unsafe { _c16_from_c32(cc.b) },
        }
    }
}
