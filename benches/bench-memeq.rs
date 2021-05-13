use criterion::{black_box, criterion_group, criterion_main, Criterion};

#[inline(never)]
pub fn plain_memeq(a: &[u8], b: &[u8]) -> bool {
    let a_len = a.len();
    let b_len = b.len();
    if a_len != b_len {
        return false;
    }
    let min_len = a_len;
    let mut a_ptr = a.as_ptr();
    let mut b_ptr = b.as_ptr();
    let end_ptr = unsafe { a_ptr.add(min_len) };
    while a_ptr < end_ptr {
        let aa = unsafe { *a_ptr };
        let bb = unsafe { *b_ptr };
        if aa != bb {
            return false;
        }
        a_ptr = unsafe { a_ptr.add(1) };
        b_ptr = unsafe { b_ptr.add(1) };
    }
    true
}

#[inline(never)]
fn process_plain_memeq(texts: &[&str], pattern: &str) -> usize {
    let pat_bytes = pattern.as_bytes();
    let pat_len = pat_bytes.len();
    let mut found: usize = 0;
    for line in texts {
        let line_bytes = line.as_bytes();
        let line_len = line_bytes.len();
        for i in 0..(line_len - pat_len) {
            if plain_memeq(&line_bytes[i..(i + pat_len)], pat_bytes) {
                found += 1;
            }
        }
    }
    found
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
            if memx::memeq_basic(&line_bytes[i..(i + pat_len)], pat_bytes) {
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
            if memx::memeq_libc(&line_bytes[i..(i + pat_len)], pat_bytes) {
                found += 1;
            }
        }
    }
    found
}

mod create_data;

fn criterion_benchmark(c: &mut Criterion) {
    let (v, match_cnt, pat_string_s, _pat_regex_s, _pat_glob_s) = create_data::create_data_eq();
    let vv: Vec<&str> = v.iter().map(|item| item.as_str()).collect();
    let pat_string = pat_string_s.to_string();
    //
    let n = process_plain_memeq(black_box(&vv), black_box(pat_string_s));
    assert_eq!(n, match_cnt);
    let n = process_std_memeq(black_box(&vv), black_box(pat_string_s));
    assert_eq!(n, match_cnt);
    let n = process_memx_memeq(black_box(&vv), black_box(&pat_string));
    assert_eq!(n, match_cnt);
    let n = process_memx_memeq_basic(black_box(&vv), black_box(&pat_string));
    assert_eq!(n, match_cnt);
    let n = process_memx_memeq_libc(black_box(&vv), black_box(&pat_string));
    assert_eq!(n, match_cnt);
    //
    c.bench_function("plain_memeq", |b| {
        b.iter(|| {
            let _r = process_plain_memeq(black_box(&vv), black_box(pat_string_s));
        })
    });
    c.bench_function("std_memeq", |b| {
        b.iter(|| {
            let _r = process_std_memeq(black_box(&vv), black_box(pat_string_s));
        })
    });
    c.bench_function("memx_memeq", |b| {
        b.iter(|| {
            let _r = process_memx_memeq(black_box(&vv), black_box(&pat_string));
        })
    });
    c.bench_function("memx_memeq_basic", |b| {
        b.iter(|| {
            let _r = process_memx_memeq_basic(black_box(&vv), black_box(&pat_string));
        })
    });
    c.bench_function("memx_memeq_libc", |b| {
        b.iter(|| {
            let _r = process_memx_memeq_libc(black_box(&vv), black_box(&pat_string));
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
