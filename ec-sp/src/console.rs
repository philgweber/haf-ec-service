use crate::drivers::uart::uarthyp;
use core::fmt;

/// Return a reference to the console.
///
/// This is the global console used by all printing macros.
pub fn console() -> impl fmt::Write {
    uarthyp::HypUart::new()
}
