//! Initialization code ("crt0") written in Rust.
//!
//! This is for bare metal systems where there is no ELF loader or OS to take
//! care of initializing RAM for the program.
//!
//! # Initializing RAM
//!
//! On the linker script side, we must assign names (symbols) to the boundaries
//! of the `.bss` and `.data` sections:
//!
//! ```text
//! .bss : ALIGN(4)
//! {
//!     _sbss = .;
//!     *(.bss.*);
//!     _ebss = ALIGN(4);
//! } > RAM
//!
//! .data : ALIGN(4)
//! {
//!     _sdata = .;
//!     *(.data.*);
//!     _edata = ALIGN(4);
//! } > RAM AT > FLASH
//!
//! _sidata = LOADADDR(.data);
//! ```
//!
//! On the Rust side, we must bind to those symbols using an `extern` block:
//!
//! ```no_run
//! # use r0::{zero_bss, init_data};
//! unsafe fn before_main() {
//!     // The type, `u32`, indicates that the memory is 4-byte aligned
//!     extern "C" {
//!         static mut _sbss: u32;
//!         static mut _ebss: u32;
//!
//!         static mut _sdata: u32;
//!         static mut _edata: u32;
//!
//!         static _sidata: u32;
//!     }
//!
//!     zero_bss(&mut _sbss, &mut _ebss);
//!     init_data(&mut _sdata, &mut _edata, &_sidata);
//! }
//! ```

#![deny(warnings)]
#![no_std]

#[cfg(test)]
mod test;

use core::{mem, ptr};

/// Initializes the `.data` section.
///
/// # Arguments
///
/// - `sdata`. Pointer to the start of the `.data` section.
/// - `edata`. Pointer to the open/non-inclusive end of the `.data` section.
///   (The value behind this pointer will not be modified)
/// - `sidata`. `.data` section Load Memory Address (LMA)
/// - Use `T` to indicate the alignment of the `.data` section and its LMA.
///
/// # Safety
///
/// - Must be called exactly once
/// - `mem::size_of::<T>()` must be non-zero
/// - `edata >= sdata`
/// - The `sdata -> edata` region must not overlap with the `sidata -> ...`
///   region
/// - `sdata`, `edata` and `sidata` must be `T` aligned.
pub unsafe fn init_data<T>(
    mut sdata: *mut T,
    edata: *mut T,
    mut sidata: *const T,
) where
    T: Copy,
{
    while sdata < edata {
        ptr::write(sdata, ptr::read(sidata));
        sdata = sdata.offset(1);
        sidata = sidata.offset(1);
    }
}

/// Zeroes the `.bss` section.
///
/// # Arguments
///
/// - `sbss`. Pointer to the start of the `.bss` section.
/// - `ebss`. Pointer to the open/non-inclusive end of the `.bss` section.
///   (The value behind this pointer will not be modified)
/// - Use `T` to indicate the alignment of the `.bss` section.
///
/// # Safety
///
/// - Must be called exactly once
/// - `mem::size_of::<T>()` must be non-zero
/// - `ebss >= sbss`
/// - `sbss` and `ebss` must be `T` aligned.
pub unsafe fn zero_bss<T>(mut sbss: *mut T, ebss: *mut T)
where
    T: Copy,
{
    while sbss < ebss {
        // NOTE(volatile) to prevent this from being transformed into `memclr`
        ptr::write_volatile(sbss, mem::zeroed());
        sbss = sbss.offset(1);
    }
}
