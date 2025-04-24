#![cfg_attr(target_os = "none", no_std)]

#[cfg(target_os = "none")]
mod executor;

#[cfg(target_os = "none")]
pub mod interrupt;

#[cfg(feature = "time-driver")]
pub mod time_driver;

mod critical_section;

#[cfg(target_os = "none")]
pub use executor::*;

#[cfg(target_os = "none")]
pub use interrupt::HafInterruptHandler;

#[cfg(target_os = "none")]
pub use interrupt::{disable_arch_interrupts, enable_arch_interrupts};
