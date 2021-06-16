use criterion::{black_box, criterion_group, criterion_main, Criterion};

mod barrier;
use barrier::memory_barrier;
use barrier::cache_line_flush;

#[inline(never)]
fn process_std_memset(texts: &mut [Vec<u8>], pat_u8: u8) {
    for i in 0..texts.len() {
        //texts[i].fill(pat_u8);
        // the follow routine is for the rust 1.41.1
        for el in texts[i].iter_mut() {
            *el = pat_u8;
        }
    }
}

#[inline(never)]
fn process_libc_memset(texts: &mut [Vec<u8>], pat_u8: u8) {
    // original libc function
    extern {
        fn memset(dest: *mut u8, c: i32, n: usize) -> *mut u8;
    }
    #[inline(always)]
    fn _x_libc_memset(dest: &mut [u8], c: u8, len: usize) -> bool {
        let dest_ptr = dest.as_mut_ptr();
        if dest.len() < len {
            return false;
        }
        let _r = unsafe { memset(dest_ptr, c.into(), len) };
        true
    }
    //
    for i in 0..texts.len() {
        let line_bytes = &mut texts[i];
        let line_len = line_bytes.len();
        let _ = _x_libc_memset(&mut *line_bytes, pat_u8, line_len);
    }
}

#[inline(never)]
fn process_memx_memset(texts: &mut [Vec<u8>], pat_u8: u8) {
    for i in 0..texts.len() {
        let line_bytes = &mut texts[i];
        let line_len = line_bytes.len();
        let _ = memx::memset(&mut *line_bytes, pat_u8, line_len);
    }
}

#[inline(never)]
fn process_memx_memset_basic(texts: &mut [Vec<u8>], pat_u8: u8) {
    for i in 0..texts.len() {
        let line_bytes = &mut texts[i];
        let line_len = line_bytes.len();
        let _ = memx::mem::memset_basic(&mut *line_bytes, pat_u8, line_len);
    }
}

#[cfg(all(any(target_arch = "x86_64", target_arch = "x86"), target_feature = "sse2"))]
#[inline(never)]
fn process_memx_memset_sse2(texts: &mut [Vec<u8>], pat_u8: u8) {
    for i in 0..texts.len() {
        let line_bytes = &mut texts[i];
        let line_len = line_bytes.len();
        let _ = unsafe { memx::arch::x86::_memset_sse2(&mut *line_bytes, pat_u8, line_len) };
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
    process_std_memset(black_box(&mut v), black_box(pat_u8));
    process_libc_memset(black_box(&mut v), black_box(pat_u8));
    process_memx_memset(black_box(&mut v), black_box(pat_u8));
    process_memx_memset_basic(black_box(&mut v), black_box(pat_u8));
    #[cfg(all(any(target_arch = "x86_64", target_arch = "x86"), target_feature = "sse2"))]
    process_memx_memset_sse2(black_box(&mut v), black_box(pat_u8));
    cache_flush(&v);
    //
    c.bench_function("std_memset", |b| {
        b.iter(|| {
            process_std_memset(black_box(&mut v), black_box(pat_u8));
            memory_barrier(&mut v);
            cache_flush(&v);
        })
    });
    c.bench_function("libc_memset", |b| {
        b.iter(|| {
            process_libc_memset(black_box(&mut v), black_box(pat_u8));
            memory_barrier(&mut v);
            cache_flush(&v);
        })
    });
    c.bench_function("memx_memset", |b| {
        b.iter(|| {
            process_memx_memset(black_box(&mut v), black_box(pat_u8));
            memory_barrier(&mut v);
            cache_flush(&v);
        })
    });
    c.bench_function("memx_memset_basic", |b| {
        b.iter(|| {
            process_memx_memset_basic(black_box(&mut v), black_box(pat_u8));
            memory_barrier(&mut v);
            cache_flush(&v);
        })
    });
    #[cfg(all(any(target_arch = "x86_64", target_arch = "x86"), target_feature = "sse2"))]
    c.bench_function("memx_memset_sse2", |b| {
        b.iter(|| {
            process_memx_memset_sse2(black_box(&mut v), black_box(pat_u8));
            memory_barrier(&mut v);
            cache_flush(&v);
        })
    });
}

criterion_group! {
    name = benches;
    config = Criterion::default()
        .warm_up_time(std::time::Duration::from_millis(300))
        .measurement_time(std::time::Duration::from_millis(1000));
    targets = criterion_benchmark
}
//criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
