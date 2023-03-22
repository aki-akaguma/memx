#![cfg(not(tarpaulin_include))]
use criterion::{black_box, criterion_group, criterion_main, Criterion};

mod barrier;
use barrier::cache_line_flush;
use barrier::memory_barrier;

#[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
cpufeatures::new!(cpuid_avx2, "avx2");

#[inline(never)]
fn statistics_std_memset(
    texts: &mut [Vec<u8>],
    pat_u8: u8,
) -> std::collections::HashMap<u32, usize> {
    #[inline(never)]
    fn _t_(buf: &mut [u8], c: u8) {
        buf.fill(c)
    }
    //
    use std::collections::HashMap;
    let mut founded: HashMap<u32, usize> = HashMap::new();
    for i in 0..texts.len() {
        let line_bytes = &mut texts[i];
        _t_(&mut *line_bytes, pat_u8);
        //
        let addr = ((&mut *line_bytes).as_ptr() as usize % 64) as u32;
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
fn print_statistics_std_memset(texts: &mut [Vec<u8>], pat_u8: u8) {
    let map = statistics_std_memset(texts, pat_u8);
    let mut vec: Vec<u32> = map.clone().into_keys().collect();
    vec.sort_unstable();
    println!("Statistics:");
    for key in vec {
        println!("0x{key:04x} => {},", map[&key]);
    }
    println!("");
}

#[inline(never)]
fn process_std_memset(texts: &mut [Vec<u8>], pat_u8: u8) {
    #[inline(never)]
    fn _t_(buf: &mut [u8], c: u8) {
        buf.fill(c)
    }
    //
    for i in 0..texts.len() {
        let line_bytes = &mut texts[i];
        _t_(&mut *line_bytes, pat_u8);
    }
}

#[inline(never)]
fn process_libc_memset(texts: &mut [Vec<u8>], pat_u8: u8) {
    // original libc function
    extern "C" {
        fn memset(dest: *mut u8, c: i32, n: usize) -> *mut u8;
    }
    #[inline(always)]
    fn _x_libc_memset(dest: &mut [u8], c: u8) -> bool {
        let dest_ptr = dest.as_mut_ptr();
        let _r = unsafe { memset(dest_ptr, c.into(), dest.len()) };
        true
    }
    #[inline(never)]
    fn _t_(buf: &mut [u8], c: u8) {
        let _ = _x_libc_memset(buf, c);
    }
    //
    for i in 0..texts.len() {
        let line_bytes = &mut texts[i];
        _t_(&mut *line_bytes, pat_u8);
    }
}

#[inline(never)]
fn process_memx_memset(texts: &mut [Vec<u8>], pat_u8: u8) {
    #[inline(never)]
    fn _t_(buf: &mut [u8], c: u8) {
        memx::memset(buf, c)
    }
    //
    for i in 0..texts.len() {
        let line_bytes = &mut texts[i];
        _t_(&mut *line_bytes, pat_u8);
    }
}

#[inline(never)]
fn process_memx_memset_basic(texts: &mut [Vec<u8>], pat_u8: u8) {
    #[inline(never)]
    fn _t_(buf: &mut [u8], c: u8) {
        memx::mem::memset_basic(buf, c)
    }
    //
    for i in 0..texts.len() {
        let line_bytes = &mut texts[i];
        _t_(&mut *line_bytes, pat_u8);
    }
}

#[cfg(all(
    any(target_arch = "x86_64", target_arch = "x86"),
    target_feature = "sse2"
))]
#[inline(never)]
fn process_memx_memset_sse2(texts: &mut [Vec<u8>], pat_u8: u8) {
    #[inline(never)]
    fn _t_(buf: &mut [u8], c: u8) {
        unsafe { memx::arch::x86::_memset_sse2(buf, c) }
    }
    //
    for i in 0..texts.len() {
        let line_bytes = &mut texts[i];
        _t_(&mut *line_bytes, pat_u8);
    }
}

#[cfg(all(
    any(target_arch = "x86_64", target_arch = "x86"),
    target_feature = "sse2"
))]
#[inline(never)]
fn process_memx_memset_avx2(texts: &mut [Vec<u8>], pat_u8: u8) {
    #[inline(never)]
    fn _t_(buf: &mut [u8], c: u8) {
        unsafe { memx::arch::x86::_memset_avx2(buf, c) }
    }
    //
    for i in 0..texts.len() {
        let line_bytes = &mut texts[i];
        _t_(&mut *line_bytes, pat_u8);
    }
}

#[inline(never)]
fn cache_flush(texts: &[Vec<u8>]) {
    for i in 0..texts.len() {
        cache_line_flush(&texts[i]);
    }
}

mod create_data;

fn criterion_benchmark(c: &mut Criterion) {
    let mut v = create_data::create_data_set();
    let pat_u8 = b'A';
    //
    if let Ok(_val) = std::env::var("AKI_TEST_DAT_CHECK") {
        process_std_memset(black_box(&mut v), black_box(pat_u8));
        process_libc_memset(black_box(&mut v), black_box(pat_u8));
        process_memx_memset(black_box(&mut v), black_box(pat_u8));
        process_memx_memset_basic(black_box(&mut v), black_box(pat_u8));
        #[cfg(all(
            any(target_arch = "x86_64", target_arch = "x86"),
            target_feature = "sse2"
        ))]
        process_memx_memset_sse2(black_box(&mut v), black_box(pat_u8));
        #[cfg(all(
            any(target_arch = "x86_64", target_arch = "x86"),
            target_feature = "sse2"
        ))]
        if cpuid_avx2::get() {
            process_memx_memset_avx2(black_box(&mut v), black_box(pat_u8));
        }
        std::process::exit(0);
    }
    //
    if let Ok(_val) = std::env::var("AKI_TEST_STATISTICS") {
        print_statistics_std_memset(black_box(&mut v), black_box(pat_u8));
        std::process::exit(0);
    }
    //
    cache_flush(&v);
    //
    c.bench_function("std_memset", |b| {
        b.iter(|| {
            process_std_memset(black_box(&mut v), black_box(pat_u8));
            memory_barrier(&mut v);
        })
    });
    cache_flush(&v);
    c.bench_function("libc_memset", |b| {
        b.iter(|| {
            process_libc_memset(black_box(&mut v), black_box(pat_u8));
            memory_barrier(&mut v);
        })
    });
    cache_flush(&v);
    c.bench_function("memx_memset", |b| {
        b.iter(|| {
            process_memx_memset(black_box(&mut v), black_box(pat_u8));
            memory_barrier(&mut v);
        })
    });
    cache_flush(&v);
    c.bench_function("memx_memset_basic", |b| {
        b.iter(|| {
            process_memx_memset_basic(black_box(&mut v), black_box(pat_u8));
            memory_barrier(&mut v);
        })
    });
    cache_flush(&v);
    #[cfg(all(
        any(target_arch = "x86_64", target_arch = "x86"),
        target_feature = "sse2"
    ))]
    c.bench_function("memx_memset_sse2", |b| {
        b.iter(|| {
            process_memx_memset_sse2(black_box(&mut v), black_box(pat_u8));
            memory_barrier(&mut v);
        })
    });
    cache_flush(&v);
    #[cfg(all(
        any(target_arch = "x86_64", target_arch = "x86"),
        target_feature = "sse2"
    ))]
    if cpuid_avx2::get() {
        c.bench_function("memx_memset_avx2", |b| {
            b.iter(|| {
                process_memx_memset_avx2(black_box(&mut v), black_box(pat_u8));
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
