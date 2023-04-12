use super::super::{__m128i, __m256i};
use super::super::{_c16_eq, _c16_from_c32, _c16_value, _c32_eq, _c32_value};

#[derive(Debug, Copy, Clone)]
pub(crate) struct MMC16Tpl {
    pub a: __m128i,
    pub b: __m128i,
    pub c: __m128i,
}
impl MMC16Tpl {
    #[allow(dead_code)]
    #[inline(always)]
    pub fn new(c1: u8, c2: u8, c3: u8) -> Self {
        Self {
            a: unsafe { _c16_value(c1) },
            b: unsafe { _c16_value(c2) },
            c: unsafe { _c16_value(c3) },
        }
    }
}
impl PartialEq for MMC16Tpl {
    fn eq(&self, other: &Self) -> bool {
        let a = unsafe { _c16_eq(self.a, other.a) };
        let b = unsafe { _c16_eq(self.b, other.b) };
        let c = unsafe { _c16_eq(self.c, other.c) };
        a && b && c
    }
}

#[derive(Debug, Copy, Clone)]
pub(crate) struct MMC32Tpl {
    pub a: __m256i,
    pub b: __m256i,
    pub c: __m256i,
}
impl MMC32Tpl {
    #[allow(dead_code)]
    #[inline(always)]
    pub fn new(c1: u8, c2: u8, c3: u8) -> Self {
        Self {
            a: unsafe { _c32_value(c1) },
            b: unsafe { _c32_value(c2) },
            c: unsafe { _c32_value(c3) },
        }
    }
}
impl PartialEq for MMC32Tpl {
    fn eq(&self, other: &Self) -> bool {
        let a = unsafe { _c32_eq(self.a, other.a) };
        let b = unsafe { _c32_eq(self.b, other.b) };
        let c = unsafe { _c32_eq(self.c, other.c) };
        a && b && c
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
        let a = MMC32Tpl::new(b'A', b'B', b'C');
        let b = a.clone();
        let c = a;
        assert_eq!(a, b);
        assert_eq!(a, c);
        assert_eq!(
            format!("{a:?}"),
            "MMC32Tpl { a: __m256i(4702111234474983745, 4702111234474983745, 4702111234474983745, 4702111234474983745), b: __m256i(4774451407313060418, 4774451407313060418, 4774451407313060418, 4774451407313060418), c: __m256i(4846791580151137091, 4846791580151137091, 4846791580151137091, 4846791580151137091) }"
        );
    }
    #[test]
    fn t_c16() {
        let a = MMC16Tpl::new(b'A', b'B', b'C');
        let b = a.clone();
        let c = a;
        assert_eq!(a, b);
        assert_eq!(a, c);
        assert_eq!(
            format!("{a:?}"),
            "MMC16Tpl { a: __m128i(4702111234474983745, 4702111234474983745), b: __m128i(4774451407313060418, 4774451407313060418), c: __m128i(4846791580151137091, 4846791580151137091) }"
        );
    }
    #[test]
    fn t_into() {
        if !cpuid::has_avx2() {
            return;
        }
        let a_c32 = MMC32Tpl::new(b'A', b'B', b'C');
        let a_c16: MMC16Tpl = a_c32.into();
        assert_eq!(a_c32, MMC32Tpl::new(b'A', b'B', b'C'));
        assert_eq!(a_c16, MMC16Tpl::new(b'A', b'B', b'C'));
    }
}
