use super::{__m128i, __m256i};
use super::{_b16_eq, _b16_from_b32, _b16_value, _b32_eq, _b32_value};

#[derive(Debug, Copy, Clone)]
pub(crate) struct MMB16Qpl {
    pub v1: __m128i,
    pub v2: __m128i,
    pub v3: __m128i,
    pub v4: __m128i,
}
impl MMB16Qpl {
    #[allow(dead_code)]
    #[inline(always)]
    pub fn new(b1: u8, b2: u8, b3: u8, b4: u8) -> Self {
        Self {
            v1: unsafe { _b16_value(b1) },
            v2: unsafe { _b16_value(b2) },
            v3: unsafe { _b16_value(b3) },
            v4: unsafe { _b16_value(b4) },
        }
    }
}
impl PartialEq for MMB16Qpl {
    fn eq(&self, other: &Self) -> bool {
        let a = unsafe { _b16_eq(self.v1, other.v1) };
        let b = unsafe { _b16_eq(self.v2, other.v2) };
        let c = unsafe { _b16_eq(self.v3, other.v3) };
        let d = unsafe { _b16_eq(self.v4, other.v4) };
        a && b && c && d
    }
}

#[derive(Debug, Copy, Clone)]
pub(crate) struct MMB32Qpl {
    pub v1: __m256i,
    pub v2: __m256i,
    pub v3: __m256i,
    pub v4: __m256i,
}
impl MMB32Qpl {
    #[allow(dead_code)]
    #[inline(always)]
    pub fn new(b1: u8, b2: u8, b3: u8, b4: u8) -> Self {
        Self {
            v1: unsafe { _b32_value(b1) },
            v2: unsafe { _b32_value(b2) },
            v3: unsafe { _b32_value(b3) },
            v4: unsafe { _b32_value(b4) },
        }
    }
}
impl PartialEq for MMB32Qpl {
    fn eq(&self, other: &Self) -> bool {
        let a = unsafe { _b32_eq(self.v1, other.v1) };
        let b = unsafe { _b32_eq(self.v2, other.v2) };
        let c = unsafe { _b32_eq(self.v3, other.v3) };
        let d = unsafe { _b32_eq(self.v4, other.v4) };
        a && b && c && d
    }
}

impl From<MMB32Qpl> for MMB16Qpl {
    fn from(cc: MMB32Qpl) -> Self {
        Self {
            v1: unsafe { _b16_from_b32(cc.v1) },
            v2: unsafe { _b16_from_b32(cc.v2) },
            v3: unsafe { _b16_from_b32(cc.v3) },
            v4: unsafe { _b16_from_b32(cc.v4) },
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
        let a = MMB32Qpl::new(b'A', b'B', b'C', b'D');
        let b = a.clone();
        let c = a;
        assert_eq!(a, b);
        assert_eq!(a, c);
        assert_eq!(
            format!("{a:?}"),
            "MMB32Qpl { v1: __m256i(4702111234474983745, 4702111234474983745, 4702111234474983745, 4702111234474983745), v2: __m256i(4774451407313060418, 4774451407313060418, 4774451407313060418, 4774451407313060418), v3: __m256i(4846791580151137091, 4846791580151137091, 4846791580151137091, 4846791580151137091), v4: __m256i(4919131752989213764, 4919131752989213764, 4919131752989213764, 4919131752989213764) }"
        );
    }
    #[test]
    fn t_b16() {
        let a = MMB16Qpl::new(b'A', b'B', b'C', b'D');
        let b = a.clone();
        let c = a;
        assert_eq!(a, b);
        assert_eq!(a, c);
        assert_eq!(
            format!("{a:?}"),
            "MMB16Qpl { v1: __m128i(4702111234474983745, 4702111234474983745), v2: __m128i(4774451407313060418, 4774451407313060418), v3: __m128i(4846791580151137091, 4846791580151137091), v4: __m128i(4919131752989213764, 4919131752989213764) }"
        );
    }
    #[test]
    fn t_into() {
        if !cpuid::has_avx2() {
            return;
        }
        let a_b32 = MMB32Qpl::new(b'A', b'B', b'C', b'D');
        let a_b16: MMB16Qpl = a_b32.into();
        assert_eq!(a_b32, MMB32Qpl::new(b'A', b'B', b'C', b'D'));
        assert_eq!(a_b16, MMB16Qpl::new(b'A', b'B', b'C', b'D'));
    }
}
