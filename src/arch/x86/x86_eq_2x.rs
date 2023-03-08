#![tarpaulin::skip]
#[inline(always)]
pub fn _memeq_impl(a: &[u8], b: &[u8]) -> bool {
    //
    let a_len = a.len();
    let b_len = b.len();
    if a_len != b_len {
        return true;
    }
    let mut a_ptr = a.as_ptr();
    let mut b_ptr = b.as_ptr();
    let end_ptr = unsafe { a_ptr.add(a_len) };
    //
    /*
    let a_align = (a_ptr as usize) & 0x0F == 0;
    let b_align = (b_ptr as usize) & 0x0F == 0;
    let loop_size = 16;
    if a_align && b_align {
        while a_ptr <= unsafe { end_ptr.sub(loop_size) } {
            if ! unsafe { _cmp_bytes_16_aa(a_ptr, b_ptr) } {
                return false;
            }
            a_ptr = unsafe { a_ptr.add(loop_size) };
            b_ptr = unsafe { b_ptr.add(loop_size) };
        }
    } else if !a_align && b_align {
        while a_ptr <= unsafe { end_ptr.sub(loop_size) } {
            if ! unsafe { _cmp_bytes_16_ua(a_ptr, b_ptr) } {
                return false;
            }
            a_ptr = unsafe { a_ptr.add(loop_size) };
            b_ptr = unsafe { b_ptr.add(loop_size) };
        }
    } else if a_align && !b_align {
        while a_ptr <= unsafe { end_ptr.sub(loop_size) } {
            if ! unsafe { _cmp_bytes_16_au(a_ptr, b_ptr) } {
                return false;
            }
            a_ptr = unsafe { a_ptr.add(loop_size) };
            b_ptr = unsafe { b_ptr.add(loop_size) };
        }
    } else {
        while a_ptr <= unsafe { end_ptr.sub(loop_size) } {
            if ! unsafe { _cmp_bytes_16_uu(a_ptr, b_ptr) } {
                return false;
            }
            a_ptr = unsafe { a_ptr.add(loop_size) };
            b_ptr = unsafe { b_ptr.add(loop_size) };
        }
    }
    */
    let loop_size = 8;
    while a_ptr <= unsafe { end_ptr.sub(loop_size) } {
        if ! unsafe { _cmp_bytes_8(a_ptr, b_ptr) } {
            return false;
        }
        a_ptr = unsafe { a_ptr.add(loop_size) };
        b_ptr = unsafe { b_ptr.add(loop_size) };
    }
    let loop_size = 4;
    while a_ptr <= unsafe { end_ptr.sub(loop_size) } {
        if ! unsafe { _cmp_bytes_4(a_ptr, b_ptr) } {
            return false;
        }
        a_ptr = unsafe { a_ptr.add(loop_size) };
        b_ptr = unsafe { b_ptr.add(loop_size) };
    }
    let loop_size = 2;
    while a_ptr <= unsafe { end_ptr.sub(loop_size) } {
        if ! unsafe { _cmp_bytes_2(a_ptr, b_ptr) } {
            return false;
        }
        a_ptr = unsafe { a_ptr.add(loop_size) };
        b_ptr = unsafe { b_ptr.add(loop_size) };
    }
    while a_ptr < end_ptr {
        unsafe {
            if *a_ptr != *b_ptr {
                return false;
            }
            a_ptr = a_ptr.add(1);
            b_ptr = b_ptr.add(1);
        }
    }
    true
}

#[inline]
unsafe fn _cmp_bytes_2(a_ptr: *const u8, b_ptr: *const u8) -> bool {
    let a_ptr = a_ptr as *const u16;
    let b_ptr = b_ptr as *const u16;
    *a_ptr == *b_ptr
}

#[inline]
unsafe fn _cmp_bytes_4(a_ptr: *const u8, b_ptr: *const u8) -> bool {
    let a_ptr = a_ptr as *const u32;
    let b_ptr = b_ptr as *const u32;
    *a_ptr == *b_ptr
}

#[inline]
unsafe fn _cmp_bytes_8(a_ptr: *const u8, b_ptr: *const u8) -> bool {
    let a_ptr = a_ptr as *const u64;
    let b_ptr = b_ptr as *const u64;
    *a_ptr == *b_ptr
}

#[inline]
unsafe fn _cmp_bytes_16_aa(a_ptr: *const u8, b_ptr: *const u8) -> bool {
    use std::arch::x86_64::__m128i;
    use std::arch::x86_64::_mm_cmpeq_epi8;
    use std::arch::x86_64::_mm_load_si128;
    use std::arch::x86_64::_mm_movemask_epi8;
    let mm_a = _mm_load_si128(a_ptr as *const __m128i);
    let mm_b = _mm_load_si128(b_ptr as *const __m128i);
    let mm_eq = _mm_cmpeq_epi8(mm_a, mm_b);
    let mask = _mm_movemask_epi8(mm_eq);
    mask == 0xffff
}

#[inline]
unsafe fn _cmp_bytes_16_ua(a_ptr: *const u8, b_ptr: *const u8) -> bool {
    use std::arch::x86_64::__m128i;
    use std::arch::x86_64::_mm_cmpeq_epi8;
    use std::arch::x86_64::_mm_load_si128;
    use std::arch::x86_64::_mm_loadu_si128;
    use std::arch::x86_64::_mm_movemask_epi8;
    let mm_a = _mm_loadu_si128(a_ptr as *const __m128i);
    let mm_b = _mm_load_si128(b_ptr as *const __m128i);
    let mm_eq = _mm_cmpeq_epi8(mm_a, mm_b);
    let mask = _mm_movemask_epi8(mm_eq);
    mask == 0xffff
}

#[inline]
unsafe fn _cmp_bytes_16_au(a_ptr: *const u8, b_ptr: *const u8) -> bool {
    use std::arch::x86_64::__m128i;
    use std::arch::x86_64::_mm_cmpeq_epi8;
    use std::arch::x86_64::_mm_load_si128;
    use std::arch::x86_64::_mm_loadu_si128;
    use std::arch::x86_64::_mm_movemask_epi8;
    let mm_a = _mm_load_si128(a_ptr as *const __m128i);
    let mm_b = _mm_loadu_si128(b_ptr as *const __m128i);
    let mm_eq = _mm_cmpeq_epi8(mm_a, mm_b);
    let mask = _mm_movemask_epi8(mm_eq);
    mask == 0xffff
}

#[inline]
unsafe fn _cmp_bytes_16_uu(a_ptr: *const u8, b_ptr: *const u8) -> bool {
    use std::arch::x86_64::__m128i;
    use std::arch::x86_64::_mm_cmpeq_epi8;
    use std::arch::x86_64::_mm_loadu_si128;
    use std::arch::x86_64::_mm_movemask_epi8;
    let mm_a = _mm_loadu_si128(a_ptr as *const __m128i);
    let mm_b = _mm_loadu_si128(b_ptr as *const __m128i);
    let mm_eq = _mm_cmpeq_epi8(mm_a, mm_b);
    let mask = _mm_movemask_epi8(mm_eq);
    mask == 0xffff
}
