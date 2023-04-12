use super::super::{_c16_value, _c32_value, _c16_from_c32};
use super::super::{__m128i, __m256i};

#[derive(Copy, Clone)]
pub(crate) struct MMC16Qpl {
    pub a: __m128i,
    pub b: __m128i,
    pub c: __m128i,
    pub d: __m128i,
}
impl MMC16Qpl {
    #[inline(always)]
    pub fn new(c1: u8, c2: u8, c3: u8, c4: u8) -> Self {
        Self {
            a: unsafe { _c16_value(c1) },
            b: unsafe { _c16_value(c2) },
            c: unsafe { _c16_value(c3) },
            d: unsafe { _c16_value(c4) },
        }
    }
}

#[derive(Copy, Clone)]
pub(crate) struct MMC32Qpl {
    pub a: __m256i,
    pub b: __m256i,
    pub c: __m256i,
    pub d: __m256i,
}
impl MMC32Qpl {
    #[inline(always)]
    pub fn new(c1: u8, c2: u8, c3: u8, c4: u8) -> Self {
        Self {
            a: unsafe { _c32_value(c1) },
            b: unsafe { _c32_value(c2) },
            c: unsafe { _c32_value(c3) },
            d: unsafe { _c32_value(c4) },
        }
    }
}

impl From<MMC32Qpl> for MMC16Qpl {
    fn from(cc: MMC32Qpl) -> Self {
        Self {
            a: unsafe { _c16_from_c32(cc.a) },
            b: unsafe { _c16_from_c32(cc.b) },
            c: unsafe { _c16_from_c32(cc.c) },
            d: unsafe { _c16_from_c32(cc.d) },
        }
    }
}
