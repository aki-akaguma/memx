use super::*;

/// Generic unrolled loop for search operations in forward direction.
///
/// Use this for operations that return an index, such as `memchr`.
///
/// # Arguments
/// * `N` - Unroll count (e.g., 4, 8).
/// * `STEP` - Bytes per step (e.g., 16 for SSE2, 32 for AVX2).
/// * `buf_ptr` - Current buffer pointer.
/// * `end_ptr` - End of buffer pointer.
/// * `f` - Closure that performs the search on a single step.
#[inline(always)]
pub(crate) fn _unroll_loop<const N: usize, const STEP: usize, F>(
    mut buf_ptr: *const u8,
    end_ptr: *const u8,
    mut f: F,
) -> (Option<usize>, *const u8)
where
    F: FnMut(*const u8) -> Option<usize>,
{
    while buf_ptr.is_not_over(end_ptr, STEP * N) {
        for i in 0..N {
            if let Some(r) = f(unsafe { buf_ptr.add(STEP * i) }) {
                return (Some(r), buf_ptr);
            }
        }
        buf_ptr = unsafe { buf_ptr.add(STEP * N) };
    }
    (None, buf_ptr)
}

/// Generic unrolled loop for search operations in forward direction with prefetching.
///
/// Same as `_unroll_loop`, but calls `prefetch_read_data` at the start of each unrolled block.
#[inline(always)]
pub(crate) fn _unroll_loop_with_prefetch<const N: usize, const STEP: usize, F>(
    mut buf_ptr: *const u8,
    end_ptr: *const u8,
    mut f: F,
) -> (Option<usize>, *const u8)
where
    F: FnMut(*const u8) -> Option<usize>,
{
    while buf_ptr.is_not_over(end_ptr, STEP * N) {
        buf_ptr.prefetch_read_data();
        for i in 0..N {
            if let Some(r) = f(unsafe { buf_ptr.add(STEP * i) }) {
                return (Some(r), buf_ptr);
            }
        }
        buf_ptr = unsafe { buf_ptr.add(STEP * N) };
    }
    (None, buf_ptr)
}

/// Generic unrolled loop for comparing two buffers in forward direction.
///
/// Use this for operations like `memcmp` or `memeq`.
///
/// # Arguments
/// * `R` - The result type of the comparison (e.g., `Ordering`, `bool`).
#[inline(always)]
pub(crate) fn _unroll_loop_dual<const N: usize, const STEP: usize, R, F>(
    mut a_ptr: *const u8,
    mut b_ptr: *const u8,
    end_ptr: *const u8,
    mut f: F,
) -> (Option<R>, *const u8, *const u8)
where
    F: FnMut(*const u8, *const u8) -> Option<R>,
{
    while a_ptr.is_not_over(end_ptr, STEP * N) {
        for i in 0..N {
            if let Some(r) = f(unsafe { a_ptr.add(STEP * i) }, unsafe { b_ptr.add(STEP * i) }) {
                return (Some(r), a_ptr, b_ptr);
            }
        }
        a_ptr = unsafe { a_ptr.add(STEP * N) };
        b_ptr = unsafe { b_ptr.add(STEP * N) };
    }
    (None, a_ptr, b_ptr)
}

/// Generic unrolled loop for comparing two buffers in forward direction with prefetching.
#[inline(always)]
pub(crate) fn _unroll_loop_dual_with_prefetch<const N: usize, const STEP: usize, R, F>(
    mut a_ptr: *const u8,
    mut b_ptr: *const u8,
    end_ptr: *const u8,
    mut f: F,
) -> (Option<R>, *const u8, *const u8)
where
    F: FnMut(*const u8, *const u8) -> Option<R>,
{
    while a_ptr.is_not_over(end_ptr, STEP * N) {
        a_ptr.prefetch_read_data();
        b_ptr.prefetch_read_data();
        for i in 0..N {
            if let Some(r) = f(unsafe { a_ptr.add(STEP * i) }, unsafe { b_ptr.add(STEP * i) }) {
                return (Some(r), a_ptr, b_ptr);
            }
        }
        a_ptr = unsafe { a_ptr.add(STEP * N) };
        b_ptr = unsafe { b_ptr.add(STEP * N) };
    }
    (None, a_ptr, b_ptr)
}

/// Generic unrolled loop for actions involving two buffers (e.g., copying).
///
/// Use this for operations like `memcpy`.
#[inline(always)]
pub(crate) fn _unroll_loop_dual_action<const N: usize, const STEP: usize, F>(
    mut a_ptr: *mut u8,
    mut b_ptr: *const u8,
    end_ptr: *mut u8,
    mut f: F,
) -> (*mut u8, *const u8)
where
    F: FnMut(*mut u8, *const u8),
{
    while a_ptr.is_not_over(end_ptr, STEP * N) {
        for i in 0..N {
            f(unsafe { a_ptr.add(STEP * i) }, unsafe { b_ptr.add(STEP * i) });
        }
        a_ptr = unsafe { a_ptr.add(STEP * N) };
        b_ptr = unsafe { b_ptr.add(STEP * N) };
    }
    (a_ptr, b_ptr)
}

/// Generic unrolled loop for actions involving two buffers with prefetching.
#[inline(always)]
pub(crate) fn _unroll_loop_dual_action_with_prefetch<const N: usize, const STEP: usize, F>(
    mut a_ptr: *mut u8,
    mut b_ptr: *const u8,
    end_ptr: *mut u8,
    mut f: F,
) -> (*mut u8, *const u8)
where
    F: FnMut(*mut u8, *const u8),
{
    while a_ptr.is_not_over(end_ptr, STEP * N) {
        a_ptr.prefetch_read_data();
        b_ptr.prefetch_read_data();
        for i in 0..N {
            f(unsafe { a_ptr.add(STEP * i) }, unsafe { b_ptr.add(STEP * i) });
        }
        a_ptr = unsafe { a_ptr.add(STEP * N) };
        b_ptr = unsafe { b_ptr.add(STEP * N) };
    }
    (a_ptr, b_ptr)
}

/// Generic unrolled loop for actions involving a single buffer (e.g., filling).
///
/// Use this for operations like `memset`.
#[inline(always)]
pub(crate) fn _unroll_loop_action<const N: usize, const STEP: usize, F>(
    mut buf_ptr: *mut u8,
    end_ptr: *const u8,
    mut f: F,
) -> *mut u8
where
    F: FnMut(*mut u8),
{
    while buf_ptr.is_not_over(end_ptr, STEP * N) {
        for i in 0..N {
            f(unsafe { buf_ptr.add(STEP * i) });
        }
        buf_ptr = unsafe { buf_ptr.add(STEP * N) };
    }
    buf_ptr
}

/// Generic unrolled loop for search operations in backward direction.
///
/// Use this for operations like `memrchr`.
///
/// # Arguments
/// * `min_ptr` - The start of the buffer (lowest address).
#[inline(always)]
pub(crate) fn _unroll_loop_backward<const N: usize, const STEP: usize, F>(
    mut buf_ptr: *const u8,
    min_ptr: *const u8,
    mut f: F,
) -> (Option<usize>, *const u8)
where
    F: FnMut(*const u8) -> Option<usize>,
{
    while buf_ptr.is_not_under(min_ptr, STEP * N) {
        buf_ptr = unsafe { buf_ptr.sub(STEP * N) };
        for i in (0..N).rev() {
            if let Some(r) = f(unsafe { buf_ptr.add(STEP * i) }) {
                return (Some(r), buf_ptr);
            }
        }
    }
    (None, buf_ptr)
}

/// Generic unrolled loop for search operations in backward direction with prefetching.
#[inline(always)]
pub(crate) fn _unroll_loop_backward_with_prefetch<const N: usize, const STEP: usize, F>(
    mut buf_ptr: *const u8,
    min_ptr: *const u8,
    mut f: F,
) -> (Option<usize>, *const u8)
where
    F: FnMut(*const u8) -> Option<usize>,
{
    while buf_ptr.is_not_under(min_ptr, STEP * N) {
        buf_ptr = unsafe { buf_ptr.sub(STEP * N) };
        buf_ptr.prefetch_read_data();
        for i in (0..N).rev() {
            if let Some(r) = f(unsafe { buf_ptr.add(STEP * i) }) {
                return (Some(r), buf_ptr);
            }
        }
    }
    (None, buf_ptr)
}
