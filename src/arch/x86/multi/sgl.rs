use super::super::{__m128i, __m256i};
use super::super::{_c16_from_c32, _c16_value, _c32_value};

#[derive(Copy, Clone)]
pub(crate) struct MMC16Sgl {
    pub a: __m128i,
}
impl MMC16Sgl {
    #[inline(always)]
    pub fn new(c1: u8) -> Self {
        Self {
            a: unsafe { _c16_value(c1) },
        }
    }
}

#[derive(Copy, Clone)]
pub(crate) struct MMC32Sgl {
    pub a: __m256i,
}
impl MMC32Sgl {
    #[inline(always)]
    pub fn new(c1: u8) -> Self {
        Self {
            a: unsafe { _c32_value(c1) },
        }
    }
}

impl From<MMC32Sgl> for MMC16Sgl {
    fn from(cc: MMC32Sgl) -> Self {
        Self {
            a: unsafe { _c16_from_c32(cc.a) },
        }
    }
}
