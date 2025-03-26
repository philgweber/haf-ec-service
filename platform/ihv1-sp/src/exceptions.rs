// This project is dual-licensed under Apache 2.0 and MIT terms.
// See LICENSE-APACHE and LICENSE-MIT for details.

use aarch64_cpu::registers::{Readable, ESR_EL1, FAR_EL1};

#[no_mangle]
extern "C" fn sync_exception_current(elr: u64, _spsr: u64) {
    panic!(
        "Unexpected sync_exception_current, esr={:#x}, far={:#x}, elr={:#x}",
        ESR_EL1.get(),
        FAR_EL1.get(),
        elr
    );
}

#[no_mangle]
extern "C" fn irq_current(_elr: u64, _spsr: u64) {
    panic!("Unexpected irq_current");
}

#[no_mangle]
extern "C" fn fiq_current(_elr: u64, _spsr: u64) {
    panic!("Unexpected fiq_current");
}

#[no_mangle]
extern "C" fn serr_current(_elr: u64, _spsr: u64) {
    panic!("Unexpected serr_current");
}

#[no_mangle]
extern "C" fn sync_lower(_elr: u64, _spsr: u64) {
    panic!("Unexpected sync_lower");
}

#[no_mangle]
extern "C" fn irq_lower(_elr: u64, _spsr: u64) {
    panic!("Unexpected irq_lower");
}

#[no_mangle]
extern "C" fn fiq_lower(_elr: u64, _spsr: u64) {
    panic!("Unexpected fiq_lower");
}

#[no_mangle]
extern "C" fn serr_lower(_elr: u64, _spsr: u64) {
    panic!("Unexpected serr_lower");
}
