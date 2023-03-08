#![cfg(not(tarpaulin_include))]
use criterion::{black_box, criterion_group, criterion_main, Criterion};

mod barrier;
use barrier::cache_line_flush;

#[inline(never)]
pub fn std_memrnechr_impl(buf: &[u8], c: u8) -> Option<usize> {
    buf.iter().rposition(|&x| x != c)
}

#[inline(never)]
fn process_std_memrnechr(texts: &[&str], pat_byte: u8) -> usize {
    let mut found: usize = 0;
    for line in texts {
        let line_bytes = line.as_bytes();
        let line_len = line_bytes.len();
        let mut curr_idx = line_len;
        while curr_idx > 0 {
            let r = std_memrnechr_impl(&line_bytes[..curr_idx], pat_byte);
            if let Some(pos) = r {
                found += 1;
                curr_idx = pos;
                //
                let r = memx::memrchr(&line_bytes[..curr_idx], pat_byte);
                if let Some(pos) = r {
                    curr_idx = pos;
                } else {
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
fn process_memx_memrnechr(texts: &[&str], pat_byte: u8) -> usize {
    let mut found: usize = 0;
    for line in texts {
        let line_bytes = line.as_bytes();
        let line_len = line_bytes.len();
        let mut curr_idx = line_len;
        while curr_idx > 0 {
            let r = memx::memrnechr(&line_bytes[..curr_idx], pat_byte);
            if let Some(pos) = r {
                found += 1;
                curr_idx = pos;
                //
                let r = memx::memrchr(&line_bytes[..curr_idx], pat_byte);
                if let Some(pos) = r {
                    curr_idx = pos;
                } else {
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
fn process_memx_memrnechr_basic(texts: &[&str], pat_byte: u8) -> usize {
    let mut found: usize = 0;
    for line in texts {
        let line_bytes = line.as_bytes();
        let line_len = line_bytes.len();
        let mut curr_idx = line_len;
        while curr_idx > 0 {
            let r = memx::mem::memrnechr_basic(&line_bytes[..curr_idx], pat_byte);
            if let Some(pos) = r {
                found += 1;
                curr_idx = pos;
                //
                let r = memx::memrchr(&line_bytes[..curr_idx], pat_byte);
                if let Some(pos) = r {
                    curr_idx = pos;
                } else {
                    break;
                }
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
fn process_memx_memrnechr_sse2(texts: &[&str], pat_byte: u8) -> usize {
    let mut found: usize = 0;
    for line in texts {
        let line_bytes = line.as_bytes();
        let line_len = line_bytes.len();
        let mut curr_idx = line_len;
        while curr_idx > 0 {
            let r = unsafe { memx::arch::x86::_memrnechr_sse2(&line_bytes[..curr_idx], pat_byte) };
            if let Some(pos) = r {
                found += 1;
                curr_idx = pos;
                //
                let r = memx::memrchr(&line_bytes[..curr_idx], pat_byte);
                if let Some(pos) = r {
                    curr_idx = pos;
                } else {
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
fn cache_flush(texts: &[&str]) {
    for i in 0..texts.len() {
        cache_line_flush(texts[i].as_bytes());
    }
}

mod create_data;

fn criterion_benchmark(c: &mut Criterion) {
    let (v, pat_byte, match_cnt) = create_data::create_data_nechr();
    let vv: Vec<&str> = v.iter().map(|item| item.as_str()).collect();
    //
    let n = process_std_memrnechr(black_box(&vv), black_box(pat_byte));
    assert_eq!(n, match_cnt);
    let n = process_memx_memrnechr(black_box(&vv), black_box(pat_byte));
    assert_eq!(n, match_cnt);
    let n = process_memx_memrnechr_basic(black_box(&vv), black_box(pat_byte));
    assert_eq!(n, match_cnt);
    #[cfg(all(
        any(target_arch = "x86_64", target_arch = "x86"),
        target_feature = "sse2"
    ))]
    {
        let n = process_memx_memrnechr_sse2(black_box(&vv), black_box(pat_byte));
        assert_eq!(n, match_cnt);
    }
    cache_flush(&vv);
    //
    c.bench_function("std_memrnechr", |b| {
        b.iter(|| {
            let _r = process_std_memrnechr(black_box(&vv), black_box(pat_byte));
        })
    });
    cache_flush(&vv);
    c.bench_function("memx_memrnechr", |b| {
        b.iter(|| {
            let _r = process_memx_memrnechr(black_box(&vv), black_box(pat_byte));
        })
    });
    cache_flush(&vv);
    c.bench_function("memx_memrnechr_basic", |b| {
        b.iter(|| {
            let _r = process_memx_memrnechr_basic(black_box(&vv), black_box(pat_byte));
        })
    });
    cache_flush(&vv);
    #[cfg(all(
        any(target_arch = "x86_64", target_arch = "x86"),
        target_feature = "sse2"
    ))]
    c.bench_function("memx_memrnechr_sse2", |b| {
        b.iter(|| {
            let _r = process_memx_memrnechr_sse2(black_box(&vv), black_box(pat_byte));
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
