use super::{_b16_value, _b2_value, _b4_value, _b8_value};

#[derive(Debug, Copy, Clone, PartialEq)]
pub(crate) struct B16Qpl {
    pub v1: u128,
    pub v2: u128,
    pub v3: u128,
    pub v4: u128,
}
impl B16Qpl {
    #[allow(dead_code)]
    #[inline(always)]
    pub fn new(b1: u8, b2: u8, b3: u8, b4: u8) -> Self {
        Self {
            v1: _b16_value(b1),
            v2: _b16_value(b2),
            v3: _b16_value(b3),
            v4: _b16_value(b4),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub(crate) struct B8Qpl {
    pub v1: u64,
    pub v2: u64,
    pub v3: u64,
    pub v4: u64,
}
impl B8Qpl {
    #[allow(dead_code)]
    #[inline(always)]
    pub fn new(b1: u8, b2: u8, b3: u8, b4: u8) -> Self {
        Self {
            v1: _b8_value(b1),
            v2: _b8_value(b2),
            v3: _b8_value(b3),
            v4: _b8_value(b4),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub(crate) struct B4Qpl {
    pub v1: u32,
    pub v2: u32,
    pub v3: u32,
    pub v4: u32,
}
impl B4Qpl {
    #[allow(dead_code)]
    #[inline(always)]
    pub fn new(b1: u8, b2: u8, b3: u8, b4: u8) -> Self {
        Self {
            v1: _b4_value(b1),
            v2: _b4_value(b2),
            v3: _b4_value(b3),
            v4: _b4_value(b4),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub(crate) struct B2Qpl {
    pub v1: u16,
    pub v2: u16,
    pub v3: u16,
    pub v4: u16,
}
impl B2Qpl {
    #[allow(dead_code)]
    #[inline(always)]
    pub fn new(b1: u8, b2: u8, b3: u8, b4: u8) -> Self {
        Self {
            v1: _b2_value(b1),
            v2: _b2_value(b2),
            v3: _b2_value(b3),
            v4: _b2_value(b4),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub(crate) struct B1Qpl {
    pub v1: u8,
    pub v2: u8,
    pub v3: u8,
    pub v4: u8,
}
impl B1Qpl {
    #[allow(dead_code)]
    #[inline(always)]
    pub fn new(b1: u8, b2: u8, b3: u8, b4: u8) -> Self {
        Self {
            v1: b1,
            v2: b2,
            v3: b3,
            v4: b4,
        }
    }
}

impl From<B16Qpl> for B8Qpl {
    fn from(cc: B16Qpl) -> Self {
        Self {
            v1: cc.v1 as u64,
            v2: cc.v2 as u64,
            v3: cc.v3 as u64,
            v4: cc.v4 as u64,
        }
    }
}

impl From<B8Qpl> for B4Qpl {
    fn from(cc: B8Qpl) -> Self {
        Self {
            v1: cc.v1 as u32,
            v2: cc.v2 as u32,
            v3: cc.v3 as u32,
            v4: cc.v4 as u32,
        }
    }
}

impl From<B4Qpl> for B2Qpl {
    fn from(cc: B4Qpl) -> Self {
        Self {
            v1: cc.v1 as u16,
            v2: cc.v2 as u16,
            v3: cc.v3 as u16,
            v4: cc.v4 as u16,
        }
    }
}

impl From<B2Qpl> for B1Qpl {
    fn from(cc: B2Qpl) -> Self {
        Self {
            v1: cc.v1 as u8,
            v2: cc.v2 as u8,
            v3: cc.v3 as u8,
            v4: cc.v4 as u8,
        }
    }
}

#[cfg(test)]
mod mini {
    #![allow(clippy::clone_on_copy)]
    use super::*;
    //
    #[test]
    fn t_b16() {
        let a = B16Qpl::new(b'A', b'B', b'C', b'D');
        let b = a.clone();
        let c = a;
        assert_eq!(a.v1, b.v1);
        assert_eq!(a.v1, c.v1);
        assert_eq!(a, b);
        assert_eq!(format!("{a:?}"), "B16Qpl { v1: 86738642548474510294585684247313465665, v2: 88073083203066425837579310158810595906, v3: 89407523857658341380572936070307726147, v4: 90741964512250256923566561981804856388 }");
    }
    #[test]
    fn t_b8() {
        let a = B8Qpl::new(b'A', b'B', b'C', b'D');
        let b = a.clone();
        let c = a;
        assert_eq!(a.v1, b.v1);
        assert_eq!(a.v1, c.v1);
        assert_eq!(a, b);
        assert_eq!(
            format!("{a:?}"),
            "B8Qpl { v1: 4702111234474983745, v2: 4774451407313060418, v3: 4846791580151137091, v4: 4919131752989213764 }"
        );
    }
    #[test]
    fn t_b4() {
        let a = B4Qpl::new(b'A', b'B', b'C', b'D');
        let b = a.clone();
        let c = a;
        assert_eq!(a.v1, b.v1);
        assert_eq!(a.v1, c.v1);
        assert_eq!(a, b);
        assert_eq!(
            format!("{a:?}"),
            "B4Qpl { v1: 1094795585, v2: 1111638594, v3: 1128481603, v4: 1145324612 }"
        );
    }
    #[test]
    fn t_b2() {
        let a = B2Qpl::new(b'A', b'B', b'C', b'D');
        let b = a.clone();
        let c = a;
        assert_eq!(a.v1, b.v1);
        assert_eq!(a.v1, c.v1);
        assert_eq!(a, b);
        assert_eq!(
            format!("{a:?}"),
            "B2Qpl { v1: 16705, v2: 16962, v3: 17219, v4: 17476 }"
        );
    }
    #[test]
    fn t_b1() {
        let a = B1Qpl::new(b'A', b'B', b'C', b'D');
        let b = a.clone();
        let c = a;
        assert_eq!(a.v1, b.v1);
        assert_eq!(a.v1, c.v1);
        assert_eq!(a, b);
        assert_eq!(format!("{a:?}"), "B1Qpl { v1: 65, v2: 66, v3: 67, v4: 68 }");
    }
    #[test]
    fn t_into() {
        let a_b16 = B16Qpl::new(b'A', b'B', b'C', b'D');
        let a_b8: B8Qpl = a_b16.into();
        let a_b4: B4Qpl = a_b8.into();
        let a_b2: B2Qpl = a_b4.into();
        let a_b1: B1Qpl = a_b2.into();
        assert_eq!(a_b8, B8Qpl::new(b'A', b'B', b'C', b'D'));
        assert_eq!(a_b4, B4Qpl::new(b'A', b'B', b'C', b'D'));
        assert_eq!(a_b2, B2Qpl::new(b'A', b'B', b'C', b'D'));
        assert_eq!(a_b1, B1Qpl::new(b'A', b'B', b'C', b'D'));
    }
}
