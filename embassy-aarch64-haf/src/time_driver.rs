use aarch64_cpu::registers::{CNTFRQ_EL0, CNTP_CTL_EL0, CNTP_CVAL_EL0, CNTP_TVAL_EL0, Readable, Writeable};
use core::cell::RefCell;
use embassy_sync::blocking_mutex::Mutex;
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_time_driver::Driver;
use embassy_time_queue_utils::Queue;
use hafnium::{InterruptId, InterruptType, hf_interrupt_set};
use log::info;

struct AArch64HafniumDriver {
    queue: Mutex<CriticalSectionRawMutex, RefCell<Queue>>,
}

impl Driver for AArch64HafniumDriver {
    fn now(&self) -> u64 {
        // Flush the instruction pipeline so that instructions are fetched from memory
        unsafe {
            core::arch::asm!("isb", options(nomem, nostack));
        }
        log::info!("now() - reading CNTP_TVAL_EL0");
        let value = CNTP_TVAL_EL0.get();
        log::info!("now() - CNTP_TVAL_EL0: {}", value);
        value
    }

    fn schedule_wake(&self, at: u64, waker: &core::task::Waker) {
        log::info!("schedule_wake() - at: {}", at);
        critical_section::with(|cs| {
            let mut queue = self.queue.borrow(cs).borrow_mut();

            if queue.schedule_wake(at, waker) {
                loop {
                    if self.set_alarm(queue.next_expiration(self.now())) {
                        break;
                    }
                }
            }
        })
    }
}

// https://hafnium.readthedocs.io/en/latest/secure-partition-manager/secure-partition-manager.html#support-for-arch-timer-and-system-counter
impl AArch64HafniumDriver {
    fn set_alarm(&self, next: u64) -> bool {
        let now = self.now();
        log::info!("set_alarm() - next: {}, now: {}", next, now);

        // If the requested expiration time has already passed, return false
        if next <= now {
            return false;
        }

        log::info!("set_alarm() - setting CNTP_CVAL_EL0 to {}", next);
        CNTP_CVAL_EL0.set(next);

        true
    }

    fn on_interupt(&self) {
        critical_section::with(|cs| {
            let mut next = self.queue.borrow(cs).borrow_mut().next_expiration(self.now());
            while !self.set_alarm(next) {
                next = self.queue.borrow(cs).borrow_mut().next_expiration(self.now());
            }
        });
    }
}

embassy_time_driver::time_driver_impl!(static DRIVER: AArch64HafniumDriver = AArch64HafniumDriver {
    queue: Mutex::new(RefCell::new(Queue::new())),
});

pub fn on_interupt() {
    DRIVER.on_interupt();
}

pub unsafe fn init() {
    info!("init() - reading CNTFRQ_EL0");
    let frequency = CNTFRQ_EL0.get();

    info!(
        "init() - frequency: {}, embassy tick hz: {}",
        frequency,
        embassy_time::TICK_HZ
    );

    info!("init() - setting CNTP_CTL_EL0");
    CNTP_CTL_EL0.write(CNTP_CTL_EL0::ENABLE::SET + CNTP_CTL_EL0::IMASK::CLEAR);

    info!("init() - enabling virtual timer interrupt");
    hf_interrupt_set(InterruptId(3), InterruptType::Irq, true)
        .expect("init() - failed to enable virtual timer interrupt");

    info!("init() - done");
}
