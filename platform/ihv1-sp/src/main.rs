// This project is dual-licensed under Apache 2.0 and MIT terms.
// See LICENSE-APACHE and LICENSE-MIT for details.

#![cfg_attr(feature = "baremetal", no_std)]
#![cfg_attr(feature = "baremetal", no_main)]
#![deny(clippy::undocumented_unsafe_blocks)]
#![deny(unsafe_op_in_unsafe_fn)]

#[cfg(feature = "baremetal")]
mod baremetal;

#[cfg(not(feature = "baremetal"))]
fn main() {
    println!("ihv1-sp stub");
}
