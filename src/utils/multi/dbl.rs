use super::super::{_c16_value, _c2_value, _c4_value, _c8_value};

#[derive(Debug, Copy, Clone, PartialEq)]
pub(crate) struct B16Dbl {
    pub a: u128,
    pub b: u128,
}
impl B16Dbl {
    #[allow(dead_code)]
    #[inline(always)]
    pub fn new(c1: u8, c2: u8) -> Self {
        Self {
            a: _c16_value(c1),
            b: _c16_value(c2),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub(crate) struct B8Dbl {
    pub a: u64,
    pub b: u64,
}
impl B8Dbl {
    #[allow(dead_code)]
    #[inline(always)]
    pub fn new(c1: u8, c2: u8) -> Self {
        Self {
            a: _c8_value(c1),
            b: _c8_value(c2),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub(crate) struct B4Dbl {
    pub a: u32,
    pub b: u32,
}
impl B4Dbl {
    #[allow(dead_code)]
    #[inline(always)]
    pub fn new(c1: u8, c2: u8) -> Self {
        Self {
            a: _c4_value(c1),
            b: _c4_value(c2),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub(crate) struct B2Dbl {
    pub a: u16,
    pub b: u16,
}
impl B2Dbl {
    #[allow(dead_code)]
    #[inline(always)]
    pub fn new(c1: u8, c2: u8) -> Self {
        Self {
            a: _c2_value(c1),
            b: _c2_value(c2),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub(crate) struct B1Dbl {
    pub a: u8,
    pub b: u8,
}
impl B1Dbl {
    #[allow(dead_code)]
    #[inline(always)]
    pub fn new(c1: u8, c2: u8) -> Self {
        Self { a: c1, b: c2 }
    }
}

impl From<B16Dbl> for B8Dbl {
    fn from(cc: B16Dbl) -> Self {
        Self {
            a: cc.a as u64,
            b: cc.b as u64,
        }
    }
}

impl From<B8Dbl> for B4Dbl {
    fn from(cc: B8Dbl) -> Self {
        Self {
            a: cc.a as u32,
            b: cc.b as u32,
        }
    }
}

impl From<B4Dbl> for B2Dbl {
    fn from(cc: B4Dbl) -> Self {
        Self {
            a: cc.a as u16,
            b: cc.b as u16,
        }
    }
}

impl From<B2Dbl> for B1Dbl {
    fn from(cc: B2Dbl) -> Self {
        Self {
            a: cc.a as u8,
            b: cc.b as u8,
        }
    }
}

#[cfg(test)]
mod mini {
    #![allow(clippy::clone_on_copy)]
    use super::*;
    //
    #[test]
    fn t_c16() {
        let a = B16Dbl::new(b'A', b'B');
        let b = a.clone();
        let c = a;
        assert_eq!(a.a, b.a);
        assert_eq!(a.a, c.a);
        assert_eq!(a, b);
        assert_eq!(format!("{a:?}"), "B16Dbl { a: 86738642548474510294585684247313465665, b: 88073083203066425837579310158810595906 }");
    }
    #[test]
    fn t_c8() {
        let a = B8Dbl::new(b'A', b'B');
        let b = a.clone();
        let c = a;
        assert_eq!(a.a, b.a);
        assert_eq!(a.a, c.a);
        assert_eq!(a, b);
        assert_eq!(
            format!("{a:?}"),
            "B8Dbl { a: 4702111234474983745, b: 4774451407313060418 }"
        );
    }
    #[test]
    fn t_c4() {
        let a = B4Dbl::new(b'A', b'B');
        let b = a.clone();
        let c = a;
        assert_eq!(a.a, b.a);
        assert_eq!(a.a, c.a);
        assert_eq!(a, b);
        assert_eq!(format!("{a:?}"), "B4Dbl { a: 1094795585, b: 1111638594 }");
    }
    #[test]
    fn t_c2() {
        let a = B2Dbl::new(b'A', b'B');
        let b = a.clone();
        let c = a;
        assert_eq!(a.a, b.a);
        assert_eq!(a.a, c.a);
        assert_eq!(a, b);
        assert_eq!(format!("{a:?}"), "B2Dbl { a: 16705, b: 16962 }");
    }
    #[test]
    fn t_c1() {
        let a = B1Dbl::new(b'A', b'B');
        let b = a.clone();
        let c = a;
        assert_eq!(a.a, b.a);
        assert_eq!(a.a, c.a);
        assert_eq!(a, b);
        assert_eq!(format!("{a:?}"), "B1Dbl { a: 65, b: 66 }");
    }
    #[test]
    fn t_into() {
        let a_c16 = B16Dbl::new(b'A', b'B');
        let a_c8: B8Dbl = a_c16.into();
        let a_c4: B4Dbl = a_c8.into();
        let a_c2: B2Dbl = a_c4.into();
        let a_c1: B1Dbl = a_c2.into();
        assert_eq!(a_c8, B8Dbl::new(b'A', b'B'));
        assert_eq!(a_c4, B4Dbl::new(b'A', b'B'));
        assert_eq!(a_c2, B2Dbl::new(b'A', b'B'));
        assert_eq!(a_c1, B1Dbl::new(b'A', b'B'));
    }
}
