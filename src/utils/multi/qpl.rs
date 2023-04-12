use super::super::{_c16_value, _c8_value, _c4_value};

#[derive(Copy, Clone)]
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

#[derive(Copy, Clone)]
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

#[derive(Copy, Clone)]
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

#[derive(Copy, Clone)]
pub(crate) struct C2Qpl {
    pub a: u16,
    pub b: u16,
    pub c: u16,
    pub d: u16,
}

#[derive(Copy, Clone)]
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
        Self { a: c1, b: c2, c: c3, d: c4 }
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
