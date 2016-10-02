//! Initialization code ("crt0") written in Rust

#![deny(warnings)]
#![no_std]

use core::{mem, ptr};

/// Initializes the `.data` section
///
/// # Arguments
///
/// - `sdata`. Pointer to the start of the `.data` section.
/// - `edata`. Pointer to the end of the `.data` section.
/// - `sidata`. `.data` section Load Memory Address (LMA)
/// - Use `T` to indicate the alignment of the `.data` section and its LMA.
///
/// # Safety
///
/// - Must be called exactly once
/// - `mem::size_of::<T>()` must be non-zero
/// - `edata >= sdata`
/// - The `sdata -> edata` region must not overlap with the `sidata -> ...` region
/// - `sdata`, `edata` and `sidata` must be `T` aligned.
pub unsafe fn init_data<T>(sdata: *mut T, edata: *const T, sidata: *const T)
    where T: Copy
{
    let n = (edata as usize - sdata as usize) / mem::size_of::<T>();

    ptr::copy_nonoverlapping(sidata, sdata, n)
}

/// Zeroes the `.bss` section
///
/// # Arguments
///
/// - `sbss`. Pointer to the start of the `.bss` section.
/// - `ebss`. Pointer to the end of the `.bss` section.
/// - Use `T` to indicate the alignment of the `.bss` section.
///
/// # Safety
///
/// - Must be called exactly once
/// - `mem::size_of::<T>()` must be non-zero
/// - `ebss >= sbss`
/// - `sbss` and `ebss` must be `T` aligned.
pub unsafe fn zero_bss<T>(sbss: *mut T, ebss: *const T)
    where T: Copy
{
    let n = (ebss as usize - sbss as usize) / mem::size_of::<T>();

    ptr::write_bytes(sbss, 0, n);
}
