use super::super::{_c16_value, _c2_value, _c4_value, _c8_value};

#[derive(Debug, Copy, Clone, PartialEq)]
pub(crate) struct B16Tpl {
    pub a: u128,
    pub b: u128,
    pub c: u128,
}
impl B16Tpl {
    #[allow(dead_code)]
    #[inline(always)]
    pub fn new(c1: u8, c2: u8, c3: u8) -> Self {
        Self {
            a: _c16_value(c1),
            b: _c16_value(c2),
            c: _c16_value(c3),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub(crate) struct B8Tpl {
    pub a: u64,
    pub b: u64,
    pub c: u64,
}
impl B8Tpl {
    #[allow(dead_code)]
    #[inline(always)]
    pub fn new(c1: u8, c2: u8, c3: u8) -> Self {
        Self {
            a: _c8_value(c1),
            b: _c8_value(c2),
            c: _c8_value(c3),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub(crate) struct B4Tpl {
    pub a: u32,
    pub b: u32,
    pub c: u32,
}
impl B4Tpl {
    #[allow(dead_code)]
    #[inline(always)]
    pub fn new(c1: u8, c2: u8, c3: u8) -> Self {
        Self {
            a: _c4_value(c1),
            b: _c4_value(c2),
            c: _c4_value(c3),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub(crate) struct B2Tpl {
    pub a: u16,
    pub b: u16,
    pub c: u16,
}
impl B2Tpl {
    #[allow(dead_code)]
    #[inline(always)]
    pub fn new(c1: u8, c2: u8, c3: u8) -> Self {
        Self {
            a: _c2_value(c1),
            b: _c2_value(c2),
            c: _c2_value(c3),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub(crate) struct B1Tpl {
    pub a: u8,
    pub b: u8,
    pub c: u8,
}
impl B1Tpl {
    #[allow(dead_code)]
    #[inline(always)]
    pub fn new(c1: u8, c2: u8, c3: u8) -> Self {
        Self {
            a: c1,
            b: c2,
            c: c3,
        }
    }
}

impl From<B16Tpl> for B8Tpl {
    fn from(cc: B16Tpl) -> Self {
        Self {
            a: cc.a as u64,
            b: cc.b as u64,
            c: cc.c as u64,
        }
    }
}

impl From<B8Tpl> for B4Tpl {
    fn from(cc: B8Tpl) -> Self {
        Self {
            a: cc.a as u32,
            b: cc.b as u32,
            c: cc.c as u32,
        }
    }
}

impl From<B4Tpl> for B2Tpl {
    fn from(cc: B4Tpl) -> Self {
        Self {
            a: cc.a as u16,
            b: cc.b as u16,
            c: cc.c as u16,
        }
    }
}

impl From<B2Tpl> for B1Tpl {
    fn from(cc: B2Tpl) -> Self {
        Self {
            a: cc.a as u8,
            b: cc.b as u8,
            c: cc.c as u8,
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
        let a = B16Tpl::new(b'A', b'B', b'C');
        let b = a.clone();
        let c = a;
        assert_eq!(a.a, b.a);
        assert_eq!(a.a, c.a);
        assert_eq!(a, b);
        assert_eq!(format!("{a:?}"), "B16Tpl { a: 86738642548474510294585684247313465665, b: 88073083203066425837579310158810595906, c: 89407523857658341380572936070307726147 }");
    }
    #[test]
    fn t_c8() {
        let a = B8Tpl::new(b'A', b'B', b'C');
        let b = a.clone();
        let c = a;
        assert_eq!(a.a, b.a);
        assert_eq!(a.a, c.a);
        assert_eq!(a, b);
        assert_eq!(
            format!("{a:?}"),
            "B8Tpl { a: 4702111234474983745, b: 4774451407313060418, c: 4846791580151137091 }"
        );
    }
    #[test]
    fn t_c4() {
        let a = B4Tpl::new(b'A', b'B', b'C');
        let b = a.clone();
        let c = a;
        assert_eq!(a.a, b.a);
        assert_eq!(a.a, c.a);
        assert_eq!(a, b);
        assert_eq!(
            format!("{a:?}"),
            "B4Tpl { a: 1094795585, b: 1111638594, c: 1128481603 }"
        );
    }
    #[test]
    fn t_c2() {
        let a = B2Tpl::new(b'A', b'B', b'C');
        let b = a.clone();
        let c = a;
        assert_eq!(a.a, b.a);
        assert_eq!(a.a, c.a);
        assert_eq!(a, b);
        assert_eq!(format!("{a:?}"), "B2Tpl { a: 16705, b: 16962, c: 17219 }");
    }
    #[test]
    fn t_c1() {
        let a = B1Tpl::new(b'A', b'B', b'C');
        let b = a.clone();
        let c = a;
        assert_eq!(a.a, b.a);
        assert_eq!(a.a, c.a);
        assert_eq!(a, b);
        assert_eq!(format!("{a:?}"), "B1Tpl { a: 65, b: 66, c: 67 }");
    }
    #[test]
    fn t_into() {
        let a_c16 = B16Tpl::new(b'A', b'B', b'C');
        let a_c8: B8Tpl = a_c16.into();
        let a_c4: B4Tpl = a_c8.into();
        let a_c2: B2Tpl = a_c4.into();
        let a_c1: B1Tpl = a_c2.into();
        assert_eq!(a_c8, B8Tpl::new(b'A', b'B', b'C'));
        assert_eq!(a_c4, B4Tpl::new(b'A', b'B', b'C'));
        assert_eq!(a_c2, B2Tpl::new(b'A', b'B', b'C'));
        assert_eq!(a_c1, B1Tpl::new(b'A', b'B', b'C'));
    }
}
