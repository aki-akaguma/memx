use super::super::{_c16_value, _c4_value, _c8_value};

#[derive(Copy, Clone)]
pub(crate) struct C16Dbl {
    pub a: u128,
    pub b: u128,
}
impl C16Dbl {
    #[allow(dead_code)]
    #[inline(always)]
    pub fn new(c1: u8, c2: u8) -> Self {
        Self {
            a: _c16_value(c1),
            b: _c16_value(c2),
        }
    }
}

#[derive(Copy, Clone)]
pub(crate) struct C8Dbl {
    pub a: u64,
    pub b: u64,
}
impl C8Dbl {
    #[allow(dead_code)]
    #[inline(always)]
    pub fn new(c1: u8, c2: u8) -> Self {
        Self {
            a: _c8_value(c1),
            b: _c8_value(c2),
        }
    }
}

#[derive(Copy, Clone)]
pub(crate) struct C4Dbl {
    pub a: u32,
    pub b: u32,
}
impl C4Dbl {
    #[allow(dead_code)]
    #[inline(always)]
    pub fn new(c1: u8, c2: u8) -> Self {
        Self {
            a: _c4_value(c1),
            b: _c4_value(c2),
        }
    }
}

#[derive(Copy, Clone)]
pub(crate) struct C2Dbl {
    pub a: u16,
    pub b: u16,
}

#[derive(Copy, Clone)]
pub(crate) struct C1Dbl {
    pub a: u8,
    pub b: u8,
}
impl C1Dbl {
    #[allow(dead_code)]
    #[inline(always)]
    pub fn new(c1: u8, c2: u8) -> Self {
        Self { a: c1, b: c2 }
    }
}

impl From<C16Dbl> for C8Dbl {
    fn from(cc: C16Dbl) -> Self {
        Self {
            a: cc.a as u64,
            b: cc.b as u64,
        }
    }
}

impl From<C8Dbl> for C4Dbl {
    fn from(cc: C8Dbl) -> Self {
        Self {
            a: cc.a as u32,
            b: cc.b as u32,
        }
    }
}

impl From<C4Dbl> for C2Dbl {
    fn from(cc: C4Dbl) -> Self {
        Self {
            a: cc.a as u16,
            b: cc.b as u16,
        }
    }
}

impl From<C2Dbl> for C1Dbl {
    fn from(cc: C2Dbl) -> Self {
        Self {
            a: cc.a as u8,
            b: cc.b as u8,
        }
    }
}
