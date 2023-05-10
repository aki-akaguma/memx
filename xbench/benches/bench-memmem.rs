#![cfg(not(tarpaulin_include))]
use criterion::{black_box, criterion_group, criterion_main, Criterion};

mod barrier;
use barrier::cache_line_flush;

#[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
cpufeatures::new!(cpuid_avx2, "avx2");

#[inline(always)]
pub fn std_memmem_impl(haystack: &str, needle: &str) -> Option<usize> {
    haystack.find(needle)
}

#[inline(never)]
fn statistics_std_memmem(texts: &[&str], pattern: &str) -> std::collections::HashMap<usize, usize> {
    #[inline(never)]
    fn _t_(haystack: &str, needle: &str) -> Option<usize> {
        std_memmem_impl(haystack, needle)
    }
    //
    let pat_len = pattern.len();
    use std::collections::HashMap;
    let mut founded: HashMap<usize, usize> = HashMap::new();
    for line in texts {
        let line_len = line.len();
        let mut curr_idx = 0;
        while curr_idx < line_len {
            let r = _t_(&line[curr_idx..], pattern);
            if let Some(pos) = r {
                if let Some(x) = founded.get_mut(&pos) {
                    *x += 1;
                } else {
                    founded.insert(pos, 0);
                }
                curr_idx = curr_idx + pos + pat_len;
            } else {
                break;
            }
        }
    }
    founded
}

#[allow(dead_code)]
#[inline(never)]
fn print_statistics_std_memmem(texts: &[&str], pattern: &str) {
    let map = statistics_std_memmem(texts, pattern);
    let mut vec: Vec<usize> = map.clone().into_keys().collect();
    vec.sort_unstable();
    println!("Statistics:");
    for key in vec {
        println!("{key} => {},", map[&key]);
    }
    println!("");
}

#[inline(never)]
fn process_std_memmem(texts: &[&str], pattern: &str) -> usize {
    #[inline(never)]
    fn _t_(haystack: &str, needle: &str) -> Option<usize> {
        std_memmem_impl(haystack, needle)
    }
    //
    let pat_len = pattern.len();
    let mut found: usize = 0;
    for line in texts {
        let line_len = line.len();
        let mut curr_idx = 0;
        while curr_idx < line_len {
            let r = _t_(&line[curr_idx..], pattern);
            if let Some(pos) = r {
                found += 1;
                curr_idx = curr_idx + pos + pat_len;
            } else {
                break;
            }
        }
    }
    found
}

#[inline(never)]
fn process_libc_memmem(texts: &[&str], pattern: &str) -> usize {
    // original libc function
    extern "C" {
        fn memmem(
            haystack: *const u8,
            hay_len: usize,
            needle: *const u8,
            nee_len: usize,
        ) -> *const u8;
    }
    #[inline(always)]
    fn _x_libc_memmem(haystack: &[u8], needle: &[u8]) -> Option<usize> {
        let hay = haystack.as_ptr();
        let nee = needle.as_ptr();
        let hay_len = haystack.len();
        let nee_len = needle.len();
        if hay_len < nee_len {
            return None;
        }
        let r = unsafe { memmem(hay, hay_len, nee, nee_len) };
        if !r.is_null() {
            Some(r as usize - hay as usize)
        } else {
            None
        }
    }
    #[inline(never)]
    fn _t_(haystack: &[u8], needle: &[u8]) -> Option<usize> {
        _x_libc_memmem(haystack, needle)
    }
    //
    let pat_bytes = pattern.as_bytes();
    let pat_len = pat_bytes.len();
    let mut found: usize = 0;
    for line in texts {
        let line_bytes = line.as_bytes();
        let line_len = line_bytes.len();
        let mut curr_idx = 0;
        while curr_idx < line_len {
            let r = _t_(&line_bytes[curr_idx..], pat_bytes);
            if let Some(pos) = r {
                found += 1;
                curr_idx = curr_idx + pos + pat_len;
            } else {
                break;
            }
        }
    }
    found
}

#[inline(never)]
fn process_memchr_memmem(texts: &[&str], pattern: &str) -> usize {
    #[inline(never)]
    fn _t_(haystack: &[u8], needle: &[u8]) -> Option<usize> {
        memchr::memmem::find(haystack, needle)
    }
    //
    let pat_bytes = pattern.as_bytes();
    let pat_len = pat_bytes.len();
    let mut found: usize = 0;
    for line in texts {
        let line_bytes = line.as_bytes();
        let line_len = line_bytes.len();
        let mut curr_idx = 0;
        while curr_idx < line_len {
            let r = _t_(&line_bytes[curr_idx..], pat_bytes);
            if let Some(pos) = r {
                found += 1;
                curr_idx = curr_idx + pos + pat_len;
            } else {
                break;
            }
        }
    }
    found
}

#[inline(never)]
fn process_memx_memmem(texts: &[&str], pattern: &str) -> usize {
    #[inline(never)]
    fn _t_(haystack: &[u8], needle: &[u8]) -> Option<usize> {
        memx::memmem(haystack, needle)
    }
    //
    let pat_bytes = pattern.as_bytes();
    let pat_len = pat_bytes.len();
    let mut found: usize = 0;
    for line in texts {
        let line_bytes = line.as_bytes();
        let line_len = line_bytes.len();
        let mut curr_idx = 0;
        while curr_idx < line_len {
            let r = _t_(&line_bytes[curr_idx..], pat_bytes);
            if let Some(pos) = r {
                found += 1;
                curr_idx = curr_idx + pos + pat_len;
            } else {
                break;
            }
        }
    }
    found
}

#[inline(never)]
fn process_memx_memmem_basic(texts: &[&str], pattern: &str) -> usize {
    #[inline(never)]
    fn _t_(haystack: &[u8], needle: &[u8]) -> Option<usize> {
        memx::mem::memmem_basic(haystack, needle)
    }
    //
    let pat_bytes = pattern.as_bytes();
    let pat_len = pat_bytes.len();
    let mut found: usize = 0;
    for line in texts {
        let line_bytes = line.as_bytes();
        let line_len = line_bytes.len();
        let mut curr_idx = 0;
        while curr_idx < line_len {
            let r = _t_(&line_bytes[curr_idx..], pat_bytes);
            if let Some(pos) = r {
                found += 1;
                curr_idx = curr_idx + pos + pat_len;
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
fn process_memx_memmem_sse2(texts: &[&str], pattern: &str) -> usize {
    #[inline(never)]
    fn _t_(haystack: &[u8], needle: &[u8]) -> Option<usize> {
        memx::arch::x86::memmem_sse2(haystack, needle)
    }
    //
    let pat_bytes = pattern.as_bytes();
    let pat_len = pat_bytes.len();
    let mut found: usize = 0;
    for line in texts {
        let line_bytes = line.as_bytes();
        let line_len = line_bytes.len();
        let mut curr_idx = 0;
        while curr_idx < line_len {
            let r = _t_(&line_bytes[curr_idx..], pat_bytes);
            if let Some(pos) = r {
                found += 1;
                curr_idx = curr_idx + pos + pat_len;
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
fn process_memx_memmem_avx2(texts: &[&str], pattern: &str) -> usize {
    #[inline(never)]
    fn _t_(haystack: &[u8], needle: &[u8]) -> Option<usize> {
        memx::arch::x86::memmem_avx2(haystack, needle)
    }
    //
    let pat_bytes = pattern.as_bytes();
    let pat_len = pat_bytes.len();
    let mut found: usize = 0;
    for line in texts {
        let line_bytes = line.as_bytes();
        let line_len = line_bytes.len();
        let mut curr_idx = 0;
        while curr_idx < line_len {
            let r = _t_(&line_bytes[curr_idx..], pat_bytes);
            if let Some(pos) = r {
                found += 1;
                curr_idx = curr_idx + pos + pat_len;
            } else {
                break;
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

fn criterion_benchmark(cr: &mut Criterion) {
    let (v, pat_string_s, match_cnt) = create_data::create_data_mem();
    let vv: Vec<&str> = v.iter().map(|item| item.as_str()).collect();
    let pat_string = pat_string_s.to_string();
    //
    if let Ok(_val) = std::env::var("AKI_TEST_DAT_CHECK") {
        let n = process_std_memmem(black_box(&vv), black_box(&pat_string));
        assert_eq!(n, match_cnt);
        let n = process_libc_memmem(black_box(&vv), black_box(&pat_string));
        assert_eq!(n, match_cnt);
        let n = process_memchr_memmem(black_box(&vv), black_box(&pat_string));
        assert_eq!(n, match_cnt);
        let n = process_memx_memmem(black_box(&vv), black_box(&pat_string));
        assert_eq!(n, match_cnt);
        let n = process_memx_memmem_basic(black_box(&vv), black_box(&pat_string));
        assert_eq!(n, match_cnt);
        #[cfg(all(
            any(target_arch = "x86_64", target_arch = "x86"),
            target_feature = "sse2"
        ))]
        {
            let n = process_memx_memmem_sse2(black_box(&vv), black_box(&pat_string));
            assert_eq!(n, match_cnt);
        }
        #[cfg(all(
            any(target_arch = "x86_64", target_arch = "x86"),
            target_feature = "sse2"
        ))]
        if cpuid_avx2::get() {
            let n = process_memx_memmem_avx2(black_box(&vv), black_box(&pat_string));
            assert_eq!(n, match_cnt);
        }
        std::process::exit(0);
    }
    //
    if let Ok(_val) = std::env::var("AKI_TEST_STATISTICS") {
        print_statistics_std_memmem(black_box(&vv), black_box(&pat_string));
        std::process::exit(0);
    }
    //
    cache_flush(&vv, &pat_string);
    //
    cr.bench_function("std_memmem", |b| {
        b.iter(|| {
            let _r = process_std_memmem(black_box(&vv), black_box(&pat_string));
        })
    });
    cache_flush(&vv, &pat_string);
    cr.bench_function("libc_memmem", |b| {
        b.iter(|| {
            let _r = process_libc_memmem(black_box(&vv), black_box(&pat_string));
        })
    });
    cache_flush(&vv, &pat_string);
    cr.bench_function("memchr_memmem", |b| {
        b.iter(|| {
            let _r = process_memchr_memmem(black_box(&vv), black_box(&pat_string));
        })
    });
    cache_flush(&vv, &pat_string);
    cr.bench_function("memx_memmem", |b| {
        b.iter(|| {
            let _r = process_memx_memmem(black_box(&vv), black_box(&pat_string));
        })
    });
    cache_flush(&vv, &pat_string);
    cr.bench_function("memx_memmem_basic", |b| {
        b.iter(|| {
            let _r = process_memx_memmem_basic(black_box(&vv), black_box(&pat_string));
        })
    });
    cache_flush(&vv, &pat_string);
    #[cfg(all(
        any(target_arch = "x86_64", target_arch = "x86"),
        target_feature = "sse2"
    ))]
    cr.bench_function("memx_memmem_sse2", |b| {
        b.iter(|| {
            let _r = process_memx_memmem_sse2(black_box(&vv), black_box(&pat_string));
        })
    });
    cache_flush(&vv, &pat_string);
    #[cfg(all(
        any(target_arch = "x86_64", target_arch = "x86"),
        target_feature = "sse2"
    ))]
    if cpuid_avx2::get() {
        cr.bench_function("memx_memmem_avx2", |b| {
            b.iter(|| {
                let _r = process_memx_memmem_avx2(black_box(&vv), black_box(&pat_string));
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
