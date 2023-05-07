#![cfg(not(tarpaulin_include))]
use criterion::{black_box, criterion_group, criterion_main, Criterion};

mod barrier;
use barrier::cache_line_flush;

#[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
cpufeatures::new!(cpuid_avx2, "avx2");

#[inline(always)]
pub fn std_memchr_qpl_impl(buf: &[u8], by1: u8, by2: u8, by3: u8, by4: u8) -> Option<usize> {
    buf.iter()
        .position(|&x| x == by1 || x == by2 || x == by3 || x == by4)
}

#[inline(never)]
fn statistics_std_memchr_qpl(
    texts: &[&str],
    pat: (u8, u8, u8, u8),
) -> std::collections::HashMap<usize, usize> {
    #[inline(never)]
    fn _t_(buf: &[u8], by1: u8, by2: u8, by3: u8, by4: u8) -> Option<usize> {
        std_memchr_qpl_impl(buf, by1, by2, by3, by4)
    }
    //
    use std::collections::HashMap;
    let mut founded: HashMap<usize, usize> = HashMap::new();
    for line in texts {
        let line_bytes = line.as_bytes();
        let line_len = line_bytes.len();
        let mut curr_idx = 0;
        while curr_idx < line_len {
            let r = _t_(&line_bytes[curr_idx..], pat.0, pat.1, pat.2, pat.3);
            if let Some(pos) = r {
                if let Some(x) = founded.get_mut(&pos) {
                    *x += 1;
                } else {
                    founded.insert(pos, 0);
                }
                curr_idx = pos + curr_idx + 1;
            } else {
                break;
            }
        }
    }
    founded
}

#[allow(dead_code)]
#[inline(never)]
fn print_statistics_std_memchr_qpl(texts: &[&str], pat: (u8, u8, u8, u8)) {
    let map = statistics_std_memchr_qpl(texts, pat);
    let mut vec: Vec<usize> = map.clone().into_keys().collect();
    vec.sort_unstable();
    println!("Statistics:");
    for key in vec {
        println!("{key} => {},", map[&key]);
    }
    println!("");
}

#[inline(never)]
fn process_std_memchr_qpl(texts: &[&str], pat: (u8, u8, u8, u8)) -> usize {
    #[inline(never)]
    fn _t_(buf: &[u8], by1: u8, by2: u8, by3: u8, by4: u8) -> Option<usize> {
        std_memchr_qpl_impl(buf, by1, by2, by3, by4)
    }
    //
    let mut found: usize = 0;
    for line in texts {
        let line_bytes = line.as_bytes();
        let line_len = line_bytes.len();
        let mut curr_idx = 0;
        while curr_idx < line_len {
            let r = _t_(&line_bytes[curr_idx..], pat.0, pat.1, pat.2, pat.3);
            if let Some(pos) = r {
                found += 1;
                curr_idx = pos + curr_idx + 1;
            } else {
                break;
            }
        }
    }
    found
}

#[inline(never)]
fn process_memx_memchr_qpl(texts: &[&str], pat: (u8, u8, u8, u8)) -> usize {
    #[inline(never)]
    fn _t_(buf: &[u8], by1: u8, by2: u8, by3: u8, by4: u8) -> Option<usize> {
        memx::memchr_qpl(buf, by1, by2, by3, by4)
    }
    //
    let mut found: usize = 0;
    for line in texts {
        let line_bytes = line.as_bytes();
        let line_len = line_bytes.len();
        let mut curr_idx = 0;
        while curr_idx < line_len {
            let r = _t_(&line_bytes[curr_idx..], pat.0, pat.1, pat.2, pat.3);
            if let Some(pos) = r {
                found += 1;
                curr_idx = pos + curr_idx + 1;
            } else {
                break;
            }
        }
    }
    found
}

#[inline(never)]
fn process_memx_memchr_qpl_basic(texts: &[&str], pat: (u8, u8, u8, u8)) -> usize {
    #[inline(never)]
    fn _t_(buf: &[u8], by1: u8, by2: u8, by3: u8, by4: u8) -> Option<usize> {
        memx::mem::memchr_qpl_basic(buf, by1, by2, by3, by4)
    }
    //
    let mut found: usize = 0;
    for line in texts {
        let line_bytes = line.as_bytes();
        let line_len = line_bytes.len();
        let mut curr_idx = 0;
        while curr_idx < line_len {
            let r = _t_(&line_bytes[curr_idx..], pat.0, pat.1, pat.2, pat.3);
            if let Some(pos) = r {
                found += 1;
                curr_idx = pos + curr_idx + 1;
            } else {
                break;
            }
        }
    }
    found
}

#[cfg(all(
    any(target_arch = "x86_64", target_arch = "x86"),
    target_feature = "sse2"
))]
#[inline(never)]
fn process_memx_memchr_qpl_sse2(texts: &[&str], pat: (u8, u8, u8, u8)) -> usize {
    #[inline(never)]
    fn _t_(buf: &[u8], by1: u8, by2: u8, by3: u8, by4: u8) -> Option<usize> {
        let needle = memx::B1Qpl::new(by1, by2, by3, by4);
        unsafe { memx::arch::x86::_memchr_qpl_sse2(buf, needle) }
    }
    //
    let mut found: usize = 0;
    for line in texts {
        let line_bytes = line.as_bytes();
        let line_len = line_bytes.len();
        let mut curr_idx = 0;
        while curr_idx < line_len {
            let r = _t_(&line_bytes[curr_idx..], pat.0, pat.1, pat.2, pat.3);
            if let Some(pos) = r {
                found += 1;
                curr_idx = pos + curr_idx + 1;
            } else {
                break;
            }
        }
    }
    found
}

#[cfg(all(
    any(target_arch = "x86_64", target_arch = "x86"),
    target_feature = "sse2"
))]
#[inline(never)]
fn process_memx_memchr_qpl_avx2(texts: &[&str], pat: (u8, u8, u8, u8)) -> usize {
    #[inline(never)]
    fn _t_(buf: &[u8], by1: u8, by2: u8, by3: u8, by4: u8) -> Option<usize> {
        let needle = memx::B1Qpl::new(by1, by2, by3, by4);
        unsafe { memx::arch::x86::_memchr_qpl_avx2(buf, needle) }
    }
    //
    let mut found: usize = 0;
    for line in texts {
        let line_bytes = line.as_bytes();
        let line_len = line_bytes.len();
        let mut curr_idx = 0;
        while curr_idx < line_len {
            let r = _t_(&line_bytes[curr_idx..], pat.0, pat.1, pat.2, pat.3);
            if let Some(pos) = r {
                found += 1;
                curr_idx = pos + curr_idx + 1;
            } else {
                break;
            }
        }
    }
    found
}

#[inline(never)]
fn cache_flush(texts: &[&str]) {
    for i in 0..texts.len() {
        cache_line_flush(texts[i].as_bytes());
    }
}

mod create_data;

fn criterion_benchmark(cr: &mut Criterion) {
    let (v, pats, match_cnt) = create_data::create_data_chr_qpl();
    let vv: Vec<&str> = v.iter().map(|item| item.as_str()).collect();
    //
    if let Ok(_val) = std::env::var("AKI_TEST_DAT_CHECK") {
        let n = process_std_memchr_qpl(black_box(&vv), black_box(pats));
        assert_eq!(n, match_cnt);
        let n = process_memx_memchr_qpl(black_box(&vv), black_box(pats));
        assert_eq!(n, match_cnt);
        let n = process_memx_memchr_qpl_basic(black_box(&vv), black_box(pats));
        assert_eq!(n, match_cnt);
        #[cfg(all(
            any(target_arch = "x86_64", target_arch = "x86"),
            target_feature = "sse2"
        ))]
        {
            let n = process_memx_memchr_qpl_sse2(black_box(&vv), black_box(pats));
            assert_eq!(n, match_cnt);
        }
        #[cfg(all(
            any(target_arch = "x86_64", target_arch = "x86"),
            target_feature = "sse2"
        ))]
        if cpuid_avx2::get() {
            let n = process_memx_memchr_qpl_avx2(black_box(&vv), black_box(pats));
            assert_eq!(n, match_cnt);
        }
        std::process::exit(0);
    }
    //
    if let Ok(_val) = std::env::var("AKI_TEST_STATISTICS") {
        print_statistics_std_memchr_qpl(black_box(&vv), black_box(pats));
        std::process::exit(0);
    }
    //
    cache_flush(&vv);
    //
    cr.bench_function("std_memchr_qpl", |b| {
        b.iter(|| {
            let _r = process_std_memchr_qpl(black_box(&vv), black_box(pats));
        })
    });
    cache_flush(&vv);
    cr.bench_function("memx_memchr_qpl", |b| {
        b.iter(|| {
            let _r = process_memx_memchr_qpl(black_box(&vv), black_box(pats));
        })
    });
    cache_flush(&vv);
    cr.bench_function("memx_memchr_q_basic", |b| {
        b.iter(|| {
            let _r = process_memx_memchr_qpl_basic(black_box(&vv), black_box(pats));
        })
    });
    cache_flush(&vv);
    #[cfg(all(
        any(target_arch = "x86_64", target_arch = "x86"),
        target_feature = "sse2"
    ))]
    cr.bench_function("memx_memchr_q_sse2", |b| {
        b.iter(|| {
            let _r = process_memx_memchr_qpl_sse2(black_box(&vv), black_box(pats));
        })
    });
    cache_flush(&vv);
    #[cfg(all(
        any(target_arch = "x86_64", target_arch = "x86"),
        target_feature = "sse2"
    ))]
    if cpuid_avx2::get() {
        cr.bench_function("memx_memchr_q_avx2", |b| {
            b.iter(|| {
                let _r = process_memx_memchr_qpl_avx2(black_box(&vv), black_box(pats));
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