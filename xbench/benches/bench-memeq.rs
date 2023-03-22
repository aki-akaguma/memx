#![cfg(not(tarpaulin_include))]
use criterion::{black_box, criterion_group, criterion_main, Criterion};

mod barrier;
use barrier::cache_line_flush;

#[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
cpufeatures::new!(cpuid_avx2, "avx2");

#[inline(always)]
pub fn std_memeq_impl(a: &[u8], b: &[u8]) -> bool {
    a == b
}

macro_rules! j_bool {
    ($j: expr) => {
        $j < 16 || $j % 4 == 0 || $j % 7 == 0
    };
}

#[inline(never)]
fn statistics_std_memeq(texts: &[&str], pattern: &str) -> std::collections::HashMap<bool, usize> {
    #[inline(never)]
    fn _t_(a: &[u8], b: &[u8]) -> bool {
        std_memeq_impl(a, b)
    }
    //
    let pat_bytes = pattern.as_bytes();
    let pat_len = pat_bytes.len();
    use std::collections::HashMap;
    let mut founded: HashMap<bool, usize> = HashMap::new();
    for line in texts {
        let line_bytes = line.as_bytes();
        let line_len = line_bytes.len();
        for i in 0..(line_len - pat_len) {
            if j_bool!(i) {
                let r = _t_(&line_bytes[i..(i + pat_len)], pat_bytes);
                if let Some(x) = founded.get_mut(&r) {
                    *x += 1;
                } else {
                    founded.insert(r, 1);
                }
            }
        }
    }
    founded
}

#[allow(dead_code)]
#[inline(never)]
fn print_statistics_std_memeq(texts: &[&str], pattern: &str) {
    let map = statistics_std_memeq(texts, pattern);
    let mut vec: Vec<bool> = map.clone().into_keys().collect();
    vec.sort_unstable();
    println!("Statistics:");
    for key in vec {
        println!("{key:#?} => {},", map[&key]);
    }
    println!("");
}

#[inline(never)]
fn process_std_memeq(texts: &[&str], pattern: &str) -> usize {
    #[inline(never)]
    fn _t_(a: &[u8], b: &[u8]) -> bool {
        std_memeq_impl(a, b)
    }
    //
    let pat_bytes = pattern.as_bytes();
    let pat_len = pat_bytes.len();
    let mut found: usize = 0;
    for line in texts {
        let line_bytes = line.as_bytes();
        let line_len = line_bytes.len();
        for i in 0..(line_len - pat_len) {
            if j_bool!(i) {
                if _t_(&line_bytes[i..(i + pat_len)], pat_bytes) {
                    found += 1;
                }
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
    #[inline(never)]
    fn _t_(a: &[u8], b: &[u8]) -> bool {
        _x_libc_memeq(a, b)
    }
    //
    let pat_bytes = pattern.as_bytes();
    let pat_len = pat_bytes.len();
    let mut found: usize = 0;
    for line in texts {
        let line_bytes = line.as_bytes();
        let line_len = line_bytes.len();
        for i in 0..(line_len - pat_len) {
            if j_bool!(i) {
                if _t_(&line_bytes[i..(i + pat_len)], pat_bytes) {
                    found += 1;
                }
            }
        }
    }
    found
}

#[inline(never)]
fn process_memx_memeq(texts: &[&str], pattern: &str) -> usize {
    #[inline(never)]
    fn _t_(a: &[u8], b: &[u8]) -> bool {
        memx::memeq(a, b)
    }
    //
    let pat_bytes = pattern.as_bytes();
    let pat_len = pat_bytes.len();
    let mut found: usize = 0;
    for line in texts {
        let line_bytes = line.as_bytes();
        let line_len = line_bytes.len();
        for i in 0..(line_len - pat_len) {
            if j_bool!(i) {
                if _t_(&line_bytes[i..(i + pat_len)], pat_bytes) {
                    found += 1;
                }
            }
        }
    }
    found
}

#[inline(never)]
fn process_memx_memeq_basic(texts: &[&str], pattern: &str) -> usize {
    #[inline(never)]
    fn _t_(a: &[u8], b: &[u8]) -> bool {
        memx::mem::memeq_basic(a, b)
    }
    //
    let pat_bytes = pattern.as_bytes();
    let pat_len = pat_bytes.len();
    let mut found: usize = 0;
    for line in texts {
        let line_bytes = line.as_bytes();
        let line_len = line_bytes.len();
        for i in 0..(line_len - pat_len) {
            if j_bool!(i) {
                if _t_(&line_bytes[i..(i + pat_len)], pat_bytes) {
                    found += 1;
                }
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
fn process_memx_memeq_sse2(texts: &[&str], pattern: &str) -> usize {
    #[inline(never)]
    fn _t_(a: &[u8], b: &[u8]) -> bool {
        unsafe { memx::arch::x86::_memeq_sse2(a, b) }
    }
    //
    let pat_bytes = pattern.as_bytes();
    let pat_len = pat_bytes.len();
    let mut found: usize = 0;
    for line in texts {
        let line_bytes = line.as_bytes();
        let line_len = line_bytes.len();
        for i in 0..(line_len - pat_len) {
            if j_bool!(i) {
                if _t_(&line_bytes[i..(i + pat_len)], pat_bytes) {
                    found += 1;
                }
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
fn process_memx_memeq_avx2(texts: &[&str], pattern: &str) -> usize {
    #[inline(never)]
    fn _t_(a: &[u8], b: &[u8]) -> bool {
        unsafe { memx::arch::x86::_memeq_avx2(a, b) }
    }
    //
    let pat_bytes = pattern.as_bytes();
    let pat_len = pat_bytes.len();
    let mut found: usize = 0;
    for line in texts {
        let line_bytes = line.as_bytes();
        let line_len = line_bytes.len();
        for i in 0..(line_len - pat_len) {
            if j_bool!(i) {
                if _t_(&line_bytes[i..(i + pat_len)], pat_bytes) {
                    found += 1;
                }
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
    if let Ok(_val) = std::env::var("AKI_TEST_DAT_CHECK") {
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
        #[cfg(all(
            any(target_arch = "x86_64", target_arch = "x86"),
            target_feature = "sse2"
        ))]
        {
            let n = process_memx_memeq_sse2(black_box(&vv), black_box(&pat_string));
            assert_eq!(n, match_cnt);
        }
        #[cfg(all(
            any(target_arch = "x86_64", target_arch = "x86"),
            target_feature = "sse2"
        ))]
        if cpuid_avx2::get() {
            let n = process_memx_memeq_avx2(black_box(&vv), black_box(&pat_string));
            assert_eq!(n, match_cnt);
        }
        std::process::exit(0);
    }
    //
    if let Ok(_val) = std::env::var("AKI_TEST_STATISTICS") {
        print_statistics_std_memeq(black_box(&vv), black_box(&pat_string));
        std::process::exit(0);
    }
    //
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
    cache_flush(&vv, &pat_string);
    #[cfg(all(
        any(target_arch = "x86_64", target_arch = "x86"),
        target_feature = "sse2"
    ))]
    c.bench_function("memx_memeq_sse2", |b| {
        b.iter(|| {
            let _r = process_memx_memeq_sse2(black_box(&vv), black_box(&pat_string));
        })
    });
    cache_flush(&vv, &pat_string);
    #[cfg(all(
        any(target_arch = "x86_64", target_arch = "x86"),
        target_feature = "sse2"
    ))]
    if cpuid_avx2::get() {
        c.bench_function("memx_memeq_avx2", |b| {
            b.iter(|| {
                let _r = process_memx_memeq_avx2(black_box(&vv), black_box(&pat_string));
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
