use criterion::{black_box, criterion_group, criterion_main, Criterion};
use naive_opt::Search;

#[inline(never)]
fn process_std_str_str(texts: &[&str], pattern: &str) -> usize {
    let pattern_len = pattern.len();
    let mut found: usize = 0;
    for line in texts {
        let line_len = line.len();
        let mut st = 0;
        loop {
            if let Some(n) = &line[st..].find(pattern) {
                found += 1;
                st = n + pattern_len;
                if st >= line_len {
                    break;
                }
            } else {
                break;
            }
        }
    }
    found
}

#[inline(never)]
fn process_std_string_string(texts: &[String], pattern: &String) -> usize {
    let pattern_len = pattern.len();
    let mut found: usize = 0;
    for line in texts {
        let line_len = line.len();
        let mut st = 0;
        loop {
            if let Some(n) = &line[st..].find(pattern) {
                found += 1;
                st = n + pattern_len;
                if st >= line_len {
                    break;
                }
            } else {
                break;
            }
        }
    }
    found
}

#[inline(never)]
fn process_func_str_str(texts: &[&str], pattern: &str) -> usize {
    let pattern_len = pattern.len();
    let mut found: usize = 0;
    for line in texts {
        let line_len = line.len();
        let mut st = 0;
        loop {
            if let Some(n) = naive_opt::string_search(&line[st..], pattern) {
                found += 1;
                st = n + pattern_len;
                if st >= line_len {
                    break;
                }
            } else {
                break;
            }
        }
    }
    found
}

#[inline(never)]
fn process_func_string_string(texts: &[String], pattern: &String) -> usize {
    let pattern_len = pattern.len();
    let mut found: usize = 0;
    for line in texts {
        let line_len = line.len();
        let mut st = 0;
        loop {
            if let Some(n) = naive_opt::string_search(&line[st..], pattern) {
                found += 1;
                st = n + pattern_len;
                if st >= line_len {
                    break;
                }
            } else {
                break;
            }
        }
    }
    found
}

#[inline(never)]
fn process_trait_str_str(texts: &[&str], pattern: &str) -> usize {
    let pattern_len = pattern.len();
    let mut found: usize = 0;
    for line in texts {
        let line_len = line.len();
        let mut st = 0;
        loop {
            if let Some(n) = (&line[st..]).search(pattern) {
                found += 1;
                st = n + pattern_len;
                if st >= line_len {
                    break;
                }
            } else {
                break;
            }
        }
    }
    found
}

#[inline(never)]
fn process_trait_string_string(texts: &[String], pattern: &String) -> usize {
    let pattern_len = pattern.len();
    let mut found: usize = 0;
    for line in texts {
        let line_len = line.len();
        let mut st = 0;
        loop {
            if let Some(n) = (&line[st..]).search(pattern) {
                found += 1;
                st = n + pattern_len;
                if st >= line_len {
                    break;
                }
            } else {
                break;
            }
        }
    }
    found
}

mod create_data;

fn criterion_benchmark(c: &mut Criterion) {
    let (v, match_cnt, pat_string_s, _pat_regex_s, _pat_glob_s) = create_data::create_data();
    let vv: Vec<&str> = v.iter().map(|item| item.as_str()).collect();
    let pat_string = pat_string_s.to_string();
    //
    let n = process_std_str_str(black_box(&vv), black_box(pat_string_s));
    assert_eq!(n, match_cnt);
    let n = process_std_string_string(black_box(&v), black_box(&pat_string));
    assert_eq!(n, match_cnt);
    let n = process_func_str_str(black_box(&vv), black_box(pat_string_s));
    assert_eq!(n, match_cnt);
    let n = process_func_string_string(black_box(&v), black_box(&pat_string));
    assert_eq!(n, match_cnt);
    let n = process_trait_str_str(black_box(&vv), black_box(pat_string_s));
    assert_eq!(n, match_cnt);
    let n = process_trait_string_string(black_box(&v), black_box(&pat_string));
    assert_eq!(n, match_cnt);
    //
    c.bench_function("std_str_str", |b| {
        b.iter(|| {
            let _r = process_std_str_str(black_box(&vv), black_box(pat_string_s));
        })
    });
    c.bench_function("std_string_string", |b| {
        b.iter(|| {
            let _r = process_std_string_string(black_box(&v), black_box(&pat_string));
        })
    });
    c.bench_function("func_str_str", |b| {
        b.iter(|| {
            let _r = process_func_str_str(black_box(&vv), black_box(pat_string_s));
        })
    });
    c.bench_function("func_string_string", |b| {
        b.iter(|| {
            let _r = process_func_string_string(black_box(&v), black_box(&pat_string));
        })
    });
    c.bench_function("trait_str_str", |b| {
        b.iter(|| {
            let _r = process_trait_str_str(black_box(&vv), black_box(pat_string_s));
        })
    });
    c.bench_function("trait_string_string", |b| {
        b.iter(|| {
            let _r = process_trait_string_string(black_box(&v), black_box(&pat_string));
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
