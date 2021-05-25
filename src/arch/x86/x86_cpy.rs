use crate::mem as basic;
use crate::RangeError;

#[inline(always)]
#[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
pub fn _memcpy_impl(dst: &mut [u8], src: &[u8]) -> Result<(), RangeError> {
    #[cfg(target_feature = "avx")]
    let r = unsafe { _memcpy_avx(dst, src) };
    #[cfg(all(target_feature = "sse2", not(target_feature = "avx")))]
    let r = unsafe { _memcpy_sse2(dst, src) };
    #[cfg(not(any(target_feature = "sse2", target_feature = "avx")))]
    let r = _memcpy_basic(dst, src);
    r
}

fn _memcpy_basic(dst: &mut [u8], src: &[u8]) -> Result<(), RangeError> {
    basic::_memcpy_impl(dst, src)
}

#[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
#[target_feature(enable = "sse2")]
pub unsafe fn _memcpy_sse2(dst: &mut [u8], src: &[u8]) -> Result<(), RangeError> {
    _memcpy_x86_impl(dst, src)
}

#[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
#[target_feature(enable = "avx")]
pub unsafe fn _memcpy_avx(dst: &mut [u8], src: &[u8]) -> Result<(), RangeError> {
    _memcpy_x86_impl(dst, src)
}

#[inline(always)]
fn _memcpy_x86_impl(dst: &mut [u8], src: &[u8]) -> Result<(), RangeError> {
    if dst.len() < src.len() {
        return Err(RangeError);
    }
    let dst_ptr = dst.as_mut_ptr();
    let src_ptr = src.as_ptr();
    let src_len = src.len();
    //
    if src_len < 32 {
        _memcpy_tiny_0_31(dst_ptr, src_ptr, src_len);
    } else if src_len < 64 {
        _memcpy_tiny_32_63(dst_ptr, src_ptr, src_len);
    } else if src_len < 96 {
        _memcpy_tiny_64_95(dst_ptr, src_ptr, src_len);
    } else if src_len < 128 {
        _memcpy_tiny_96_127(dst_ptr, src_ptr, src_len);
    } else {
        _memcpy_large_128(dst_ptr, src_ptr, src_len);
    }
    //
    Ok(())
}

fn _copy_32_bytes(dst_ptr: *mut u8, src_ptr: *const u8) {
    _copy_8_bytes(dst_ptr, src_ptr);
    _copy_8_bytes(unsafe { dst_ptr.add(8) }, unsafe { src_ptr.add(8) });
    _copy_8_bytes(unsafe { dst_ptr.add(8 + 8) }, unsafe { src_ptr.add(8 + 8) });
    _copy_8_bytes(unsafe { dst_ptr.add(8 + 8 + 8) }, unsafe {
        src_ptr.add(8 + 8 + 8)
    });
}

fn _copy_16_bytes(dst_ptr: *mut u8, src_ptr: *const u8) {
    _copy_8_bytes(dst_ptr, src_ptr);
    _copy_8_bytes(unsafe { dst_ptr.add(8) }, unsafe { src_ptr.add(8) });
}

fn _copy_8_bytes(dst_ptr: *mut u8, src_ptr: *const u8) {
    let a_ptr = dst_ptr as *mut u64;
    let b_ptr = src_ptr as *const u64;
    unsafe { *a_ptr = *b_ptr };
}

fn _copy_4_bytes(dst_ptr: *mut u8, src_ptr: *const u8) {
    let a_ptr = dst_ptr as *mut u32;
    let b_ptr = src_ptr as *const u32;
    unsafe { *a_ptr = *b_ptr };
}

fn _copy_2_bytes(dst_ptr: *mut u8, src_ptr: *const u8) {
    let a_ptr = dst_ptr as *mut u16;
    let b_ptr = src_ptr as *const u16;
    unsafe { *a_ptr = *b_ptr };
}

fn _copy_1_bytes(dst_ptr: *mut u8, src_ptr: *const u8) {
    let a_ptr = dst_ptr as *mut u8;
    let b_ptr = src_ptr as *const u8;
    unsafe { *a_ptr = *b_ptr };
}

#[inline(always)]
fn _memcpy_tiny_0_31(dst_ptr: *mut u8, src_ptr: *const u8, len: usize) {
    match len {
        0 => {}
        1 => _copy_1_bytes(dst_ptr, src_ptr),
        2 => _copy_2_bytes(dst_ptr, src_ptr),
        3 => {
            _copy_1_bytes(dst_ptr, src_ptr);
            _copy_2_bytes(unsafe { dst_ptr.add(1) }, unsafe { src_ptr.add(1) });
        }
        4 => _copy_4_bytes(dst_ptr, src_ptr),
        5 => {
            _copy_1_bytes(dst_ptr, src_ptr);
            _copy_4_bytes(unsafe { dst_ptr.add(1) }, unsafe { src_ptr.add(1) });
        }
        6 => {
            _copy_2_bytes(dst_ptr, src_ptr);
            _copy_4_bytes(unsafe { dst_ptr.add(2) }, unsafe { src_ptr.add(2) });
        }
        7 => {
            _copy_1_bytes(dst_ptr, src_ptr);
            _copy_2_bytes(unsafe { dst_ptr.add(1) }, unsafe { src_ptr.add(1) });
            _copy_4_bytes(unsafe { dst_ptr.add(1 + 2) }, unsafe { src_ptr.add(1 + 2) });
        }
        8 => _copy_8_bytes(dst_ptr, src_ptr),
        9 => {
            _copy_1_bytes(dst_ptr, src_ptr);
            _copy_8_bytes(unsafe { dst_ptr.add(1) }, unsafe { src_ptr.add(1) });
        }
        10 => {
            _copy_2_bytes(dst_ptr, src_ptr);
            _copy_8_bytes(unsafe { dst_ptr.add(2) }, unsafe { src_ptr.add(2) });
        }
        11 => {
            _copy_1_bytes(dst_ptr, src_ptr);
            _copy_2_bytes(unsafe { dst_ptr.add(1) }, unsafe { src_ptr.add(1) });
            _copy_8_bytes(unsafe { dst_ptr.add(1 + 2) }, unsafe { src_ptr.add(1 + 2) });
        }
        12 => {
            _copy_4_bytes(dst_ptr, src_ptr);
            _copy_8_bytes(unsafe { dst_ptr.add(4) }, unsafe { src_ptr.add(4) });
        }
        13 => {
            _copy_1_bytes(dst_ptr, src_ptr);
            _copy_4_bytes(unsafe { dst_ptr.add(1) }, unsafe { src_ptr.add(1) });
            _copy_8_bytes(unsafe { dst_ptr.add(1 + 4) }, unsafe { src_ptr.add(1 + 4) });
        }
        14 => {
            _copy_2_bytes(dst_ptr, src_ptr);
            _copy_4_bytes(unsafe { dst_ptr.add(2) }, unsafe { src_ptr.add(2) });
            _copy_8_bytes(unsafe { dst_ptr.add(2 + 4) }, unsafe { src_ptr.add(2 + 4) });
        }
        15 => {
            _copy_1_bytes(dst_ptr, src_ptr);
            _copy_2_bytes(unsafe { dst_ptr.add(1) }, unsafe { src_ptr.add(1) });
            _copy_4_bytes(unsafe { dst_ptr.add(1 + 2) }, unsafe { src_ptr.add(1 + 2) });
            _copy_8_bytes(unsafe { dst_ptr.add(1 + 2 + 4) }, unsafe {
                src_ptr.add(1 + 2 + 4)
            });
        }
        //
        16 => _copy_16_bytes(dst_ptr, src_ptr),
        17 => {
            _copy_1_bytes(dst_ptr, src_ptr);
            _copy_16_bytes(unsafe { dst_ptr.add(1) }, unsafe { src_ptr.add(1) });
        }
        18 => {
            _copy_2_bytes(dst_ptr, src_ptr);
            _copy_16_bytes(unsafe { dst_ptr.add(2) }, unsafe { src_ptr.add(2) });
        }
        19 => {
            _copy_1_bytes(dst_ptr, src_ptr);
            _copy_2_bytes(unsafe { dst_ptr.add(1) }, unsafe { src_ptr.add(1) });
            _copy_16_bytes(unsafe { dst_ptr.add(1 + 2) }, unsafe { src_ptr.add(1 + 2) });
        }
        20 => {
            _copy_4_bytes(dst_ptr, src_ptr);
            _copy_16_bytes(unsafe { dst_ptr.add(4) }, unsafe { src_ptr.add(4) });
        }
        21 => {
            _copy_1_bytes(dst_ptr, src_ptr);
            _copy_4_bytes(unsafe { dst_ptr.add(1) }, unsafe { src_ptr.add(1) });
            _copy_16_bytes(unsafe { dst_ptr.add(1 + 4) }, unsafe { src_ptr.add(1 + 4) });
        }
        22 => {
            _copy_2_bytes(dst_ptr, src_ptr);
            _copy_4_bytes(unsafe { dst_ptr.add(2) }, unsafe { src_ptr.add(2) });
            _copy_16_bytes(unsafe { dst_ptr.add(2 + 4) }, unsafe { src_ptr.add(2 + 4) });
        }
        23 => {
            _copy_1_bytes(dst_ptr, src_ptr);
            _copy_2_bytes(unsafe { dst_ptr.add(1) }, unsafe { src_ptr.add(1) });
            _copy_4_bytes(unsafe { dst_ptr.add(1 + 2) }, unsafe { src_ptr.add(1 + 2) });
            _copy_16_bytes(unsafe { dst_ptr.add(1 + 2 + 4) }, unsafe {
                src_ptr.add(1 + 2 + 4)
            });
        }
        24 => {
            _copy_8_bytes(dst_ptr, src_ptr);
            _copy_16_bytes(unsafe { dst_ptr.add(8) }, unsafe { src_ptr.add(8) });
        }
        25 => {
            _copy_1_bytes(dst_ptr, src_ptr);
            _copy_8_bytes(unsafe { dst_ptr.add(1) }, unsafe { src_ptr.add(1) });
            _copy_16_bytes(unsafe { dst_ptr.add(1 + 8) }, unsafe { src_ptr.add(1 + 8) });
        }
        26 => {
            _copy_2_bytes(dst_ptr, src_ptr);
            _copy_8_bytes(unsafe { dst_ptr.add(2) }, unsafe { src_ptr.add(2) });
            _copy_16_bytes(unsafe { dst_ptr.add(2 + 8) }, unsafe { src_ptr.add(2 + 8) });
        }
        27 => {
            _copy_1_bytes(dst_ptr, src_ptr);
            _copy_2_bytes(unsafe { dst_ptr.add(1) }, unsafe { src_ptr.add(1) });
            _copy_8_bytes(unsafe { dst_ptr.add(1 + 2) }, unsafe { src_ptr.add(1 + 2) });
            _copy_16_bytes(unsafe { dst_ptr.add(1 + 2 + 8) }, unsafe {
                src_ptr.add(1 + 2 + 8)
            });
        }
        28 => {
            _copy_4_bytes(dst_ptr, src_ptr);
            _copy_8_bytes(unsafe { dst_ptr.add(4) }, unsafe { src_ptr.add(4) });
            _copy_16_bytes(unsafe { dst_ptr.add(4 + 8) }, unsafe { src_ptr.add(4 + 8) });
        }
        29 => {
            _copy_1_bytes(dst_ptr, src_ptr);
            _copy_4_bytes(unsafe { dst_ptr.add(1) }, unsafe { src_ptr.add(1) });
            _copy_8_bytes(unsafe { dst_ptr.add(1 + 4) }, unsafe { src_ptr.add(1 + 4) });
            _copy_16_bytes(unsafe { dst_ptr.add(1 + 4 + 8) }, unsafe {
                src_ptr.add(1 + 4 + 8)
            });
        }
        30 => {
            _copy_2_bytes(dst_ptr, src_ptr);
            _copy_4_bytes(unsafe { dst_ptr.add(2) }, unsafe { src_ptr.add(2) });
            _copy_8_bytes(unsafe { dst_ptr.add(2 + 4) }, unsafe { src_ptr.add(2 + 4) });
            _copy_16_bytes(unsafe { dst_ptr.add(2 + 4 + 8) }, unsafe {
                src_ptr.add(2 + 4 + 8)
            });
        }
        31 => {
            _copy_1_bytes(dst_ptr, src_ptr);
            _copy_2_bytes(unsafe { dst_ptr.add(1) }, unsafe { src_ptr.add(1) });
            _copy_4_bytes(unsafe { dst_ptr.add(1 + 2) }, unsafe { src_ptr.add(1 + 2) });
            _copy_8_bytes(unsafe { dst_ptr.add(1 + 2 + 4) }, unsafe {
                src_ptr.add(1 + 2 + 4)
            });
            _copy_16_bytes(unsafe { dst_ptr.add(1 + 2 + 4 + 8) }, unsafe {
                src_ptr.add(1 + 2 + 4 + 8)
            });
        }
        _ => unreachable!(),
    }
}

fn _memcpy_tiny_32_63(dst_ptr: *mut u8, src_ptr: *const u8, len: usize) {
    _copy_32_bytes(dst_ptr, src_ptr);
    _memcpy_tiny_0_31(
        unsafe { dst_ptr.add(32) },
        unsafe { src_ptr.add(32) },
        len - 32,
    );
}

fn _memcpy_tiny_64_95(dst_ptr: *mut u8, src_ptr: *const u8, len: usize) {
    _copy_32_bytes(dst_ptr, src_ptr);
    _copy_32_bytes(unsafe { dst_ptr.add(32) }, unsafe { src_ptr.add(32) });
    _memcpy_tiny_0_31(
        unsafe { dst_ptr.add(32 + 32) },
        unsafe { src_ptr.add(32 + 32) },
        len - (32 + 32),
    );
}

fn _memcpy_tiny_96_127(dst_ptr: *mut u8, src_ptr: *const u8, len: usize) {
    _copy_32_bytes(dst_ptr, src_ptr);
    _copy_32_bytes(unsafe { dst_ptr.add(32) }, unsafe { src_ptr.add(32) });
    _copy_32_bytes(unsafe { dst_ptr.add(32 + 32) }, unsafe {
        src_ptr.add(32 + 32)
    });
    _memcpy_tiny_0_31(
        unsafe { dst_ptr.add(32 + 32 + 32) },
        unsafe { src_ptr.add(32 + 32 + 32) },
        len - (32 + 32 + 32),
    );
}

fn _memcpy_large_128(dst_ptr: *mut u8, src_ptr: *const u8, len: usize) {
    let mut dst_ptr = dst_ptr;
    let mut src_ptr = src_ptr;
    let end_ptr = unsafe { src_ptr.add(len) };
    //
    {
        let unroll = 4;
        let loop_size = 32;
        let end_ptr_32_4 = unsafe { end_ptr.sub(loop_size * unroll) };
        while src_ptr <= end_ptr_32_4 {
            for i in 0..unroll {
                _copy_32_bytes(unsafe { dst_ptr.add(32 * i) }, unsafe {
                    src_ptr.add(32 * i)
                });
            }
            //
            dst_ptr = unsafe { dst_ptr.add(loop_size * unroll) };
            src_ptr = unsafe { src_ptr.add(loop_size * unroll) };
        }
    }
    {
        let loop_size = 32;
        let end_ptr_32 = unsafe { end_ptr.sub(loop_size) };
        while src_ptr <= end_ptr_32 {
            _copy_32_bytes(dst_ptr, src_ptr);
            dst_ptr = unsafe { dst_ptr.add(loop_size) };
            src_ptr = unsafe { src_ptr.add(loop_size) };
        }
    }
    _memcpy_tiny_0_31(dst_ptr, src_ptr, unsafe { end_ptr.offset_from(src_ptr) }
        as usize);
}
