#![no_std]
#![no_main]
#![feature(asm_const)]
#![feature(error_in_core)]
#![feature(format_args_nl)]
#![feature(panic_info_message)]

#[macro_use]
extern crate cfg_if;

use core::arch::global_asm;

mod console;
mod drivers;
mod error;
mod exception;
mod panic;
mod print;

pub type Result<T> = core::result::Result<T, error::Error>;

global_asm!(
    include_str!("start.s"),
    CONST_CORE_ID_MASK = const 0b11,
    CONST_CURRENTEL_EL1 = const 0x04,
);

#[no_mangle]
#[link_section = ".text._start_arguments"]
pub static BOOT_CORE_ID: u64 = 0;

#[no_mangle]
pub extern "C" fn sp_main(_sp_params: u64) -> ! {
    let el = exception::ExceptionLevel::current();

    println!("Hello from {}", el);

    // We need to finish rest of FFA initialization
    panic!("Stopping in {}", el)

}
