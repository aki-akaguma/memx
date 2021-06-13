use criterion::{black_box, criterion_group, criterion_main, Criterion};

mod barrier;
use barrier::memory_barrier;

#[inline(never)]
fn process_std_memcpy(texts: &mut [Vec<u8>], pat_bytes: &[u8]) {
    let pat_len = pat_bytes.len();
    for i in 0..texts.len() {
        let line_bytes = &mut texts[i];
        let line_len = line_bytes.len();
        for i in 0..(line_len - pat_len) {
            line_bytes[i..(i + pat_len)].copy_from_slice(pat_bytes);
        }
    }
}

#[inline(never)]
fn process_libc_memcpy(texts: &mut [Vec<u8>], pat_bytes: &[u8]) {
    // original libc function
    extern {
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
    //
    let pat_len = pat_bytes.len();
    for i in 0..texts.len() {
        let line_bytes = &mut texts[i];
        let line_len = line_bytes.len();
        for i in 0..(line_len - pat_len) {
            let _r = _x_libc_memcpy(&mut line_bytes[i..], pat_bytes);
        }
    }
}

#[inline(never)]
fn process_memx_memcpy(texts: &mut [Vec<u8>], pat_bytes: &[u8]) {
    let pat_len = pat_bytes.len();
    for i in 0..texts.len() {
        let line_bytes = &mut texts[i];
        let line_len = line_bytes.len();
        for i in 0..(line_len - pat_len) {
            let _ = memx::memcpy(&mut line_bytes[i..], pat_bytes);
        }
    }
}

#[inline(never)]
fn process_memx_memcpy_basic(texts: &mut [Vec<u8>], pat_bytes: &[u8]) {
    let pat_len = pat_bytes.len();
    for i in 0..texts.len() {
        let line_bytes = &mut texts[i];
        let line_len = line_bytes.len();
        for i in 0..(line_len - pat_len) {
            let _ = memx::mem::memcpy_basic(&mut line_bytes[i..], pat_bytes);
        }
    }
}

/*
#[inline(never)]
fn process_memx_memcpy_libc(texts: &mut [Vec<u8>], pat_bytes: &[u8]) {
    let pat_len = pat_bytes.len();
    for i in 0..texts.len() {
        let line_bytes = &mut texts[i];
        let line_len = line_bytes.len();
        for i in 0..(line_len - pat_len) {
            let _ = memx::libc::memcpy_libc(&mut line_bytes[i..], pat_bytes);
        }
    }
}
*/

mod create_data;

fn criterion_benchmark(c: &mut Criterion) {
    let (mut v, pat_bytes) = create_data::create_data_cpy();
    //
    process_std_memcpy(black_box(&mut v), black_box(pat_bytes));
    process_libc_memcpy(black_box(&mut v), black_box(pat_bytes));
    process_memx_memcpy(black_box(&mut v), black_box(pat_bytes));
    process_memx_memcpy_basic(black_box(&mut v), black_box(pat_bytes));
    //process_memx_memcpy_libc(black_box(&mut v), black_box(pat_bytes));
    memory_barrier(&mut v);
    //
    c.bench_function("std_memcpy", |b| {
        b.iter(|| {
            process_std_memcpy(black_box(&mut v), black_box(pat_bytes));
            memory_barrier(&mut v);
        })
    });
    c.bench_function("libc_memcpy", |b| {
        b.iter(|| {
            process_libc_memcpy(black_box(&mut v), black_box(pat_bytes));
            memory_barrier(&mut v);
        })
    });
    c.bench_function("memx_memcpy", |b| {
        b.iter(|| {
            process_memx_memcpy(black_box(&mut v), black_box(pat_bytes));
            memory_barrier(&mut v);
        })
    });
    c.bench_function("memx_memcpy_basic", |b| {
        b.iter(|| {
            process_memx_memcpy_basic(black_box(&mut v), black_box(pat_bytes));
            memory_barrier(&mut v);
        })
    });
    /*
    c.bench_function("memx_memcpy_libc", |b| {
        b.iter(|| {
            process_memx_memcpy_libc(black_box(&mut v), black_box(pat_bytes));
            memory_barrier(&mut v);
        })
    });
    */
}

criterion_group! {
    name = benches;
    config = Criterion::default()
        .warm_up_time(std::time::Duration::from_millis(300))
        .measurement_time(std::time::Duration::from_millis(2000));
    targets = criterion_benchmark
}
//criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
