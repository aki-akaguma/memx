#![cfg(not(tarpaulin_include))]
use criterion::{black_box, criterion_group, criterion_main, Criterion};

mod barrier;
use barrier::cache_line_flush;
use barrier::memory_barrier;

#[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
cpufeatures::new!(cpuid_avx2, "avx2");

#[inline(always)]
pub fn std_memcpy_impl(dst: &mut [u8], src: &[u8]) -> Result<(), memx::RangeError> {
    dst[..src.len()].copy_from_slice(src);
    Ok(())
}

#[inline(never)]
fn statistics_std_memcpy(
    texts: &mut [Vec<u8>],
    pat_bytes: &[u8],
) -> std::collections::HashMap<u32, usize> {
    #[inline(never)]
    fn _t_(dst: &mut [u8], src: &[u8]) -> Result<(), memx::RangeError> {
        std_memcpy_impl(dst, src)
    }
    //
    let pat_len = pat_bytes.len();
    use std::collections::HashMap;
    let mut founded: HashMap<u32, usize> = HashMap::new();
    for line_bytes in texts {
        let line_len = line_bytes.len();
        let mut curr_idx = 0;
        while curr_idx < line_len - pat_len {
            let tt = &mut line_bytes[curr_idx..(curr_idx + pat_len)];
            let _r = _t_(tt, pat_bytes);
            //
            let addr = (tt.as_ptr() as usize % 64) as u32;
            if let Some(x) = founded.get_mut(&addr) {
                *x += 1;
            } else {
                founded.insert(addr, 0);
            }
            curr_idx = curr_idx + pat_len.max(1);
        }
    }
    founded
}

#[allow(dead_code)]
#[inline(never)]
fn print_statistics_std_memcpy(texts: &mut [Vec<u8>], pat_bytes: &[u8]) {
    let map = statistics_std_memcpy(texts, pat_bytes);
    let mut vec: Vec<u32> = map.clone().into_keys().collect();
    vec.sort_unstable();
    println!("Statistics:");
    for key in vec {
        println!("0x{key:04x} => {},", map[&key]);
    }
    println!("");
}

#[inline(never)]
fn process_std_memcpy(texts: &mut [Vec<u8>], pat_bytes: &[u8]) {
    #[inline(never)]
    fn _t_(dst: &mut [u8], src: &[u8]) -> Result<(), memx::RangeError> {
        std_memcpy_impl(dst, src)
    }
    //
    let pat_len = pat_bytes.len();
    for line_bytes in texts {
        let line_len = line_bytes.len();
        let mut curr_idx = 0;
        while curr_idx < line_len - pat_len {
            let tt = &mut line_bytes[curr_idx..(curr_idx + pat_len)];
            let _r = _t_(tt, pat_bytes);
            curr_idx = curr_idx + pat_len.max(1);
        }
    }
}

#[inline(never)]
fn process_libc_memcpy(texts: &mut [Vec<u8>], pat_bytes: &[u8]) {
    // original libc function
    extern "C" {
        fn memcpy(dest: *mut u8, src: *const u8, n: usize) -> *mut u8;
    }
    #[inline(always)]
    fn _x_libc_memcpy(dest: &mut [u8], src: &[u8]) -> bool {
        let dest_ptr = dest.as_mut_ptr();
        let src_ptr = src.as_ptr();
        let dest_len = dest.len();
        let src_len = src.len();
        if dest_len < src_len {
            return false;
        }
        let _r = unsafe { memcpy(dest_ptr, src_ptr, src_len) };
        true
    }
    #[inline(never)]
    fn _t_(dst: &mut [u8], src: &[u8]) -> Result<(), memx::RangeError> {
        let _ = _x_libc_memcpy(dst, src);
        Ok(())
    }
    //
    let pat_len = pat_bytes.len();
    for line_bytes in texts {
        let line_len = line_bytes.len();
        let mut curr_idx = 0;
        while curr_idx < line_len - pat_len {
            let tt = &mut line_bytes[curr_idx..(curr_idx + pat_len)];
            let _r = _t_(tt, pat_bytes);
            curr_idx = curr_idx + pat_len.max(1);
        }
    }
}

#[inline(never)]
fn process_memx_memcpy(texts: &mut [Vec<u8>], pat_bytes: &[u8]) {
    #[inline(never)]
    fn _t_(dst: &mut [u8], src: &[u8]) -> Result<(), memx::RangeError> {
        memx::memcpy(dst, src)
    }
    //
    let pat_len = pat_bytes.len();
    for line_bytes in texts {
        let line_len = line_bytes.len();
        let mut curr_idx = 0;
        while curr_idx < line_len - pat_len {
            let tt = &mut line_bytes[curr_idx..(curr_idx + pat_len)];
            let _r = _t_(tt, pat_bytes);
            curr_idx = curr_idx + pat_len.max(1);
        }
    }
}

#[inline(never)]
fn process_memx_memcpy_basic(texts: &mut [Vec<u8>], pat_bytes: &[u8]) {
    #[inline(never)]
    fn _t_(dst: &mut [u8], src: &[u8]) -> Result<(), memx::RangeError> {
        memx::mem::memcpy_basic(dst, src)
    }
    //
    let pat_len = pat_bytes.len();
    for line_bytes in texts {
        let line_len = line_bytes.len();
        let mut curr_idx = 0;
        while curr_idx < line_len - pat_len {
            let tt = &mut line_bytes[curr_idx..(curr_idx + pat_len)];
            let _r = _t_(tt, pat_bytes);
            curr_idx = curr_idx + pat_len.max(1);
        }
    }
}

#[cfg(all(
    any(target_arch = "x86_64", target_arch = "x86"),
    target_feature = "sse2"
))]
#[inline(never)]
fn process_memx_memcpy_sse2(texts: &mut [Vec<u8>], pat_bytes: &[u8]) {
    #[inline(never)]
    fn _t_(dst: &mut [u8], src: &[u8]) -> Result<(), memx::RangeError> {
        memx::arch::x86::memcpy_sse2(dst, src)
    }
    //
    let pat_len = pat_bytes.len();
    for line_bytes in texts {
        let line_len = line_bytes.len();
        let mut curr_idx = 0;
        while curr_idx < line_len - pat_len {
            let tt = &mut line_bytes[curr_idx..(curr_idx + pat_len)];
            let _r = _t_(tt, pat_bytes);
            curr_idx = curr_idx + pat_len.max(1);
        }
    }
}

#[cfg(all(
    any(target_arch = "x86_64", target_arch = "x86"),
    target_feature = "sse2"
))]
#[inline(never)]
fn process_memx_memcpy_avx2(texts: &mut [Vec<u8>], pat_bytes: &[u8]) {
    #[inline(never)]
    fn _t_(dst: &mut [u8], src: &[u8]) -> Result<(), memx::RangeError> {
        memx::arch::x86::memcpy_avx2(dst, src)
    }
    //
    let pat_len = pat_bytes.len();
    for line_bytes in texts {
        let line_len = line_bytes.len();
        let mut curr_idx = 0;
        while curr_idx < line_len - pat_len {
            let tt = &mut line_bytes[curr_idx..(curr_idx + pat_len)];
            let _r = _t_(tt, pat_bytes);
            curr_idx = curr_idx + pat_len.max(1);
        }
    }
}

#[inline(never)]
fn assert_result(texts: &[Vec<u8>], pat_bytes: &[u8]) {
    let pat_len = pat_bytes.len();
    for line_bytes in texts {
        let line_len = line_bytes.len();
        let mut curr_idx = 0;
        while curr_idx < line_len - pat_len {
            let tt = &line_bytes[curr_idx..(curr_idx + pat_len)];
            assert_eq!(tt, pat_bytes);
            curr_idx = curr_idx + pat_len.max(1);
        }
    }
}

#[inline(never)]
fn cache_flush(texts: &[Vec<u8>], pattern: &[u8]) {
    for i in 0..texts.len() {
        cache_line_flush(&texts[i]);
    }
    cache_line_flush(pattern)
}

mod create_data;

fn criterion_benchmark(cr: &mut Criterion) {
    let (mut v, pat_bytes) = create_data::create_data_cpy();
    //
    if let Ok(_val) = std::env::var("AKI_TEST_DAT_CHECK") {
        {
            let mut v1 = v.clone();
            process_std_memcpy(black_box(&mut v1), black_box(&pat_bytes));
            assert_result(&v1, &pat_bytes);
        }
        {
            let mut v1 = v.clone();
            process_libc_memcpy(black_box(&mut v1), black_box(&pat_bytes));
            assert_result(&v1, &pat_bytes);
        }
        {
            let mut v1 = v.clone();
            process_memx_memcpy(black_box(&mut v1), black_box(&pat_bytes));
            assert_result(&v1, &pat_bytes);
        }
        {
            let mut v1 = v.clone();
            process_memx_memcpy_basic(black_box(&mut v1), black_box(&pat_bytes));
            assert_result(&v1, &pat_bytes);
        }
        #[cfg(all(
            any(target_arch = "x86_64", target_arch = "x86"),
            target_feature = "sse2"
        ))]
        {
            let mut v1 = v.clone();
            process_memx_memcpy_sse2(black_box(&mut v1), black_box(&pat_bytes));
            assert_result(&v1, &pat_bytes);
        }
        #[cfg(all(
            any(target_arch = "x86_64", target_arch = "x86"),
            target_feature = "sse2"
        ))]
        if cpuid_avx2::get() {
            let mut v1 = v.clone();
            process_memx_memcpy_avx2(black_box(&mut v1), black_box(&pat_bytes));
            assert_result(&v1, &pat_bytes);
        }
        std::process::exit(0);
    }
    //
    if let Ok(_val) = std::env::var("AKI_TEST_STATISTICS") {
        let mut v1 = v.clone();
        print_statistics_std_memcpy(black_box(&mut v1), black_box(&pat_bytes));
        std::process::exit(0);
    }
    //
    cache_flush(&v, &pat_bytes);
    //
    cr.bench_function("std_memcpy", |b| {
        b.iter(|| {
            process_std_memcpy(black_box(&mut v), black_box(&pat_bytes));
            memory_barrier(&mut v);
        })
    });
    cache_flush(&v, &pat_bytes);
    cr.bench_function("libc_memcpy", |b| {
        b.iter(|| {
            process_libc_memcpy(black_box(&mut v), black_box(&pat_bytes));
            memory_barrier(&mut v);
        })
    });
    cache_flush(&v, &pat_bytes);
    cr.bench_function("memx_memcpy", |b| {
        b.iter(|| {
            process_memx_memcpy(black_box(&mut v), black_box(&pat_bytes));
            memory_barrier(&mut v);
        })
    });
    cache_flush(&v, &pat_bytes);
    cr.bench_function("memx_memcpy_basic", |b| {
        b.iter(|| {
            process_memx_memcpy_basic(black_box(&mut v), black_box(&pat_bytes));
            memory_barrier(&mut v);
        })
    });
    cache_flush(&v, &pat_bytes);
    #[cfg(all(
        any(target_arch = "x86_64", target_arch = "x86"),
        target_feature = "sse2"
    ))]
    {
        cr.bench_function("memx_memcpy_sse2", |b| {
            b.iter(|| {
                process_memx_memcpy_sse2(black_box(&mut v), black_box(&pat_bytes));
                memory_barrier(&mut v);
            })
        });
    }
    cache_flush(&v, &pat_bytes);
    #[cfg(all(
        any(target_arch = "x86_64", target_arch = "x86"),
        target_feature = "sse2"
    ))]
    if cpuid_avx2::get() {
        cr.bench_function("memx_memcpy_avx2", |b| {
            b.iter(|| {
                process_memx_memcpy_avx2(black_box(&mut v), black_box(&pat_bytes));
                memory_barrier(&mut v);
            })
        });
    }
}

criterion_group! {
    name = benches;
    config = Criterion::default()
        .sample_size(200)
        .warm_up_time(std::time::Duration::from_millis(100))
        .measurement_time(std::time::Duration::from_millis(1000));
    targets = criterion_benchmark
}
//criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
