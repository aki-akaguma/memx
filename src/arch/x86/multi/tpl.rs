use super::{__m128i, __m256i};
use super::{_b16_eq, _b16_from_b32, _b16_value, _b32_eq, _b32_value};

#[derive(Debug, Copy, Clone)]
pub(crate) struct MMB16Tpl {
    pub v1: __m128i,
    pub v2: __m128i,
    pub v3: __m128i,
}
impl MMB16Tpl {
    #[allow(dead_code)]
    #[inline(always)]
    pub fn new(b1: u8, b2: u8, b3: u8) -> Self {
        Self {
            v1: unsafe { _b16_value(b1) },
            v2: unsafe { _b16_value(b2) },
            v3: unsafe { _b16_value(b3) },
        }
    }
}
impl PartialEq for MMB16Tpl {
    fn eq(&self, other: &Self) -> bool {
        let a = unsafe { _b16_eq(self.v1, other.v1) };
        let b = unsafe { _b16_eq(self.v2, other.v2) };
        let c = unsafe { _b16_eq(self.v3, other.v3) };
        a && b && c
    }
}

#[derive(Debug, Copy, Clone)]
pub(crate) struct MMB32Tpl {
    pub v1: __m256i,
    pub v2: __m256i,
    pub v3: __m256i,
}
impl MMB32Tpl {
    #[allow(dead_code)]
    #[inline(always)]
    pub fn new(b1: u8, b2: u8, b3: u8) -> Self {
        Self {
            v1: unsafe { _b32_value(b1) },
            v2: unsafe { _b32_value(b2) },
            v3: unsafe { _b32_value(b3) },
        }
    }
}
impl PartialEq for MMB32Tpl {
    fn eq(&self, other: &Self) -> bool {
        let a = unsafe { _b32_eq(self.v1, other.v1) };
        let b = unsafe { _b32_eq(self.v2, other.v2) };
        let c = unsafe { _b32_eq(self.v3, other.v3) };
        a && b && c
    }
}

impl From<MMB32Tpl> for MMB16Tpl {
    fn from(cc: MMB32Tpl) -> Self {
        Self {
            v1: unsafe { _b16_from_b32(cc.v1) },
            v2: unsafe { _b16_from_b32(cc.v2) },
            v3: unsafe { _b16_from_b32(cc.v3) },
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
    fn t_b32() {
        if !cpuid::has_avx2() {
            return;
        }
        let a = MMB32Tpl::new(b'A', b'B', b'C');
        let b = a.clone();
        let c = a;
        assert_eq!(a, b);
        assert_eq!(a, c);
        assert_eq!(
            format!("{a:?}"),
            "MMB32Tpl { v1: __m256i(4702111234474983745, 4702111234474983745, 4702111234474983745, 4702111234474983745), v2: __m256i(4774451407313060418, 4774451407313060418, 4774451407313060418, 4774451407313060418), v3: __m256i(4846791580151137091, 4846791580151137091, 4846791580151137091, 4846791580151137091) }"
        );
    }
    #[test]
    fn t_b16() {
        let a = MMB16Tpl::new(b'A', b'B', b'C');
        let b = a.clone();
        let c = a;
        assert_eq!(a, b);
        assert_eq!(a, c);
        assert_eq!(
            format!("{a:?}"),
            "MMB16Tpl { v1: __m128i(4702111234474983745, 4702111234474983745), v2: __m128i(4774451407313060418, 4774451407313060418), v3: __m128i(4846791580151137091, 4846791580151137091) }"
        );
    }
    #[test]
    fn t_into() {
        if !cpuid::has_avx2() {
            return;
        }
        let a_b32 = MMB32Tpl::new(b'A', b'B', b'C');
        let a_b16: MMB16Tpl = a_b32.into();
        assert_eq!(a_b32, MMB32Tpl::new(b'A', b'B', b'C'));
        assert_eq!(a_b16, MMB16Tpl::new(b'A', b'B', b'C'));
    }
}
