use criterion::{black_box, criterion_group, criterion_main, Criterion};

mod barrier;
use barrier::memory_barrier;

#[inline(never)]
fn process_std_memrchr(texts: &[&str], pat_byte: u8) -> usize {
    let mut found: usize = 0;
    for line in texts {
        let line_bytes = line.as_bytes();
        let line_len = line_bytes.len();
        let mut curr_idx = line_len;
        while curr_idx > 0 {
            let r = &line_bytes[..curr_idx].iter().rposition(|&x| x == pat_byte);
            if let Some(pos) = r {
                found += 1;
                curr_idx = *pos;
            } else {
                break;
            }
        }
    }
    found
}

#[inline(never)]
fn process_libc_memrchr(texts: &[&str], pat_byte: u8) -> usize {
    // original libc function
    extern {
        fn memrchr(cx: *const u8, c: i32, n: usize) -> *const u8;
    }
    #[inline(always)]
    fn _x_libc_memrchr(buf: &[u8], c: u8) -> Option<usize> {
        let cx = buf.as_ptr();
        let len = buf.len();
        let r = unsafe { memrchr(cx, c.into(), len) };
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
        let mut curr_idx = line_len;
        while curr_idx > 0 {
            let r = _x_libc_memrchr(&line_bytes[..curr_idx], pat_byte);
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
fn process_memchr_memrchr(texts: &[&str], pat_byte: u8) -> usize {
    let mut found: usize = 0;
    for line in texts {
        let line_bytes = line.as_bytes();
        let line_len = line_bytes.len();
        let mut curr_idx = line_len;
        while curr_idx > 0 {
            let r = memchr::memrchr(pat_byte, &line_bytes[..curr_idx]);
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
fn process_memx_memrchr(texts: &[&str], pat_byte: u8) -> usize {
    let mut found: usize = 0;
    for line in texts {
        let line_bytes = line.as_bytes();
        let line_len = line_bytes.len();
        let mut curr_idx = line_len;
        while curr_idx > 0 {
            let r = memx::memrchr(&line_bytes[..curr_idx], pat_byte);
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
fn process_memx_memrchr_basic(texts: &[&str], pat_byte: u8) -> usize {
    let mut found: usize = 0;
    for line in texts {
        let line_bytes = line.as_bytes();
        let line_len = line_bytes.len();
        let mut curr_idx = line_len;
        while curr_idx > 0 {
            let r = memx::mem::memrchr_basic(&line_bytes[..curr_idx], pat_byte);
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

#[cfg(all(any(target_arch = "x86_64", target_arch = "x86"), target_feature = "sse2"))]
#[inline(never)]
fn process_memx_memrchr_sse2(texts: &[&str], pat_byte: u8) -> usize {
    let mut found: usize = 0;
    for line in texts {
        let line_bytes = line.as_bytes();
        let line_len = line_bytes.len();
        let mut curr_idx = line_len;
        while curr_idx > 0 {
            let r = unsafe { memx::arch::x86::_memrchr_sse2(&line_bytes[..curr_idx], pat_byte) };
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

/*
#[inline(never)]
fn process_memx_memrchr_libc(texts: &[&str], pat_byte: u8) -> usize {
    let mut found: usize = 0;
    for line in texts {
        let line_bytes = line.as_bytes();
        let line_len = line_bytes.len();
        let mut curr_idx = line_len;
        while curr_idx > 0 {
            let r = memx::libc::memrchr_libc(&line_bytes[..curr_idx], pat_byte);
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
*/

mod create_data;

fn criterion_benchmark(c: &mut Criterion) {
    let (v, pat_byte, match_cnt) = create_data::create_data_chr();
    let vv: Vec<&str> = v.iter().map(|item| item.as_str()).collect();
    //
    let n = process_std_memrchr(black_box(&vv), black_box(pat_byte));
    assert_eq!(n, match_cnt);
    let n = process_libc_memrchr(black_box(&vv), black_box(pat_byte));
    assert_eq!(n, match_cnt);
    let n = process_memchr_memrchr(black_box(&vv), black_box(pat_byte));
    assert_eq!(n, match_cnt);
    let n = process_memx_memrchr(black_box(&vv), black_box(pat_byte));
    assert_eq!(n, match_cnt);
    let n = process_memx_memrchr_basic(black_box(&vv), black_box(pat_byte));
    assert_eq!(n, match_cnt);
    #[cfg(all(any(target_arch = "x86_64", target_arch = "x86"), target_feature = "sse2"))]
    {
        let n = process_memx_memrchr_sse2(black_box(&vv), black_box(pat_byte));
        assert_eq!(n, match_cnt);
    }
    //let n = process_memx_memrchr_libc(black_box(&vv), black_box(pat_byte));
    //assert_eq!(n, match_cnt);
    memory_barrier(&vv);
    //
    c.bench_function("std_memrchr", |b| {
        b.iter(|| {
            let _r = process_std_memrchr(black_box(&vv), black_box(pat_byte));
            memory_barrier(&vv);
        })
    });
    c.bench_function("libc_memrchr", |b| {
        b.iter(|| {
            let _r = process_libc_memrchr(black_box(&vv), black_box(pat_byte));
            memory_barrier(&vv);
        })
    });
    c.bench_function("memchr_memrchr", |b| {
        b.iter(|| {
            let _r = process_memchr_memrchr(black_box(&vv), black_box(pat_byte));
            memory_barrier(&vv);
        })
    });
    c.bench_function("memx_memrchr", |b| {
        b.iter(|| {
            let _r = process_memx_memrchr(black_box(&vv), black_box(pat_byte));
            memory_barrier(&vv);
        })
    });
    c.bench_function("memx_memrchr_basic", |b| {
        b.iter(|| {
            let _r = process_memx_memrchr_basic(black_box(&vv), black_box(pat_byte));
            memory_barrier(&vv);
        })
    });
    #[cfg(all(any(target_arch = "x86_64", target_arch = "x86"), target_feature = "sse2"))]
    c.bench_function("memx_memrchr_sse2", |b| {
        b.iter(|| {
            let _r = process_memx_memrchr_sse2(black_box(&vv), black_box(pat_byte));
            memory_barrier(&vv);
        })
    });
    /*
    c.bench_function("memx_memrchr_libc", |b| {
        b.iter(|| {
            let _r = process_memx_memrchr_libc(black_box(&vv), black_box(pat_byte));
            memory_barrier(&vv);
        })
    });
    */
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
