//! Initialization code ("crt0") written in Rust
//!
//! This is for bare metal systems where there is no ELF loader or OS to take
//! care of initializing RAM for the program.
//!
//! # Example
//!
//! On the linker script side, we must assign names (symbols) to the boundaries
//! of the `.bss` and `.data` sections.
//!
//! ``` text
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
//! On the Rust side, we must bind to those symbols using an `extern` block.
//!
//! ```
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

use core::{mem, ptr, slice};

/// Initializes the `.data` section
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
pub unsafe fn init_data<T>(sdata: *mut T, edata: *mut T, sidata: *const T)
    where T: Copy
{
    let n = (edata as usize - sdata as usize) / mem::size_of::<T>();

    ptr::copy_nonoverlapping(sidata, sdata, n)
}

pub unsafe fn run_init_array(init_array_start: &extern "C" fn(),
                             init_array_end: &extern "C" fn()) {
    let n = (init_array_end as *const _ as usize -
             init_array_start as *const _ as usize) /
            mem::size_of::<extern "C" fn()>();

    for f in slice::from_raw_parts(init_array_start, n) {
        f();
    }
}

/// Zeroes the `.bss` section
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
pub unsafe fn zero_bss<T>(sbss: *mut T, ebss: *mut T)
    where T: Copy
{
    let n = (ebss as usize - sbss as usize) / mem::size_of::<T>();

    ptr::write_bytes(sbss, 0, n);
}

#[macro_export]
    macro_rules! pre_init_array {
    ($name:ident, $body:expr) => {
        #[allow(dead_code)]
        unsafe extern "C" fn $name() {
            #[link_section = ".pre_init_array"]
            #[used]
            static PRE_INIT_ARRAY_ELEMENT: unsafe extern "C" fn() = $name;

            #[inline(always)]
            fn inner() {
                $body
            }

            inner()
        }
    }
}
#[macro_export]
macro_rules! init_array {
    ($name:ident, $body:expr) => {
        #[allow(dead_code)]
        unsafe extern "C" fn $name() {
            #[link_section = ".init_array"]
            #[used]
            static INIT_ARRAY_ELEMENT: unsafe extern "C" fn() = $name;

            #[inline(always)]
            fn inner() {
                $body
            }

            inner()
        }
    }
}
