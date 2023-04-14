use super::super::{__m128i, __m256i};
use super::super::{_c16_eq, _c16_from_c32, _c16_value, _c32_eq, _c32_value};

#[derive(Debug, Copy, Clone)]
pub(crate) struct MMB16Qpl {
    pub a: __m128i,
    pub b: __m128i,
    pub c: __m128i,
    pub d: __m128i,
}
impl MMB16Qpl {
    #[allow(dead_code)]
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
impl PartialEq for MMB16Qpl {
    fn eq(&self, other: &Self) -> bool {
        let a = unsafe { _c16_eq(self.a, other.a) };
        let b = unsafe { _c16_eq(self.b, other.b) };
        let c = unsafe { _c16_eq(self.c, other.c) };
        let d = unsafe { _c16_eq(self.d, other.d) };
        a && b && c && d
    }
}

#[derive(Debug, Copy, Clone)]
pub(crate) struct MMB32Qpl {
    pub a: __m256i,
    pub b: __m256i,
    pub c: __m256i,
    pub d: __m256i,
}
impl MMB32Qpl {
    #[allow(dead_code)]
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
impl PartialEq for MMB32Qpl {
    fn eq(&self, other: &Self) -> bool {
        let a = unsafe { _c32_eq(self.a, other.a) };
        let b = unsafe { _c32_eq(self.b, other.b) };
        let c = unsafe { _c32_eq(self.c, other.c) };
        let d = unsafe { _c32_eq(self.d, other.d) };
        a && b && c && d
    }
}

impl From<MMB32Qpl> for MMB16Qpl {
    fn from(cc: MMB32Qpl) -> Self {
        Self {
            a: unsafe { _c16_from_c32(cc.a) },
            b: unsafe { _c16_from_c32(cc.b) },
            c: unsafe { _c16_from_c32(cc.c) },
            d: unsafe { _c16_from_c32(cc.d) },
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
        let a = MMB32Qpl::new(b'A', b'B', b'C', b'D');
        let b = a.clone();
        let c = a;
        assert_eq!(a, b);
        assert_eq!(a, c);
        assert_eq!(
            format!("{a:?}"),
            "MMB32Qpl { a: __m256i(4702111234474983745, 4702111234474983745, 4702111234474983745, 4702111234474983745), b: __m256i(4774451407313060418, 4774451407313060418, 4774451407313060418, 4774451407313060418), c: __m256i(4846791580151137091, 4846791580151137091, 4846791580151137091, 4846791580151137091), d: __m256i(4919131752989213764, 4919131752989213764, 4919131752989213764, 4919131752989213764) }"
        );
    }
    #[test]
    fn t_c16() {
        let a = MMB16Qpl::new(b'A', b'B', b'C', b'D');
        let b = a.clone();
        let c = a;
        assert_eq!(a, b);
        assert_eq!(a, c);
        assert_eq!(
            format!("{a:?}"),
            "MMB16Qpl { a: __m128i(4702111234474983745, 4702111234474983745), b: __m128i(4774451407313060418, 4774451407313060418), c: __m128i(4846791580151137091, 4846791580151137091), d: __m128i(4919131752989213764, 4919131752989213764) }"
        );
    }
    #[test]
    fn t_into() {
        if !cpuid::has_avx2() {
            return;
        }
        let a_c32 = MMB32Qpl::new(b'A', b'B', b'C', b'D');
        let a_c16: MMB16Qpl = a_c32.into();
        assert_eq!(a_c32, MMB32Qpl::new(b'A', b'B', b'C', b'D'));
        assert_eq!(a_c16, MMB16Qpl::new(b'A', b'B', b'C', b'D'));
    }
}
