// This project is dual-licensed under Apache 2.0 and MIT terms.
// See LICENSE-APACHE and LICENSE-MIT for details.

#![no_main]
#![no_std]
#![deny(clippy::undocumented_unsafe_blocks)]
#![deny(unsafe_op_in_unsafe_fn)]
#![feature(format_args_nl)]

extern crate haf_ec_service;

use aarch64_rt::entry;
use haf_ec_service::HafEcService;

mod exceptions;
mod panic;

entry!(main);
fn main(_arg0: u64, _arg1: u64, _arg2: u64, _arg3: u64) -> ! {
    // Program VBAR_EL1 to our exception handlers
    // Call into the haf-ec-service sp_main
    let service = HafEcService {
        rx_buffer_base: 0,
        tx_buffer_base: 0,
        rxtx_page_count: 0,
    };
    service.sp_main();
}
