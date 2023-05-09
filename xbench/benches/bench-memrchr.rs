#![cfg(not(tarpaulin_include))]
use criterion::{black_box, criterion_group, criterion_main, Criterion};

mod barrier;
use barrier::cache_line_flush;

#[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
cpufeatures::new!(cpuid_avx2, "avx2");

#[inline(always)]
pub fn std_memrchr_impl(buf: &[u8], by1: u8) -> Option<usize> {
    buf.iter().rposition(|&x| x == by1)
}

#[inline(never)]
fn statistics_std_memrchr(texts: &[&str], pat_byte: u8) -> std::collections::HashMap<usize, usize> {
    #[inline(never)]
    fn _t_(buf: &[u8], by1: u8) -> Option<usize> {
        std_memrchr_impl(buf, by1)
    }
    //
    use std::collections::HashMap;
    let mut founded: HashMap<usize, usize> = HashMap::new();
    for line in texts {
        let line_bytes = line.as_bytes();
        let line_len = line_bytes.len();
        let mut curr_idx = line_len;
        while curr_idx > 0 {
            let r = _t_(&line_bytes[..curr_idx], pat_byte);
            if let Some(pos) = r {
                let rpos = curr_idx - pos - 1;
                if let Some(x) = founded.get_mut(&rpos) {
                    *x += 1;
                } else {
                    founded.insert(rpos, 0);
                }
                curr_idx = pos;
            } else {
                break;
            }
        }
    }
    founded
}

#[allow(dead_code)]
#[inline(never)]
fn print_statistics_std_memrchr(texts: &[&str], pat_byte: u8) {
    let map = statistics_std_memrchr(texts, pat_byte);
    let mut vec: Vec<usize> = map.clone().into_keys().collect();
    vec.sort_unstable();
    println!("Statistics:");
    for key in vec {
        println!("{key} => {},", map[&key]);
    }
    println!("");
}

#[inline(never)]
fn process_std_memrchr(texts: &[&str], pat_byte: u8) -> usize {
    #[inline(never)]
    fn _t_(buf: &[u8], by1: u8) -> Option<usize> {
        std_memrchr_impl(buf, by1)
    }
    //
    let mut found: usize = 0;
    for line in texts {
        let line_bytes = line.as_bytes();
        let line_len = line_bytes.len();
        let mut curr_idx = line_len;
        while curr_idx > 0 {
            let r = _t_(&line_bytes[..curr_idx], pat_byte);
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
fn process_libc_memrchr(texts: &[&str], pat_byte: u8) -> usize {
    // original libc function
    extern "C" {
        fn memrchr(cx: *const u8, c: i32, n: usize) -> *const u8;
    }
    #[inline(always)]
    fn _x_libc_memrchr(buf: &[u8], by1: u8) -> Option<usize> {
        let cx = buf.as_ptr();
        let len = buf.len();
        let r = unsafe { memrchr(cx, by1.into(), len) };
        if !r.is_null() {
            Some(r as usize - cx as usize)
        } else {
            None
        }
    }
    #[inline(never)]
    fn _t_(buf: &[u8], by1: u8) -> Option<usize> {
        _x_libc_memrchr(buf, by1)
    }
    //
    let mut found: usize = 0;
    for line in texts {
        let line_bytes = line.as_bytes();
        let line_len = line_bytes.len();
        let mut curr_idx = line_len;
        while curr_idx > 0 {
            let r = _t_(&line_bytes[..curr_idx], pat_byte);
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
    #[inline(never)]
    fn _t_(buf: &[u8], by1: u8) -> Option<usize> {
        memchr::memrchr(by1, buf)
    }
    //
    let mut found: usize = 0;
    for line in texts {
        let line_bytes = line.as_bytes();
        let line_len = line_bytes.len();
        let mut curr_idx = line_len;
        while curr_idx > 0 {
            let r = _t_(&line_bytes[..curr_idx], pat_byte);
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
    #[inline(never)]
    fn _t_(buf: &[u8], by1: u8) -> Option<usize> {
        memx::memrchr(buf, by1)
    }
    //
    let mut found: usize = 0;
    for line in texts {
        let line_bytes = line.as_bytes();
        let line_len = line_bytes.len();
        let mut curr_idx = line_len;
        while curr_idx > 0 {
            let r = _t_(&line_bytes[..curr_idx], pat_byte);
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
    #[inline(never)]
    fn _t_(buf: &[u8], by1: u8) -> Option<usize> {
        memx::mem::memrchr_basic(buf, by1)
    }
    //
    let mut found: usize = 0;
    for line in texts {
        let line_bytes = line.as_bytes();
        let line_len = line_bytes.len();
        let mut curr_idx = line_len;
        while curr_idx > 0 {
            let r = _t_(&line_bytes[..curr_idx], pat_byte);
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
fn process_memx_memrchr_sse2(texts: &[&str], pat_byte: u8) -> usize {
    #[inline(never)]
    fn _t_(buf: &[u8], by1: u8) -> Option<usize> {
        let needle = memx::B1Sgl::new(by1);
        unsafe { memx::arch::x86::_memrchr_sgl_sse2(buf, needle) }
    }
    //
    let mut found: usize = 0;
    for line in texts {
        let line_bytes = line.as_bytes();
        let line_len = line_bytes.len();
        let mut curr_idx = line_len;
        while curr_idx > 0 {
            let r = _t_(&line_bytes[..curr_idx], pat_byte);
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
fn process_memx_memrchr_avx2(texts: &[&str], pat_byte: u8) -> usize {
    #[inline(never)]
    fn _t_(buf: &[u8], by1: u8) -> Option<usize> {
        let needle = memx::B1Sgl::new(by1);
        unsafe { memx::arch::x86::_memrchr_sgl_avx2(buf, needle) }
    }
    //
    let mut found: usize = 0;
    for line in texts {
        let line_bytes = line.as_bytes();
        let line_len = line_bytes.len();
        let mut curr_idx = line_len;
        while curr_idx > 0 {
            let r = _t_(&line_bytes[..curr_idx], pat_byte);
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
fn cache_flush(texts: &[&str]) {
    for i in 0..texts.len() {
        cache_line_flush(texts[i].as_bytes());
    }
}

mod create_data;

fn criterion_benchmark(cr: &mut Criterion) {
    let (v, pat_byte, match_cnt) = create_data::create_data_chr();
    let vv: Vec<&str> = v.iter().map(|item| item.as_str()).collect();
    //
    if let Ok(_val) = std::env::var("AKI_TEST_DAT_CHECK") {
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
        #[cfg(all(
            any(target_arch = "x86_64", target_arch = "x86"),
            target_feature = "sse2"
        ))]
        {
            let n = process_memx_memrchr_sse2(black_box(&vv), black_box(pat_byte));
            assert_eq!(n, match_cnt);
        }
        #[cfg(all(
            any(target_arch = "x86_64", target_arch = "x86"),
            target_feature = "sse2"
        ))]
        if cpuid_avx2::get() {
            let n = process_memx_memrchr_avx2(black_box(&vv), black_box(pat_byte));
            assert_eq!(n, match_cnt);
        }
        std::process::exit(0);
    }
    //
    if let Ok(_val) = std::env::var("AKI_TEST_STATISTICS") {
        print_statistics_std_memrchr(black_box(&vv), black_box(pat_byte));
        std::process::exit(0);
    }
    //
    cache_flush(&vv);
    //
    cr.bench_function("std_memrchr", |b| {
        b.iter(|| {
            let _r = process_std_memrchr(black_box(&vv), black_box(pat_byte));
        })
    });
    cache_flush(&vv);
    cr.bench_function("libc_memrchr", |b| {
        b.iter(|| {
            let _r = process_libc_memrchr(black_box(&vv), black_box(pat_byte));
        })
    });
    cache_flush(&vv);
    cr.bench_function("memchr_memrchr", |b| {
        b.iter(|| {
            let _r = process_memchr_memrchr(black_box(&vv), black_box(pat_byte));
        })
    });
    cache_flush(&vv);
    cr.bench_function("memx_memrchr", |b| {
        b.iter(|| {
            let _r = process_memx_memrchr(black_box(&vv), black_box(pat_byte));
        })
    });
    cache_flush(&vv);
    cr.bench_function("memx_memrchr_basic", |b| {
        b.iter(|| {
            let _r = process_memx_memrchr_basic(black_box(&vv), black_box(pat_byte));
        })
    });
    cache_flush(&vv);
    #[cfg(all(
        any(target_arch = "x86_64", target_arch = "x86"),
        target_feature = "sse2"
    ))]
    cr.bench_function("memx_memrchr_sse2", |b| {
        b.iter(|| {
            let _r = process_memx_memrchr_sse2(black_box(&vv), black_box(pat_byte));
        })
    });
    cache_flush(&vv);
    #[cfg(all(
        any(target_arch = "x86_64", target_arch = "x86"),
        target_feature = "sse2"
    ))]
    if cpuid_avx2::get() {
        cr.bench_function("memx_memrchr_avx2", |b| {
            b.iter(|| {
                let _r = process_memx_memrchr_avx2(black_box(&vv), black_box(pat_byte));
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
