use criterion::{black_box, criterion_group, criterion_main, Criterion};

// ref.) https://en.wikipedia.org/wiki/Memory_ordering
fn memory_barrier(_arg: &Vec<&str>) {
    #[cfg(target_feature = "sse2")]
    {
        #[cfg(target_arch = "x86")]
        use std::arch::x86 as mmx;
        #[cfg(target_arch = "x86_64")]
        use std::arch::x86_64 as mmx;

        unsafe { mmx::_mm_mfence() };
    }
    /*
    #[cfg(target_arch = "arm")]
    {
        unsafe { core::arch::arm::__dmb(_arg) };
    }
    #[cfg(target_arch = "aarch64")]
    {
        unsafe { core::arch::aarch64::__dmb(_arg) };
    }
    */
}

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
        /*
        for i in 0..line_len {
            if line_bytes[i] == pat_byte {
                found += 1;
            }
        }
        */
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

mod create_data;

fn criterion_benchmark(c: &mut Criterion) {
    let (v, pat_byte, match_cnt) = create_data::create_data_chr();
    let vv: Vec<&str> = v.iter().map(|item| item.as_str()).collect();
    //
    let n = process_std_memrchr(black_box(&vv), black_box(pat_byte));
    assert_eq!(n, match_cnt);
    let n = process_memx_memrchr(black_box(&vv), black_box(pat_byte));
    assert_eq!(n, match_cnt);
    let n = process_memx_memrchr_basic(black_box(&vv), black_box(pat_byte));
    assert_eq!(n, match_cnt);
    let n = process_memx_memrchr_libc(black_box(&vv), black_box(pat_byte));
    assert_eq!(n, match_cnt);
    let n = process_memchr_memrchr(black_box(&vv), black_box(pat_byte));
    assert_eq!(n, match_cnt);
    memory_barrier(&vv);
    //
    c.bench_function("std_memrchr", |b| {
        b.iter(|| {
            let _r = process_std_memrchr(black_box(&vv), black_box(pat_byte));
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
    c.bench_function("memx_memrchr_libc", |b| {
        b.iter(|| {
            let _r = process_memx_memrchr_libc(black_box(&vv), black_box(pat_byte));
            memory_barrier(&vv);
        })
    });
    c.bench_function("memchr_memrchr", |b| {
        b.iter(|| {
            let _r = process_memchr_memrchr(black_box(&vv), black_box(pat_byte));
            memory_barrier(&vv);
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
