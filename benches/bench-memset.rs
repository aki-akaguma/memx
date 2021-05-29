use criterion::{black_box, criterion_group, criterion_main, Criterion};

// ref.) https://en.wikipedia.org/wiki/Memory_ordering
fn memory_barrier(_arg: &mut [Vec<u8>]) {
    #[cfg(target_feature = "sse2")]
    {
        #[cfg(target_arch = "x86")]
        use std::arch::x86 as mmx;
        #[cfg(target_arch = "x86_64")]
        use std::arch::x86_64 as mmx;

        unsafe { mmx::_mm_mfence() };
    }
    /*
    #[cfg(target_arch = "arm")]
    {
        unsafe { core::arch::arm::__dmb(_arg) };
    }
    #[cfg(target_arch = "aarch64")]
    {
        unsafe { core::arch::aarch64::__dmb(_arg) };
    }
    */
}

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

#[inline(never)]
fn process_memx_memset_libc(texts: &mut [Vec<u8>], pat_u8: u8) {
    for i in 0..texts.len() {
        let line_bytes = &mut texts[i];
        let line_len = line_bytes.len();
        let _ = memx::libc::memset_libc(&mut *line_bytes, pat_u8, line_len);
    }
}

mod create_data;

fn criterion_benchmark(c: &mut Criterion) {
    let mut v = create_data::create_data_set();
    let pat_u8 = b'A';
    //
    process_std_memset(black_box(&mut v), black_box(pat_u8));
    process_memx_memset(black_box(&mut v), black_box(pat_u8));
    process_memx_memset_basic(black_box(&mut v), black_box(pat_u8));
    process_memx_memset_libc(black_box(&mut v), black_box(pat_u8));
    //
    c.bench_function("std_memset", |b| {
        b.iter(|| {
            process_std_memset(black_box(&mut v), black_box(pat_u8));
            memory_barrier(&mut v);
        })
    });
    c.bench_function("memx_memset", |b| {
        b.iter(|| {
            process_memx_memset(black_box(&mut v), black_box(pat_u8));
            memory_barrier(&mut v);
        })
    });
    c.bench_function("memx_memset_basic", |b| {
        b.iter(|| {
            process_memx_memset_basic(black_box(&mut v), black_box(pat_u8));
            memory_barrier(&mut v);
        })
    });
    c.bench_function("memx_memset_libc", |b| {
        b.iter(|| {
            process_memx_memset_libc(black_box(&mut v), black_box(pat_u8));
            memory_barrier(&mut v);
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
