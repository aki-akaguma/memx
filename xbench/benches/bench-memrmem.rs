#![cfg(not(tarpaulin_include))]
use criterion::{black_box, criterion_group, criterion_main, Criterion};

mod barrier;
use barrier::cache_line_flush;

#[inline(never)]
fn process_std_memrmem(texts: &[&str], pattern: &str) -> usize {
    let mut found: usize = 0;
    for line in texts {
        let line_len = line.len();
        let mut curr_idx = line_len;
        while curr_idx > 0 {
            let r = line[..curr_idx].rfind(pattern);
            if let Some(pos) = r {
                found += 1;
                curr_idx = pos;
            } else {
                break;
            }
        }
    }
    found
}

#[cfg(target_arch = "x86_64")]
#[inline(never)]
fn process_libc_memrmem(texts: &[&str], pattern: &str) -> usize {
    // debian support: apt install publib-dev
    // original libc function
    #[link(name = "pub")]
    extern "C" {
        fn memrmem(
            haystack: *const u8,
            hay_len: usize,
            needle: *const u8,
            nee_len: usize,
        ) -> *const u8;
    }
    #[inline(always)]
    fn _x_libc_memrmem(haystack: &[u8], needle: &[u8]) -> Option<usize> {
        let hay = haystack.as_ptr();
        let nee = needle.as_ptr();
        let hay_len = haystack.len();
        let nee_len = needle.len();
        if hay_len < nee_len {
            return None;
        }
        let r = unsafe { memrmem(hay, hay_len, nee, nee_len) };
        if !r.is_null() {
            Some(r as usize - hay as usize)
        } else {
            None
        }
    }
    //
    let pat_bytes = pattern.as_bytes();
    let mut found: usize = 0;
    for line in texts {
        let line_bytes = line.as_bytes();
        let line_len = line_bytes.len();
        let mut curr_idx = line_len;
        while curr_idx > 0 {
            let r = _x_libc_memrmem(&line_bytes[..curr_idx], pat_bytes);
            if let Some(pos) = r {
                found += 1;
                curr_idx = pos;
            } else {
                break;
            }
        }
    }
    found
}

#[inline(never)]
fn process_memchr_memrmem(texts: &[&str], pattern: &str) -> usize {
    let pat_bytes = pattern.as_bytes();
    let mut found: usize = 0;
    for line in texts {
        let line_bytes = line.as_bytes();
        let line_len = line_bytes.len();
        let mut curr_idx = line_len;
        while curr_idx > 0 {
            let r = memchr::memmem::rfind(&line_bytes[..curr_idx], pat_bytes);
            if let Some(pos) = r {
                found += 1;
                curr_idx = pos;
            } else {
                break;
            }
        }
    }
    found
}

#[inline(never)]
fn process_memx_memrmem(texts: &[&str], pattern: &str) -> usize {
    let pat_bytes = pattern.as_bytes();
    let mut found: usize = 0;
    for line in texts {
        let line_bytes = line.as_bytes();
        let line_len = line_bytes.len();
        let mut curr_idx = line_len;
        while curr_idx > 0 {
            let r = memx::memrmem(&line_bytes[..curr_idx], pat_bytes);
            if let Some(pos) = r {
                found += 1;
                curr_idx = pos;
            } else {
                break;
            }
        }
    }
    found
}

#[inline(never)]
fn process_memx_memrmem_basic(texts: &[&str], pattern: &str) -> usize {
    let pat_bytes = pattern.as_bytes();
    let mut found: usize = 0;
    for line in texts {
        let line_bytes = line.as_bytes();
        let line_len = line_bytes.len();
        let mut curr_idx = line_len;
        while curr_idx > 0 {
            let r = memx::mem::memrmem_basic(&line_bytes[..curr_idx], pat_bytes);
            if let Some(pos) = r {
                found += 1;
                curr_idx = pos;
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
fn process_memx_memrmem_sse2_avx2(texts: &[&str], pattern: &str) -> usize {
    let pat_bytes = pattern.as_bytes();
    let mut found: usize = 0;
    for line in texts {
        let line_bytes = line.as_bytes();
        let line_len = line_bytes.len();
        let mut curr_idx = line_len;
        while curr_idx > 0 {
            let r =
                unsafe { memx::arch::x86::_memrmem_sse2_avx2(&line_bytes[..curr_idx], pat_bytes) };
            if let Some(pos) = r {
                found += 1;
                curr_idx = pos;
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

fn criterion_benchmark(c: &mut Criterion) {
    let (v, pat_string_s, match_cnt) = create_data::create_data_mem();
    let vv: Vec<&str> = v.iter().map(|item| item.as_str()).collect();
    let pat_string = pat_string_s.to_string();
    //
    let n = process_std_memrmem(black_box(&vv), black_box(&pat_string));
    assert_eq!(n, match_cnt);
    #[cfg(target_arch = "x86_64")]
    {
        let n = process_libc_memrmem(black_box(&vv), black_box(&pat_string));
        assert_eq!(n, match_cnt);
    }
    let n = process_memchr_memrmem(black_box(&vv), black_box(&pat_string));
    assert_eq!(n, match_cnt);
    let n = process_memx_memrmem(black_box(&vv), black_box(&pat_string));
    assert_eq!(n, match_cnt);
    let n = process_memx_memrmem_basic(black_box(&vv), black_box(&pat_string));
    assert_eq!(n, match_cnt);
    #[cfg(all(
        any(target_arch = "x86_64", target_arch = "x86"),
        target_feature = "sse2"
    ))]
    {
        let n = process_memx_memrmem_sse2_avx2(black_box(&vv), black_box(&pat_string));
        assert_eq!(n, match_cnt);
    }
    cache_flush(&vv, &pat_string);
    //
    c.bench_function("std_memrmem", |b| {
        b.iter(|| {
            let _r = process_std_memrmem(black_box(&vv), black_box(&pat_string));
        })
    });
    cache_flush(&vv, &pat_string);
    #[cfg(target_arch = "x86_64")]
    c.bench_function("libc_memrmem", |b| {
        b.iter(|| {
            let _r = process_libc_memrmem(black_box(&vv), black_box(&pat_string));
        })
    });
    cache_flush(&vv, &pat_string);
    c.bench_function("memchr_memrmem", |b| {
        b.iter(|| {
            let _r = process_memchr_memrmem(black_box(&vv), black_box(&pat_string));
        })
    });
    cache_flush(&vv, &pat_string);
    c.bench_function("memx_memrmem", |b| {
        b.iter(|| {
            let _r = process_memx_memrmem(black_box(&vv), black_box(&pat_string));
        })
    });
    cache_flush(&vv, &pat_string);
    c.bench_function("memx_memrmem_basic", |b| {
        b.iter(|| {
            let _r = process_memx_memrmem_basic(black_box(&vv), black_box(&pat_string));
        })
    });
    cache_flush(&vv, &pat_string);
    #[cfg(all(
        any(target_arch = "x86_64", target_arch = "x86"),
        target_feature = "sse2"
    ))]
    c.bench_function("memx_memrmem_sse2", |b| {
        b.iter(|| {
            let _r = process_memx_memrmem_sse2_avx2(black_box(&vv), black_box(&pat_string));
        })
    });
}

criterion_group! {
    name = benches;
    config = Criterion::default()
        .warm_up_time(std::time::Duration::from_millis(300))
        .measurement_time(std::time::Duration::from_millis(4000));
    targets = criterion_benchmark
}
//criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
