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
    len: usize,
) -> std::collections::HashMap<u32, usize> {
    #[inline(never)]
    fn _t_(buf: &mut [u8], by1: u8) {
        buf.fill(by1)
    }
    //
    use std::collections::HashMap;
    let mut founded: HashMap<u32, usize> = HashMap::new();
    for line in texts {
        if len == 0 {
            continue;
        }
        let line_len = line.len();
        let mut curr_idx = 0;
        while curr_idx < line_len {
            let tt = &mut line[curr_idx..(line_len.min(curr_idx + len))];
            _t_(tt, pat_u8);
            //
            let addr = (tt.as_ptr() as usize % 64) as u32;
            if let Some(x) = founded.get_mut(&addr) {
                *x += 1;
            } else {
                founded.insert(addr, 0);
            }
            curr_idx = curr_idx + len;
        }
    }
    founded
}

#[allow(dead_code)]
#[inline(never)]
fn print_statistics_std_memset(texts: &mut [Vec<u8>], pat_u8: u8, len: usize) {
    let map = statistics_std_memset(texts, pat_u8, len);
    let mut vec: Vec<u32> = map.clone().into_keys().collect();
    vec.sort_unstable();
    println!("Statistics:");
    for key in vec {
        println!("0x{key:04x} => {},", map[&key]);
    }
    println!("");
}

#[inline(never)]
fn process_std_memset(texts: &mut [Vec<u8>], pat_u8: u8, len: usize) {
    #[inline(never)]
    fn _t_(buf: &mut [u8], by1: u8) {
        buf.fill(by1)
    }
    //
    for line in texts {
        if len == 0 {
            continue;
        }
        let line_len = line.len();
        let mut curr_idx = 0;
        while curr_idx < line_len {
            let tt = &mut line[curr_idx..(line_len.min(curr_idx + len))];
            _t_(tt, pat_u8);
            curr_idx = curr_idx + len;
        }
    }
}

#[inline(never)]
fn process_libc_memset(texts: &mut [Vec<u8>], pat_u8: u8, len: usize) {
    // original libc function
    extern "C" {
        fn memset(dest: *mut u8, c: i32, n: usize) -> *mut u8;
    }
    #[inline(always)]
    fn _x_libc_memset(dest: &mut [u8], by1: u8) -> bool {
        let dest_ptr = dest.as_mut_ptr();
        let _r = unsafe { memset(dest_ptr, by1.into(), dest.len()) };
        true
    }
    #[inline(never)]
    fn _t_(buf: &mut [u8], by1: u8) {
        let _ = _x_libc_memset(buf, by1);
    }
    //
    for line in texts {
        if len == 0 {
            continue;
        }
        let line_len = line.len();
        let mut curr_idx = 0;
        while curr_idx < line_len {
            let tt = &mut line[curr_idx..(line_len.min(curr_idx + len))];
            _t_(tt, pat_u8);
            curr_idx = curr_idx + len;
        }
    }
}

#[inline(never)]
fn process_memx_memset(texts: &mut [Vec<u8>], pat_u8: u8, len: usize) {
    #[inline(never)]
    fn _t_(buf: &mut [u8], by1: u8) {
        memx::memset(buf, by1)
    }
    //
    for line in texts {
        if len == 0 {
            continue;
        }
        let line_len = line.len();
        let mut curr_idx = 0;
        while curr_idx < line_len {
            let tt = &mut line[curr_idx..(line_len.min(curr_idx + len))];
            _t_(tt, pat_u8);
            curr_idx = curr_idx + len;
        }
    }
}

#[inline(never)]
fn process_memx_memset_basic(texts: &mut [Vec<u8>], pat_u8: u8, len: usize) {
    #[inline(never)]
    fn _t_(buf: &mut [u8], by1: u8) {
        memx::mem::memset_basic(buf, by1)
    }
    //
    for line in texts {
        if len == 0 {
            continue;
        }
        let line_len = line.len();
        let mut curr_idx = 0;
        while curr_idx < line_len {
            let tt = &mut line[curr_idx..(line_len.min(curr_idx + len))];
            _t_(tt, pat_u8);
            curr_idx = curr_idx + len;
        }
    }
}

#[cfg(all(
    any(target_arch = "x86_64", target_arch = "x86"),
    target_feature = "sse2"
))]
#[inline(never)]
fn process_memx_memset_sse2(texts: &mut [Vec<u8>], pat_u8: u8, len: usize) {
    #[inline(never)]
    fn _t_(buf: &mut [u8], by1: u8) {
        unsafe { memx::arch::x86::_memset_sse2(buf, by1) }
    }
    //
    for line in texts {
        if len == 0 {
            continue;
        }
        let line_len = line.len();
        let mut curr_idx = 0;
        while curr_idx < line_len {
            let tt = &mut line[curr_idx..(line_len.min(curr_idx + len))];
            _t_(tt, pat_u8);
            curr_idx = curr_idx + len;
        }
    }
}

#[cfg(all(
    any(target_arch = "x86_64", target_arch = "x86"),
    target_feature = "sse2"
))]
#[inline(never)]
fn process_memx_memset_avx2(texts: &mut [Vec<u8>], pat_u8: u8, len: usize) {
    #[inline(never)]
    fn _t_(buf: &mut [u8], by1: u8) {
        unsafe { memx::arch::x86::_memset_avx2(buf, by1) }
    }
    //
    for line in texts {
        if len == 0 {
            continue;
        }
        let line_len = line.len();
        let mut curr_idx = 0;
        while curr_idx < line_len {
            let tt = &mut line[curr_idx..(line_len.min(curr_idx + len))];
            _t_(tt, pat_u8);
            curr_idx = curr_idx + len;
        }
    }
}

#[inline(never)]
fn cache_flush(texts: &[Vec<u8>]) {
    for i in 0..texts.len() {
        cache_line_flush(&texts[i]);
    }
}

mod create_data;

fn criterion_benchmark(cr: &mut Criterion) {
    let (mut v, len) = create_data::create_data_set();
    let pat_u8 = b'A';
    //
    if let Ok(_val) = std::env::var("AKI_TEST_DAT_CHECK") {
        process_std_memset(black_box(&mut v), black_box(pat_u8), len);
        process_libc_memset(black_box(&mut v), black_box(pat_u8), len);
        process_memx_memset(black_box(&mut v), black_box(pat_u8), len);
        process_memx_memset_basic(black_box(&mut v), black_box(pat_u8), len);
        #[cfg(all(
            any(target_arch = "x86_64", target_arch = "x86"),
            target_feature = "sse2"
        ))]
        process_memx_memset_sse2(black_box(&mut v), black_box(pat_u8), len);
        #[cfg(all(
            any(target_arch = "x86_64", target_arch = "x86"),
            target_feature = "sse2"
        ))]
        if cpuid_avx2::get() {
            process_memx_memset_avx2(black_box(&mut v), black_box(pat_u8), len);
        }
        std::process::exit(0);
    }
    //
    if let Ok(_val) = std::env::var("AKI_TEST_STATISTICS") {
        print_statistics_std_memset(black_box(&mut v), black_box(pat_u8), len);
        std::process::exit(0);
    }
    //
    cache_flush(&v);
    //
    cr.bench_function("std_memset", |b| {
        b.iter(|| {
            process_std_memset(black_box(&mut v), black_box(pat_u8), len);
            memory_barrier(&mut v);
        })
    });
    cache_flush(&v);
    cr.bench_function("libc_memset", |b| {
        b.iter(|| {
            process_libc_memset(black_box(&mut v), black_box(pat_u8), len);
            memory_barrier(&mut v);
        })
    });
    cache_flush(&v);
    cr.bench_function("memx_memset", |b| {
        b.iter(|| {
            process_memx_memset(black_box(&mut v), black_box(pat_u8), len);
            memory_barrier(&mut v);
        })
    });
    cache_flush(&v);
    cr.bench_function("memx_memset_basic", |b| {
        b.iter(|| {
            process_memx_memset_basic(black_box(&mut v), black_box(pat_u8), len);
            memory_barrier(&mut v);
        })
    });
    cache_flush(&v);
    #[cfg(all(
        any(target_arch = "x86_64", target_arch = "x86"),
        target_feature = "sse2"
    ))]
    cr.bench_function("memx_memset_sse2", |b| {
        b.iter(|| {
            process_memx_memset_sse2(black_box(&mut v), black_box(pat_u8), len);
            memory_barrier(&mut v);
        })
    });
    cache_flush(&v);
    #[cfg(all(
        any(target_arch = "x86_64", target_arch = "x86"),
        target_feature = "sse2"
    ))]
    if cpuid_avx2::get() {
        cr.bench_function("memx_memset_avx2", |b| {
            b.iter(|| {
                process_memx_memset_avx2(black_box(&mut v), black_box(pat_u8), len);
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
