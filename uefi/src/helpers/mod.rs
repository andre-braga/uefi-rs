//! This module provides miscellaneous opinionated but optional helpers to
//! better integrate your application with the Rust runtime and the Rust
//! ecosystem.
//!
//! For now, this includes:
//! - using [`uefi::allocator::Allocator`] as global allocator (feature `global_allocator`)
//! - an implementation of  [`log::Log`] (feature `logger`) which logs to
//!   the stdout text protocol of UEFI (as long as boot services were not
//!   excited) and to the [debugcon device](https://phip1611.de/blog/how-to-use-qemus-debugcon-feature/)
//!   (only on x86)  (feature `log-debugcon`).
//! - [`print!`][print_macro] and [`println!`][println_macro] macros defaulting
//!   to the uefi boot service stdout stream
//! - default panic handler (feature `panic_handler`)
//!
//! **PLEASE NOTE** that these helpers are meant for the pre exit boot service
//! epoch.
//!
//! [print_macro]: uefi::print!
//! [println_macro]: uefi::println!

use crate::prelude::{Boot, SystemTable};
use crate::{table, Result};
#[doc(hidden)]
pub use println::_print;

#[cfg(feature = "global_allocator")]
mod global_allocator;
#[cfg(feature = "logger")]
mod logger;
#[cfg(feature = "panic_handler")]
mod panic_handler;
mod println;

/// Obtains a pointer to the system table.
///
/// This is meant to be used by higher-level libraries,
/// which want a convenient way to access the system table singleton.
///
/// `init` must have been called first by the UEFI app.
///
/// The returned pointer is only valid until boot services are exited.
#[must_use]
// TODO do we want to keep this public?
pub fn system_table() -> SystemTable<Boot> {
    table::system_table_boot().expect("boot services are not active")
}

/// Initialize all helpers defined in [`uefi::helpers`] whose Cargo features
/// are activated.
///
/// This must be called as early as possible, before trying to use logging or
/// memory allocation capabilities.
///
/// **PLEASE NOTE** that these helpers are meant for the pre exit boot service
/// epoch. Limited functionality might work after exiting them, such as logging
/// to the debugcon device.
#[allow(unused_variables)] // `st` is unused if logger and allocator are disabled
pub fn init(st: &mut SystemTable<Boot>) -> Result<()> {
    // Setup logging and memory allocation

    #[cfg(feature = "logger")]
    unsafe {
        logger::init(st);
    }

    #[cfg(feature = "global_allocator")]
    unsafe {
        crate::allocator::init(st);
    }

    Ok(())
}

pub(crate) fn exit() {
    #[cfg(feature = "logger")]
    logger::disable();

    #[cfg(feature = "global_allocator")]
    crate::allocator::exit_boot_services();
}
