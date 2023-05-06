// multiplicative
pub(crate) mod dbl;
pub(crate) mod qpl;
pub(crate) mod sgl;
pub(crate) mod tpl;

#[cfg(target_arch = "x86")]
use core::arch::x86 as mmx;
#[cfg(target_arch = "x86_64")]
use core::arch::x86_64 as mmx;

use mmx::__m128i;
use mmx::_mm_cmpeq_epi8;
use mmx::_mm_movemask_epi8;
use mmx::_mm_set1_epi8;

use mmx::__m256i;
use mmx::_mm256_cmpeq_epi8;
use mmx::_mm256_cvtsi256_si32;
use mmx::_mm256_movemask_epi8;
use mmx::_mm256_set1_epi8;

#[inline(always)]
unsafe fn _b16_value(c: u8) -> __m128i {
    _mm_set1_epi8(c as i8)
}

#[inline(always)]
unsafe fn _b32_value(c: u8) -> __m256i {
    _mm256_set1_epi8(c as i8)
}

#[inline(always)]
unsafe fn _b16_from_b32(cc: __m256i) -> __m128i {
    _b16_value(_mm256_cvtsi256_si32(cc) as u8)
}

#[inline(always)]
unsafe fn _b16_eq(mm_a: __m128i, mm_b: __m128i) -> bool {
    let mm_eq = _mm_cmpeq_epi8(mm_a, mm_b);
    let mask = _mm_movemask_epi8(mm_eq);
    mask == 0xFFFF
}

#[inline(always)]
unsafe fn _b32_eq(mm_a: __m256i, mm_b: __m256i) -> bool {
    let mm_eq = _mm256_cmpeq_epi8(mm_a, mm_b);
    let mask = _mm256_movemask_epi8(mm_eq) as u32;
    mask == 0xFFFF_FFFF
}
