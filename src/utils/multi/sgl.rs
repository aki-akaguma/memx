use super::super::{_c16_value, _c4_value, _c8_value};

#[derive(Copy, Clone)]
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

#[derive(Copy, Clone)]
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

#[derive(Copy, Clone)]
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

#[derive(Copy, Clone)]
pub(crate) struct C2Sgl {
    pub a: u16,
}

#[derive(Copy, Clone)]
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
