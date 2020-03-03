//! Memory initialization code ("[crt0]") written in Rust.
//!
//! This crate is meant for bare metal systems where there is no ELF loader or
//! OS to take care of initializing RAM for the program. It provides functions
//! for initializing the `.data` and `.bss` sections.
//!
//! [crt0]: https://en.wikipedia.org/wiki/Crt0
//!
//! # Initializing RAM
//!
//! On the linker script side, we must assign names (symbols) to the boundaries
//! of the `.bss` and `.data` sections. For example:
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
//! This script defines symbols `_sbss`/`_ebss`, and `_sdata`/`_edata` to point
//! at the boundaries of the `.bss` and `.data` sections in RAM, respectively.
//! The `AT > FLASH` directive places the actual contents of the `.data` section
//! in the `FLASH` memory region (which needs to be defined separately from this
//! linker script snippet). Then `_sidata` is set to the address of that data in
//! flash.
//!
//! Note that while `_sbss`, `_ebss`, `_sdata` and `_edata` are Virtual Memory
//! Addresses (VMAs), `_sidata` is the Load Memory Address (LMA) of the `.data`
//! section.
//!
//! On the Rust side, we must bind to those symbols using an `extern` block,
//! and can then call into this crate to perform RAM initialization:
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
//!
//! # Minimum Supported Rust Version (MSRV)
//!
//! The MSRV of this release is Rust 1.31.0

#![deny(warnings)]
#![no_std]
#![doc(html_root_url = "https://docs.rs/r0/1.0.0")]

#[cfg(test)]
mod test;

use core::{mem, ptr};

mod sealed {
    pub trait Sealed {}
}

/// Trait for machine word types.
///
/// This trait is implemented by unsigned integers representing common machine
/// word sizes. It can not be implemented by the user.
///
/// Types implementing this trait can be used by the [`init_data`] and
/// [`zero_bss`] functions. For that to be sound, all bit patterns need to be
/// valid for the type, the type must implement `Copy`, and the type must not
/// be zero-sized.
///
/// [`init_data`]: fn.init_data.html
/// [`zero_bss`]: fn.zero_bss.html
pub unsafe trait Word: sealed::Sealed + Copy {}

impl sealed::Sealed for u8 {}
impl sealed::Sealed for u16 {}
impl sealed::Sealed for u32 {}
impl sealed::Sealed for u64 {}
impl sealed::Sealed for u128 {}

unsafe impl Word for u8 {}
unsafe impl Word for u16 {}
unsafe impl Word for u32 {}
unsafe impl Word for u64 {}
unsafe impl Word for u128 {}

/// Initializes the `.data` section by copying it from the location indicated
/// by `sidata`.
///
/// # Arguments
///
/// - `sdata`: Pointer to the start of the `.data` section in RAM.
/// - `edata`: Pointer to the open/non-inclusive end of the `.data` section in
///   RAM (the value behind this pointer will not be modified).
/// - `sidata`: `.data` section Load Memory Address (LMA). Data will be copied
///   from here.
/// - Use `T` to indicate the alignment of the `.data` section and its LMA.
///
/// # Safety
///
/// - Must be called exactly once, before the application has started.
/// - `edata >= sdata`.
/// - The `sdata -> edata` region must not overlap with the `sidata -> ...`
///   region.
/// - `sdata`, `edata` and `sidata` must be `T` aligned.
pub unsafe fn init_data<T>(mut sdata: *mut T, edata: *mut T, mut sidata: *const T)
where
    T: Word,
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
/// - `sbss`: Pointer to the start of the `.bss` section in RAM.
/// - `ebss`: Pointer to the open/non-inclusive end of the `.bss` section in
///   RAM (the value behind this pointer will not be modified).
/// - Use `T` to indicate the alignment of the `.bss` section.
///
/// # Safety
///
/// - Must be called exactly once, before the application has started.
/// - `ebss >= sbss`.
/// - `sbss` and `ebss` must be `T` aligned.
pub unsafe fn zero_bss<T>(mut sbss: *mut T, ebss: *mut T)
where
    T: Word,
{
    while sbss < ebss {
        // NOTE(volatile) to prevent this from being transformed into `memclr`
        ptr::write_volatile(sbss, mem::zeroed());
        sbss = sbss.offset(1);
    }
}
