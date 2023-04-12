use super::super::{_c16_value, _c32_value, _c16_from_c32};
use super::super::{__m128i, __m256i};

#[derive(Copy, Clone)]
pub(crate) struct MMC16Tpl {
    pub a: __m128i,
    pub b: __m128i,
    pub c: __m128i,
}
impl MMC16Tpl {
    #[inline(always)]
    pub fn new(c1: u8, c2: u8, c3: u8) -> Self {
        Self {
            a: unsafe { _c16_value(c1) },
            b: unsafe { _c16_value(c2) },
            c: unsafe { _c16_value(c3) },
        }
    }
}

#[derive(Copy, Clone)]
pub(crate) struct MMC32Tpl {
    pub a: __m256i,
    pub b: __m256i,
    pub c: __m256i,
}
impl MMC32Tpl {
    #[inline(always)]
    pub fn new(c1: u8, c2: u8, c3: u8) -> Self {
        Self {
            a: unsafe { _c32_value(c1) },
            b: unsafe { _c32_value(c2) },
            c: unsafe { _c32_value(c3) },
        }
    }
}

impl From<MMC32Tpl> for MMC16Tpl {
    fn from(cc: MMC32Tpl) -> Self {
        Self {
            a: unsafe { _c16_from_c32(cc.a) },
            b: unsafe { _c16_from_c32(cc.b) },
            c: unsafe { _c16_from_c32(cc.c) },
        }
    }
}
