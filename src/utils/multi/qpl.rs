use super::super::{_c16_value, _c2_value, _c4_value, _c8_value};

#[derive(Debug, Copy, Clone, PartialEq)]
pub(crate) struct C16Qpl {
    pub a: u128,
    pub b: u128,
    pub c: u128,
    pub d: u128,
}
impl C16Qpl {
    #[allow(dead_code)]
    #[inline(always)]
    pub fn new(c1: u8, c2: u8, c3: u8, c4: u8) -> Self {
        Self {
            a: _c16_value(c1),
            b: _c16_value(c2),
            c: _c16_value(c3),
            d: _c16_value(c4),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub(crate) struct C8Qpl {
    pub a: u64,
    pub b: u64,
    pub c: u64,
    pub d: u64,
}
impl C8Qpl {
    #[allow(dead_code)]
    #[inline(always)]
    pub fn new(c1: u8, c2: u8, c3: u8, c4: u8) -> Self {
        Self {
            a: _c8_value(c1),
            b: _c8_value(c2),
            c: _c8_value(c3),
            d: _c8_value(c4),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub(crate) struct C4Qpl {
    pub a: u32,
    pub b: u32,
    pub c: u32,
    pub d: u32,
}
impl C4Qpl {
    #[allow(dead_code)]
    #[inline(always)]
    pub fn new(c1: u8, c2: u8, c3: u8, c4: u8) -> Self {
        Self {
            a: _c4_value(c1),
            b: _c4_value(c2),
            c: _c4_value(c3),
            d: _c4_value(c4),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub(crate) struct C2Qpl {
    pub a: u16,
    pub b: u16,
    pub c: u16,
    pub d: u16,
}
impl C2Qpl {
    #[allow(dead_code)]
    #[inline(always)]
    pub fn new(c1: u8, c2: u8, c3: u8, c4: u8) -> Self {
        Self {
            a: _c2_value(c1),
            b: _c2_value(c2),
            c: _c2_value(c3),
            d: _c2_value(c4),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub(crate) struct C1Qpl {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
}
impl C1Qpl {
    #[allow(dead_code)]
    #[inline(always)]
    pub fn new(c1: u8, c2: u8, c3: u8, c4: u8) -> Self {
        Self {
            a: c1,
            b: c2,
            c: c3,
            d: c4,
        }
    }
}

impl From<C16Qpl> for C8Qpl {
    fn from(cc: C16Qpl) -> Self {
        Self {
            a: cc.a as u64,
            b: cc.b as u64,
            c: cc.c as u64,
            d: cc.d as u64,
        }
    }
}

impl From<C8Qpl> for C4Qpl {
    fn from(cc: C8Qpl) -> Self {
        Self {
            a: cc.a as u32,
            b: cc.b as u32,
            c: cc.c as u32,
            d: cc.d as u32,
        }
    }
}

impl From<C4Qpl> for C2Qpl {
    fn from(cc: C4Qpl) -> Self {
        Self {
            a: cc.a as u16,
            b: cc.b as u16,
            c: cc.c as u16,
            d: cc.d as u16,
        }
    }
}

impl From<C2Qpl> for C1Qpl {
    fn from(cc: C2Qpl) -> Self {
        Self {
            a: cc.a as u8,
            b: cc.b as u8,
            c: cc.c as u8,
            d: cc.d as u8,
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
        let a = C16Qpl::new(b'A', b'B', b'C', b'D');
        let b = a.clone();
        let c = a;
        assert_eq!(a.a, b.a);
        assert_eq!(a.a, c.a);
        assert_eq!(a, b);
        assert_eq!(format!("{a:?}"), "C16Qpl { a: 86738642548474510294585684247313465665, b: 88073083203066425837579310158810595906, c: 89407523857658341380572936070307726147, d: 90741964512250256923566561981804856388 }");
    }
    #[test]
    fn t_c8() {
        let a = C8Qpl::new(b'A', b'B', b'C', b'D');
        let b = a.clone();
        let c = a;
        assert_eq!(a.a, b.a);
        assert_eq!(a.a, c.a);
        assert_eq!(a, b);
        assert_eq!(
            format!("{a:?}"),
            "C8Qpl { a: 4702111234474983745, b: 4774451407313060418, c: 4846791580151137091, d: 4919131752989213764 }"
        );
    }
    #[test]
    fn t_c4() {
        let a = C4Qpl::new(b'A', b'B', b'C', b'D');
        let b = a.clone();
        let c = a;
        assert_eq!(a.a, b.a);
        assert_eq!(a.a, c.a);
        assert_eq!(a, b);
        assert_eq!(
            format!("{a:?}"),
            "C4Qpl { a: 1094795585, b: 1111638594, c: 1128481603, d: 1145324612 }"
        );
    }
    #[test]
    fn t_c2() {
        let a = C2Qpl::new(b'A', b'B', b'C', b'D');
        let b = a.clone();
        let c = a;
        assert_eq!(a.a, b.a);
        assert_eq!(a.a, c.a);
        assert_eq!(a, b);
        assert_eq!(
            format!("{a:?}"),
            "C2Qpl { a: 16705, b: 16962, c: 17219, d: 17476 }"
        );
    }
    #[test]
    fn t_c1() {
        let a = C1Qpl::new(b'A', b'B', b'C', b'D');
        let b = a.clone();
        let c = a;
        assert_eq!(a.a, b.a);
        assert_eq!(a.a, c.a);
        assert_eq!(a, b);
        assert_eq!(format!("{a:?}"), "C1Qpl { a: 65, b: 66, c: 67, d: 68 }");
    }
    #[test]
    fn t_into() {
        let a_c16 = C16Qpl::new(b'A', b'B', b'C', b'D');
        let a_c8: C8Qpl = a_c16.into();
        let a_c4: C4Qpl = a_c8.into();
        let a_c2: C2Qpl = a_c4.into();
        let a_c1: C1Qpl = a_c2.into();
        assert_eq!(a_c8, C8Qpl::new(b'A', b'B', b'C', b'D'));
        assert_eq!(a_c4, C4Qpl::new(b'A', b'B', b'C', b'D'));
        assert_eq!(a_c2, C2Qpl::new(b'A', b'B', b'C', b'D'));
        assert_eq!(a_c1, C1Qpl::new(b'A', b'B', b'C', b'D'));
    }
}
