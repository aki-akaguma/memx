use super::super::{_c16_value, _c2_value, _c4_value, _c8_value};

#[derive(Debug, Copy, Clone, PartialEq)]
pub(crate) struct B16Sgl {
    pub a: u128,
}
impl B16Sgl {
    #[allow(dead_code)]
    #[inline(always)]
    pub fn new(c1: u8) -> Self {
        Self { a: _c16_value(c1) }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub(crate) struct B8Sgl {
    pub a: u64,
}
impl B8Sgl {
    #[allow(dead_code)]
    #[inline(always)]
    pub fn new(c1: u8) -> Self {
        Self { a: _c8_value(c1) }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub(crate) struct B4Sgl {
    pub a: u32,
}
impl B4Sgl {
    #[allow(dead_code)]
    #[inline(always)]
    pub fn new(c1: u8) -> Self {
        Self { a: _c4_value(c1) }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub(crate) struct B2Sgl {
    pub a: u16,
}
impl B2Sgl {
    #[allow(dead_code)]
    #[inline(always)]
    pub fn new(c1: u8) -> Self {
        Self { a: _c2_value(c1) }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub(crate) struct B1Sgl {
    pub a: u8,
}
impl B1Sgl {
    #[allow(dead_code)]
    #[inline(always)]
    pub fn new(c1: u8) -> Self {
        Self { a: c1 }
    }
}

impl From<B16Sgl> for B8Sgl {
    fn from(cc: B16Sgl) -> Self {
        Self { a: cc.a as u64 }
    }
}

impl From<B8Sgl> for B4Sgl {
    fn from(cc: B8Sgl) -> Self {
        Self { a: cc.a as u32 }
    }
}

impl From<B4Sgl> for B2Sgl {
    fn from(cc: B4Sgl) -> Self {
        Self { a: cc.a as u16 }
    }
}

impl From<B2Sgl> for B1Sgl {
    fn from(cc: B2Sgl) -> Self {
        Self { a: cc.a as u8 }
    }
}

#[cfg(test)]
mod mini {
    #![allow(clippy::clone_on_copy)]
    use super::*;
    //
    #[test]
    fn t_c16() {
        let a = B16Sgl::new(b'A');
        let b = a.clone();
        let c = a;
        assert_eq!(a.a, b.a);
        assert_eq!(a.a, c.a);
        assert_eq!(a, b);
        assert_eq!(
            format!("{a:?}"),
            "B16Sgl { a: 86738642548474510294585684247313465665 }"
        );
    }
    #[test]
    fn t_c8() {
        let a = B8Sgl::new(b'A');
        let b = a.clone();
        let c = a;
        assert_eq!(a.a, b.a);
        assert_eq!(a.a, c.a);
        assert_eq!(a, b);
        assert_eq!(format!("{a:?}"), "B8Sgl { a: 4702111234474983745 }");
    }
    #[test]
    fn t_c4() {
        let a = B4Sgl::new(b'A');
        let b = a.clone();
        let c = a;
        assert_eq!(a.a, b.a);
        assert_eq!(a.a, c.a);
        assert_eq!(a, b);
        assert_eq!(format!("{a:?}"), "B4Sgl { a: 1094795585 }");
    }
    #[test]
    fn t_c2() {
        let a = B2Sgl::new(b'A');
        let b = a.clone();
        let c = a;
        assert_eq!(a.a, b.a);
        assert_eq!(a.a, c.a);
        assert_eq!(a, b);
        assert_eq!(format!("{a:?}"), "B2Sgl { a: 16705 }");
    }
    #[test]
    fn t_c1() {
        let a = B1Sgl::new(b'A');
        let b = a.clone();
        let c = a;
        assert_eq!(a.a, b.a);
        assert_eq!(a.a, c.a);
        assert_eq!(a, b);
        assert_eq!(format!("{a:?}"), "B1Sgl { a: 65 }");
    }
    #[test]
    fn t_into() {
        let a_c16 = B16Sgl::new(b'A');
        let a_c8: B8Sgl = a_c16.into();
        let a_c4: B4Sgl = a_c8.into();
        let a_c2: B2Sgl = a_c4.into();
        let a_c1: B1Sgl = a_c2.into();
        assert_eq!(a_c8, B8Sgl::new(b'A'));
        assert_eq!(a_c4, B4Sgl::new(b'A'));
        assert_eq!(a_c2, B2Sgl::new(b'A'));
        assert_eq!(a_c1, B1Sgl::new(b'A'));
    }
}
