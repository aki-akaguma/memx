use super::super::{__m128i, __m256i};
use super::super::{_c16_eq, _c16_from_c32, _c16_value, _c32_eq, _c32_value};

#[derive(Debug, Copy, Clone)]
pub(crate) struct MMB16Dbl {
    pub a: __m128i,
    pub b: __m128i,
}
impl MMB16Dbl {
    #[inline(always)]
    pub fn new(c1: u8, c2: u8) -> Self {
        Self {
            a: unsafe { _c16_value(c1) },
            b: unsafe { _c16_value(c2) },
        }
    }
}
impl PartialEq for MMB16Dbl {
    fn eq(&self, other: &Self) -> bool {
        let a = unsafe { _c16_eq(self.a, other.a) };
        let b = unsafe { _c16_eq(self.b, other.b) };
        a && b
    }
}

#[derive(Debug, Copy, Clone)]
pub(crate) struct MMB32Dbl {
    pub a: __m256i,
    pub b: __m256i,
}
impl MMB32Dbl {
    #[inline(always)]
    pub fn new(c1: u8, c2: u8) -> Self {
        Self {
            a: unsafe { _c32_value(c1) },
            b: unsafe { _c32_value(c2) },
        }
    }
}
impl PartialEq for MMB32Dbl {
    fn eq(&self, other: &Self) -> bool {
        let a = unsafe { _c32_eq(self.a, other.a) };
        let b = unsafe { _c32_eq(self.b, other.b) };
        a && b
    }
}

impl From<MMB32Dbl> for MMB16Dbl {
    fn from(cc: MMB32Dbl) -> Self {
        Self {
            a: unsafe { _c16_from_c32(cc.a) },
            b: unsafe { _c16_from_c32(cc.b) },
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
        let a = MMB32Dbl::new(b'A', b'B');
        let b = a.clone();
        let c = a;
        assert_eq!(a, b);
        assert_eq!(a, c);
        assert_eq!(
            format!("{a:?}"),
            "MMB32Dbl { a: __m256i(4702111234474983745, 4702111234474983745, 4702111234474983745, 4702111234474983745), b: __m256i(4774451407313060418, 4774451407313060418, 4774451407313060418, 4774451407313060418) }"
        );
    }
    #[test]
    fn t_c16() {
        let a = MMB16Dbl::new(b'A', b'B');
        let b = a.clone();
        let c = a;
        assert_eq!(a, b);
        assert_eq!(a, c);
        assert_eq!(
            format!("{a:?}"),
            "MMB16Dbl { a: __m128i(4702111234474983745, 4702111234474983745), b: __m128i(4774451407313060418, 4774451407313060418) }"
        );
    }
    #[test]
    fn t_into() {
        if !cpuid::has_avx2() {
            return;
        }
        let a_c32 = MMB32Dbl::new(b'A', b'B');
        let a_c16: MMB16Dbl = a_c32.into();
        assert_eq!(a_c32, MMB32Dbl::new(b'A', b'B'));
        assert_eq!(a_c16, MMB16Dbl::new(b'A', b'B'));
    }
}
