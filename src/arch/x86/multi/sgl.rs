use super::{__m128i, __m256i};
use super::{_b16_eq, _b16_from_b32, _b16_value, _b32_eq, _b32_value};

#[derive(Debug, Copy, Clone)]
pub(crate) struct MMB16Sgl {
    pub v1: __m128i,
}
impl MMB16Sgl {
    #[inline(always)]
    pub fn new(b1: u8) -> Self {
        Self {
            v1: unsafe { _b16_value(b1) },
        }
    }
}
impl PartialEq for MMB16Sgl {
    fn eq(&self, other: &Self) -> bool {
        unsafe { _b16_eq(self.v1, other.v1) }
    }
}

#[derive(Debug, Copy, Clone)]
pub(crate) struct MMB32Sgl {
    pub v1: __m256i,
}
impl MMB32Sgl {
    #[inline(always)]
    pub fn new(b1: u8) -> Self {
        Self {
            v1: unsafe { _b32_value(b1) },
        }
    }
}
impl PartialEq for MMB32Sgl {
    fn eq(&self, other: &Self) -> bool {
        unsafe { _b32_eq(self.v1, other.v1) }
    }
}

impl From<MMB32Sgl> for MMB16Sgl {
    fn from(cc: MMB32Sgl) -> Self {
        Self {
            v1: unsafe { _b16_from_b32(cc.v1) },
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
        let a = MMB32Sgl::new(b'A');
        let b = a.clone();
        let c = a;
        assert_eq!(a, b);
        assert_eq!(a, c);
        assert_eq!(
            format!("{a:?}"),
            "MMB32Sgl { v1: __m256i(4702111234474983745, 4702111234474983745, 4702111234474983745, 4702111234474983745) }"
        );
    }
    #[test]
    fn t_b16() {
        let a = MMB16Sgl::new(b'A');
        let b = a.clone();
        let c = a;
        assert_eq!(a, b);
        assert_eq!(a, c);
        assert_eq!(
            format!("{a:?}"),
            "MMB16Sgl { v1: __m128i(4702111234474983745, 4702111234474983745) }"
        );
    }
    #[test]
    fn t_into() {
        if !cpuid::has_avx2() {
            return;
        }
        let a_b32 = MMB32Sgl::new(b'A');
        let a_b16: MMB16Sgl = a_b32.into();
        assert_eq!(a_b32, MMB32Sgl::new(b'A'));
        assert_eq!(a_b16, MMB16Sgl::new(b'A'));
    }
}
