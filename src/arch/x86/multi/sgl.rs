use super::super::{__m128i, __m256i};
use super::super::{_c16_eq, _c16_from_c32, _c16_value, _c32_eq, _c32_value};

#[derive(Debug, Copy, Clone)]
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
impl PartialEq for MMC16Sgl {
    fn eq(&self, other: &Self) -> bool {
        unsafe { _c16_eq(self.a, other.a) }
    }
}

#[derive(Debug, Copy, Clone)]
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
impl PartialEq for MMC32Sgl {
    fn eq(&self, other: &Self) -> bool {
        unsafe { _c32_eq(self.a, other.a) }
    }
}

impl From<MMC32Sgl> for MMC16Sgl {
    fn from(cc: MMC32Sgl) -> Self {
        Self {
            a: unsafe { _c16_from_c32(cc.a) },
        }
    }
}

#[cfg(test)]
mod mini {
    #![allow(clippy::clone_on_copy)]
    use super::super::super::cpuid;
    use super::*;
    //
    #[test]
    fn t_c32() {
        if !cpuid::has_avx2() {
            return;
        }
        let a = MMC32Sgl::new(b'A');
        let b = a.clone();
        let c = a;
        assert_eq!(a, b);
        assert_eq!(a, c);
        assert_eq!(
            format!("{a:?}"),
            "MMC32Sgl { a: __m256i(4702111234474983745, 4702111234474983745, 4702111234474983745, 4702111234474983745) }"
        );
    }
    #[test]
    fn t_c16() {
        let a = MMC16Sgl::new(b'A');
        let b = a.clone();
        let c = a;
        assert_eq!(a, b);
        assert_eq!(a, c);
        assert_eq!(
            format!("{a:?}"),
            "MMC16Sgl { a: __m128i(4702111234474983745, 4702111234474983745) }"
        );
    }
    #[test]
    fn t_into() {
        if !cpuid::has_avx2() {
            return;
        }
        let a_c32 = MMC32Sgl::new(b'A');
        let a_c16: MMC16Sgl = a_c32.into();
        assert_eq!(a_c32, MMC32Sgl::new(b'A'));
        assert_eq!(a_c16, MMC16Sgl::new(b'A'));
    }
}
