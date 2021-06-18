use criterion::{black_box, criterion_group, criterion_main, Criterion};

mod barrier;
use barrier::cache_line_flush;

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

#[cfg(not(target_os = "android"))]
#[inline(never)]
fn process_libc_memeq(texts: &[&str], pattern: &str) -> usize {
    // original libc function
    extern "C" {
        fn bcmp(cx: *const u8, ct: *const u8, n: usize) -> i32;
    }
    #[inline(always)]
    fn _x_libc_memeq(a: &[u8], b: &[u8]) -> bool {
        let cx = a.as_ptr();
        let ct = b.as_ptr();
        let a_len = a.len();
        let b_len = b.len();
        if a_len != b_len {
            return false;
        }
        let r = unsafe { bcmp(cx, ct, a_len) };
        if r == 0 {
            true
        } else {
            false
        }
    }
    //
    let pat_bytes = pattern.as_bytes();
    let pat_len = pat_bytes.len();
    let mut found: usize = 0;
    for line in texts {
        let line_bytes = line.as_bytes();
        let line_len = line_bytes.len();
        for i in 0..(line_len - pat_len) {
            if _x_libc_memeq(&line_bytes[i..(i + pat_len)], pat_bytes) {
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
fn cache_flush(texts: &[&str], pattern: &str) {
    for i in 0..texts.len() {
        cache_line_flush(texts[i].as_bytes());
    }
    cache_line_flush(pattern.as_bytes());
}

mod create_data;

fn criterion_benchmark(c: &mut Criterion) {
    let (v, pat_string_s, match_cnt, _less_cnt, _greater_count) = create_data::create_data_cmp();
    let vv: Vec<&str> = v.iter().map(|item| item.as_str()).collect();
    let pat_string = pat_string_s.to_string();
    //
    let n = process_std_memeq(black_box(&vv), black_box(pat_string_s));
    assert_eq!(n, match_cnt);
    #[cfg(not(target_os = "android"))]
    {
        let n = process_libc_memeq(black_box(&vv), black_box(pat_string_s));
        assert_eq!(n, match_cnt);
    }
    let n = process_memx_memeq(black_box(&vv), black_box(&pat_string));
    assert_eq!(n, match_cnt);
    let n = process_memx_memeq_basic(black_box(&vv), black_box(&pat_string));
    assert_eq!(n, match_cnt);
    cache_flush(&vv, &pat_string);
    //
    c.bench_function("std_memeq", |b| {
        b.iter(|| {
            let _r = process_std_memeq(black_box(&vv), black_box(pat_string_s));
        })
    });
    cache_flush(&vv, &pat_string);
    #[cfg(not(target_os = "android"))]
    c.bench_function("libc_memeq", |b| {
        b.iter(|| {
            let _r = process_libc_memeq(black_box(&vv), black_box(&pat_string));
        })
    });
    cache_flush(&vv, &pat_string);
    c.bench_function("memx_memeq", |b| {
        b.iter(|| {
            let _r = process_memx_memeq(black_box(&vv), black_box(&pat_string));
        })
    });
    cache_flush(&vv, &pat_string);
    c.bench_function("memx_memeq_basic", |b| {
        b.iter(|| {
            let _r = process_memx_memeq_basic(black_box(&vv), black_box(&pat_string));
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
