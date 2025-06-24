#![cfg_attr(target_os = "none", no_std)]

/// Hypervisor call function codes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u64)]
#[allow(unused)]
enum HfCall {
    MailboxWritableGet = 0xff01,
    MailboxWaiterGet = 0xff02,
    InterruptEnable = 0xff03,
    InterruptGet = 0xff04,
    InterruptInject = 0xff05,
    InterruptDeactivate = 0xff08,
    InterruptReconfigure = 0xff09,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct InterruptId(pub u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(unused)]
pub enum InterruptType {
    Irq = 0,
    Fiq = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(unused)]
#[repr(u64)]
pub enum InterruptReconfigureCommand {
    TargetPe = 0,
    SecState = 1,
    Enable = 2,
}

/// Makes a hypervisor call with the specified arguments.
///
/// # Arguments
///
/// * `arg0` - First argument, passed in x0
/// * `arg1` - Second argument, passed in x1
/// * `arg2` - Third argument, passed in x2
/// * `arg3` - Fourth argument, passed in x3
///
/// # Returns
///
/// The value returned by the hypervisor in x0
#[cfg(all(target_os = "none", target_arch = "aarch64"))]
#[allow(unused_assignments)]
fn hf_call(arg0: HfCall, arg1: u64, arg2: u64, arg3: u64) -> i64 {
    let mut r0 = arg0 as u64;
    let mut r1 = arg1;
    let mut r2 = arg2;
    let mut r3 = arg3;

    unsafe {
        core::arch::asm!(
            "hvc #0",
            inout("x0") r0,
            inout("x1") r1,
            inout("x2") r2,
            inout("x3") r3,
            // clobber x4-x7
            out("x4") _,
            out("x5") _,
            out("x6") _,
            out("x7") _,
        );
    }

    r0 as i64
}

#[cfg(not(all(target_os = "none", target_arch = "aarch64")))]
fn hf_call(arg0: HfCall, arg1: u64, arg2: u64, arg3: u64) -> i64 {
    println!("hf_call: {:?} - {} - {} - {}", arg0, arg1, arg2, arg3);
    0
}

pub fn hf_interrupt_set(intid: InterruptId, int_type: InterruptType, enable: bool) -> Result<(), i64> {
    let result = hf_call(HfCall::InterruptEnable, intid.0 as u64, enable as u64, int_type as u64);
    match result {
        0 => {
            log::debug!("hf_interrupt_set: {:?} - {} - {:?}", intid, enable, int_type);
            Ok(())
        }
        _ => Err(result),
    }
}

const INVALID_ID: u32 = 0xffffffff;
/// Gets the ID of the pending interrupt (if any) and acknowledge it.
pub fn hf_interrupt_get() -> Option<InterruptId> {
    let intid = hf_call(HfCall::InterruptGet, 0, 0, 0) as u32;
    if intid == INVALID_ID {
        return None;
    }
    Some(InterruptId(intid))
}

/// Deactivate the physical interrupt.
pub fn hf_interrupt_deactivate(intid: InterruptId) -> Result<(), i64> {
    let intid = intid.0 as u64;
    let result = hf_call(HfCall::InterruptDeactivate, intid, intid, 0);
    match result {
        0 => Ok(()),
        _ => Err(result),
    }
}

pub fn hf_interrupt_reconfigure(
    intid: InterruptId,
    command: InterruptReconfigureCommand,
    value: u64,
) -> Result<(), i64> {
    let intid = intid.0 as u64;
    let result = hf_call(HfCall::InterruptReconfigure, intid, command as u64, value);
    match result {
        0 => Ok(()),
        _ => Err(result),
    }
}
