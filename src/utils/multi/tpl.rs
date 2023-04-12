use super::super::{_c16_value, _c8_value, _c4_value};

#[derive(Copy, Clone)]
pub(crate) struct C16Tpl {
    pub a: u128,
    pub b: u128,
    pub c: u128,
}
impl C16Tpl {
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

#[derive(Copy, Clone)]
pub(crate) struct C8Tpl {
    pub a: u64,
    pub b: u64,
    pub c: u64,
}
impl C8Tpl {
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

#[derive(Copy, Clone)]
pub(crate) struct C4Tpl {
    pub a: u32,
    pub b: u32,
    pub c: u32,
}
impl C4Tpl {
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

#[derive(Copy, Clone)]
pub(crate) struct C2Tpl {
    pub a: u16,
    pub b: u16,
    pub c: u16,
}

#[derive(Copy, Clone)]
pub(crate) struct C1Tpl {
    pub a: u8,
    pub b: u8,
    pub c: u8,
}
impl C1Tpl {
    #[allow(dead_code)]
    #[inline(always)]
    pub fn new(c1: u8, c2: u8, c3: u8) -> Self {
        Self { a: c1, b: c2, c: c3 }
    }
}

impl From<C16Tpl> for C8Tpl {
    fn from(cc: C16Tpl) -> Self {
        Self {
            a: cc.a as u64,
            b: cc.b as u64,
            c: cc.c as u64,
        }
    }
}

impl From<C8Tpl> for C4Tpl {
    fn from(cc: C8Tpl) -> Self {
        Self {
            a: cc.a as u32,
            b: cc.b as u32,
            c: cc.c as u32,
        }
    }
}

impl From<C4Tpl> for C2Tpl {
    fn from(cc: C4Tpl) -> Self {
        Self {
            a: cc.a as u16,
            b: cc.b as u16,
            c: cc.c as u16,
        }
    }
}

impl From<C2Tpl> for C1Tpl {
    fn from(cc: C2Tpl) -> Self {
        Self {
            a: cc.a as u8,
            b: cc.b as u8,
            c: cc.c as u8,
        }
    }
}
