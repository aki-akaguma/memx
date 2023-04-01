#![doc(hidden)]

#[cfg(not(feature = "test_pointer_width"))]
#[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
pub mod x86;
