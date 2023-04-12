use super::super::{_c16_value, _c2_value, _c4_value, _c8_value};

#[derive(Debug, Copy, Clone, PartialEq)]
pub(crate) struct C16Sgl {
    pub a: u128,
}
impl C16Sgl {
    #[allow(dead_code)]
    #[inline(always)]
    pub fn new(c1: u8) -> Self {
        Self { a: _c16_value(c1) }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub(crate) struct C8Sgl {
    pub a: u64,
}
impl C8Sgl {
    #[allow(dead_code)]
    #[inline(always)]
    pub fn new(c1: u8) -> Self {
        Self { a: _c8_value(c1) }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub(crate) struct C4Sgl {
    pub a: u32,
}
impl C4Sgl {
    #[allow(dead_code)]
    #[inline(always)]
    pub fn new(c1: u8) -> Self {
        Self { a: _c4_value(c1) }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub(crate) struct C2Sgl {
    pub a: u16,
}
impl C2Sgl {
    #[allow(dead_code)]
    #[inline(always)]
    pub fn new(c1: u8) -> Self {
        Self { a: _c2_value(c1) }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub(crate) struct C1Sgl {
    pub a: u8,
}
impl C1Sgl {
    #[allow(dead_code)]
    #[inline(always)]
    pub fn new(c1: u8) -> Self {
        Self { a: c1 }
    }
}

impl From<C16Sgl> for C8Sgl {
    fn from(cc: C16Sgl) -> Self {
        Self { a: cc.a as u64 }
    }
}

impl From<C8Sgl> for C4Sgl {
    fn from(cc: C8Sgl) -> Self {
        Self { a: cc.a as u32 }
    }
}

impl From<C4Sgl> for C2Sgl {
    fn from(cc: C4Sgl) -> Self {
        Self { a: cc.a as u16 }
    }
}

impl From<C2Sgl> for C1Sgl {
    fn from(cc: C2Sgl) -> Self {
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
        let a = C16Sgl::new(b'A');
        let b = a.clone();
        let c = a;
        assert_eq!(a.a, b.a);
        assert_eq!(a.a, c.a);
        assert_eq!(a, b);
        assert_eq!(
            format!("{a:?}"),
            "C16Sgl { a: 86738642548474510294585684247313465665 }"
        );
    }
    #[test]
    fn t_c8() {
        let a = C8Sgl::new(b'A');
        let b = a.clone();
        let c = a;
        assert_eq!(a.a, b.a);
        assert_eq!(a.a, c.a);
        assert_eq!(a, b);
        assert_eq!(format!("{a:?}"), "C8Sgl { a: 4702111234474983745 }");
    }
    #[test]
    fn t_c4() {
        let a = C4Sgl::new(b'A');
        let b = a.clone();
        let c = a;
        assert_eq!(a.a, b.a);
        assert_eq!(a.a, c.a);
        assert_eq!(a, b);
        assert_eq!(format!("{a:?}"), "C4Sgl { a: 1094795585 }");
    }
    #[test]
    fn t_c2() {
        let a = C2Sgl::new(b'A');
        let b = a.clone();
        let c = a;
        assert_eq!(a.a, b.a);
        assert_eq!(a.a, c.a);
        assert_eq!(a, b);
        assert_eq!(format!("{a:?}"), "C2Sgl { a: 16705 }");
    }
    #[test]
    fn t_c1() {
        let a = C1Sgl::new(b'A');
        let b = a.clone();
        let c = a;
        assert_eq!(a.a, b.a);
        assert_eq!(a.a, c.a);
        assert_eq!(a, b);
        assert_eq!(format!("{a:?}"), "C1Sgl { a: 65 }");
    }
    #[test]
    fn t_into() {
        let a_c16 = C16Sgl::new(b'A');
        let a_c8: C8Sgl = a_c16.into();
        let a_c4: C4Sgl = a_c8.into();
        let a_c2: C2Sgl = a_c4.into();
        let a_c1: C1Sgl = a_c2.into();
        assert_eq!(a_c8, C8Sgl::new(b'A'));
        assert_eq!(a_c4, C4Sgl::new(b'A'));
        assert_eq!(a_c2, C2Sgl::new(b'A'));
        assert_eq!(a_c1, C1Sgl::new(b'A'));
    }
}
