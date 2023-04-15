use super::{_b16_value, _b2_value, _b4_value, _b8_value};

#[derive(Debug, Copy, Clone, PartialEq)]
pub(crate) struct B16Dbl {
    pub v1: u128,
    pub v2: u128,
}
impl B16Dbl {
    #[allow(dead_code)]
    #[inline(always)]
    pub fn new(b1: u8, b2: u8) -> Self {
        Self {
            v1: _b16_value(b1),
            v2: _b16_value(b2),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub(crate) struct B8Dbl {
    pub v1: u64,
    pub v2: u64,
}
impl B8Dbl {
    #[allow(dead_code)]
    #[inline(always)]
    pub fn new(b1: u8, b2: u8) -> Self {
        Self {
            v1: _b8_value(b1),
            v2: _b8_value(b2),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub(crate) struct B4Dbl {
    pub v1: u32,
    pub v2: u32,
}
impl B4Dbl {
    #[allow(dead_code)]
    #[inline(always)]
    pub fn new(b1: u8, b2: u8) -> Self {
        Self {
            v1: _b4_value(b1),
            v2: _b4_value(b2),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub(crate) struct B2Dbl {
    pub v1: u16,
    pub v2: u16,
}
impl B2Dbl {
    #[allow(dead_code)]
    #[inline(always)]
    pub fn new(b1: u8, b2: u8) -> Self {
        Self {
            v1: _b2_value(b1),
            v2: _b2_value(b2),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub(crate) struct B1Dbl {
    pub v1: u8,
    pub v2: u8,
}
impl B1Dbl {
    #[allow(dead_code)]
    #[inline(always)]
    pub fn new(b1: u8, b2: u8) -> Self {
        Self { v1: b1, v2: b2 }
    }
}

impl From<B16Dbl> for B8Dbl {
    fn from(cc: B16Dbl) -> Self {
        Self {
            v1: cc.v1 as u64,
            v2: cc.v2 as u64,
        }
    }
}

impl From<B8Dbl> for B4Dbl {
    fn from(cc: B8Dbl) -> Self {
        Self {
            v1: cc.v1 as u32,
            v2: cc.v2 as u32,
        }
    }
}

impl From<B4Dbl> for B2Dbl {
    fn from(cc: B4Dbl) -> Self {
        Self {
            v1: cc.v1 as u16,
            v2: cc.v2 as u16,
        }
    }
}

impl From<B2Dbl> for B1Dbl {
    fn from(cc: B2Dbl) -> Self {
        Self {
            v1: cc.v1 as u8,
            v2: cc.v2 as u8,
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
        let a = B16Dbl::new(b'A', b'B');
        let b = a.clone();
        let c = a;
        assert_eq!(a.v1, b.v1);
        assert_eq!(a.v1, c.v1);
        assert_eq!(a, b);
        assert_eq!(format!("{a:?}"), "B16Dbl { v1: 86738642548474510294585684247313465665, v2: 88073083203066425837579310158810595906 }");
    }
    #[test]
    fn t_b8() {
        let a = B8Dbl::new(b'A', b'B');
        let b = a.clone();
        let c = a;
        assert_eq!(a.v1, b.v1);
        assert_eq!(a.v1, c.v1);
        assert_eq!(a, b);
        assert_eq!(
            format!("{a:?}"),
            "B8Dbl { v1: 4702111234474983745, v2: 4774451407313060418 }"
        );
    }
    #[test]
    fn t_b4() {
        let a = B4Dbl::new(b'A', b'B');
        let b = a.clone();
        let c = a;
        assert_eq!(a.v1, b.v1);
        assert_eq!(a.v1, c.v1);
        assert_eq!(a, b);
        assert_eq!(format!("{a:?}"), "B4Dbl { v1: 1094795585, v2: 1111638594 }");
    }
    #[test]
    fn t_b2() {
        let a = B2Dbl::new(b'A', b'B');
        let b = a.clone();
        let c = a;
        assert_eq!(a.v1, b.v1);
        assert_eq!(a.v1, c.v1);
        assert_eq!(a, b);
        assert_eq!(format!("{a:?}"), "B2Dbl { v1: 16705, v2: 16962 }");
    }
    #[test]
    fn t_b1() {
        let a = B1Dbl::new(b'A', b'B');
        let b = a.clone();
        let c = a;
        assert_eq!(a.v1, b.v1);
        assert_eq!(a.v1, c.v1);
        assert_eq!(a, b);
        assert_eq!(format!("{a:?}"), "B1Dbl { v1: 65, v2: 66 }");
    }
    #[test]
    fn t_into() {
        let a_b16 = B16Dbl::new(b'A', b'B');
        let a_b8: B8Dbl = a_b16.into();
        let a_b4: B4Dbl = a_b8.into();
        let a_b2: B2Dbl = a_b4.into();
        let a_b1: B1Dbl = a_b2.into();
        assert_eq!(a_b8, B8Dbl::new(b'A', b'B'));
        assert_eq!(a_b4, B4Dbl::new(b'A', b'B'));
        assert_eq!(a_b2, B2Dbl::new(b'A', b'B'));
        assert_eq!(a_b1, B1Dbl::new(b'A', b'B'));
    }
}
