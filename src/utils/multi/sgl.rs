use super::{_b16_value, _b2_value, _b4_value, _b8_value};

#[derive(Debug, Copy, Clone, PartialEq)]
pub(crate) struct B16Sgl {
    pub v1: u128,
}
impl B16Sgl {
    #[allow(dead_code)]
    #[inline(always)]
    pub fn new(b1: u8) -> Self {
        Self { v1: _b16_value(b1) }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub(crate) struct B8Sgl {
    pub v1: u64,
}
impl B8Sgl {
    #[allow(dead_code)]
    #[inline(always)]
    pub fn new(b1: u8) -> Self {
        Self { v1: _b8_value(b1) }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub(crate) struct B4Sgl {
    pub v1: u32,
}
impl B4Sgl {
    #[allow(dead_code)]
    #[inline(always)]
    pub fn new(b1: u8) -> Self {
        Self { v1: _b4_value(b1) }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub(crate) struct B2Sgl {
    pub v1: u16,
}
impl B2Sgl {
    #[allow(dead_code)]
    #[inline(always)]
    pub fn new(b1: u8) -> Self {
        Self { v1: _b2_value(b1) }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub(crate) struct B1Sgl {
    pub v1: u8,
}
impl B1Sgl {
    #[allow(dead_code)]
    #[inline(always)]
    pub fn new(b1: u8) -> Self {
        Self { v1: b1 }
    }
}

impl From<B16Sgl> for B8Sgl {
    fn from(cc: B16Sgl) -> Self {
        Self { v1: cc.v1 as u64 }
    }
}

impl From<B8Sgl> for B4Sgl {
    fn from(cc: B8Sgl) -> Self {
        Self { v1: cc.v1 as u32 }
    }
}

impl From<B4Sgl> for B2Sgl {
    fn from(cc: B4Sgl) -> Self {
        Self { v1: cc.v1 as u16 }
    }
}

impl From<B8Sgl> for B1Sgl {
    fn from(cc: B8Sgl) -> Self {
        Self { v1: cc.v1 as u8 }
    }
}

impl From<B4Sgl> for B1Sgl {
    fn from(cc: B4Sgl) -> Self {
        Self { v1: cc.v1 as u8 }
    }
}

impl From<B2Sgl> for B1Sgl {
    fn from(cc: B2Sgl) -> Self {
        Self { v1: cc.v1 as u8 }
    }
}

#[cfg(test)]
mod mini {
    #![allow(clippy::clone_on_copy)]
    use super::*;
    //
    #[test]
    fn t_b16() {
        let a = B16Sgl::new(b'A');
        let b = a.clone();
        let c = a;
        assert_eq!(a.v1, b.v1);
        assert_eq!(a.v1, c.v1);
        assert_eq!(a, b);
        assert_eq!(
            format!("{a:?}"),
            "B16Sgl { v1: 86738642548474510294585684247313465665 }"
        );
    }
    #[test]
    fn t_b8() {
        let a = B8Sgl::new(b'A');
        let b = a.clone();
        let c = a;
        assert_eq!(a.v1, b.v1);
        assert_eq!(a.v1, c.v1);
        assert_eq!(a, b);
        assert_eq!(format!("{a:?}"), "B8Sgl { v1: 4702111234474983745 }");
    }
    #[test]
    fn t_b4() {
        let a = B4Sgl::new(b'A');
        let b = a.clone();
        let c = a;
        assert_eq!(a.v1, b.v1);
        assert_eq!(a.v1, c.v1);
        assert_eq!(a, b);
        assert_eq!(format!("{a:?}"), "B4Sgl { v1: 1094795585 }");
    }
    #[test]
    fn t_b2() {
        let a = B2Sgl::new(b'A');
        let b = a.clone();
        let c = a;
        assert_eq!(a.v1, b.v1);
        assert_eq!(a.v1, c.v1);
        assert_eq!(a, b);
        assert_eq!(format!("{a:?}"), "B2Sgl { v1: 16705 }");
    }
    #[test]
    fn t_b1() {
        let a = B1Sgl::new(b'A');
        let b = a.clone();
        let c = a;
        assert_eq!(a.v1, b.v1);
        assert_eq!(a.v1, c.v1);
        assert_eq!(a, b);
        assert_eq!(format!("{a:?}"), "B1Sgl { v1: 65 }");
    }
    #[test]
    fn t_into() {
        let a_b16 = B16Sgl::new(b'A');
        let a_b8: B8Sgl = a_b16.into();
        let a_b4: B4Sgl = a_b8.into();
        let a_b2: B2Sgl = a_b4.into();
        let a_b1: B1Sgl = a_b2.into();
        assert_eq!(a_b8, B8Sgl::new(b'A'));
        assert_eq!(a_b4, B4Sgl::new(b'A'));
        assert_eq!(a_b2, B2Sgl::new(b'A'));
        assert_eq!(a_b1, B1Sgl::new(b'A'));
    }
}
