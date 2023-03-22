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

macro_rules! j_bool {
    ($j: expr) => {
        $j < 16 || $j % 4 == 0
    };
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
    for i in 0..texts.len() {
        let line_bytes = &mut texts[i];
        let line_len = line_bytes.len();
        for j in 0..=(line_len - pat_len) {
            if j_bool!(j) {
                let _r = _t_(&mut line_bytes[j..], pat_bytes);
                //
                let addr = ((&line_bytes[j..]).as_ptr() as usize % 64) as u32;
                if let Some(x) = founded.get_mut(&addr) {
                    *x += 1;
                } else {
                    founded.insert(addr, 0);
                }
            }
        }
        let _r = _t_(&mut line_bytes[(line_len - pat_bytes.len())..], pat_bytes);
        //
        let addr = ((&line_bytes[(line_len - pat_bytes.len())..]).as_ptr() as usize % 64) as u32;
        if let Some(x) = founded.get_mut(&addr) {
            *x += 1;
        } else {
            founded.insert(addr, 0);
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
    for i in 0..texts.len() {
        let line_bytes = &mut texts[i];
        let line_len = line_bytes.len();
        for j in 0..=(line_len - pat_len) {
            if j_bool!(j) {
                let _r = _t_(&mut line_bytes[j..], pat_bytes);
            }
        }
        let _r = _t_(&mut line_bytes[(line_len - pat_bytes.len())..], pat_bytes);
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
    for i in 0..texts.len() {
        let line_bytes = &mut texts[i];
        let line_len = line_bytes.len();
        for j in 0..=(line_len - pat_len) {
            if j_bool!(j) {
                let _r = _t_(&mut line_bytes[j..], pat_bytes);
            }
        }
        let _r = _t_(&mut line_bytes[(line_len - pat_bytes.len())..], pat_bytes);
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
    for i in 0..texts.len() {
        let line_bytes = &mut texts[i];
        let line_len = line_bytes.len();
        for j in 0..=(line_len - pat_len) {
            if j_bool!(j) {
                let _ = _t_(&mut line_bytes[j..], pat_bytes);
            }
        }
        let _ = _t_(&mut line_bytes[(line_len - pat_bytes.len())..], pat_bytes);
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
    for i in 0..texts.len() {
        let line_bytes = &mut texts[i];
        let line_len = line_bytes.len();
        for j in 0..=(line_len - pat_len) {
            if j_bool!(j) {
                let _ = _t_(&mut line_bytes[j..], pat_bytes);
            }
        }
        let _ = _t_(&mut line_bytes[(line_len - pat_bytes.len())..], pat_bytes);
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
        unsafe { memx::arch::x86::_memcpy_sse2(dst, src) }
    }
    //
    let pat_len = pat_bytes.len();
    for i in 0..texts.len() {
        let line_bytes = &mut texts[i];
        let line_len = line_bytes.len();
        for j in 0..=(line_len - pat_len) {
            if j_bool!(j) {
                let _ = _t_(&mut line_bytes[j..], pat_bytes);
            }
        }
        let _ = _t_(&mut line_bytes[(line_len - pat_bytes.len())..], pat_bytes);
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
        unsafe { memx::arch::x86::_memcpy_avx2(dst, src) }
    }
    //
    let pat_len = pat_bytes.len();
    for i in 0..texts.len() {
        let line_bytes = &mut texts[i];
        let line_len = line_bytes.len();
        for j in 0..=(line_len - pat_len) {
            if j_bool!(j) {
                let _ = _t_(&mut line_bytes[j..], pat_bytes);
            }
        }
        let _ = _t_(&mut line_bytes[(line_len - pat_bytes.len())..], pat_bytes);
    }
}

#[inline(never)]
fn assert_result(texts: &[Vec<u8>], pat_bytes: &[u8]) {
    let pat_len = pat_bytes.len();
    for i in 0..texts.len() {
        let line_bytes = &texts[i];
        let line_len = line_bytes.len();
        for j in 0..=(line_len - pat_len) {
            if j_bool!(j) {
                assert_eq!(line_bytes[j], pat_bytes[0]);
            }
        }
        assert_eq!(&line_bytes[(line_len - pat_len)..], pat_bytes);
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

fn criterion_benchmark(c: &mut Criterion) {
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
    c.bench_function("std_memcpy", |b| {
        b.iter(|| {
            process_std_memcpy(black_box(&mut v), black_box(&pat_bytes));
            memory_barrier(&mut v);
        })
    });
    cache_flush(&v, &pat_bytes);
    c.bench_function("libc_memcpy", |b| {
        b.iter(|| {
            process_libc_memcpy(black_box(&mut v), black_box(&pat_bytes));
            memory_barrier(&mut v);
        })
    });
    cache_flush(&v, &pat_bytes);
    c.bench_function("memx_memcpy", |b| {
        b.iter(|| {
            process_memx_memcpy(black_box(&mut v), black_box(&pat_bytes));
            memory_barrier(&mut v);
        })
    });
    cache_flush(&v, &pat_bytes);
    c.bench_function("memx_memcpy_basic", |b| {
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
        c.bench_function("memx_memcpy_sse2", |b| {
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
        c.bench_function("memx_memcpy_avx2", |b| {
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
