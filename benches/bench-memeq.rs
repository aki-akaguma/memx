use criterion::{black_box, criterion_group, criterion_main, Criterion};

// ref.) https://en.wikipedia.org/wiki/Memory_ordering
fn memory_barrier(_arg: &Vec<&str>) {
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
fn process_std_memeq(texts: &[&str], pattern: &str) -> usize {
    let pat_bytes = pattern.as_bytes();
    let pat_len = pat_bytes.len();
    let mut found: usize = 0;
    for line in texts {
        let line_bytes = line.as_bytes();
        let line_len = line_bytes.len();
        for i in 0..(line_len - pat_len) {
            if &line_bytes[i..(i + pat_len)] == pat_bytes {
                found += 1;
            }
        }
    }
    found
}

#[inline(never)]
fn process_memx_memeq(texts: &[&str], pattern: &str) -> usize {
    let pat_bytes = pattern.as_bytes();
    let pat_len = pat_bytes.len();
    let mut found: usize = 0;
    for line in texts {
        let line_bytes = line.as_bytes();
        let line_len = line_bytes.len();
        for i in 0..(line_len - pat_len) {
            if memx::memeq(&line_bytes[i..(i + pat_len)], pat_bytes) {
                found += 1;
            }
        }
    }
    found
}

#[inline(never)]
fn process_memx_memeq_basic(texts: &[&str], pattern: &str) -> usize {
    let pat_bytes = pattern.as_bytes();
    let pat_len = pat_bytes.len();
    let mut found: usize = 0;
    for line in texts {
        let line_bytes = line.as_bytes();
        let line_len = line_bytes.len();
        for i in 0..(line_len - pat_len) {
            if memx::mem::memeq_basic(&line_bytes[i..(i + pat_len)], pat_bytes) {
                found += 1;
            }
        }
    }
    found
}

#[inline(never)]
fn process_memx_memeq_libc(texts: &[&str], pattern: &str) -> usize {
    let pat_bytes = pattern.as_bytes();
    let pat_len = pat_bytes.len();
    let mut found: usize = 0;
    for line in texts {
        let line_bytes = line.as_bytes();
        let line_len = line_bytes.len();
        for i in 0..(line_len - pat_len) {
            if memx::libc::memeq_libc(&line_bytes[i..(i + pat_len)], pat_bytes) {
                found += 1;
            }
        }
    }
    found
}

mod create_data;

fn criterion_benchmark(c: &mut Criterion) {
    let (v, pat_string_s, match_cnt, _less_cnt, _greater_count) = create_data::create_data_cmp();
    let vv: Vec<&str> = v.iter().map(|item| item.as_str()).collect();
    let pat_string = pat_string_s.to_string();
    //
    let n = process_std_memeq(black_box(&vv), black_box(pat_string_s));
    assert_eq!(n, match_cnt);
    let n = process_memx_memeq(black_box(&vv), black_box(&pat_string));
    assert_eq!(n, match_cnt);
    let n = process_memx_memeq_basic(black_box(&vv), black_box(&pat_string));
    assert_eq!(n, match_cnt);
    let n = process_memx_memeq_libc(black_box(&vv), black_box(&pat_string));
    assert_eq!(n, match_cnt);
    memory_barrier(&vv);
    //
    c.bench_function("std_memeq", |b| {
        b.iter(|| {
            let _r = process_std_memeq(black_box(&vv), black_box(pat_string_s));
            memory_barrier(&vv);
        })
    });
    c.bench_function("memx_memeq", |b| {
        b.iter(|| {
            let _r = process_memx_memeq(black_box(&vv), black_box(&pat_string));
            memory_barrier(&vv);
        })
    });
    c.bench_function("memx_memeq_basic", |b| {
        b.iter(|| {
            let _r = process_memx_memeq_basic(black_box(&vv), black_box(&pat_string));
            memory_barrier(&vv);
        })
    });
    c.bench_function("memx_memeq_libc", |b| {
        b.iter(|| {
            let _r = process_memx_memeq_libc(black_box(&vv), black_box(&pat_string));
            memory_barrier(&vv);
        })
    });
}

criterion_group! {
    name = benches;
    config = Criterion::default()
        .warm_up_time(std::time::Duration::from_millis(300))
        .measurement_time(std::time::Duration::from_millis(3000));
    targets = criterion_benchmark
}
//criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
