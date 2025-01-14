#![no_std]
#![cfg_attr(reexport_core, forbid(unsafe_code))]

#[cfg(reexport_core)]
pub use core::sync::atomic::*;

#[cfg(not(reexport_core))]
mod polyfill;
#[cfg(not(reexport_core))]
pub use polyfill::*;
