//#![tarpaulin::skip]
#![cfg(not(tarpaulin_include))]
use criterion::{black_box, criterion_group, criterion_main, Criterion};

mod barrier;
use barrier::cache_line_flush;

#[inline(never)]
fn process_std_memchr(texts: &[&str], pat_byte: u8) -> usize {
    let mut found: usize = 0;
    for line in texts {
        let line_bytes = line.as_bytes();
        let line_len = line_bytes.len();
        let mut curr_idx = 0;
        while curr_idx < line_len {
            let r = &line_bytes[curr_idx..].iter().position(|&x| x == pat_byte);
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
fn process_libc_memchr(texts: &[&str], pat_byte: u8) -> usize {
    // original libc function
    extern "C" {
        fn memchr(cx: *const u8, c: i32, n: usize) -> *const u8;
    }
    #[inline(always)]
    fn _x_libc_memchr(buf: &[u8], c: u8) -> Option<usize> {
        let cx = buf.as_ptr();
        let len = buf.len();
        let r = unsafe { memchr(cx, c.into(), len) };
        if !r.is_null() {
            Some(r as usize - cx as usize)
        } else {
            None
        }
    }
    //
    let mut found: usize = 0;
    for line in texts {
        let line_bytes = line.as_bytes();
        let line_len = line_bytes.len();
        let mut curr_idx = 0;
        while curr_idx < line_len {
            let r = _x_libc_memchr(&line_bytes[curr_idx..], pat_byte);
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
fn process_memchr_memchr(texts: &[&str], pat_byte: u8) -> usize {
    let mut found: usize = 0;
    for line in texts {
        let line_bytes = line.as_bytes();
        let line_len = line_bytes.len();
        let mut curr_idx = 0;
        while curr_idx < line_len {
            let r = memchr::memchr(pat_byte, &line_bytes[curr_idx..]);
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
fn process_memx_memchr(texts: &[&str], pat_byte: u8) -> usize {
    let mut found: usize = 0;
    for line in texts {
        let line_bytes = line.as_bytes();
        let line_len = line_bytes.len();
        let mut curr_idx = 0;
        while curr_idx < line_len {
            let r = memx::memchr(&line_bytes[curr_idx..], pat_byte);
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
fn process_memx_memchr_basic(texts: &[&str], pat_byte: u8) -> usize {
    let mut found: usize = 0;
    for line in texts {
        let line_bytes = line.as_bytes();
        let line_len = line_bytes.len();
        let mut curr_idx = 0;
        while curr_idx < line_len {
            let r = memx::mem::memchr_basic(&line_bytes[curr_idx..], pat_byte);
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
fn process_memx_memchr_sse2(texts: &[&str], pat_byte: u8) -> usize {
    let mut found: usize = 0;
    for line in texts {
        let line_bytes = line.as_bytes();
        let line_len = line_bytes.len();
        let mut curr_idx = 0;
        while curr_idx < line_len {
            let r = unsafe { memx::arch::x86::_memchr_sse2(&line_bytes[curr_idx..], pat_byte) };
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

fn criterion_benchmark(c: &mut Criterion) {
    let (v, pat_byte, match_cnt) = create_data::create_data_chr();
    let vv: Vec<&str> = v.iter().map(|item| item.as_str()).collect();
    //
    let n = process_std_memchr(black_box(&vv), black_box(pat_byte));
    assert_eq!(n, match_cnt);
    let n = process_libc_memchr(black_box(&vv), black_box(pat_byte));
    assert_eq!(n, match_cnt);
    let n = process_memchr_memchr(black_box(&vv), black_box(pat_byte));
    assert_eq!(n, match_cnt);
    let n = process_memx_memchr(black_box(&vv), black_box(pat_byte));
    assert_eq!(n, match_cnt);
    let n = process_memx_memchr_basic(black_box(&vv), black_box(pat_byte));
    assert_eq!(n, match_cnt);
    #[cfg(all(
        any(target_arch = "x86_64", target_arch = "x86"),
        target_feature = "sse2"
    ))]
    {
        let n = process_memx_memchr_sse2(black_box(&vv), black_box(pat_byte));
        assert_eq!(n, match_cnt);
    }
    cache_flush(&vv);
    //
    c.bench_function("std_memchr", |b| {
        b.iter(|| {
            let _r = process_std_memchr(black_box(&vv), black_box(pat_byte));
        })
    });
    cache_flush(&vv);
    c.bench_function("libc_memchr", |b| {
        b.iter(|| {
            let _r = process_libc_memchr(black_box(&vv), black_box(pat_byte));
        })
    });
    cache_flush(&vv);
    c.bench_function("memchr_memchr", |b| {
        b.iter(|| {
            let _r = process_memchr_memchr(black_box(&vv), black_box(pat_byte));
        })
    });
    cache_flush(&vv);
    c.bench_function("memx_memchr", |b| {
        b.iter(|| {
            let _r = process_memx_memchr(black_box(&vv), black_box(pat_byte));
        })
    });
    cache_flush(&vv);
    c.bench_function("memx_memchr_basic", |b| {
        b.iter(|| {
            let _r = process_memx_memchr_basic(black_box(&vv), black_box(pat_byte));
        })
    });
    cache_flush(&vv);
    #[cfg(all(
        any(target_arch = "x86_64", target_arch = "x86"),
        target_feature = "sse2"
    ))]
    c.bench_function("memx_memchr_sse2", |b| {
        b.iter(|| {
            let _r = process_memx_memchr_sse2(black_box(&vv), black_box(pat_byte));
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
